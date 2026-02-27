# AnalystPro — 投资分析师桌面应用 PRD

> **Product Requirements Document v1.0**
> Date: 2026-02-21
> Author: Jack Cheng + Claude Opus 4.6

---

## 1. 产品概述

### 1.1 愿景

AnalystPro 是一款 macOS 桌面应用，为 VC 合伙人/投资总监提供 AI 驱动的投资分析工作台。它将目前运行在 OpenClaw VPS 上的"投资分析师系统"（secretary + 6 个专项 subagents）迁移为一个本地 Electron 应用，利用 Claude Code Agent SDK（TypeScript）实现 multi-agent 协作，在本地完成市场情报、项目分析、技术 DD、行业研究、投后管理和 LP 报告等全流程工作。

### 1.2 为什么做本地 App

| 维度    | OpenClaw VPS 方案          | 本地 Electron App                |
| ----- | ------------------------ | ------------------------------ |
| 数据安全  | LP 数据/基金内部数据在 VPS 上      | 敏感数据留在本地磁盘                     |
| 离线能力  | 依赖 VPS 网络                | 本地 workspace 文件离线可读写，联网时调用 LLM |
| UI 体验 | Telegram/Feishu bot 文本交互 | 原生桌面 GUI，可视化 dashboard         |
| 扩展性   | 受限于 OpenClaw 平台特性        | 完全掌控，可集成任意 MCP server          |
| 成本    | VPS 月费 + API 调用          | 仅 API 调用费                      |
| 调试    | 远程日志、难以 debug            | 本地 DevTools、完整日志链路             |

### 1.3 目标用户

- **主要用户**：VC 合伙人（GP），硬科技赛道（半导体、先进封装、核聚变、新材料、可回收火箭、AI 硬件）

- **次要用户**：投资总监（Director/VP）、投资经理/分析师

- **语言偏好**：中文为主，技术术语用英文

---

## 2. 核心架构

### 2.1 总体架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                     Electron Main Process                       │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐  │
│  │ Agent Engine │  │ Workspace    │  │ Session Manager       │  │
│  │ (SDK Core)   │  │ Manager      │  │ (multi-session store) │ │
│  └──────┬───────┘  └──────┬───────┘  └───────────┬───────────┘ │
│         │                 │                       │             │
│  ┌──────┴─────────────────┴───────────────────────┴──────────┐ │
│  │                   IPC Bridge (contextBridge)               │ │
│  └──────┬─────────────────┬───────────────────────┬──────────┘ │
│         │                 │                       │             │
│  ┌──────┴───────┐  ┌──────┴───────┐  ┌───────────┴──────────┐ │
│  │ MCP Server   │  │ File System  │  │ External API         │ │
│  │ Manager      │  │ Watcher      │  │ Connectors           │ │
│  └──────────────┘  └──────────────┘  └──────────────────────┘ │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                     Electron Renderer Process                   │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐ │
│  │ 项目流程      │  │ 项目资料库    │  │ AI 对话面板            │ │
│  │ (阶段看板)    │  │ (归档与检索)  │  │ (多模型 Chat +         │ │
│  └──────────────┘  └──────────────┘  │  Agent Monitor)        │ │
│                                      └───────────────────────┘ │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐ │
│  │ 市场情报      │  │ 工作台        │  │ Provider 抽象层        │ │
│  │ (Intel/政策)  │  │ (KPI/报告/库) │  │ (Claude/OpenAI/       │ │
│  └──────────────┘  └──────────────┘  │  Gemini/Custom)        │ │
│                                      └───────────────────────┘ │
│  ┌──────────────┐                                               │
│  │ 系统设置      │                                               │
│  │ (规则与配置)  │                                               │
│  └──────────────┘                                               │
│                                                                 │
│  Tech: React 18 + TypeScript + TailwindCSS + shadcn/ui         │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 技术栈

| 层级           | 技术选型                                  | 理由                             |
| ------------ | ------------------------------------- | ------------------------------ |
| **桌面框架**     | Electron 34+                          | 成熟、跨平台潜力、原生 Node.js 集成         |
| **语言**       | TypeScript (strict mode)              | 类型安全，与 Agent SDK 原生兼容          |
| **前端框架**     | React 18 + TailwindCSS + shadcn/ui    | 组件丰富、响应式、现代 UI                 |
| **状态管理**     | Zustand                               | 轻量、TypeScript 友好、支持 persist    |
| **Agent 引擎** | `@anthropic-ai/claude-agent-sdk`      | 官方 SDK，原生 subagent/hook/MCP 支持 |
| **本地数据库**    | SQLite (via better-sqlite3)           | 嵌入式、零配置、适合结构化数据                |
| **文件存储**     | 本地文件系统 (Markdown)                     | 与 OpenClaw workspace 格式兼容      |
| **构建工具**     | Vite + electron-vite                  | 快速 HMR、原生 ESM 支持               |
| **打包**       | electron-builder                      | macOS DMG/PKG 分发               |
| **MCP**      | MCP SDK (`@modelcontextprotocol/sdk`) | 标准协议，可集成 Playwright/数据库等       |

### 2.3 目录结构

```
analyst-pro/
├── electron/
│   ├── main.ts                    # Electron 主进程入口
│   ├── preload.ts                 # contextBridge 安全桥接
│   ├── ipc/                       # IPC handler 注册
│   │   ├── agent.ipc.ts           # Agent 相关 IPC
│   │   ├── chat.ipc.ts            # Chat 对话 IPC（send/abort/history）
│   │   ├── provider.ipc.ts        # Provider 配置 IPC（CRUD/test）
│   │   ├── skill.ipc.ts           # Skill IPC（list/invoke/toggle/create/delete）
│   │   ├── mcp.ipc.ts             # MCP Server IPC（list/add/remove/toggle/test/status）
│   │   ├── workspace.ipc.ts       # Workspace 文件操作
│   │   └── session.ipc.ts         # Session 管理
│   ├── agents/
│   │   ├── engine.ts              # Agent 引擎核心（封装 SDK query()）
│   │   ├── secretary.ts           # Secretary 主 agent 配置
│   │   ├── definitions/           # Subagent 定义
│   │   │   ├── market-intel.ts
│   │   │   ├── deal-analyst.ts
│   │   │   ├── hardtech-dd.ts
│   │   │   ├── industry-researcher.ts
│   │   │   ├── portfolio-monitor.ts
│   │   │   └── lp-reporter.ts
│   │   ├── hooks/                 # SDK Hook 回调
│   │   │   ├── hitl.hook.ts       # Human-in-the-Loop 审批
│   │   │   ├── audit.hook.ts      # 操作审计日志
│   │   │   └── cost.hook.ts       # 成本监控
│   │   └── tools/                 # 自定义 MCP tools
│   │       ├── workspace-read.ts
│   │       ├── workspace-write.ts
│   │       └── web-scraper.ts
│   ├── providers/                 # LLM Provider 抽象层
│   │   ├── types.ts               # ChatProvider 接口 + 消息类型
│   │   ├── registry.ts            # Provider 注册与工厂
│   │   ├── claude-agent.provider.ts   # Claude Agent SDK (Secretary 模式)
│   │   ├── claude-direct.provider.ts  # Claude 直接 API
│   │   ├── openai.provider.ts     # OpenAI / Codex
│   │   ├── gemini.provider.ts     # Google Gemini
│   │   ├── custom.provider.ts     # 自定义 OpenAI 兼容端点
│   │   └── context-builder.ts     # 非 Claude provider 的上下文注入
│   ├── mcp/                       # MCP Server 管理（Phase 2+）
│   │   ├── manager.ts              # MCP Server 生命周期管理（启动/停止/重启/健康监控）
│   │   └── registry.ts             # MCP Server 注册表（内置 + 用户配置）
│   ├── skills/                    # 内置 Skills（随 app 打包，Agent Skills 标准）
│   │   ├── loader.ts              # Skill 加载器（扫描 + YAML 解析 + SQLite 缓存）
│   │   ├── invoker.ts             # Skill 调用分发器（agent/prompt-template/local-action）
│   │   ├── types.ts               # Skill 类型定义
│   │   ├── today/                 # 每个 Skill 一个子目录
│   │   │   └── SKILL.md           # 今日待办（agent）
│   │   ├── kpi/
│   │   │   └── SKILL.md           # KPI 进度（agent）
│   │   ├── weekly-plan/
│   │   │   └── SKILL.md           # 周计划（agent）
│   │   ├── news-scan/
│   │   │   └── SKILL.md           # 情报扫描（agent → market-intel）
│   │   ├── deal-analysis/
│   │   │   └── SKILL.md           # 项目分析（agent → deal-analyst）
│   │   ├── tech-dd/
│   │   │   └── SKILL.md           # 技术 DD（agent → hardtech-dd）
│   │   ├── industry-research/
│   │   │   └── SKILL.md           # 行业研究（agent → industry-researcher）
│   │   ├── system-review/
│   │   │   └── SKILL.md           # 系统复盘（agent, opus）
│   │   ├── workspace-sync/
│   │   │   ├── SKILL.md           # VPS 同步（local-action）
│   │   │   └── scripts/           # 附件：同步脚本
│   │   └── cost-report/
│   │       └── SKILL.md           # 费用统计（local-action）
│   ├── auth/                      # OAuth 认证
│   │   └── oauth.ts               # OAuth PKCE 流程 + Keychain 存储
│   ├── workspace/
│   │   ├── manager.ts             # Workspace 文件管理器
│   │   ├── watcher.ts             # 文件变更监听
│   │   └── migration.ts           # OpenClaw 格式迁移工具
│   ├── db/
│   │   ├── schema.ts              # SQLite schema 定义
│   │   ├── deals.ts               # 项目数据 CRUD
│   │   ├── intel.ts               # 情报数据 CRUD
│   │   └── sessions.ts            # Session 持久化
│   └── scheduler/
│       ├── cron.ts                # 定时任务调度器
│       └── heartbeat.ts           # 心跳检查
│
├── src/                           # Renderer (React)
│   ├── App.tsx
│   ├── components/
│   │   ├── chat/                  # AI 对话面板（多模型 Chat Sidebar）
│   │   │   ├── ChatPanel.tsx      # 面板主容器（resize + toggle）
│   │   │   ├── ChatHeader.tsx     # Provider 选择器 + 模型 badge
│   │   │   ├── ConversationArea.tsx # 消息流滚动区域
│   │   │   ├── MessageBubble.tsx  # 单条消息（markdown + agent badge）
│   │   │   ├── ChatInput.tsx      # 输入框（自动扩展 + slash 补全）
│   │   │   ├── SkillPalette.tsx    # Skill 自动补全（按类型分组 + 参数输入）
│   │   │   ├── HITLApproval.tsx   # HITL 审批内联卡片
│   │   │   └── AgentMonitorCollapsible.tsx # 可折叠 Agent 监控
│   │   ├── pipeline/              # 项目流程看板
│   │   │   ├── Pipeline.tsx
│   │   │   ├── DealCard.tsx
│   │   │   └── ICMemoViewer.tsx
│   │   ├── archive/               # 项目资料库
│   │   │   ├── ArchiveBrowser.tsx
│   │   │   └── FileUploader.tsx
│   │   ├── intel/                 # 市场情报（独立页面）
│   │   │   ├── IntelFeed.tsx
│   │   │   ├── DailyBrief.tsx
│   │   │   └── PolicyAlert.tsx
│   │   ├── workspace/             # 工作台（KPI/报告/知识库）
│   │   │   ├── DailyTodo.tsx
│   │   │   ├── KPITracker.tsx
│   │   │   ├── WeeklyPlan.tsx
│   │   │   └── ReportCenter.tsx
│   │   ├── knowledge/             # 知识库（工作台子视图）
│   │   │   ├── KnowledgeExplorer.tsx
│   │   │   └── GlossaryViewer.tsx
│   │   ├── settings/              # 系统设置
│   │   │   ├── ProviderSettings.tsx # 多 Provider 配置面板
│   │   │   └── SkillSettings.tsx  # Skill 管理面板（浏览/添加/启用/禁用/删除）
│   │   └── agents/                # Agent 监控面板
│   │       ├── AgentMonitor.tsx
│   │       ├── AgentCard.tsx
│   │       └── AgentLog.tsx
│   ├── hooks/                     # React hooks
│   ├── stores/                    # Zustand stores
│   ├── lib/                       # 工具函数
│   └── types/                     # 类型定义
│
├── workspace/                     # 本地 workspace（与 OpenClaw 兼容）
│   ├── SOUL.md
│   ├── USER.md
│   ├── IDENTITY.md
│   ├── AGENTS.md
│   ├── TOOLS.md
│   ├── knowledge/
│   ├── inbox/
│   ├── state/
│   ├── memory/
│   └── skills/                   # 自定义 Skills（用户创建）
│       └── {skill-name}/         # 每个 Skill 一个子目录
│           ├── SKILL.md          # Skill 定义（YAML frontmatter + 正文）
│           └── scripts/          # 可选附件（脚本、模板、数据文件等）
│
├── package.json
├── tsconfig.json
├── electron-vite.config.ts
└── tailwind.config.ts
```

