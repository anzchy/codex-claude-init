// Health check polling logic
// Feature: 001-tauri-sidecar-shell

use crate::error::StartupError;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Result of a single health check attempt
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// HTTP status code (200 = success)
    pub status_code: Option<u16>,

    /// Response time in milliseconds
    pub latency_ms: u64,

    /// Timestamp of the check
    pub timestamp: Instant,

    /// Error message if check failed
    pub error: Option<String>,
}

impl HealthCheckResult {
    pub fn is_healthy(&self) -> bool {
        self.status_code == Some(200)
    }
}

/// Health check configuration
pub struct HealthCheckConfig {
    /// URL to poll
    pub url: String,

    /// Time between poll attempts
    pub poll_interval: Duration,

    /// Maximum time to wait for healthy response
    pub timeout: Duration,

    /// Timeout for each individual HTTP request
    pub request_timeout: Duration,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:8123/".to_string(),
            poll_interval: Duration::from_millis(500),
            timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(3),
        }
    }
}

/// Poll the sidecar health endpoint until healthy or timeout
pub async fn poll_health_check(config: &HealthCheckConfig) -> Result<(), StartupError> {
    let client = reqwest::Client::builder()
        .timeout(config.request_timeout)
        .build()
        .map_err(|e| StartupError::SidecarSpawnFailed(e.to_string()))?;

    let start_time = Instant::now();
    let max_attempts = (config.timeout.as_millis() / config.poll_interval.as_millis()) as u32;

    info!(
        "Starting health check polling (max {} attempts over {:?})",
        max_attempts, config.timeout
    );

    for attempt in 1..=max_attempts {
        let check_start = Instant::now();

        match client.get(&config.url).send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let latency = check_start.elapsed().as_millis() as u64;

                let result = HealthCheckResult {
                    status_code: Some(status),
                    latency_ms: latency,
                    timestamp: check_start,
                    error: None,
                };

                if result.is_healthy() {
                    info!(
                        "Health check passed on attempt {} ({}ms)",
                        attempt, latency
                    );
                    return Ok(());
                } else {
                    debug!(
                        "Health check attempt {}: status {} ({}ms)",
                        attempt, status, latency
                    );
                }
            }
            Err(e) => {
                let latency = check_start.elapsed().as_millis() as u64;
                debug!(
                    "Health check attempt {}: error {} ({}ms)",
                    attempt, e, latency
                );
            }
        }

        // Check if we've exceeded total timeout
        if start_time.elapsed() >= config.timeout {
            warn!("Health check timeout after {:?}", start_time.elapsed());
            return Err(StartupError::HealthCheckTimeout);
        }

        // Wait before next attempt (unless it's the last one)
        if attempt < max_attempts {
            tokio::time::sleep(config.poll_interval).await;
        }
    }

    Err(StartupError::HealthCheckTimeout)
}

/// Check if port is available (not in use)
pub fn is_port_available(port: u16) -> bool {
    std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}
