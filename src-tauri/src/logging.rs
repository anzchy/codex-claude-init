// Logging infrastructure (tracing + tracing-oslog + file appender)
// Feature: 001-tauri-sidecar-shell
// FR-012: Dual logging to file and macOS unified logging

use tracing_subscriber::prelude::*;

/// Initialize the logging infrastructure
/// Outputs to both:
/// 1. macOS unified logging (Console.app) via tracing-oslog
/// 2. Rolling log files in ~/Library/Application Support/Reader3/logs/
pub fn init_logging() {
    // Get the log directory (we'll use a default path for now,
    // since AppConfig isn't available yet at this point)
    let log_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("Reader3")
        .join("logs");

    // Create log directory if it doesn't exist
    let _ = std::fs::create_dir_all(&log_dir);

    // File appender with daily rotation
    let file_appender = tracing_appender::rolling::daily(&log_dir, "reader3.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Keep the guard alive for the lifetime of the application
    // by leaking it (this is intentional for logging)
    Box::leak(Box::new(_guard));

    // Build the subscriber with multiple layers
    let subscriber = tracing_subscriber::registry()
        // macOS unified logging (Console.app)
        .with(tracing_oslog::OsLogger::new("com.reader3.app", "default"))
        // File output
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_file(true)
                .with_line_number(true)
                .with_target(true),
        )
        // Console output for development
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_target(true)
                .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG),
        );

    // Set as the global default
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    tracing::info!("Logging initialized - file: {:?}", log_dir);
}