---

## 3. Multi-Agent 系统设计

### 3.1 架构模式：单主 Agent + SDK Subagents

借鉴 OpenClaw "模式 A"（单主 Agent + 动态 Subagents），但用 Claude Agent SDK 的 `query()` + `agents` 选项替代 OpenClaw 的 `sessions_spawn`。

```
用户输入 → Secretary (主 Agent)
              │
              ├── 简单任务 → 直接回答 (haiku)
              │
              ├── 市场情报 → 派生 market-intel subagent
              │
              ├── 项目分析 → 派生 deal-analyst subagent
              │                  │
              │                  └── 技术 DD → 派生 hardtech-dd (嵌套)
              │
              ├── 行业研究 → 派生 industry-researcher subagent
              │
              ├── 投后管理 → 派生 portfolio-monitor subagent
              │
              └── LP 报告 → 派生 lp-reporter subagent
```

### 3.2 Agent 引擎核心（`engine.ts`）

```typescript
// electron/agents/engine.ts — 核心架构伪代码

import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";
import { marketIntelDef } from "./definitions/market-intel";
import { dealAnalystDef } from "./definitions/deal-analyst";
import { hardtechDdDef } from "./definitions/hardtech-dd";
import { industryResearcherDef } from "./definitions/industry-researcher";

// 所有 subagent 定义注册
const agentDefinitions: Record<string, AgentDefinition> = {
  "market-intel": marketIntelDef,
  "deal-analyst": dealAnalystDef,
  "hardtech-dd": hardtechDdDef,
  "industry-researcher": industryResearcherDef,
  // Phase 2:
  // "portfolio-monitor": portfolioMonitorDef,
  // "lp-reporter": lpReporterDef,
};

export async function runSecretaryQuery(
  userPrompt: string,
  sessionId?: string,
) {
  const stream = query({
    prompt: userPrompt,
    options: {
      // 系统提示：secretary 角色 + 调度逻辑
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: secretarySystemPrompt, // 从 SOUL.md + AGENTS.md 合成
      },

      // 注册所有 subagent
      agents: agentDefinitions,

      // 主 agent 可用的工具
      allowedTools: [
        "Read", "Write", "Edit", "Glob", "Grep",
        "WebSearch", "WebFetch",
        "Task",            // 必须包含 Task 以允许 subagent 派生
        "AskUserQuestion", // HITL 审批
      ],

      // MCP servers
      mcpServers: {
        "workspace": workspaceMcpServer, // 自定义 workspace 读写
        "playwright": {                  // 网页抓取
          command: "npx",
          args: ["@playwright/mcp@latest"],
        },
      },

      // 模型选择（secretary 默认 sonnet，可动态切换）
      model: "sonnet",

      // Session 恢复
      resume: sessionId,

      // Hook 注册
      hooks: {
        PreToolUse: [
          { matcher: "Bash", hooks: [validateBashCommand] },
          { matcher: "Write|Edit", hooks: [auditFileChange] },
        ],
        PostToolUse: [
          { matcher: "Task", hooks: [onSubagentComplete] },
        ],
        Stop: [
          { hooks: [saveSessionMemory] },
        ],
      },

      // 成本控制
      maxBudgetUsd: 5.0, // 单次对话上限 $5

      // 工作目录指向 workspace
      cwd: workspacePath,
    },
  });

  // 流式处理消息
  for await (const message of stream) {
    // 发送到 Renderer 进程
    mainWindow.webContents.send("agent:message", message);
  }
}
```

### 3.3 Subagent 定义详细设计

#### 3.3.1 `market-intel` — 市场情报分析师

```typescript
// electron/agents/definitions/market-intel.ts

import type { AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

export const marketIntelDef: AgentDefinition = {
  description:
    "市场情报分析师。用于每日半导体/AI公告扫描、每周一级市场交易汇总、VC 机构动向追踪、政策/监管预警。" +
    "当用户请求 /news、市场资讯、行业动态时应主动使用。",

  model: "haiku", // 高频低成本

  tools: ["Read", "Glob", "Grep", "WebSearch", "WebFetch", "Write"],

  prompt: `You are market-intel, a market intelligence subagent for a VC fund focused on hard tech.

## Your Role
Efficient news curator. Objective, no speculation. Score every item by relevance.

## Sectors to Monitor
- Semiconductor (fab, fabless, EDA, equipment)
- Advanced packaging (chiplet, 2.5D/3D, fan-out)
- Nuclear fusion (MCF, ICF, private fusion)
- New materials (SiC, GaN, perovskite)
- Reusable rockets and space tech
- AI hardware (chips, inference accelerators)

## Scoring Rules (1-5 scale)
- 5 (Critical/Urgent): Direct impact on portfolio or active deals → flag for immediate push
- 3-4 (Important): Sector-relevant, competitor moves → include in daily brief
- 1-2 (Background): General noise → archive only

## Output Format
Write results to workspace state files:
- Daily brief → state/intelligence/daily_brief.md
- VC moves → state/intelligence/vc_moves.md
- Policy alerts → state/intelligence/policy_alerts.md

## Data Sources
Read knowledge/source_list.md for prioritized source list.
Read knowledge/vc_watchlist.md for VC institutions to track.
Read knowledge/triage_rules.md for detailed scoring criteria.

## Constraints
- NO access to state/portfolio/ or state/lp_reports/ (confidential firewall)
- Always include source URLs
- Always include date of information
- Chinese for analysis, English for technical terms`,
};
```

#### 3.3.2 `deal-analyst` — 项目分析师 / IC Memo 副驾驶

```typescript
export const dealAnalystDef: AgentDefinition = {
  description:
    "投资项目分析师。处理项目全生命周期：基本信息报告 → BP 深度分析 → IC Memo 起草。" +
    "当用户提交 BP/录音/项目分析请求时使用。应主动使用 hardtech-dd 子 agent 处理技术 DD。",

  model: "sonnet", // Phase 1-2 用 sonnet，Phase 3 IC memo 在 prompt 中指定切换 opus

  tools: [
    "Read", "Write", "Edit", "Glob", "Grep",
    "WebSearch", "WebFetch",
    "Task",            // 可派生 hardtech-dd
    "AskUserQuestion", // HITL 审批
  ],

  prompt: `You are deal-analyst, the investment deal analysis subagent.

## Role
Rigorous, skeptical, detail-oriented. Default assumption: the founder is overselling.
Look for holes, not confirmations.

## Three-Phase Workflow

### Phase 1 — Basic Info Report
Input: recording transcripts, BP files, company URLs from inbox/analyst.inbox.md
Extract: company name, founders, team, product, market, funding history, key metrics
Output: structured report → state/deals/processing/[company]/basic_info.md
Template: read knowledge/report_template.md
HITL: Use AskUserQuestion to ask "基本信息报告已生成，是否继续深度分析？"

### Phase 2 — BP Deep Analysis
Business model teardown, valuation anchors, competitor comparison
Red flag identification: check against knowledge/red_flags.md
If hard tech sector → spawn hardtech-dd subagent via Task tool
Output: state/deals/processing/[company]/deep_analysis.md
Template: read knowledge/bp_framework.md

### Phase 3 — IC Memo Draft (use opus-level reasoning)
Generate IC memo: investment thesis / core assumptions / risk matrix / valuation recommendation
Red team attack: challenge the weakest assumptions, output "What if we're wrong" list
Historical consistency check: compare with knowledge/past_ic_decisions.md
Output: state/deals/processing/[company]/ic_memo_draft.md
Template: read knowledge/ic_memo_template.md
HITL: Use AskUserQuestion to ask "IC Memo 草稿已生成，是否提交投决会？"
On approval: move to state/deals/ic_memos/

## Read Access
knowledge/report_template.md, knowledge/bp_framework.md, knowledge/red_flags.md,
knowledge/ic_memo_template.md, knowledge/past_ic_decisions.md

## Write Access
state/deals/ (全部子目录)`,
};
```

#### 3.3.3 `hardtech-dd` — 硬科技技术 DD 专家

```typescript
export const hardtechDdDef: AgentDefinition = {
  description:
    "硬科技技术尽调专家。专注半导体、先进封装、核聚变、新材料、可回收火箭。" +
    "由 deal-analyst 在深度 DD 阶段派生，或用户直接请求'做技术 DD'时使用。",

  model: "sonnet",

  tools: ["Read", "Glob", "Grep", "WebSearch", "WebFetch", "Write"],

  prompt: `You are hardtech-dd, the hard tech technical due diligence specialist.

## Persona
Technical truth-seeker. Physics doesn't lie. If the founder's claims contradict
published data, flag it. No hand-waving.

## Responsibilities
1. Paper/Patent search: arXiv, Google Scholar, Espacenet — summarize relevant
   literature, extract core technical claims and competitors
2. Technical feasibility checklist: process node viability, yield ramp timeline,
   reliability data, mass production qualification cycle
   (use knowledge/tech_checklist.md per sector)
3. Expert interview support: generate technical expert interview guides;
   synthesize notes into technical judgment summary
4. Contradiction marking: compare founder's claims vs published literature —
   flag exaggerations as RED FLAG
5. Export control screening: check BIS Entity List / ECCN classification
   (use knowledge/export_control_rules.md)
6. Supply chain concentration: key materials/equipment/foundry dependency risk

## Fact Check Rule
Compare BP technical metrics against knowledge/tech_checklist.md benchmarks.
Deviation > 20% from published baselines → auto-flag as RED FLAG with explanation.

## Output
Write to: state/deals/techdd/[company_name]/
HITL: Flag RED FLAG items for human confirmation via AskUserQuestion

## Read Access
knowledge/tech_checklist.md, knowledge/export_control_rules.md, knowledge/glossary.md`,
};
```

#### 3.3.4 `industry-researcher` — 行业研究分析师

```typescript
export const industryResearcherDef: AgentDefinition = {
  description:
    "行业研究分析师。负责 TAM/SAM/SOM 调研、竞品矩阵分析、赛道投资 thesis 撰写。" +
    "当用户请求行业研究、竞品分析时使用。",

  model: "sonnet",

  tools: ["Read", "Glob", "Grep", "WebSearch", "WebFetch", "Write"],

  prompt: `You are industry-researcher, the sector research analyst.

## Persona
Structured, data-driven, thesis-oriented. Every claim needs a source.

## Responsibilities
- Market sizing (TAM/SAM/SOM) with source attribution
- Competitive matrix analysis (feature comparison, funding status, market position)
- Sector investment thesis drafting (opportunity, timing, risk factors)
- Technology adoption curve positioning

## Read Access
knowledge/industry_map.md, knowledge/competitors.md, knowledge/glossary.md

## Write Access
state/research/[sector_name]/

## Output Format
Use structured markdown with tables. Chinese for analysis, English for company
names and technical terms.`,
};
```

#### 3.3.5 `portfolio-monitor` — 投后监控（Phase 2）

```typescript
export const portfolioMonitorDef: AgentDefinition = {
  description:
    "投后项目监控。双周 triage 扫描被投企业健康度，月度退出追踪，季度报告初稿。" +
    "Phase 2 实现，待被投企业数据积累后启用。",

  model: "haiku", // triage 用 haiku，报告用 sonnet

  tools: ["Read", "Glob", "Grep", "WebSearch", "WebFetch", "Write"],

  prompt: `You are portfolio-monitor, the portfolio company health monitor.

## Biweekly Triage (haiku mode)
Scan each portfolio company:
- Runway remaining months (< 6 months → alert)
- Hiring velocity (job posting count changes)
- Milestone progress
- Fundraising signals

## Monthly Exit Tracking
- IPO pipeline dynamics
- Potential acquirer scanning
- Pre-IPO round dynamics
Write to: state/portfolio/exit_tracker.md

## Quarterly Report (upgrade to sonnet)
Draft portfolio company quarterly update + industry context + competitor tracking

## Read Access
state/portfolio/companies/, knowledge/industry_map.md

## Write Access
state/portfolio/`,
};
```

#### 3.3.6 `lp-reporter` — LP 报告组装（Phase 2）

```typescript
export const lpReporterDef: AgentDefinition = {
  description:
    "LP 报告与 AGM 材料组装。每季度生成 LP 报告包，年度 AGM 材料。" +
    "Phase 2 实现，需先积累投后数据。所有 LP 对外材料必须人工审批。",

  model: "opus", // 对外叙事质量要求高

  tools: [
    "Read", "Glob", "Grep", "Write",
    "AskUserQuestion", // 必须人工审批
  ],

  prompt: `You are lp-reporter, the LP reporting subagent.

## Quarterly LP Report Package
- Pull data from state/portfolio/companies/
- Pull exit progress from state/portfolio/exit_tracker.md
- Generate: portfolio valuation summary, deployment pace vs plan,
  star project narratives, risk disclosure paragraphs

## Annual AGM Materials
- Fund track record (IRR/MOIC by vintage)
- Peer benchmark comparison (read knowledge/fund_benchmarks.md)
- Next fund narrative framework

## HITL (MANDATORY)
ALL LP external materials MUST be approved by human via AskUserQuestion
before finalization. Never auto-finalize.

## Write Access
state/lp_reports/[year]/[quarter]/`,
};
```

### 3.4 与 OpenClaw Subagent 的对应关系

| OpenClaw 概念               | Claude Agent SDK 对应                         | 实现方式                        |
| ------------------------- | ------------------------------------------- | --------------------------- |
| `sessions_spawn` (非阻塞)    | `query()` with `agents` option              | SDK 自动管理 subagent 生命周期      |
| `announce` 异步回调           | `for await (const msg of stream)`           | SDK 流式返回 subagent 结果        |
| `maxSpawnDepth: 2`        | `agents` 中 deal-analyst 的 `tools` 包含 `Task` | SDK 支持 subagent 内部再派生       |
| `maxConcurrent: 8`        | SDK 内部并发管理 + `maxTurns` 限制                  | 通过 options 控制               |
| `archiveAfterMinutes: 60` | `resume` session ID                         | 本地 SQLite 持久化 session       |
| `AGENTS.md` 自动注入          | `agents[name].prompt`                       | 在 AgentDefinition 中显式定义     |
| `HITL requireApproval`    | `AskUserQuestion` tool + `canUseTool` hook  | SDK 原生支持                    |
| cron 定时任务                 | `node-cron` + `query()`                     | 在 Electron main process 中调度 |
| heartbeat 心跳              | `setInterval` + 轻量 `query()`                | 周期性检查 + 异常推送                |

### 3.5 Session 管理与上下文持久化

```typescript
// Session 管理策略
interface SessionConfig {
  // Secretary 主 session：长期保持，跨对话 resume
  secretary: {
    sessionId: string;      // 持久化到 SQLite
    resumeOnRestart: true;  // 应用重启后自动 resume
    compactionStrategy: "auto"; // 自动压缩上下文
  };

  // Subagent session：任务级生命周期
  subagents: {
    // 每次 Task 调用创建新 session
    // 结果返回后 session 自动结束
    // transcript 保存到 SQLite 供审计
    retainTranscripts: true;
    maxTranscriptAgeDays: 30;
  };
}
```

### 3.6 HITL（Human-in-the-Loop）实现

SDK 提供了两种 HITL 机制，AnalystPro 都会使用：

#### 方式 1：`AskUserQuestion` tool（Subagent 内部触发）

Subagent 在工作流中的关键决策点调用 `AskUserQuestion`，SDK 自动暂停等待用户响应。
在 Renderer 端渲染为一个审批卡片。

```typescript
// Renderer 端 HITL 审批组件
interface HITLApprovalProps {
  question: string;
  options: { label: string; description: string }[];
  onApprove: (answer: string) => void;
  agentName: string; // 显示是哪个 subagent 在请求审批
}
```

#### 方式 2：`canUseTool` hook（全局权限控制）

在 `query()` options 中注册 `canUseTool` 回调，对高风险操作进行拦截。

```typescript
const canUseTool: CanUseTool = async (toolName, input, { signal }) => {
  // Bash 命令全部需要人工确认
  if (toolName === "Bash") {
    const approved = await showApprovalDialog(
      `Agent 请求执行命令: ${(input as BashInput).command}`
    );
    return approved
      ? { behavior: "allow", updatedInput: input }
      : { behavior: "deny", message: "用户拒绝执行" };
  }

  // 写入 ic_memos/ 或 lp_reports/ 需要确认
  if (toolName === "Write" || toolName === "Edit") {
    const filePath = (input as FileWriteInput).file_path;
    if (filePath.includes("ic_memos/") || filePath.includes("lp_reports/")) {
      const approved = await showApprovalDialog(
        `Agent 请求写入敏感文件: ${filePath}`
      );
      return approved
        ? { behavior: "allow", updatedInput: input }
        : { behavior: "deny", message: "用户拒绝写入" };
    }
  }

  return { behavior: "allow", updatedInput: input };
};
```

### 3.7 Hook 系统设计

| Hook 事件                   | 用途                                   | 实现              |
| ------------------------- | ------------------------------------ | --------------- |
| `PreToolUse(Bash)`        | 拦截所有 shell 命令，弹出确认                   | `canUseTool` 回调 |
| `PreToolUse(Write\|Edit)` | 敏感路径写入审批                             | `canUseTool` 回调 |
| `PostToolUse(Task)`       | Subagent 完成后记录结果到 SQLite             | Hook callback   |
| `PostToolUse(Write)`      | 文件变更审计日志                             | Hook callback   |
| `Stop`                    | Session 结束时写入 memory/YYYY-MM-DD.md   | Hook callback   |
| `SessionStart`            | 自动加载 SOUL.md / USER.md / IDENTITY.md | Hook callback   |
| `PreCompact`              | 上下文压缩前保存关键信息到 memory                 | Hook callback   |

---

## 4. 数据模型

### 4.1 SQLite Schema（结构化数据）

```sql
-- 项目管道
CREATE TABLE deals (
  id TEXT PRIMARY KEY,
  company_name TEXT NOT NULL,
  sector TEXT,             -- semiconductor, fusion, packaging, etc.
  stage TEXT,              -- inbox, basic_info, deep_analysis, ic_memo, approved, rejected
  source TEXT,             -- BP, referral, conference, etc.
  valuation_rmb REAL,
  round TEXT,              -- Pre-A, A, B, etc.
  created_at TEXT,
  updated_at TEXT,
  assigned_analyst TEXT,   -- subagent that processed it
  red_flags TEXT,          -- JSON array of red flag items
  score INTEGER            -- 1-5 投资评分
);

-- 情报记录
CREATE TABLE intel_items (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  source_url TEXT,
  sector TEXT,
  score INTEGER,           -- 1-5 重要性评分
  category TEXT,           -- daily_brief, vc_moves, policy_alert, archive
  content_summary TEXT,
  published_at TEXT,
  ingested_at TEXT
);

-- Agent session 日志
CREATE TABLE agent_sessions (
  id TEXT PRIMARY KEY,
  agent_type TEXT,         -- secretary, market-intel, deal-analyst, etc.
  session_id TEXT,         -- SDK session ID
  started_at TEXT,
  ended_at TEXT,
  prompt_summary TEXT,
  result_summary TEXT,
  cost_usd REAL,
  input_tokens INTEGER,
  output_tokens INTEGER,
  status TEXT              -- success, error, timeout
);

-- KPI 指标
CREATE TABLE kpi_records (
  id TEXT PRIMARY KEY,
  metric_name TEXT,
  target_value REAL,
  actual_value REAL,
  period TEXT,             -- weekly, monthly, quarterly
  recorded_at TEXT
);

-- HITL 审批记录
CREATE TABLE hitl_approvals (
  id TEXT PRIMARY KEY,
  agent_type TEXT,
  action_type TEXT,        -- ic_memo, tech_dd, lp_report, bash_command
  question TEXT,
  user_decision TEXT,      -- approved, rejected
  decided_at TEXT,
  context TEXT             -- JSON: 相关文件路径、命令内容等
);

-- 对话记录（多模型 Chat）
CREATE TABLE conversations (
  id TEXT PRIMARY KEY,
  title TEXT,                        -- 自动从首条消息生成
  provider TEXT NOT NULL,            -- claude-agent | claude-direct | openai | gemini | custom
  model TEXT NOT NULL,
  session_id TEXT,                   -- Agent SDK session ID（仅 claude-agent）
  deal_id TEXT,                      -- 关联项目（可选）
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  message_count INTEGER DEFAULT 0,
  total_cost_usd REAL DEFAULT 0,
  total_input_tokens INTEGER DEFAULT 0,
  total_output_tokens INTEGER DEFAULT 0
);

-- 聊天消息
CREATE TABLE chat_messages (
  id TEXT PRIMARY KEY,
  conversation_id TEXT NOT NULL REFERENCES conversations(id),
  role TEXT NOT NULL,                -- user | assistant | system
  content TEXT NOT NULL,
  provider TEXT NOT NULL,
  model TEXT NOT NULL,
  agent_name TEXT,                   -- subagent 名称（如 deal-analyst）
  input_tokens INTEGER DEFAULT 0,
  output_tokens INTEGER DEFAULT 0,
  cost_usd REAL DEFAULT 0,
  files_generated TEXT,              -- JSON array of workspace file paths
  hitl_request_json TEXT,            -- JSON: HITL 请求内容
  hitl_response TEXT,                -- approved | rejected
  created_at TEXT NOT NULL
);

-- Provider 配置
CREATE TABLE provider_configs (
  id TEXT PRIMARY KEY,
  type TEXT NOT NULL UNIQUE,         -- claude-agent | claude-direct | openai | gemini | custom
  label TEXT NOT NULL,               -- 用户可见名称
  is_enabled INTEGER DEFAULT 0,
  default_model TEXT,
  available_models TEXT,             -- JSON array
  api_base_url TEXT,                 -- 仅自定义 provider
  max_budget_usd REAL,
  created_at TEXT,
  updated_at TEXT
  -- 注意：API keys 和 OAuth tokens 存于 macOS Keychain，不入库
);

-- 生成文件追踪
CREATE TABLE file_outputs (
  id TEXT PRIMARY KEY,
  conversation_id TEXT REFERENCES conversations(id),
  message_id TEXT REFERENCES chat_messages(id),
  file_path TEXT NOT NULL,           -- workspace 相对路径
  report_type TEXT,                  -- basic_info, deep_analysis, ic_memo, daily_brief, etc.
  deal_id TEXT,
  agent_name TEXT,
  provider TEXT,
  model TEXT,
  created_at TEXT NOT NULL
);
```

> **注意**：`agent_sessions` 表新增 `conversation_id TEXT REFERENCES conversations(id)` 列，关联 Agent 活动到 Chat 对话。

### 4.2 Workspace 文件（与 OpenClaw 兼容的 Markdown）

保持与 OpenClaw `workspace-secretary/` 完全相同的文件结构和格式，确保：

- 可以从 VPS 双向同步（`scp` / `rsync` / Git）

- 同一套 knowledge 文件在两个系统间通用

- 迁移成本为零

### 4.3 项目资料归档模型（Project Archive Model）

为满足“每个项目按归档指引自动归类，并作为各阶段报告输入”的要求，新增项目资料库规范。

#### 4.3.1 目录规范（去掉 `6X-` 前缀）

按“立项会是否通过”进行双路径归档：

1) 未通过立项会（浅度跟踪，年归档）

- 根目录：`workspace/state/deals/archive/shallow/{yyyy}/{yyyymmdd}-{company}/`
- 示例：
- `workspace/state/deals/archive/shallow/2025/20250101-寒武纪半导体/`
- `workspace/state/deals/archive/shallow/2025/20250102-比特大陆/`
- 子目录（最小）：
- `01-项目收集/`
- `02-项目初筛/`

2) 通过立项会（重点跟进，全流程归档）

- 根目录：`workspace/state/deals/archive/key/{deal_code}-{company}/`

子目录：

- `1-项目准备/项目收集/`
- `1-项目准备/项目初筛/`
- `2-项目立项/`
- `3-项目尽调/财务尽调/`
- `3-项目尽调/法律尽调/`
- `3-项目尽调/业务尽调/`
- `3-项目尽调/投决预备/`
- `4-项目投决/`
- `5-项目执行/项目方材料/`
- `5-项目执行/协议文件/`
- `5-项目执行/划款材料/`
- `6-投后管理/股东会材料/`
- `6-投后管理/经营管理材料/`
- `6-投后管理/其他会议材料/`
- `6-投后管理/投后监测/`
- `7-追加投资/`
- `8-投资退出/退出准备/`
- `8-投资退出/退出投决/`
- `8-投资退出/退出执行材料/`

#### 4.3.2 上传后自动处理（under the hood）

上传任意文件（PDF/Word/Excel/图片/录音）后执行：

1. 文档抽取：OCR + 文本抽取 + 基础元数据识别（公司名、日期、文档类型、签章状态、来源方）。
2. 路由判定：先根据“立项会是否通过”路由到 `shallow` 或 `key` 根目录。
3. 分类判定：映射到“阶段 + 类别 + 子目录”；低置信度触发 HITL 人工确认。
4. 自动重命名并落盘：按命名规范写入目标目录。
   - 规范：`{doc_date}_{company}_{doc_type}_{stage}_{counterparty}_v{n}.{ext}`
   - 示例：`20250101_寒武纪半导体_BP_项目收集_企业方_v1.pdf`
5. 台账登记：写入档案清单（文件名、版本、在档状态、经手人、日期、路径、是否纸质必留）。
6. 引用绑定：生成 `document_id`，供 basic_info/deep_analysis/ic_memo/投后报告引用。

#### 4.3.3 Skills 元数据缓存

```sql
-- Skill 注册表（从 SKILL.md frontmatter 扫描缓存）
CREATE TABLE skills (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,              -- kebab-case 标识符（即 /skill-name）
  description TEXT,
  source TEXT NOT NULL,                    -- built-in | custom
  file_path TEXT NOT NULL,                 -- SKILL.md 文件绝对路径
  -- 标准字段（Agent Skills / Claude Code 兼容）
  argument_hint TEXT,                      -- 自动补全参数提示，如 "[公司名]"
  disable_model_invocation INTEGER DEFAULT 0,  -- 1=仅手动调用
  user_invocable INTEGER DEFAULT 1,        -- 0=从 / 菜单隐藏
  allowed_tools TEXT,                      -- 逗号分隔的工具白名单
  model TEXT,                              -- 模型覆盖
  context TEXT,                            -- fork | null
  agent_name TEXT,                         -- context=fork 时的 subagent 类型
  -- AnalystPro 扩展字段（ap- 前缀）
  ap_type TEXT,                            -- agent | prompt-template | local-action
  ap_ui_response TEXT,                     -- 调用后切换的视图
  ap_icon TEXT,                            -- emoji 图标
  ap_tags TEXT,                            -- JSON array 分类标签
  --
  is_enabled INTEGER DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE INDEX idx_skills_name ON skills(name);
CREATE INDEX idx_skills_ap_type ON skills(ap_type);
CREATE INDEX idx_skills_source ON skills(source);
```

#### 4.3.4 MCP Server 配置

```sql
-- MCP Server 注册表（Phase 2+）
CREATE TABLE mcp_servers (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,           -- MCP server 标识符（如 playwright, arxiv）
  description TEXT,
  transport TEXT NOT NULL,             -- stdio | http
  command TEXT,                        -- stdio: 可执行命令
  args TEXT,                           -- stdio: JSON array 启动参数
  url TEXT,                            -- http: 服务端 URL
  env TEXT,                            -- JSON object 环境变量
  headers TEXT,                        -- http: JSON object HTTP 头
  source TEXT NOT NULL,                -- built-in | user
  is_enabled INTEGER DEFAULT 1,
  auto_restart INTEGER DEFAULT 1,      -- stdio 崩溃后自动重启
  allowed_agents TEXT,                 -- JSON array: 允许使用的 agent 列表（null=全部）
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
  -- OAuth tokens 存于 macOS Keychain，不入库
);

CREATE INDEX idx_mcp_servers_name ON mcp_servers(name);
CREATE INDEX idx_mcp_servers_source ON mcp_servers(source);
```

#### 4.3.5 阶段输入门禁（Stage Input Gates）

- 基本信息报告：必须至少关联 `BP + 交流纪要 + 行业辅助资料`。
- 立项材料：必须至少关联 `初筛材料 + 立项预备会材料 + 核心 DD 输入`。
- 投决材料：必须关联 `财务/法律/业务尽调关键结论 + 风控材料`。
- 投后报告：必须关联 `股东会材料 + 财务数据（PDF/Excel）+ 经营更新`。

当必需输入不满足时，系统禁止“提交下一阶段”，并提示缺失项。

#### 4.3.6 数据表扩展（SQLite）

```sql
CREATE TABLE archive_documents (
  id TEXT PRIMARY KEY,
  deal_id TEXT NOT NULL,
  deal_code TEXT,
  company_name TEXT NOT NULL,
  archive_track TEXT,         -- shallow/key
  archive_year TEXT,          -- shallow 路径年份，如 2025
  stage TEXT,                 -- 1-项目准备/2-项目立项/3-项目尽调/4-项目投决/5-项目执行/6-投后管理/7-追加投资/8-投资退出
  category TEXT,              -- 项目收集/财务尽调/协议文件/股东会材料...
  file_name TEXT NOT NULL,
  file_ext TEXT,
  file_path TEXT NOT NULL,
  source_type TEXT,           -- upload/email/chat/import
  version_no INTEGER DEFAULT 1,
  is_final_version INTEGER DEFAULT 0,
  is_counterparty_version INTEGER DEFAULT 0,
  has_stamp INTEGER DEFAULT 0,
  is_required_paper INTEGER DEFAULT 0,
  original_file_name TEXT,
  archived_status TEXT,       -- in_archive/missing/exempted
  archived_by TEXT,
  archived_at TEXT,
  created_at TEXT,
  updated_at TEXT
);

CREATE TABLE stage_input_bundles (
  id TEXT PRIMARY KEY,
  deal_id TEXT NOT NULL,
  stage TEXT NOT NULL,
  report_type TEXT NOT NULL,  -- basic_info/deep_analysis/ic_memo/portfolio_report
  required_rules_json TEXT,   -- 必填规则快照
  included_document_ids TEXT, -- JSON array
  missing_items_json TEXT,    -- JSON array
  gate_status TEXT,           -- pass/fail/override
  reviewed_by TEXT,
  reviewed_at TEXT
);
```

---

## 5. UI/UX 设计

### 5.0 信息架构（一级菜单）

按当前单人使用场景，一级菜单固定为 5 个：

- `项目流程` — 投资全流程看板（8 阶段 pipeline）
- `项目资料库` — 按项目归档的文档管理（shallow/key 双轨）
- `市场情报` — 实时信息流：公告扫描、一级市场交易、政策预警、VC 动向
- `工作台` — 运营仪表板 + 报告中心 + 知识库
- `系统设置` — 模型/成本/同步/命名规则/用户偏好

其中 `项目流程` 使用二级阶段标签（Tag/Filter）：

- `项目寻源` — 01 周项目例会 + 02 行业专题研究 + 03 项目初筛
- `立项审批` — 04 立项预备会 + 05 立项投决会
- `投资执行` — 06 TS 签署 + 07 尽调与风控预审 + 08 最终投决与执行
- `投后与退出` — 投后管理 + 退出

### 5.1 布局架构

采用双侧栏 push layout，右侧 Chat 面板可通过 `▶`/`◀` 折叠按钮或 `Cmd+J` 切换显示/隐藏。左侧 Sidebar 仅包含 5 个导航菜单项，Chat 入口不占用导航栏位。

```
Chat 关闭时:
┌──────────┬──────────────────────────────────┬──┐
│  Sidebar │        Main Content Area         │▶│ ← 折叠条（垂直居中）
│          │                                  │  │
│ ┌──────┐ │  ┌──────────────────────────┐    │  │
│ │项目流程│ │  │                          │    │  │
│ ├──────┤ │  │   当前选中菜单的内容区域    │    │  │
│ │ 资料库 │ │  │                          │    │  │
│ ├──────┤ │  │   项目流程 / 资料库        │    │  │
│ │ 市场情报│ │  │   / 市场情报 / 工作台      │    │  │
│ ├──────┤ │  │   / 系统设置              │    │  │
│ │ 工作台 │ │  │                          │    │  │
│ ├──────┤ │  │                          │    │  │
│ │ 系统设置│ │  │                          │    │  │
│ └──────┘ │  └──────────────────────────┘    │  │
└──────────┴──────────────────────────────────┴──┘

Chat 打开时:
┌──────────┬──────────────────────────┬───────────────────┐
│  Sidebar │    Main Content Area     │◀│ [Claude ▾] [×]  │
│          │                          │ ├─────────────────│
│ ┌──────┐ │  ┌──────────────────┐    │ │  Chat Messages  │
│ │项目流程│ │  │                  │    │ │  ┌───────────┐  │
│ ├──────┤ │  │   内容区域        │    │ │  │ User: ... │  │
│ │ 资料库 │ │  │                  │    │ │  │ Agent: ...│  │
│ ├──────┤ │  │                  │    │ │  └───────────┘  │
│ │ 市场情报│ │  │                  │    │ │                 │
│ ├──────┤ │  │                  │    │ │ Agent Monitor▾  │
│ │ 工作台 │ │  └──────────────────┘    │ │                 │
│ ├──────┤ │                          │ │ 💬 输入消息...   │
│ │ 系统设置│ │                          │ └─────────────────│
│ └──────┘ │                          │                   │
└──────────┴──────────────────────────┴───────────────────┘

Chat 关闭: grid-template-columns: 248px 1fr 24px;  （24px = 折叠条宽度）
Chat 打开: grid-template-columns: 248px 1fr minmax(320px, 480px);
切换方式: ▶/◀ 折叠按钮 | Cmd+J 快捷键 | Chat header × 按钮
缩放: 左边缘拖拽, 280px ≤ width ≤ 50% viewport
Sidebar 切换: Cmd+\ 显示/隐藏左侧导航栏
```

> **设计变更**：原 Command Input Bar（主内容区底部固定输入框）已移除，所有输入统一到 Chat 面板。原 Right Panel 的 Gate Panel 移入项目详情页内联，Agent Monitor 移入 Chat 面板可折叠区域。

### 5.2 核心视图

#### 5.2.1 项目流程（投资全流程看板）

- 看板视图按二级阶段过滤：`项目寻源`、`立项审批`、`投资执行`、`投后与退出`
- 每张卡片展示：公司名、当前阶段、红旗数量、最近更新、下一步动作
- 点击进入项目详情：报告全文、资料引用、阶段门禁状态、操作历史
- 拖拽改变项目阶段（触发对应 subagent 工作流）
- 详情页内嵌 StageGatePanel（Gate 门禁面板，从右侧面板移入）：显示当前阶段所需材料、缺失项、由用户本人执行"通过/驳回/备注"

#### 5.2.2 项目资料库（按项目归档）

- 目录浏览 `workspace/state/deals/archive/`（`shallow` / `key` 双轨）
- 支持文件拖拽上传（BP PDF、录音、Excel、扫描件）
- 上传后自动重命名、自动路由与自动归档（低置信度触发 HITL）
- 支持按"流程/文件夹/标签/文件类型/日期"检索

#### 5.2.3 市场情报（实时信息流）

- 独立一级菜单，每日高频使用入口
- Intel Feed：半导体/AI 板块公告实时流，按评分排序（1-5 分）
- Daily Brief：每日情报简报（由 market-intel subagent 自动生成）
- Policy Alert：政策/监管/出口管制预警
- VC Moves：头部 VC 机构动向追踪
- 支持标记"已读/关注/归档"

#### 5.2.4 工作台（运营仪表板 + 报告 + 知识库）

- KPI Tracker：年度/季度考核指标进度追踪
- Daily Todo：今日待办清单
- Weekly Plan：周计划查看与更新
- 报告中心：基本信息报告、IC Memo、投后季度报告、LP 报告的统一查阅与管理
- 知识库：KnowledgeExplorer（行业知识浏览）+ GlossaryViewer（术语库）

#### 5.2.5 系统设置

- **AI 模型配置**（ProviderSettings 面板）：
  - Claude (Agent SDK)：OAuth / API Key 认证，Secretary 模型选择（sonnet 默认），单次对话预算限制
  - Claude (Direct)：共享 API Key，模型选择，用于无 Agent 的直接对话
  - OpenAI：OAuth（ChatGPT Plus/Pro）/ API Key，模型选择（gpt-4o 默认）
  - Gemini：OAuth（Google 账号）/ API Key，模型选择（gemini-2.5-pro 默认）
  - 自定义端点：Base URL + API Key + 模型名称（兼容 OpenAI API 格式）
  - 每个 Provider 支持 [Test Connection] 连通性检测
  - API Key 存储于 macOS Keychain（safeStorage），不入 SQLite
- **Skills 管理**（SkillSettings 面板）：
  - 内置 Skills（Built-in）：随 app 打包的 10 个 Skill，支持启用/禁用，不可删除
  - 自定义 Skills（Custom）：用户创建，存储在 `workspace/skills/`，支持启用/禁用/删除
  - 添加 Skill：弹窗编辑器（名称、类型选择、描述、SKILL.md 内容编辑，支持 `$ARGUMENTS` 占位符）
  - `ap-type` badge：🤖 Agent（仅 Claude Secretary）| 📝 Template（全 Provider）| ⚡ Action（本地执行）
  - 每个 Skill 显示：调用控制（`disable-model-invocation` / `user-invocable`）、`argument-hint`、关联 subagent
- **MCP Servers**（Phase 2+，McpSettings 面板）：
  - 内置 MCP Servers：随 app 预配置（playwright, sqlite 等），支持启用/禁用
  - 用户 MCP Servers：手动添加，填写 transport（stdio/http）、command/url、环境变量
  - 每个 server 显示：连接状态（运行中/已停止/错误）、transport 类型、`allowed_agents` 限制
  - [Test Connection] 按钮验证连通性
  - 安全提示：添加自定义 MCP server 时显示 prompt injection 风险警告
- **快捷键**（KeyboardShortcuts 面板）：
  - 列出所有可用快捷键，分三组：通用、导航、Chat
  - 通用：`Cmd+K` 命令面板/Skill 搜索 | `Cmd+J` 切换 Chat 面板 | `Cmd+\` 切换左侧导航栏 | `Cmd+N` 新建对话 | `Cmd+F` 搜索 | `Cmd+,` 系统设置 | `Cmd+/` 快捷键参考
  - 导航：`Cmd+1` 项目流程 | `Cmd+2` 项目资料库 | `Cmd+3` 市场情报 | `Cmd+4` 工作台 | `Cmd+5` 系统设置 | `Cmd+[` 后退 | `Cmd+]` 前进
  - Chat：`Cmd+Enter` 发送消息 | `Esc` 取消/关闭 SkillPalette
  - 遵循 macOS 桌面应用惯例（参考 Notion/Linear/Slack/VS Code/Arc/Obsidian 行业标准）
- **文件输出**：默认输出目录、项目关联模式（自动/手动）
- **成本控制**：单次对话上限、日预算、月预算
- 归档命名规则、同步策略
- 用户偏好与默认视图配置

#### 5.2.6 AI 对话面板（多模型 Chat Sidebar）

跨页面持久化的右侧 Chat 面板，是应用的核心交互入口。

- **面板行为**：通过主内容区右边缘的 `▶`/`◀` 折叠按钮切换显示/隐藏（Chat 关闭时显示 `▶`，打开时显示 `◀`），也可通过 `Cmd+J` 快捷键或 Chat header `×` 按钮操作。Push layout（打开时主内容区收缩），左边缘可拖拽缩放（280px–50% viewport），面板状态（开关、宽度）持久化到 Zustand。左侧 Sidebar 不包含 Chat 入口
- **Provider 选择器**：顶部下拉菜单，支持切换 Claude (Secretary) / Claude (Direct) / OpenAI / Gemini / Custom；切换 provider 时自动开始新对话
- **消息流**：markdown 渲染（react-markdown + 语法高亮），subagent 消息带分色 badge（如 `deal-analyst`、`market-intel`），生成文件显示为可点击的 workspace 路径链接，每条响应显示 token 数与费用
- **HITL 审批**：内联卡片形式（琥珀色边框），包含审批问题 + 确认/拒绝按钮，同时发送 macOS 通知
- **Skill 调用**：输入 `/` 触发 SkillPalette 弹窗，按 `ap-type` 分组（🤖 Agent / 📝 Template / ⚡ Action），支持模糊搜索；Agent 类型 Skill 仅 Claude (Secretary) 模式可用，其他模式灰色显示；`argument-hint` 显示为灰色提示；`disable-model-invocation: true` 的 Skill 仅手动触发，`user-invocable: false` 的 Skill 从菜单隐藏；参数通过 `$ARGUMENTS` 占位符传递
- **Agent Monitor**：Chat 面板底部可折叠区域，折叠时显示单行当前活跃 agent 状态（名称 + 运行时长），展开时显示近期 agent 会话列表（状态、时长、token、费用）
- **对话历史**：所有对话持久化到 SQLite `conversations` + `chat_messages` 表，支持列表浏览和搜索
- **上下文感知**：当前页面/项目自动注入相关 workspace 文件（Claude Agent 通过 Secretary 原生 tool 访问；其他 provider 通过 ContextBuilder 注入 system prompt）
- **文件输出路由**：Agent 生成的文件通过 OutputRouter 按上下文自动路由——项目相关 → deal 归档目录，情报 → `state/intelligence/`，默认 → `outputs/{date}/`；所有生成文件包含 YAML frontmatter 元数据

### 5.3 交互设计要点

| 场景           | 交互方式                            |
| ------------ | ------------------------------- |
| HITL 审批      | 对话流内卡片 + 右侧面板 badge + macOS 通知  |
| Subagent 运行中 | 底部状态栏 spinner + agent 名称 + 已用时间 |
| 文件更新         | 文件变更 toast 提示 + 仪表板自动刷新         |
| 错误/异常        | 红色 toast + agent 日志面板高亮         |
| 成本预警         | 当单次对话 > \$2 时显示黄色警告             |
| 定时任务         | 系统托盘图标 + 完成后桌面通知                |

### 5.4 AI 需求到菜单映射（R1-R18 → 视图/组件）

> 来源：投资分析师系统.md 第 5 部分 AI 需求清单。每个需求明确落在哪个一级菜单和子视图中。

| ID  | 需求                 | 一级菜单   | 子视图/组件                           |
| --- | ------------------ | ------ | -------------------------------- |
| R1  | 周项目自动汇总与预评分        | 项目流程   | Pipeline 看板 + 项目卡片预评分 badge      |
| R2  | 会议纪要与 Action 提取    | 工作台    | 报告中心/会议纪要列表                      |
| R3  | 初筛打分卡引擎            | 项目流程   | 项目详情页/初筛打分卡面板                    |
| R4  | 基本信息报告生成           | 工作台    | 报告中心/基本信息报告                      |
| R5  | 行业研究初稿生成           | 工作台    | 报告中心/行业研究报告                      |
| R6  | IC Memo 副驾驶        | 工作台    | 报告中心/IC Memo + 项目详情页内嵌           |
| R7  | 投后预警引擎             | 项目流程   | 投后与退出标签/预警 badge + 详情页内嵌 StageGatePanel |
| R8  | LP 报告组装            | 工作台    | 报告中心/LP 报告                       |
| R9  | DD 知识图谱            | 项目流程   | 项目详情页/知识图谱面板                     |
| R10 | 立项 briefing 包自动组装  | 项目流程   | 项目详情页/立项材料包                      |
| R11 | TS 条款分析器           | 项目流程   | 项目详情页/TS 分析面板（投资执行阶段）            |
| R12 | term redline 建议助手  | 项目流程   | 项目详情页/redline 建议                 |
| R13 | FDD/LDD 报告抽取与风险矩阵 | 项目流程   | 项目详情页/风险矩阵面板（投资执行阶段）             |
| R14 | 投决答辩预演             | 项目流程   | 项目详情页/答辩预演                       |
| R15 | 董事会材料异常检测          | 项目流程   | 投后与退出标签/异常检测 badge               |
| R16 | 硬科技技术 DD 助手        | 项目流程   | 项目详情页/技术 DD 面板                   |
| R17 | 退出准备副驾驶            | 项目流程   | 投后与退出标签/退出准备面板                   |
| R18 | 知识库与术语库自动维护        | 工作台    | 知识库/KnowledgeExplorer + Glossary |

---

## 6. Skills 系统与命令映射

### 6.1 Skill 架构概述

AnalystPro 采用 **[Agent Skills 开放标准](https://agentskills.io)**（SKILL.md + YAML frontmatter），与 Claude Code、Cowork、OpenClaw 生态兼容，并在此基础上扩展了 AnalystPro 特有的字段以适配多 Provider + 桌面 GUI 场景。

- **内置 Skills**：随 app 打包，存储在 `electron/skills/{name}/`，用户可启用/禁用但不可删除
- **自定义 Skills**：用户创建，存储在 `workspace/skills/{name}/`，支持完整 CRUD
- **Skill 目录结构**：每个 Skill 一个子目录，包含 `SKILL.md`（必需）+ 可选附件（`scripts/`、模板文件、数据文件等）
- **渐进式加载**：启动时仅解析 YAML frontmatter（~100 tokens/skill）缓存到 SQLite `skills` 表，调用时按需加载正文和附件
- **参数传递**：遵循 Claude Code 标准，在 SKILL.md 正文中使用 `$ARGUMENTS`（全部参数）、`$ARGUMENTS[N]` 或 `$N`（位置参数）占位符；调用时用户输入替换占位符

### 6.2 SKILL.md 定义格式

每个 Skill 以 YAML frontmatter + Markdown 正文组成。示例：

```markdown
---
name: deal-analysis
description: 启动项目分析流程，生成基本信息报告。当用户提到"分析项目"、"看看这个公司"时自动触发。
argument-hint: "[公司名]"
context: fork
agent: deal-analyst
model: sonnet
allowed-tools: Read, Grep, WebSearch
# ── AnalystPro 扩展字段 ──
ap-type: agent
ap-ui-response: pipeline
---

# Deal Analysis（项目分析）

对 $ARGUMENTS 进行投资分析。Secretary 将派遣 deal-analyst subagent。

## 工作流程

1. 搜索 $0 的公司信息、解析 BP
2. 生成基本信息报告 → workspace/state/deals/archive/{project}/
3. 触发 HITL 确认是否继续深度分析

## 输出

- 基本信息报告（Markdown + YAML frontmatter）
- 初筛打分卡
- 项目卡片（自动添加到 Pipeline 看板）
```

### 6.3 Frontmatter 字段参考

#### 标准字段（Agent Skills / Claude Code 兼容）

| 字段 | 必填 | 类型 | 说明 |
|------|------|------|------|
| `name` | No | string | Skill 唯一标识符（kebab-case，max 64 字符）。省略时使用目录名。即 `/` 后的命令名 |
| `description` | 推荐 | string | 一行描述，用于 SkillPalette 显示和 Claude 自动判断是否加载 |
| `argument-hint` | No | string | 自动补全提示，如 `[公司名]`、`[行业] [深度]` |
| `disable-model-invocation` | No | boolean | `true` 时仅用户可通过 `/name` 手动调用，Claude 不会自动触发。默认 `false` |
| `user-invocable` | No | boolean | `false` 时从 `/` 菜单隐藏，仅 Claude 在相关场景自动加载。默认 `true` |
| `allowed-tools` | No | string | 限制 Skill 活跃时 Claude 可使用的工具，如 `Read, Grep, Glob` |
| `model` | No | string | Skill 活跃时使用的模型覆盖 |
| `context` | No | string | `fork` 表示在独立 subagent 中运行（与主对话隔离） |
| `agent` | No | string | `context: fork` 时使用的 subagent 类型。对应 `electron/agents/definitions/` 中的定义 |
| `hooks` | No | object | Skill 生命周期 hooks（预留） |

#### AnalystPro 扩展字段（`ap-` 前缀）

| 字段 | 必填 | 类型 | 说明 |
|------|------|------|------|
| `ap-type` | No | enum | `agent` / `prompt-template` / `local-action`。影响 Provider 兼容性和调用分发 |
| `ap-ui-response` | No | string | 调用后自动切换的视图：`pipeline` / `intel` / `workspace` / `settings` / `none` |
| `ap-icon` | No | string | emoji 图标，显示在 SkillPalette |
| `ap-tags` | No | string[] | 分类标签，用于 SkillPalette 模糊搜索 |

> **命名约定**：AnalystPro 扩展字段统一使用 `ap-` 前缀，与标准字段区分，确保 SKILL.md 可在 Claude Code 等其他客户端正常解析（未知字段被忽略）。

#### 参数占位符（标准）

| 占位符 | 说明 |
|--------|------|
| `$ARGUMENTS` | 用户输入的全部参数。若正文中无 `$ARGUMENTS`，参数自动追加到末尾 |
| `$ARGUMENTS[N]` | 按位置访问参数（0-based），如 `$ARGUMENTS[0]` 为第一个参数 |
| `$N` | `$ARGUMENTS[N]` 的简写，如 `$0`、`$1` |

#### 动态上下文注入（标准）

在 SKILL.md 正文中使用 `` !`command` `` 语法可在发送给 Claude 之前执行 shell 命令并注入输出：

```markdown
## 当前项目上下文
- 项目列表: !`ls workspace/state/deals/processing/`
- 今日情报: !`head -20 workspace/state/intelligence/daily_brief.md`
```

### 6.4 Skill 类型与 Provider 兼容性

AnalystPro 通过 `ap-type` 扩展字段定义三种 Skill 类型，决定调用分发路径和 Provider 兼容性：

| ap-type | 说明 | Claude (Secretary) | Claude (Direct) | OpenAI | Gemini | Custom |
|---------|------|:-:|:-:|:-:|:-:|:-:|
| `agent` | `context: fork` 触发 Secretary 派遣 subagent，支持 HITL/tool use/file output | ✅ | — | — | — | — |
| `prompt-template` | SKILL.md 正文作为 prompt 模板，`$ARGUMENTS` 替换后作为 user message 发送 | ✅ | ✅ | ✅ | ✅ | ✅ |
| `local-action` | 由 app 本地处理（Electron main process），不需要 LLM 调用 | ✅ | ✅ | ✅ | ✅ | ✅ |

> 未设置 `ap-type` 时，行为由标准字段推断：有 `context: fork` + `agent` → 按 agent 处理；否则按 prompt-template 处理。

### 6.5 内置 Skills 清单

| 命令 | Skill Name | ap-type | argument-hint | 处理方 | ap-ui-response |
|------|-----------|---------|---------------|--------|---------------|
| `/today` | today | agent | — | Secretary 直接 | workspace |
| `/kpi` | kpi | agent | — | Secretary 直接 | workspace |
| `/plan` | weekly-plan | agent | — | Secretary 直接 | workspace |
| `/news` | news-scan | agent | — | Secretary → market-intel | intel |
| `/deal` | deal-analysis | agent | `[公司名]` | Secretary → deal-analyst | pipeline |
| `/dd` | tech-dd | agent | `[公司名]` | Secretary → hardtech-dd | pipeline |
| `/research` | industry-research | agent | `[行业]` | Secretary → industry-researcher | workspace |
| `/review` | system-review | agent | — | Secretary (opus) | none |
| `/sync` | workspace-sync | local-action | — | 本地处理 | none |
| `/cost` | cost-report | local-action | — | 本地处理 | settings |

### 6.6 SkillPalette（Chat 面板 Skill 选择器）

在 Chat 输入框输入 `/` 时触发 SkillPalette 弹窗：
- 按 `ap-type` 分组显示：🤖 Agent | 📝 Template | ⚡ Action
- 每项显示：`ap-icon` + `name` + `description` + `argument-hint`
- 当前非 Claude (Secretary) 模式时，agent 类型 Skills 灰色显示 + "需要 Claude Agent 模式" 提示
- `disable-model-invocation: true` 的 Skill 仅在用户手动输入 `/` 时出现，Claude 不会自动触发
- `user-invocable: false` 的 Skill 从菜单隐藏
- 支持模糊搜索（按 name + description + ap-tags）
- 选中后自动填充到输入框，`argument-hint` 显示为灰色提示文字

---

## 7. 定时任务与心跳

### 7.1 Cron 任务（替代 OpenClaw cron）

```typescript
// electron/scheduler/cron.ts
import cron from "node-cron";

// 每日 07:00 (UTC+8) 公告扫描
cron.schedule("0 23 * * *", () => { // UTC 23:00 = CST 07:00
  runSubagentTask("market-intel", {
    task: "执行每日半导体板块公告扫描，生成今日情报简报",
    model: "haiku",
  });
}, { timezone: "Asia/Shanghai" });

// 每周一 08:00 一级市场周报
cron.schedule("0 8 * * 1", () => {
  runSubagentTask("market-intel", {
    task: "生成本周一级市场交易情况周报",
    model: "sonnet",
  });
}, { timezone: "Asia/Shanghai" });

// 每周三 09:00 VC 机构动向
cron.schedule("0 9 * * 3", () => {
  runSubagentTask("market-intel", {
    task: "汇总本周头部 VC 机构动向",
    model: "haiku",
  });
}, { timezone: "Asia/Shanghai" });
```

### 7.2 Heartbeat（替代 OpenClaw heartbeat）

```typescript
// electron/scheduler/heartbeat.ts

// 每 55 分钟心跳检查
setInterval(async () => {
  const result = await query({
    prompt: `执行心跳检查：
1. 读取 state/dashboard/kpi_tracker.md，检查是否有指标落后预期
2. 读取 inbox/ 下所有 .inbox.md，检查是否有新待处理项
3. 读取 state/deals/processing/，检查是否有项目卡在某阶段超过 3 天
如有异常，简要汇总。无异常返回 "心跳正常"。`,
    options: {
      model: "haiku",
      maxTurns: 3,
      allowedTools: ["Read", "Glob"],
      permissionMode: "bypassPermissions",
    },
  });

  // 有异常时发送桌面通知
  if (!result.includes("心跳正常")) {
    showDesktopNotification("AnalystPro 心跳异常", result);
  }
}, 55 * 60 * 1000);
```

---

## 8. 安全与隐私

### 8.1 数据安全

| 层级              | 措施                                                        |
| --------------- | --------------------------------------------------------- |
| **API Key**     | 存储在 macOS Keychain（via `keytar`），不存文件                     |
| **Workspace**   | 本地文件，不上传云端（用户自主 Git/scp 备份）                               |
| **LP 数据隔离**     | `state/portfolio/` 和 `state/lp_reports/` 标记为 CONFIDENTIAL |
| **Agent 数据防火墙** | market-intel 被禁止读取 portfolio/lp\_reports 路径               |
| **审计日志**        | 所有 agent 操作记录到 SQLite `agent_sessions` 表                  |
| **HITL 强制**     | IC memo / LP 材料 / shell 命令 必须人工确认                         |
| **成本限制**        | 单次对话 `maxBudgetUsd` 上限，可配置                                |

### 8.2 Electron 安全最佳实践

- `contextIsolation: true` — Renderer 无法直接访问 Node.js API

- `nodeIntegration: false` — 禁止 Renderer 中使用 Node.js

- `sandbox: true` — Renderer 进程沙箱化

- 所有 Renderer → Main 通信通过 `contextBridge.exposeInMainWorld`

- CSP header 限制外部资源加载

- 自动更新签名验证（macOS code signing）

---

## 9. MCP Server 集成

### 9.1 架构概述

MCP（Model Context Protocol）是连接 AI 应用到外部系统的开放标准协议。在 AnalystPro 中，**MCP 是 Agent 的工具层，Skills 是用户的工作流层**——两者互补而非替代：

```
用户输入 /deal TSMC
  → [Skill 系统] → 解析为 deal-analysis SKILL.md (ap-type: agent)
    → [Secretary] → 派遣 deal-analyst subagent
      → [Agent SDK query()] → MCP tools 提供能力：
           - 内置 SDK 工具 (Read/Write/WebSearch)
           - MCP: playwright（网页抓取）
           - MCP: sqlite-query（结构化数据）
```

**核心设计原则**：

- **Phase 1 不需要自定义 MCP server**：Agent SDK 内置工具（Read, Write, Edit, Glob, Grep, WebSearch, WebFetch）已覆盖 MVP 需求
- **数据防火墙用 `canUseTool` hook 实现**，比自定义 MCP server 更简单直接
- **MCP server 作为子进程运行**：Electron main process 通过 `child_process.spawn()` 管理（stdio transport）
- **Agent SDK 原生集成**：`query()` 的 `mcpServers` 配置是主要接入点

### 9.2 MCP 分层引入计划

| Tier | Phase | 内容 | 理由 |
|------|-------|------|------|
| **Tier 0** | Phase 1 (MVP) | 不使用自定义 MCP server；使用 Agent SDK 内置工具 | SDK 已提供文件操作 + 网页搜索，避免 MCP 进程管理增加 MVP 复杂度 |
| **Tier 1** | Phase 2 | 添加 **Playwright MCP**（网页抓取）+ MCP Server 生命周期管理器 | 首个超出 SDK 内置工具能力的 MCP server；hardtech-dd 和 deal-analyst 需要深度网页分析 |
| **Tier 2** | Phase 3 | 添加可选 MCP servers（arxiv, financial-data, github）+ 完整配置 UI + HTTP transport | 投后/LP 阶段需要更多外部数据源 |
| **Tier 3** | Phase 4 | MCP-Skill 桥接 + 热加载 + 社区生态 | MCP 从基础设施升级为平台特性 |

### 9.3 MCP Server 清单

#### 内置 MCP Servers（随 app 预配置）

| MCP Server | 用途 | Transport | Agent 使用方 | Phase |
|------------|------|-----------|-------------|-------|
| playwright | 网页抓取（公司官网、新闻站、LinkedIn） | stdio | market-intel, deal-analyst | Phase 2 |
| sqlite-query | 结构化数据查询（deals, intel, KPI） | stdio | dashboard, reports, Secretary | Phase 2 |

#### 可选扩展 MCP Servers

| MCP Server | 用途 | Transport | Agent 使用方 | Phase |
|------------|------|-----------|-------------|-------|
| arxiv | 论文检索（半导体、核聚变） | stdio | hardtech-dd | Phase 2 |
| financial-data | 公司财务/融资数据（PitchBook/Crunchbase API） | http | deal-analyst | Phase 3 |
| github | Workspace 版本管理 | http | Secretary | Phase 3 |
| google-drive | BP PDF / 财务模型文件访问 | http | deal-analyst | Phase 3 |
| slack / telegram | 消息推送通知 | http | Secretary | Phase 4 |
| email (Gmail/Outlook) | 自动导入 BP 邮件附件到 `inbox/` | http | Secretary | Phase 4 |
| calendar | DD 会议 / IC 会议日程管理 | http | Secretary | Phase 4 |

### 9.4 MCP Server 管理架构（Phase 2+）

```
Electron Main Process
├── Agent Engine (engine.ts)
│   └── query() options.mcpServers: { ... }  // SDK 原生 MCP 配置
│
├── MCP Server Manager (electron/mcp/manager.ts)
│   ├── startServer(config) → 启动子进程
│   ├── stopServer(name) → 优雅终止
│   ├── restartServer(name) → 崩溃恢复
│   ├── getServerStatus(name) → 运行状态
│   └── listServers() → 全部配置
│
├── MCP Server Registry (electron/mcp/registry.ts)
│   ├── 内置配置（playwright, sqlite 等）
│   ├── 用户配置（从 SQLite mcp_servers 表读取）
│   └── 环境变量解析
│
└── IPC: mcp.ipc.ts
    ├── mcp:list-servers → McpServerMeta[]
    ├── mcp:add-server { config } → void
    ├── mcp:remove-server { name } → void
    ├── mcp:toggle-server { name, enabled } → void
    ├── mcp:test-connection { name } → TestResult
    └── mcp:get-status { name } → ServerStatus
```

**Agent SDK 集成方式**：活跃的 MCP server 配置直接传入 `query()` 的 `mcpServers` 选项，由 SDK 负责创建 MCP client 和 tool 注册。

**Per-agent 访问控制**：通过 `canUseTool` hook 按 MCP tool 命名空间（`mcp__servername__toolname`）执行。例如 market-intel 被禁止访问 `mcp__sqlite__` 中的 portfolio 相关查询。

### 9.5 MCP 安全策略

| 策略 | 实现方式 |
|------|---------|
| 子进程沙箱 | `child_process.spawn()` 使用受限环境变量，不传递 `ANTHROPIC_API_KEY` |
| HITL 拦截 | 高风险 MCP 工具（数据库写入、workspace 外文件操作）通过 `canUseTool` hook 要求人工确认 |
| 输出 token 限制 | `MAX_MCP_OUTPUT_TOKENS` 防止单个 MCP server 消耗过多上下文窗口 |
| 用户安全提示 | 添加自定义 MCP server 时弹窗警告 prompt injection 风险 |
| 数据防火墙 | MCP tool 命名空间 + `canUseTool` hook 实现 per-agent 访问控制 |
| 自动重启 | stdio server 崩溃后自动重启（可配置，默认开启） |
| OAuth token 安全 | HTTP transport 的 OAuth tokens 存于 macOS Keychain，不入 SQLite |

---

## 10. 与 OpenClaw VPS 的协同

### 10.1 双向同步

AnalystPro 可以与 VPS 上的 OpenClaw workspace 保持同步：

```
本地 Electron App                      VPS OpenClaw
workspace/                    ←→      ~/.openclaw/workspace-secretary/
  ├── knowledge/              ←→        ├── knowledge/
  ├── state/                  ←→        ├── state/
  ├── inbox/                  ←→        ├── inbox/
  └── memory/                 ←→        └── memory/
```

- **同步方式**：rsync over SSH（通过 Tailscale）或 Git（Gitee 备份仓库）

- **冲突解决**：以最新修改时间为准，冲突时保留两个版本并提示用户

- ``** 命令**：手动触发同步

### 10.2 渐进式迁移路径

1. **阶段 0**：两套系统并行运行，workspace 通过 Git 同步

2. **阶段 1**：日常对话转移到 AnalystPro，VPS 仅保留 cron 任务

3. **阶段 2**：cron 任务也迁移到 AnalystPro，VPS 降级为纯备份

4. **阶段 3**：VPS 可选下线（或保留作为 Telegram bot 入口）

---

## 11. 实现路线图

### Phase 1：MVP（核心对话 + 情报 + 项目分析）

**目标**：可用的 secretary 对话系统 + 两个核心 subagent

- [ ] Electron 项目脚手架（electron-vite + React + TailwindCSS + shadcn/ui）

- [ ] Agent Engine 核心（封装 `query()`，session 管理）

- [ ] Secretary 主 agent（系统提示 + Skills 调用）

- [ ] Chat Panel UI（对话、Markdown 渲染、SkillPalette）

- [ ] market-intel subagent + Intel Feed UI

- [ ] deal-analyst subagent（Phase 1 基本信息报告）+ Deal Pipeline UI

- [ ] HITL 审批 UI（AskUserQuestion 渲染 + canUseTool 拦截）

- [ ] Workspace 文件管理器（读取现有 OpenClaw 格式文件）

- [ ] SQLite 数据库（deals, intel\_items, agent\_sessions）

- [ ] macOS Keychain API key 存储

### Phase 2：完整分析链 + 仪表板

- [ ] deal-analyst Phase 2-3（BP 深度分析 + IC Memo）

- [ ] hardtech-dd subagent

- [ ] industry-researcher subagent

- [ ] Dashboard 完整实现（KPI, Todo, Weekly Plan）

- [ ] Cron 调度器（每日/每周定时任务）

- [ ] Heartbeat 心跳检查

- [ ] Agent Monitor 面板（实时状态 + 历史日志 + 成本统计）

- [ ] Knowledge Explorer（知识库浏览 + 编辑）

- [ ] `/review` 系统复盘功能

- [ ] 桌面通知系统（macOS Notification Center）

- [ ] MCP Server 生命周期管理器（`electron/mcp/manager.ts` + `registry.ts`）

- [ ] Playwright MCP 集成（网页抓取：公司官网、新闻站）

- [ ] MCP Server 设置 UI（启用/禁用/状态监控）

### Phase 3：投后 + LP + 高级功能

- [ ] portfolio-monitor subagent

- [ ] lp-reporter subagent

- [ ] VPS 双向同步（`/sync` 命令 + rsync/Git）

- [ ] 可选 MCP Servers（arxiv 论文检索、financial-data 财务数据、github workspace 版本管理）

- [ ] MCP Server 完整配置 UI（添加/删除/环境变量/连通性测试）+ HTTP transport 支持

- [ ] 文件拖拽上传（BP PDF、录音）

- [ ] 导出功能（PDF 报告、Excel 数据表）

- [ ] 多窗口支持（同时查看多个项目详情）

- [ ] 自动更新（electron-builder auto-update）

- [ ] 成本分析仪表板（按 agent/model/时间 的费用明细）

### Phase 4：生态扩展

- [ ] Telegram bot 集成（通过 AnalystPro 作为 gateway）

- [ ] 语音输入（录音转写 → deal-analyst）

- [ ] MCP Server 热加载（无需重启 app 添加/移除 MCP server）

- [ ] MCP-Skill 桥接（MCP prompts 自动注册为 Skills + 社区 MCP Skill 包）

- [ ] Skill 生态（社区 Skill 注册表）

- [ ] 团队协作（多用户 + 权限管理）

---

## 12. 技术决策说明

### 12.1 为什么选 Claude Agent SDK 而非直接调 Anthropic API

| 维度         | 直接 API 调用           | Agent SDK                        |
| ---------- | ------------------- | -------------------------------- |
| 工具执行       | 需自行实现 tool loop     | 内置 tool 执行引擎                     |
| Subagent   | 需手动管理多个 API session | 原生 `agents` 选项，自动管理              |
| Hook 系统    | 无                   | 内置 PreToolUse/PostToolUse/Stop 等 |
| Session 恢复 | 手动序列化               | `resume` session ID              |
| 权限控制       | 需自行实现               | `canUseTool` + `permissionMode`  |
| MCP 集成     | 需自行接入               | 原生 `mcpServers` 配置               |
| 上下文压缩      | 需手动实现               | 自动 auto-compaction               |
| 文件操作       | 需实现 Read/Write/Edit | 内置工具                             |

**结论**：Agent SDK 省去了大量基础设施代码，让我们专注于业务逻辑。

### 12.2 为什么不直接用 OpenClaw 架构在 Electron 中

- OpenClaw 是一个完整的 gateway 服务，包含 channel 路由、binding 规则、WebSocket 协议层，这些对桌面 App 来说是多余的

- OpenClaw 的 multi-agent 是"多个独立 agent + binding 路由"，而桌面 App 更适合"单主 agent + SDK subagent"模式

- OpenClaw 的 `sessions_spawn` 是平台私有 API，无法在本地复用

- Agent SDK 的 `query()` + `agents` 提供了等价能力，且 API 更简洁

### 12.3 为什么保留 Workspace Markdown 文件格式

- 与 OpenClaw 双向兼容，支持渐进式迁移

- Markdown 人类可读，方便直接编辑

- Git 友好，支持版本管理和 diff

- Agent 原生支持读写 Markdown（Read/Write/Edit 工具）

- 知识库文件不适合放数据库（非结构化、需要 context 注入）

### 12.4 为什么 MCP 分层引入而非 Phase 1 全量集成

- Phase 1 的 Agent SDK 内置工具（Read/Write/Edit/Glob/Grep/WebSearch/WebFetch）已完全覆盖 Secretary、market-intel、deal-analyst P1 的需求
- 自定义 MCP server 需要子进程生命周期管理（spawn、health check、crash recovery），增加 MVP 复杂度
- 数据防火墙（禁止 market-intel 访问 portfolio）通过 `canUseTool` hook 比 MCP server 权限更简单直接
- Playwright MCP 是第一个超出 SDK 内置能力的 MCP server（深度网页抓取），Phase 2 时 hardtech-dd 和 deal-analyst P2 才真正需要
- Phase 3-4 按需引入外部数据源（arxiv、financial-data）和生态特性（热加载、Skill 桥接），避免过早建设

### 12.5 嵌套 Subagent 的实现方式

OpenClaw 需要 `maxSpawnDepth: 2` 来支持 deal-analyst → hardtech-dd 的嵌套。在 Agent SDK 中：

- `deal-analyst` 的 `tools` 列表包含 `Task`

- `deal-analyst` 的 `prompt` 中明确指示："遇到硬科技项目时，使用 hardtech-dd subagent"

- SDK 自动处理 subagent 嵌套（subagent 内部可以再调用 Task 工具）

> **注意**：SDK 文档提到 "Subagents cannot spawn other subagents"。这是 Claude Code 交互模式的限制。在 Agent SDK programmatic 模式下，如果 deal-analyst 的 `tools` 包含 `Task` 且 `agents` 定义中包含 hardtech-dd，SDK 应支持此嵌套。如果 SDK 不支持，备选方案是：secretary 负责协调，先运行 deal-analyst Phase 1-2，结果返回后 secretary 自动启动 hardtech-dd，最后汇总。

---

## 13. 性能与成本预算

### 13.1 API 成本估算（月度）

| Agent               | 频率              | Model        | 预估 token/次      | 月度调用次数 | 月成本 (USD)      |
| ------------------- | --------------- | ------------ | --------------- | ------ | -------------- |
| secretary           | 每日 20 次对话       | sonnet       | 5K in + 2K out  | 600    | \~\$12         |
| market-intel        | 每日 1 次 + 每周 2 次 | haiku/sonnet | 10K in + 3K out | 38     | \~\$2          |
| deal-analyst        | 每周 2-3 个项目      | sonnet/opus  | 20K in + 8K out | 12     | \~\$15         |
| hardtech-dd         | 每月 2-3 次        | sonnet       | 15K in + 5K out | 3      | \~\$2          |
| industry-researcher | 每月 2 次          | sonnet       | 15K in + 5K out | 2      | \~\$1.5        |
| heartbeat           | 每 55 分钟         | haiku        | 2K in + 500 out | 780    | \~\$1          |
| **合计**              |                 |              |                 |        | **\~\$33.5/月** |

### 13.2 本地资源要求

- **磁盘**：\~500MB（App + workspace + SQLite）

- **内存**：\~300MB（Electron + Node.js + React）

- **CPU**：minimal（计算在 API 端）

- **网络**：需要互联网连接以调用 Claude API

---

## 14. 测试策略

### 14.1 单元测试

- Agent definition 正确性（prompt 内容、tools 列表、model 选择）

- Workspace manager 文件操作（读取、写入、监听）

- SQLite CRUD 操作

- Cron 调度逻辑

- HITL 审批流程

- MCP Server 管理器（启动/停止/重启/健康监控）

### 14.2 集成测试

- Secretary → subagent 派发链路（mock SDK `query()`）

- HITL 全流程：subagent 请求 → UI 渲染 → 用户操作 → 结果回传

- Skill 端到端测试（`/skill-name` → invoker → agent/template/action）

- MCP Server 连接 + tool 调用链路（mock MCP server）

- Workspace 文件同步一致性

### 14.3 E2E 测试

- Playwright 测试 Electron Renderer UI

- 完整对话流程（输入 → agent 处理 → 结果渲染）

- Deal pipeline 拖拽操作

---

## 15. 关键风险与缓解

| 风险                 | 影响                              | 缓解措施                                             |
| ------------------ | ------------------------------- | ------------------------------------------------ |
| SDK subagent 不支持嵌套 | deal-analyst 无法直接派生 hardtech-dd | Secretary 作为 orchestrator 协调两级调用                 |
| API 成本超预期          | 月费用过高                           | `maxBudgetUsd` 限制 + 成本仪表板 + haiku 优先策略           |
| 上下文窗口溢出            | 长对话质量下降                         | auto-compaction + session resume + memory 文件     |
| Electron 安全漏洞      | 数据泄露                            | context isolation + sandbox + CSP + code signing |
| OpenClaw 升级导致格式不兼容 | 同步失败                            | workspace 格式保持向后兼容；版本化迁移脚本                       |
| macOS 沙箱限制         | 文件系统访问受限                        | 使用 user data 目录；必要时 entitlement 配置               |
| MCP server 崩溃/不稳定 | Agent 工具调用失败                     | 自动重启 + 健康监控 + 优雅降级（回退到 SDK 内置工具）            |
| MCP prompt injection | 恶意 MCP server 注入有害指令             | 添加时安全警告 + `canUseTool` hook 拦截 + 输出 token 限制  |

---

## 16. 成功指标

| 指标        | 目标值                        | 衡量方式                 |
| --------- | -------------------------- | -------------------- |
| 每日活跃使用    | 用户每工作日使用 > 30 分钟           | App 使用时长统计           |
| 情报覆盖率     | 每日 > 90% 行业重大事件被捕获         | 对比手工刷新闻的遗漏率          |
| 报告产出效率    | 基本信息报告 < 10 分钟（vs 手工 2 小时） | agent\_sessions 时长   |
| HITL 通过率  | > 80% 报告无需大幅修改             | hitl\_approvals 统计   |
| 月度 API 成本 | < \$50                     | agent\_sessions 费用汇总 |
| 系统可靠性     | < 5% agent 任务失败率           | agent\_sessions 错误率  |

---

*本 PRD 基于投资分析师系统 v1.0 设计文档、OpenClaw 多 agent 架构、Claude Code Agent SDK (TypeScript) 文档编写。下一步：进入技术设计阶段，细化 API 接口定义和组件交互协议。*
