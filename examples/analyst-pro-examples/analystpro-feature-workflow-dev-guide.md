# AnalystPro Feature Workflow 开发指南

> 本文档详细讲解如何使用项目内置的 `/feature-workflow`、`/fix`、`/audit-fix`、Speckit 命令套件以及 Planning/Audit 技能（Skills），系统性地完成 AnalystPro 从 Phase 1 到 Phase 4 的全部开发工作。
>
> 参考：[How to Use Speckit and Feature Workflow for MiniClaw](https://github.com/anzchy/codex-claude-init/blob/master/examples/How%20to%20Use%20Speckit%20and%20Feature%20Workflow%20for%20MiniClaw.md)

---

## 目录

1. [工具全景：两套工作流 + 三个独立命令](#1-工具全景)

2. [选哪条路？决策树](#2-选哪条路决策树)

3. [推荐方案：Feature Workflow Only](#3-推荐方案feature-workflow-only)

4. [备选方案：Hybrid（Speckit + Feature Workflow）](#4-备选方案hybrid)

5. [实战演练：Phase 1 MVP 完整开发流程](#5-实战演练phase-1-mvp)

6. [单个模块开发示例：Agent Engine](#6-单个模块开发示例)

7. [Bug 修复：/fix 命令](#7-bug-修复)

8. [代码审计：/audit-fix 命令](#8-代码审计)

9. [Planning 和 Audit 技能（Skills）](#9-planning-和-audit-技能)

10. [Speckit 命令套件详解（备选路径）](#10-speckit-命令套件详解)

11. [Codex 双模型工作流（可选）](#11-codex-双模型工作流)

12. [Session 上下文管理](#12-session-上下文管理)

13. [完整开发节奏速查表](#13-完整开发节奏速查表)

14. [常见问题 FAQ](#14-faq)

---

## 1. 工具全景

AnalystPro 项目配置了两套独立但互补的开发工作流，加上三个独立命令和多个辅助技能：

### 1.1 两套工作流管线

```
┌─────────────────────────────────────────────────────────────────┐
│ Speckit 管线（规格驱动，适合需求模糊时）                              │
│                                                                 │
│  speckit.specify → speckit.clarify → speckit.plan               │
│       → speckit.checklist → speckit.tasks → speckit.analyze     │
│       → speckit.implement                                       │
│                                                                 │
│  产出: spec.md, plan.md, tasks.md (T001 格式)                    │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│ Feature Workflow 管线（执行驱动，9 个 Agent 接力）                   │
│                                                                 │
│  /feature-workflow [name]                                       │
│  → Planner → Spec Guardian → Impact Analyst → Implementer      │
│  → Test Runner → Auditor → Manual Test Author → Verifier       │
│  → Release Steward                                              │
│                                                                 │
│  产出: plan (WI-001 格式), 代码, 测试, 文档, 提交                   │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 三个独立命令

| 命令                     | 用途                 | 适用场景        |
| ---------------------- | ------------------ | ----------- |
| `/fix [描述]`            | 根因修复，禁止打补丁         | 单个 bug，已知问题 |
| `/audit-fix [scope]`   | 审计→修复→验证循环（最多 3 轮） | 代码审查、提交前清扫  |
| `/codex-audit [scope]` | Codex 独立审计（双模型）    | 发布前深度审计     |

### 1.3 辅助技能（Skills）

| 技能             | 触发方式                            | 用途            |
| -------------- | ------------------------------- | ------------- |
| `planning`     | Planner agent 内部或直接 `/planning` | 生成结构化计划文件     |
| `plan-audit`   | `/plan-audit`                   | 检查实现是否符合计划    |
| `plan-verify`  | `/plan-verify`                  | 运行测试 + 验收标准检查 |
| `release-gate` | `/release-gate`                 | 运行完整质量门禁      |
| `save-context` | `/save-context`                 | 保存会话上下文（防丢失）  |
| `load-context` | `/load-context`                 | 恢复上一次会话       |

### 1.4 关键区别：两套管线互不消费

> **重要**：Speckit 产出 `tasks.md`（T001 格式），Feature Workflow 的 Planner agent 产出自己的计划（WI-001 格式）。两者之间**没有内置的自动衔接**。Feature Workflow 的 Planner 不会读取 Speckit 的 `tasks.md`，它会重新创建自己的计划。

---

## 2. 选哪条路？决策树

```
你的需求清晰吗？
│
├─ YES → PRD/Blueprint 已有详细模块分解
│         → 推荐 Option A: Feature Workflow Only ✅
│
├─ MOSTLY → 大方向清楚，细节需要澄清
│            → 推荐 Option B: Hybrid（Speckit 补规格 + Feature Workflow 执行）
│
└─ NO → 需求模糊，需要从零探索
          → 使用 Speckit 完整管线先定义规格
```

### 对于 AnalystPro 项目的判断

AnalystPro 已经有一份完整的 PRD（`analyst-pro-prd.md`），包含：

- 详细的目录结构

- 每个 Agent 的完整定义（包括 TypeScript 伪代码）

- SQLite Schema

- UI 布局设计

- 4 个 Phase 的实现路线图

- Skill 系统规范

- MCP 集成规范

**结论：推荐 Option A — Feature Workflow Only。** PRD 的详细程度已经超过了 Speckit 的 spec.md 能提供的信息。

---

## 3. 推荐方案：Feature Workflow Only

### 3.1 前置准备

在开始第一个 `/feature-workflow` 之前，确保 `CLAUDE.md` 和 `AGENTS.md` 已经正确引用了 PRD：

```bash
# 确认 CLAUDE.md 中有 PRD 引用
grep "analyst-pro-prd" CLAUDE.md

# 确认 AGENTS.md 中有项目结构和规则
grep "Project Structure" AGENTS.md
```

AnalystPro 的 `CLAUDE.md` 和 `AGENTS.md` 已经包含完整的架构决策、目录结构和开发规则，无需额外配置。

### 3.2 为什么不能只输入 slug？（防幻觉关键）

> **核心问题**：`/feature-workflow electron-scaffold` 中的 `electron-scaffold` 只是一个 slug 标签。Planner agent 拥有 `Read` 和 `Grep` 工具，它**可以**搜索项目文件——但**不保证**会读到 PRD 中正确的章节。
>
> PRD 有 1840 行。Planner 可能只读到开头，或者完全跳过关键细节（如 Electron 安全配置、Agent SDK 的 `query()` 用法、SQLite schema 定义）。这会导致它基于"通用常识"而非**项目实际规格**来生成计划——这就是幻觉。

#### 正确做法：每次调用都附带精确的上下文指引

```bash
# ❌ 错误：只给 slug，Planner 可能猜测内容
/feature-workflow electron-scaffold

# ✅ 正确：告诉 Planner 去读哪些章节、要实现什么
/feature-workflow electron-scaffold
```

然后在 Planner 开始工作前（或在同一条消息中），**追加具体的上下文描述**：

```
/feature-workflow electron-scaffold

请参考以下文件和章节：
- analyst-pro-prd.md §2.2 技术栈（Electron 34+, electron-vite, React 18, TailwindCSS, shadcn/ui）
- analyst-pro-prd.md §2.3 目录结构（electron/ 和 src/ 的完整结构）
- analyst-pro-prd.md §8.2 Electron 安全最佳实践（contextIsolation, sandbox, CSP）
- AGENTS.md "Electron Security" 和 "Project Structure" 章节

目标：初始化 Electron 项目脚手架，包括：
1. package.json + electron-vite + TypeScript strict mode 配置
2. electron/main.ts 主进程入口（contextIsolation: true, nodeIntegration: false, sandbox: true）
3. electron/preload.ts + contextBridge 桥接骨架
4. src/App.tsx + TailwindCSS + shadcn/ui 基础主题
5. CSP header 配置
6. npm run check:all 脚本（eslint + vitest + electron-vite build）
```

#### 上下文指引的三要素

| 要素       | 说明                      | 示例                                             |
| -------- | ----------------------- | ---------------------------------------------- |
| **文件引用** | 告诉 Planner 必须读哪些文件的哪些章节 | `analyst-pro-prd.md §3.2 Agent 引擎核心`           |
| **目标描述** | 用自然语言说清楚这个模块要做什么        | "封装 SDK query()，支持 streaming 和 session resume" |
| **关键约束** | 列出不可违反的规则               | "contextIsolation: true 是非协商的安全要求"             |

> **为什么这能防幻觉？** Planner 收到明确的文件指引后，会用 `Read` 工具去读取这些具体章节，而不是凭空想象。关键约束的重复强调则确保即使 Planner 跳过了某些文件，核心规则也不会被遗漏。

### 3.3 模块分解策略

将 PRD 的 Phase 1 拆分为独立的 feature-workflow 单元。每个单元应该是**可独立测试、可独立提交**的模块。

#### Phase 1 MVP 推荐执行顺序（含上下文指引）

下表中 **PRD 参考** 列是你每次调用时应该告诉 Planner 去读的章节：

| #  | 命令                   | PRD 参考章节                                                  | 关键约束/目标                                                   |
| -- | -------------------- | --------------------------------------------------------- | --------------------------------------------------------- |
| 1  | `electron-scaffold`  | §2.2 技术栈, §2.3 目录结构, §8.2 Electron 安全                     | contextIsolation+sandbox+CSP, electron-vite 配置            |
| 2  | `sqlite-schema`      | §4.1 SQLite Schema (全部表定义), §4.3.3 Skills 表, §4.3.4 MCP 表 | 7 张核心表 + 索引, better-sqlite3, in-memory 测试                 |
| 3  | `workspace-manager`  | §4.2 Workspace 文件, §10 VPS 协同 (目录结构)                      | OpenClaw 兼容格式, SOUL/USER/IDENTITY.md                      |
| 4  | `provider-types`     | §5.2.6 "Provider 选择器" 段, §6.4 Skill 类型与 Provider 兼容性      | ChatProvider 接口, 5 种 provider 类型定义                        |
| 5  | `agent-engine`       | §3.2 Agent 引擎核心 (完整伪代码), §3.5 Session 管理, §3.7 Hook 系统    | 封装 SDK query(), streaming, session resume, maxBudgetUsd   |
| 6  | `secretary-agent`    | §3.1 架构模式, §3.2 engine.ts 伪代码中 Secretary 配置               | 系统提示合成, subagent 注册, model: sonnet                        |
| 7  | `market-intel-agent` | §3.3.1 market-intel 完整定义                                  | model: haiku, 数据防火墙 (禁止读 portfolio/lp\_reports)           |
| 8  | `deal-analyst-p1`    | §3.3.2 deal-analyst 定义 (仅 Phase 1 基本信息报告)                 | 三阶段工作流中的 Phase 1, HITL 审批                                 |
| 9  | `skill-system`       | §6 Skills 系统完整章节 (§6.1-§6.6)                              | SKILL.md + YAML frontmatter, loader+invoker, 10 内置 Skills |
| 10 | `ipc-bridge`         | §2.3 ipc/ 目录, AGENTS.md "IPC Pattern"                     | typed channels, contextBridge, 按功能域拆分                     |
| 11 | `chat-panel`         | §5.2.6 AI 对话面板完整定义, §5.1 布局架构                             | push layout, Cmd+J 切换, SkillPalette, markdown 渲染          |
| 12 | `pipeline-view`      | §5.2.1 项目流程看板, §5.0 信息架构                                  | 8 阶段 pipeline, 拖拽, StageGatePanel                         |
| 13 | `intel-feed`         | §5.2.3 市场情报                                               | Feed+DailyBrief+PolicyAlert+VCMoves, 1-5 评分               |
| 14 | `workspace-view`     | §5.2.4 工作台                                                | KPI+Todo+WeeklyPlan+ReportCenter+知识库                      |
| 15 | `hitl-system`        | §3.6 HITL 实现 (两种方式), §3.7 Hook 系统                         | AskUserQuestion + canUseTool hook, 审批卡片 UI                |
| 16 | `keychain-auth`      | §8.1 数据安全 ("API Key → macOS Keychain"), §5.2.5 系统设置       | keytar/safeStorage, 不入 SQLite                             |
| 17 | `output-router`      | §5.2.6 "文件输出路由" 段                                         | 按上下文路由: deal→归档, intel→state/intelligence/                |

#### 3.3.1 模块 1：Electron 脚手架

```
/feature-workflow electron-scaffold

请参考以下文件和章节：
- analyst-pro-prd.md §2.2 技术栈（Electron 34+, electron-vite, React 18, TailwindCSS, shadcn/ui, Zustand, TypeScript strict mode）
- analyst-pro-prd.md §2.3 目录结构（electron/ 和 src/ 的完整目录树）
- analyst-pro-prd.md §8.2 Electron 安全最佳实践（contextIsolation, sandbox, CSP, nodeIntegration: false）
- analyst-pro-prd.md §5.1 布局架构（grid-template-columns 双侧栏 push layout 的 CSS 规格）
- AGENTS.md "Electron Security (Non-negotiable)" 章节
- AGENTS.md "Project Structure" 章节
- AGENTS.md "Tech Stack" 章节

目标：
1. package.json — 声明 electron, electron-vite, react 18, tailwindcss, shadcn/ui, zustand, better-sqlite3, vitest, eslint, prettier 等核心依赖
2. electron-vite.config.ts — main/preload/renderer 三进程构建配置
3. tsconfig.json — strict mode 启用
4. electron/main.ts — 主进程入口，创建 BrowserWindow（contextIsolation: true, nodeIntegration: false, sandbox: true）
5. electron/preload.ts — contextBridge.exposeInMainWorld 骨架（空的 electronAPI 对象）
6. src/App.tsx — React 根组件 + TailwindCSS 初始化 + shadcn/ui 主题配置
7. tailwind.config.ts — 配置 content 扫描路径
8. CSP header — 在 main.ts 中通过 session.defaultSession.webRequest 设置
9. package.json scripts — "dev", "build", "check:all" (eslint + vitest run + electron-vite build)

关键约束：
- Electron 安全三件套不可省略：contextIsolation: true + nodeIntegration: false + sandbox: true
- 所有 Renderer→Main 通信必须走 contextBridge，Renderer 中禁止 require('electron')
- 文件结构必须与 PRD §2.3 一致（electron/ 放 main process 代码，src/ 放 renderer 代码）
- shadcn/ui 组件按需引入，不全量安装
```

#### 3.3.2 模块 2：SQLite Schema

```
/feature-workflow sqlite-schema

请参考以下文件和章节：
- analyst-pro-prd.md §4.1 SQLite Schema（从 deals 到 file_outputs 共 9 张核心表的完整 SQL CREATE 语句）
- analyst-pro-prd.md §4.3.3 Skills 元数据缓存（skills 表 CREATE 语句 + 3 个索引）
- analyst-pro-prd.md §4.3.4 MCP Server 配置（mcp_servers 表 CREATE 语句 + 2 个索引）
- analyst-pro-prd.md §4.3.6 数据表扩展（archive_documents 表 + stage_input_bundles 表的完整 SQL）
- analyst-pro-prd.md 第 4.1 节末尾注释 "agent_sessions 表新增 conversation_id 列"
- AGENTS.md "Data Separation" 章节（SQLite 存结构化数据，Markdown 存非结构化数据）

目标：
1. electron/db/schema.ts — 全部表定义 + 索引（共 11 张表：deals, intel_items, agent_sessions, kpi_records, hitl_approvals, conversations, chat_messages, provider_configs, file_outputs, skills, mcp_servers + archive_documents, stage_input_bundles）
2. electron/db/deals.ts — deals CRUD（createDeal, getDeal, updateDeal, listDeals, updateDealStage）
3. electron/db/intel.ts — intel_items CRUD（createIntelItem, getIntelItem, listByCategory, listByScore）
4. electron/db/sessions.ts — agent_sessions + conversations + chat_messages CRUD
5. electron/db/migration.ts — schema 版本管理机制（schema_version 表 + 增量迁移函数）
6. 每个 CRUD 模块的测试文件（*.test.ts）使用 in-memory SQLite（new Database(":memory:")）

关键约束：
- 使用 better-sqlite3（同步 API，Electron main process 安全）
- 测试必须用 in-memory database，每个 beforeEach 创建新实例确保隔离
- API keys 和 OAuth tokens 不入 SQLite（存 macOS Keychain）
- agent_sessions 表必须包含 conversation_id 外键关联到 conversations 表
- deals.stage 枚举值：inbox, basic_info, deep_analysis, ic_memo, approved, rejected
- intel_items.category 枚举值：daily_brief, vc_moves, policy_alert, archive
```

#### 3.3.3 模块 3：Workspace Manager

```
/feature-workflow workspace-manager

请参考以下文件和章节：
- analyst-pro-prd.md §4.2 Workspace 文件（与 OpenClaw 兼容的 Markdown 格式说明）
- analyst-pro-prd.md §2.3 目录结构中的 workspace/ 部分（SOUL.md, USER.md, IDENTITY.md, AGENTS.md, TOOLS.md, knowledge/, inbox/, state/, memory/, skills/）
- analyst-pro-prd.md §10 与 OpenClaw VPS 的协同（§10.1 双向同步的目录映射关系）
- analyst-pro-prd.md §4.3.1 目录规范（归档路径 shallow/key 双轨）
- analyst-pro-prd.md §4.3.2 上传后自动处理流程（文档抽取→路由判定→分类→重命名→台账登记→引用绑定）
- AGENTS.md "Data Separation" 章节

目标：
1. electron/workspace/manager.ts — WorkspaceManager 类：初始化 workspace 目录结构、读/写/列出文件、路径解析、确保 OpenClaw 格式兼容
2. electron/workspace/watcher.ts — 文件变更监听（fs.watch 或 chokidar），变更事件发送到 Renderer
3. electron/workspace/migration.ts — OpenClaw 格式迁移工具（验证现有 VPS workspace 可直接使用）
4. workspace/ 目录模板 — 初始化时创建 SOUL.md, USER.md, IDENTITY.md, AGENTS.md, TOOLS.md 和所有子目录

关键约束：
- workspace/ 目录结构必须与 OpenClaw VPS 的 ~/.openclaw/workspace-secretary/ 完全兼容
- 文件格式为 Markdown，知识库文件在 knowledge/，状态数据在 state/，收件箱在 inbox/
- state/deals/archive/ 下使用 shallow/{yyyy}/ 和 key/{deal_code}/ 双轨归档
- WorkspaceManager 运行在 Electron main process 中，Renderer 通过 IPC 调用
- 路径操作使用 path.join，不要硬编码路径分隔符
```

#### 3.3.4 模块 4：Provider Types

```
/feature-workflow provider-types

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.6 AI 对话面板 — "Provider 选择器" 段（顶部下拉菜单支持的 5 种 Provider）
- analyst-pro-prd.md §5.2.5 系统设置 — "AI 模型配置 (ProviderSettings 面板)" 段（5 种 Provider 的配置项明细）
- analyst-pro-prd.md §6.4 Skill 类型与 Provider 兼容性（ap-type 与 Provider 的兼容矩阵表）
- analyst-pro-prd.md §4.1 provider_configs 表 SQL 定义
- analyst-pro-prd.md §2.3 providers/ 目录结构（types.ts, registry.ts, 5 个 provider 文件, context-builder.ts）
- CLAUDE.md "Key Architecture Decisions" 中 "为什么多 Provider 用 Adapter 模式" 段

目标：
1. electron/providers/types.ts — ChatProvider 接口定义（sendMessage, streamMessage, testConnection, getAvailableModels），ChatMessage 类型，ProviderConfig 类型，ProviderType 枚举（claude-agent, claude-direct, openai, gemini, custom）
2. electron/providers/registry.ts — ProviderRegistry 类（registerProvider, getProvider, listProviders, 工厂方法 createProvider）
3. electron/providers/context-builder.ts — ContextBuilder（为非 Claude provider 注入 workspace 上下文到 system prompt）

关键约束：
- ChatProvider 是统一接口，所有 provider 必须实现它
- Claude Agent SDK 提供"富模式"（Secretary/subagent/HITL/tool use），其他 Provider 提供"直接对话"模式
- 接口设计要支持 streaming（AsyncIterator<StreamChunk>）
- provider_configs 中 API keys 不存数据库，通过 keyId 引用 Keychain 条目
- Phase 1 只实现 claude-agent provider 的完整功能，其他 provider 的 types/interface 先定义好留空实现
```

#### 3.3.5 模块 5：Agent Engine

```
/feature-workflow agent-engine

请参考以下文件和章节：
- analyst-pro-prd.md §3.2 Agent 引擎核心（完整 engine.ts TypeScript 伪代码，包含 import、agentDefinitions 注册、runSecretaryQuery 函数、query() 完整 options 配置、流式处理循环）
- analyst-pro-prd.md §3.5 Session 管理与上下文持久化（SessionConfig 接口定义：secretary 主 session 的持久化策略 + subagent session 的任务级生命周期）
- analyst-pro-prd.md §3.7 Hook 系统设计（7 种 Hook 事件表：PreToolUse(Bash), PreToolUse(Write|Edit), PostToolUse(Task), PostToolUse(Write), Stop, SessionStart, PreCompact）
- analyst-pro-prd.md §3.4 与 OpenClaw Subagent 的对应关系（SDK 映射参考表）
- AGENTS.md "Agent SDK Architecture" 章节

目标：
1. electron/agents/types.ts — AgentEngineConfig, QueryOptions, StreamMessage, SessionState 等核心类型定义
2. electron/agents/engine.ts — AgentEngine 类，封装 SDK query() 调用：
   - runSecretaryQuery(prompt, sessionId?) — 主函数
   - streaming 消息通过回调或事件发送到 Renderer
   - session resume 支持
   - maxBudgetUsd 成本控制
   - cwd 指向 workspace 目录
   - Hook 注册框架（PreToolUse, PostToolUse, Stop）
3. electron/agents/hooks/hitl.hook.ts — canUseTool 实现（Bash 全部确认，写入 ic_memos/lp_reports 确认）
4. electron/agents/hooks/audit.hook.ts — PostToolUse(Write|Edit) 文件变更审计日志
5. electron/agents/hooks/cost.hook.ts — 成本监控和超限警告

关键约束：
- 使用 @anthropic-ai/claude-agent-sdk 的 query() API，不直接调 Anthropic REST API
- Secretary 默认 model: sonnet
- allowedTools 必须包含 Task（允许 subagent 派生）和 AskUserQuestion（HITL 审批）
- mcpServers 配置在 Phase 1 暂不填入自定义 server，仅使用 SDK 内置工具
- maxBudgetUsd 默认 5.0（单次对话上限 $5）
- stream 中的每条 message 需要发送到 Renderer 进程（通过 mainWindow.webContents.send）
```

#### 3.3.6 模块 6：Secretary Agent

```
/feature-workflow secretary-agent

请参考以下文件和章节：
- analyst-pro-prd.md §3.1 架构模式（单主 Agent + SDK Subagents 的调度流程图）
- analyst-pro-prd.md §3.2 engine.ts 伪代码中 Secretary 的配置（systemPrompt 合成方式、agents 注册、allowedTools、model 选择）
- analyst-pro-prd.md §3.3 全部 Subagent 定义（§3.3.1-§3.3.6，Phase 1 需要 market-intel 和 deal-analyst 的 description 字段）
- analyst-pro-prd.md §4.2 Workspace 文件（SOUL.md, USER.md, IDENTITY.md 的用途说明）
- AGENTS.md "Agent SDK Architecture" 章节（Single Secretary + SDK Subagents）

目标：
1. electron/agents/secretary.ts — Secretary 主 agent 配置：
   - secretarySystemPrompt 合成函数：读取 workspace/SOUL.md + USER.md + IDENTITY.md + AGENTS.md 拼接为系统提示
   - agentDefinitions 注册：Phase 1 注册 market-intel、deal-analyst、hardtech-dd（hardtech-dd 必须注册以供 deal-analyst 通过 Task tool 派生；industry-researcher Phase 2 添加）
   - buildSecretaryOptions() — 构建传给 engine.runSecretaryQuery 的完整 options
2. 系统提示模板 — 包含 Secretary 的角色定义（投资分析工作台调度中心）、调度逻辑（简单任务直接回答，情报任务派 market-intel，项目分析派 deal-analyst）、中文为主的回复风格

关键约束：
- Secretary 默认 model: sonnet
- systemPrompt 使用 SDK 的 { type: "preset", preset: "claude_code", append: ... } 格式
- Secretary 的 allowedTools 必须包含 Task（派生 subagent）
- 系统提示中必须明确说明何时派生哪个 subagent（参考 §3.1 流程图）
- workspace 目录下的 SOUL.md / USER.md / IDENTITY.md 可能不存在，需优雅处理
```

#### 3.3.7 模块 7：Market-Intel Agent

```
/feature-workflow market-intel-agent

请参考以下文件和章节：
- analyst-pro-prd.md §3.3.1 market-intel 完整定义（TypeScript AgentDefinition 对象，包含 description、model、tools、完整 prompt 文本）
- analyst-pro-prd.md §5.2.3 市场情报 UI 视图（Feed + DailyBrief + PolicyAlert + VCMoves 四个子视图）
- analyst-pro-prd.md §7.1 Cron 任务（每日 07:00 公告扫描、每周一周报、每周三 VC 动向 的 cron 配置）
- AGENTS.md "HITL Enforcement" 章节（market-intel 被禁止读取 state/portfolio/ 和 state/lp_reports/）

目标：
1. electron/agents/definitions/market-intel.ts — 完整 AgentDefinition 导出（直接参考 PRD §3.3.1 的 TypeScript 代码）
2. electron/agents/definitions/market-intel.test.ts — 测试：
   - model 必须是 haiku
   - tools 列表正确（含 WebSearch, WebFetch，不含 Task）
   - prompt 包含 "Scoring Rules"
   - prompt 包含 "NO access to state/portfolio/" 数据防火墙声明
   - prompt 包含 6 个监控赛道

关键约束：
- model: haiku（高频低成本场景）
- 数据防火墙：禁止读取 state/portfolio/ 和 state/lp_reports/（通过 prompt 限制 + canUseTool hook 双重保障）
- tools 不包含 Task（market-intel 不能派生子 agent）
- 输出路径：state/intelligence/daily_brief.md, state/intelligence/vc_moves.md, state/intelligence/policy_alerts.md
- 评分体系 1-5 分（5=Critical/Urgent, 3-4=Important, 1-2=Background）
- 中文分析，英文技术术语
```

#### 3.3.8 模块 8：Deal Analyst（Phase 1）

```
/feature-workflow deal-analyst-p1

请参考以下文件和章节：
- analyst-pro-prd.md §3.3.2 deal-analyst 完整定义（TypeScript AgentDefinition，包含三阶段工作流，Phase 1 只实现 Phase 1 基本信息报告）
- analyst-pro-prd.md §3.3.3 hardtech-dd 定义（hardtech-dd Phase 1 已在 secretary.ts 注册，deal-analyst 的 tools 必须包含 Task 以便 Phase 1 即可派生它）
- analyst-pro-prd.md §4.3.1 归档目录规范（基本信息报告输出到 state/deals/processing/[company]/basic_info.md）
- analyst-pro-prd.md §4.3.5 阶段输入门禁（基本信息报告需关联 BP + 交流纪要 + 行业辅助资料）

目标：
1. electron/agents/definitions/deal-analyst.ts — 完整 AgentDefinition 导出（参考 PRD §3.3.2 代码，但 Phase 1 的 prompt 中 Phase 2 和 Phase 3 标记为"Phase 2 实现"注释）
2. electron/agents/definitions/deal-analyst.test.ts — 测试：
   - model 是 sonnet
   - tools 包含 Task（为 Phase 2 hardtech-dd 预留）和 AskUserQuestion（HITL）
   - prompt 包含 "Phase 1 — Basic Info Report" 工作流
   - prompt 包含 HITL 触发点（"基本信息报告已生成，是否继续深度分析？"）
   - prompt 包含 "default assumption: the founder is overselling"（怀疑论分析态度）

关键约束：
- Phase 1 只实现第一阶段（Basic Info Report），Phase 2-3 在 prompt 中声明但标注为后续实现
- 输出路径：state/deals/processing/[company]/basic_info.md
- 需使用 knowledge/report_template.md 作为模板（如不存在则降级处理）
- HITL：使用 AskUserQuestion 在基本信息报告完成后询问"是否继续深度分析？"
- tools 包含 Task 和 AskUserQuestion（比 market-intel 多这两个）
```

#### 3.3.9 模块 9：Skill 系统

```
/feature-workflow skill-system

请参考以下文件和章节：
- analyst-pro-prd.md §6 Skills 系统完整章节：
  - §6.1 Skill 架构概述（内置/自定义 Skills, 目录结构, 渐进式加载策略, 参数传递规范）
  - §6.2 SKILL.md 定义格式（完整示例：deal-analysis Skill 的 YAML frontmatter + Markdown 正文）
  - §6.3 Frontmatter 字段参考（标准字段表 + ap- 扩展字段表 + 参数占位符表 + 动态上下文注入语法）
  - §6.4 Skill 类型与 Provider 兼容性矩阵（agent/prompt-template/local-action × 5 providers）
  - §6.5 内置 Skills 清单（10 个 Skill 的完整表格）
  - §6.6 SkillPalette 交互规范
- analyst-pro-prd.md §4.3.3 skills 表 SQL 定义（缓存 SKILL.md frontmatter）
- analyst-pro-prd.md §2.3 中 electron/skills/ 和 workspace/skills/ 的目录结构

目标：
1. electron/skills/types.ts — SkillDefinition 接口（匹配 §6.3 所有标准字段 + ap- 扩展字段）、SkillType 枚举（agent, prompt-template, local-action）、SkillInvocation 接口
2. electron/skills/loader.ts — SkillLoader 类：
   - 扫描 electron/skills/*/SKILL.md（内置）和 workspace/skills/*/SKILL.md（自定义）
   - 解析 YAML frontmatter（使用 gray-matter 或类似库）
   - 缓存到 SQLite skills 表
   - 支持增量加载（文件 mtime 变更时刷新缓存）
3. electron/skills/invoker.ts — SkillInvoker 类：
   - agent 类型 → 组装 prompt 发送给 Secretary（context: fork 时通过 Task tool 派生 subagent）
   - prompt-template 类型 → $ARGUMENTS 替换后作为 user message 发送给当前 provider
   - local-action 类型 → 直接在 Electron main process 执行（如 workspace-sync, cost-report）
4. 10 个内置 SKILL.md 文件 — 按 §6.5 表格创建，每个 Skill 一个目录（today/, kpi/, weekly-plan/, news-scan/, deal-analysis/, tech-dd/, industry-research/, system-review/, workspace-sync/, cost-report/）

关键约束：
- SKILL.md 格式必须与 Agent Skills 开放标准兼容（Claude Code / Cowork / OpenClaw 可解析）
- ap- 前缀字段是 AnalystPro 扩展，标准客户端会忽略它们
- agent 类型 Skill 仅 Claude (Secretary) provider 可用
- prompt-template 和 local-action 全 provider 可用
- $ARGUMENTS 占位符替换逻辑：$ARGUMENTS 替换为全部参数，$0/$1 替换为位置参数
- loader 启动时仅解析 frontmatter（~100 tokens/skill），正文在调用时按需加载
```

#### 3.3.10 模块 10：IPC Bridge

```
/feature-workflow ipc-bridge

请参考以下文件和章节：
- analyst-pro-prd.md §2.3 目录结构中 electron/ipc/ 部分（agent.ipc.ts, chat.ipc.ts, provider.ipc.ts, skill.ipc.ts, mcp.ipc.ts, workspace.ipc.ts, session.ipc.ts 共 7 个 IPC handler 文件）
- analyst-pro-prd.md §9.4 MCP Server 管理架构中 "IPC: mcp.ipc.ts" 段（6 个 mcp IPC channel 定义）
- AGENTS.md "IPC Pattern" 章节（typed channels, contextBridge, 按功能域拆分）
- AGENTS.md "Electron Security (Non-negotiable)" 章节

目标：
1. electron/ipc/agent.ipc.ts — Agent 相关 IPC handler（agent:query, agent:abort, agent:status）
2. electron/ipc/chat.ipc.ts — Chat 对话 IPC（chat:send, chat:abort, chat:history, chat:list-conversations, chat:new-conversation）
3. electron/ipc/provider.ipc.ts — Provider 配置 IPC（provider:list, provider:get-config, provider:save-config, provider:test-connection, provider:get-models）
4. electron/ipc/skill.ipc.ts — Skill IPC（skill:list, skill:invoke, skill:toggle, skill:create, skill:delete）
5. electron/ipc/workspace.ipc.ts — Workspace 文件操作 IPC（workspace:read-file, workspace:write-file, workspace:list-dir, workspace:watch）
6. electron/ipc/session.ipc.ts — Session 管理 IPC（session:list, session:get, session:resume, session:delete）
7. electron/ipc/mcp.ipc.ts — MCP Server IPC（mcp:list-servers, mcp:add-server, mcp:remove-server, mcp:toggle-server, mcp:test-connection, mcp:get-status）— Phase 1 先定义接口，mcp handler 内部返回空/placeholder
8. electron/preload.ts 更新 — 在 contextBridge.exposeInMainWorld 中暴露所有 IPC channel
9. src/types/electron-api.d.ts — TypeScript 类型定义，声明 window.electronAPI 的完整类型

关键约束：
- 所有 IPC handler 在 electron/ipc/ 中按功能域拆分注册
- Renderer 调用 window.electronAPI.xxx()，不直接使用 ipcRenderer
- 每个 channel 名称使用 "domain:action" 格式（如 "chat:send", "skill:invoke"）
- typed channels：src/types/ 中为每个 domain 定义请求/响应类型
- preload.ts 只做桥接，不包含业务逻辑
- mcp.ipc.ts 在 Phase 1 注册所有 channel 但返回 placeholder（Phase 2 填充实现）
```

#### 3.3.11 模块 11：Chat Panel

```
/feature-workflow chat-panel

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.6 AI 对话面板完整定义（面板行为、Provider 选择器、消息流、HITL 审批、Skill 调用、Agent Monitor、对话历史、上下文感知、文件输出路由 — 共 9 个功能点的详细说明）
- analyst-pro-prd.md §5.1 布局架构（push layout ASCII 示意图 + CSS grid 规格：关闭时 248px 1fr 24px，打开时 248px 1fr minmax(320px, 480px)）
- analyst-pro-prd.md §2.3 目录结构中 src/components/chat/ 部分（ChatPanel.tsx, ChatHeader.tsx, ConversationArea.tsx, MessageBubble.tsx, ChatInput.tsx, SkillPalette.tsx, HITLApproval.tsx, AgentMonitorCollapsible.tsx 共 8 个组件）
- analyst-pro-prd.md §6.6 SkillPalette 交互规范
- analyst-pro-prd.md §3.6 HITL 实现 — 方式 1 "HITLApprovalProps 接口" 定义

目标：
1. src/components/chat/ChatPanel.tsx — 面板主容器（右侧 push layout，折叠/展开，拖拽缩放 280px-50% viewport，Cmd+J 切换，状态持久化到 Zustand）
2. src/components/chat/ChatHeader.tsx — 顶部 Provider 选择器下拉菜单 + 模型 badge + × 关闭按钮
3. src/components/chat/ConversationArea.tsx — 消息流滚动区域（自动滚底，加载历史消息）
4. src/components/chat/MessageBubble.tsx — 单条消息渲染（react-markdown + 语法高亮，subagent 消息带分色 badge，token 数 + 费用显示，workspace 路径链接可点击）
5. src/components/chat/ChatInput.tsx — 输入框（自动扩展高度，Cmd+Enter 发送，/ 触发 SkillPalette）
6. src/components/chat/SkillPalette.tsx — Skill 自动补全弹窗（按 ap-type 分组：🤖Agent / 📝Template / ⚡Action，模糊搜索，非 Claude 模式 agent Skills 灰色禁用）
7. src/components/chat/HITLApproval.tsx — HITL 审批内联卡片（琥珀色边框，question + 确认/拒绝按钮，发送 macOS 通知）
8. src/components/chat/AgentMonitorCollapsible.tsx — Chat 底部可折叠 Agent 监控（折叠时单行状态，展开时 session 列表）
9. src/stores/chatStore.ts — Zustand store（conversations, messages, activeConversationId, panelOpen, panelWidth）

关键约束：
- Push layout：Chat 打开时主内容区收缩，不是 overlay 浮层
- 折叠按钮在主内容区右边缘（关闭时 ▶，打开时 ◀）
- SkillPalette 中 agent 类型 Skill 仅 Claude (Secretary) 模式可用，其他 provider 灰色 + 提示文字
- HITL 审批卡片使用琥珀色边框样式，同时触发 macOS Notification
- Provider 切换时自动开始新对话（conversations 表新建记录）
- 消息持久化通过 IPC 调用 chat:send / chat:history
```

#### 3.3.12 模块 12：Pipeline View

```
/feature-workflow pipeline-view

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.1 项目流程（投资全流程看板的完整描述：看板视图、卡片内容、项目详情页、拖拽交互、StageGatePanel 内嵌）
- analyst-pro-prd.md §5.0 信息架构（一级菜单 + 二级阶段标签：项目寻源、立项审批、投资执行、投后与退出 四大阶段及其子阶段）
- analyst-pro-prd.md §4.1 deals 表 SQL 定义（id, company_name, sector, stage, source, valuation_rmb, round, red_flags, score 等字段）
- analyst-pro-prd.md §4.3.5 阶段输入门禁（各阶段必需关联材料列表）
- analyst-pro-prd.md §2.3 目录结构中 src/components/pipeline/ 部分（Pipeline.tsx, DealCard.tsx, ICMemoViewer.tsx）

目标：
1. src/components/pipeline/Pipeline.tsx — 看板主视图（二级阶段标签过滤器，responsive grid 布局，按 stage 分列显示 DealCard）
2. src/components/pipeline/DealCard.tsx — 项目卡片（公司名、当前阶段 badge、红旗数量、最近更新时间、下一步动作按钮，可拖拽）
3. src/components/pipeline/DealDetail.tsx — 项目详情页（报告全文查看、资料引用列表、操作历史时间线）
4. src/components/pipeline/StageGatePanel.tsx — 阶段门禁面板（内嵌详情页，显示当前阶段所需材料、缺失项高亮、通过/驳回/备注按钮）
5. src/components/pipeline/ICMemoViewer.tsx — IC Memo 查看器（Markdown 渲染 + YAML frontmatter 元数据展示）
6. src/stores/pipelineStore.ts — Zustand store（deals 列表, 当前筛选阶段, 活跃 dealId）

关键约束：
- deals.stage 8 阶段枚举：inbox → basic_info → deep_analysis → ic_memo → approved → rejected（加投后/退出）
- 二级过滤标签对应 §5.0 的 4 大阶段分组，不是 8 个 stage 逐个显示
- 拖拽改变阶段需要触发对应 subagent 工作流（通过 IPC 调用 agent:query）
- StageGatePanel 内嵌在项目详情页中，不是独立的右侧面板
- 数据通过 IPC 调用 SQLite deals CRUD
```

#### 3.3.13 模块 13：Intel Feed

```
/feature-workflow intel-feed

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.3 市场情报（完整描述：Intel Feed, Daily Brief, Policy Alert, VC Moves 四个子视图 + 标记功能）
- analyst-pro-prd.md §4.1 intel_items 表 SQL 定义（id, title, source_url, sector, score, category, content_summary, published_at, ingested_at）
- analyst-pro-prd.md §3.3.1 market-intel agent 的输出路径（state/intelligence/daily_brief.md, vc_moves.md, policy_alerts.md）
- analyst-pro-prd.md §2.3 目录结构中 src/components/intel/ 部分（IntelFeed.tsx, DailyBrief.tsx, PolicyAlert.tsx）

目标：
1. src/components/intel/IntelFeed.tsx — 市场情报主页面（独立一级菜单视图，Tab 切换 4 个子视图）
2. src/components/intel/IntelList.tsx — 情报列表（按评分排序 1-5 分，显示分色评分 badge，标题 + 来源 + 日期 + 摘要）
3. src/components/intel/DailyBrief.tsx — 每日情报简报渲染（Markdown，由 market-intel agent 自动生成）
4. src/components/intel/PolicyAlert.tsx — 政策/监管预警列表（高优先级红色标记）
5. src/components/intel/VCMoves.tsx — VC 机构动向（表格视图：机构名 + 动作 + 金额 + 赛道）
6. src/stores/intelStore.ts — Zustand store（items, 当前 category 筛选, 已读/关注/归档状态）

关键约束：
- 市场情报是独立一级菜单，每日高频使用入口
- 评分 badge 分色：5=红色(Critical), 4=橙色, 3=黄色, 2=灰色, 1=浅灰
- 支持标记"已读/关注/归档"三种状态
- 数据来源：SQLite intel_items 表（结构化索引）+ workspace state/intelligence/*.md（agent 原始输出）
- Intel Feed 默认按发布时间倒序，可切换为按评分排序
```

#### 3.3.14 模块 14：Workspace View

```
/feature-workflow workspace-view

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.4 工作台（完整描述：KPI Tracker + Daily Todo + Weekly Plan + 报告中心 + 知识库 五个子视图）
- analyst-pro-prd.md §4.1 kpi_records 表 SQL 定义（metric_name, target_value, actual_value, period, recorded_at）
- analyst-pro-prd.md §5.4 AI 需求到菜单映射表中工作台相关的 R2/R4/R5/R8/R18 映射
- analyst-pro-prd.md §2.3 目录结构中 src/components/workspace/ 和 src/components/knowledge/ 部分

目标：
1. src/components/workspace/WorkspaceView.tsx — 工作台主容器（Tab 或 Grid 布局切换子视图）
2. src/components/workspace/KPITracker.tsx — KPI 追踪面板（年度/季度指标进度条，目标值 vs 实际值，来自 kpi_records 表）
3. src/components/workspace/DailyTodo.tsx — 今日待办清单（从 workspace/inbox/ 和 state/ 汇总，支持勾选完成）
4. src/components/workspace/WeeklyPlan.tsx — 周计划查看（Markdown 渲染 workspace 中的周计划文件）
5. src/components/workspace/ReportCenter.tsx — 报告中心（按类型分组：基本信息报告 / IC Memo / 投后报告 / LP 报告，链接到 workspace 文件）
6. src/components/knowledge/KnowledgeExplorer.tsx — 知识库目录浏览器（树形视图浏览 workspace/knowledge/）
7. src/components/knowledge/GlossaryViewer.tsx — 术语库查看器（渲染 knowledge/glossary.md）
8. src/stores/workspaceStore.ts — Zustand store（kpiRecords, todos, 活跃子视图标签）

关键约束：
- 工作台是一级菜单，包含 5 个子视图，默认展示 KPI + Todo 概览
- KPI 数据来自 SQLite kpi_records 表，Todo 数据聚合自 workspace 文件
- 报告中心统一查阅所有 agent 生成的报告，按 file_outputs 表的 report_type 分组
- 知识库浏览 workspace/knowledge/ 目录，支持 Markdown 文件在线查看
```

#### 3.3.15 模块 15：HITL System

```
/feature-workflow hitl-system

请参考以下文件和章节：
- analyst-pro-prd.md §3.6 HITL 实现（两种机制的完整说明 + TypeScript 代码）：
  - 方式 1：AskUserQuestion tool（Subagent 内部触发）— HITLApprovalProps 接口定义
  - 方式 2：canUseTool hook（全局权限控制）— 完整 canUseTool 回调函数代码（Bash 全确认 + ic_memos/lp_reports 写入确认）
- analyst-pro-prd.md §3.7 Hook 系统设计表（7 种 Hook 事件，其中 PreToolUse(Bash) 和 PreToolUse(Write|Edit) 是 HITL 相关）
- analyst-pro-prd.md §4.1 hitl_approvals 表 SQL 定义（agent_type, action_type, question, user_decision, context）
- analyst-pro-prd.md §5.3 交互设计要点（HITL 审批的 3 种反馈渠道：对话流内卡片 + badge + macOS 通知）
- AGENTS.md "HITL Enforcement" 章节

目标：
1. electron/agents/hooks/hitl.hook.ts 更新 — 完善 canUseTool 实现：
   - Bash 命令全部拦截弹确认
   - Write/Edit 到 ic_memos/ 或 lp_reports/ 路径拦截弹确认
   - 返回 { behavior: "allow" } 或 { behavior: "deny", message }
2. electron/ipc/hitl.ipc.ts — HITL 专用 IPC（hitl:approve, hitl:reject, hitl:pending-list）
3. src/components/chat/HITLApproval.tsx 更新 — 完善审批卡片 UI：
   - 琥珀色边框卡片
   - 显示 agentName + question + options
   - 确认/拒绝按钮 + 拒绝时的理由输入
   - 触发 macOS 桌面通知（通过 Electron Notification API）
4. electron/db/hitl.ts — hitl_approvals 表 CRUD（记录每次审批决定供审计）
5. 桌面通知集成 — Electron main process 使用 new Notification() 推送 HITL 请求

关键约束：
- IC Memo、LP 材料、Bash shell 命令必须人工确认——这是非协商的安全要求
- HITL 审批记录必须持久化到 SQLite hitl_approvals 表（审计追踪）
- AskUserQuestion 是 SDK 原生 tool，canUseTool 是 hook 回调——两种机制并存互补
- HITL 请求必须同时在 Chat 面板内联显示和发送 macOS 桌面通知
- 用户拒绝后 agent 收到 deny message，不会重试同一操作
```

#### 3.3.16 模块 16：Keychain Auth

```
/feature-workflow keychain-auth

请参考以下文件和章节：
- analyst-pro-prd.md §8.1 数据安全表（"API Key → macOS Keychain (via keytar)" 条目）
- analyst-pro-prd.md §5.2.5 系统设置 — "AI 模型配置" 段（5 种 Provider 的认证方式：OAuth/API Key，API Key 存储于 macOS Keychain (safeStorage)，不入 SQLite）
- analyst-pro-prd.md §4.1 provider_configs 表注释 "注意：API keys 和 OAuth tokens 存于 macOS Keychain，不入库"
- analyst-pro-prd.md §4.3.4 mcp_servers 表注释 "OAuth tokens 存于 macOS Keychain，不入库"
- AGENTS.md "Electron Security (Non-negotiable)" 章节

目标：
1. electron/auth/keychain.ts — KeychainService 类：
   - setApiKey(provider: ProviderType, key: string) — 存储 API key 到 macOS Keychain
   - getApiKey(provider: ProviderType) — 从 Keychain 读取
   - deleteApiKey(provider: ProviderType) — 删除
   - hasApiKey(provider: ProviderType) — 检查是否存在
   - 使用 Electron 的 safeStorage API（优先）或 keytar 库
2. electron/auth/oauth.ts — OAuth PKCE 流程骨架（Phase 1 定义接口，实际 OAuth 对接在 Phase 2 实现）
3. electron/ipc/auth.ipc.ts — 认证 IPC（auth:set-key, auth:has-key, auth:delete-key, auth:test-key）
4. src/components/settings/ProviderSettings.tsx — Provider 配置面板 UI（API key 输入框 + [Save] + [Test Connection] + 状态指示器）

关键约束：
- API keys 和 OAuth tokens 绝对不能存入 SQLite 或任何明文文件
- 优先使用 Electron safeStorage API（基于 macOS Keychain），无需额外依赖
- provider_configs 表中通过 is_enabled 和 label 字段识别 provider，key 通过 KeychainService 单独获取
- Test Connection 按钮调用 provider 的 testConnection() 方法验证 key 有效性
- Phase 1 只实现 Claude API key 存储，其他 provider 的 key 管理接口先定义好
```

#### 3.3.17 模块 17：Output Router

```
/feature-workflow output-router

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.6 AI 对话面板 — "文件输出路由" 段（OutputRouter 按上下文自动路由规则：项目相关→deal 归档目录，情报→state/intelligence/，默认→outputs/{date}/；所有文件包含 YAML frontmatter）
- analyst-pro-prd.md §4.1 file_outputs 表 SQL 定义（conversation_id, message_id, file_path, report_type, deal_id, agent_name, provider, model）
- analyst-pro-prd.md §4.3.1 归档目录规范（shallow/key 双轨路径）
- analyst-pro-prd.md §4.3.2 上传后自动处理 — 第 5 步 "台账登记" 和第 6 步 "引用绑定"

目标：
1. electron/workspace/output-router.ts — OutputRouter 类：
   - routeOutput(context: OutputContext) → 根据 agent 类型和当前对话上下文确定输出路径：
     - deal-analyst 输出 → state/deals/processing/[company]/ 或 state/deals/archive/
     - market-intel 输出 → state/intelligence/
     - industry-researcher 输出 → state/research/[sector]/
     - hardtech-dd 输出 → state/deals/techdd/[company]/
     - 默认 → outputs/{YYYY-MM-DD}/
   - addFrontmatter(content, metadata) → 给每个输出文件添加 YAML frontmatter（agent_name, model, conversation_id, deal_id, created_at, report_type）
2. electron/db/file-outputs.ts — file_outputs 表 CRUD（registerOutput, getOutputsByConversation, getOutputsByDeal）
3. electron/workspace/output-router.test.ts — 路由规则测试（每种 agent 类型 → 正确路径，frontmatter 格式验证）

关键约束：
- 所有 agent 生成的文件必须经过 OutputRouter（不允许 agent 直接写任意路径）
- 每个输出文件必须包含 YAML frontmatter 元数据（供后续检索和溯源）
- file_outputs 表记录每个文件的完整来源链路（conversation → message → file）
- 路由规则基于 agent_name 和 deal_id 判断，不基于文件内容
- outputs/{date}/ 是 fallback 路径，未匹配任何规则时使用
```

### 3.4 单模块执行流程（含上下文喂入）

对于每个模块，执行以下步骤：

```
# ══════════════════════════════════════════════════════
# Step 0: 准备上下文指引（最关键的一步）
# ══════════════════════════════════════════════════════

# 查上面 §3.3 的表格，找到该模块对应的 PRD 章节和关键约束
# 把它们写成一段自然语言描述

# ══════════════════════════════════════════════════════
# Step 1: 启动 feature-workflow + 上下文
# ══════════════════════════════════════════════════════

/feature-workflow sqlite-schema

请参考以下文件：
- analyst-pro-prd.md §4.1 SQLite Schema（从 deals 表到 file_outputs 表的完整 SQL 定义）
- analyst-pro-prd.md §4.3.3 Skills 元数据缓存（skills 表）
- analyst-pro-prd.md §4.3.4 MCP Server 配置（mcp_servers 表）
- analyst-pro-prd.md §4.3.6 数据表扩展（archive_documents + stage_input_bundles 表）
- AGENTS.md "Data Separation" 章节

目标：
1. electron/db/schema.ts — 全部表定义 + 索引（9 张表）
2. electron/db/deals.ts — deals CRUD (create/get/update/list/updateStage)
3. electron/db/intel.ts — intel_items CRUD
4. electron/db/sessions.ts — agent_sessions + conversations + chat_messages CRUD
5. 使用 better-sqlite3，测试用 in-memory database
6. 包含 schema 版本管理机制（migration）

关键约束：
- API keys 和 OAuth tokens 不入 SQLite（存 Keychain）
- SQLite 存结构化数据，Markdown workspace 存非结构化数据，两者不重复

# ══════════════════════════════════════════════════════
# Step 2: 审阅 Planner 产出的计划
# ══════════════════════════════════════════════════════

# Planner 会生成 docs/plans/YYYYMMDD-HHMM-sqlite-schema.md
# 仔细检查：
#   - Work Items 是否覆盖了你列出的所有目标？
#   - 有没有遗漏 PRD 中的表？
#   - 测试计划是否用了 in-memory SQLite？
# 如果有问题，直接指出让 Planner 修改

# ══════════════════════════════════════════════════════
# Step 3: 后续 8 个 Agent 自动接力
# ══════════════════════════════════════════════════════

#   2) Spec Guardian → 验证计划符合 AGENTS.md 规则
#   3) Impact Analyst→ 确认影响的文件集合
#   4) Implementer   → TDD 实现（RED → GREEN → REFACTOR）
#   5) Test Runner   → 运行 npm run check:all
#   6) Auditor       → 审查代码质量
#   7) Manual Test   → 更新手动测试文档
#   8) Verifier      → 最终验证
#   9) Release Steward→ 提议 commit（等你确认）

# ══════════════════════════════════════════════════════
# Step 4: 确认提交
# ══════════════════════════════════════════════════════

# Release Steward 展示 commit message → 你说 "accept + commit"

# ══════════════════════════════════════════════════════
# Step 5: 提交后审计（可选但推荐）
# ══════════════════════════════════════════════════════

/audit-fix commit -1
```

### 3.5 上下文喂入模板（复制即用）

每次启动 `/feature-workflow` 时，复制以下模板并填写：

```
/feature-workflow [module-slug]

请参考以下文件：
- analyst-pro-prd.md §[章节号] [章节名]
- analyst-pro-prd.md §[章节号] [章节名]
- AGENTS.md "[相关段落]" 章节

目标：
1. [具体产出文件1] — [做什么]
2. [具体产出文件2] — [做什么]
3. ...

关键约束：
- [不可违反的规则1]
- [不可违反的规则2]
```

> **经验法则**：上下文指引的长度应该在 5-15 行之间。太短 → Planner 信息不足会猜测；太长 → 等于把 PRD 复制一遍，失去了 feature-workflow 的自动化优势。关键是**指明文件路径和章节号**，让 Planner 用 `Read` 工具自己去读原文。

### 3.6 每个模块完成后的检查清单

```
□ feature-workflow 9 个阶段全部通过
□ npm run check:all 绿灯
□ CHANGELOG.md 已更新
□ PROGRESS.md 已记录经验教训（如有）
□ /audit-fix commit -1 通过（可选）
□ 文档已更新（如影响用户行为）
```

---

## 4. 备选方案：Hybrid（Speckit + Feature Workflow）

如果你想先生成一个总体任务清单再逐模块执行，可以用 Hybrid 方案。

### 4.1 Phase A：用 Speckit 生成总体规划（只跑一次）

```bash
# 1. 生成规格文档（基于 PRD 内容）
/speckit.specify AnalystPro Phase 1 MVP - Electron app with Agent Engine, Secretary, market-intel, deal-analyst, Chat UI, Skill system

# 2. 澄清模糊点（最多 5 个问题）
/speckit.clarify

# 3. 生成技术计划
/speckit.plan

# 4. 生成需求质量清单
/speckit.checklist

# 5. 生成任务列表（T001 格式）
/speckit.tasks

# 6. 一致性分析（只读，不修改）
/speckit.analyze
```

产出文件（在 `specs/<feature>/` 目录下）：

```
specs/analyst-pro-mvp/
├── spec.md            # 功能规格
├── plan.md            # 技术计划
├── research.md        # 技术调研
├── data-model.md      # 数据模型
├── tasks.md           # 任务列表（T001-T0xx）
├── checklists/        # 需求质量清单
│   ├── requirements.md
│   └── ux.md
├── contracts/         # API 契约
└── diagrams/          # 特性图表
```

### 4.2 Phase B：用 Feature Workflow 逐模块执行

Speckit 的 `tasks.md` 作为**参考文档**（不自动消费），每个模块用 feature-workflow 独立执行：

```bash
# 参考 tasks.md 中的 T001-T003（Electron scaffold 相关）
/feature-workflow electron-scaffold

# 参考 tasks.md 中的 T004-T006（SQLite schema 相关）
/feature-workflow sqlite-schema

# ... 每完成一个，手动在 tasks.md 中标记 [X]
```

### 4.3 Hybrid 的权衡

| 维度   | 优势                       | 劣势                                 |
| ---- | ------------------------ | ---------------------------------- |
| 可追踪性 | 有总体任务清单可跟踪进度             | tasks.md 需要手动维护                    |
| 规格验证 | Speckit 的 analyze 能发现不一致 | 前期需要额外投入                           |
| 质量保证 | 两层质量检查                   | Feature workflow 不会自动消费 Speckit 产出 |

---

## 5. 实战演练：Phase 1 MVP 完整开发流程

以下是完整的 Phase 1 开发流程，使用推荐的 **Feature Workflow Only** 方案。

### 5.0 项目初始化

```bash
# 确保 git 仓库干净
git status -sb

# 确认 PRD 和规则文件就位
ls analyst-pro-prd.md CLAUDE.md AGENTS.md

# 确认 Claude Code 工具链就位
ls .claude/commands/feature-workflow.md
ls .claude/agents/planner.md
ls .claude/skills/planning/SKILL.md
```

### 5.1 模块 1：Electron 脚手架

```
/feature-workflow electron-scaffold

请参考以下文件：
- analyst-pro-prd.md §2.2 技术栈（Electron 34+, electron-vite, React 18, TailwindCSS, shadcn/ui）
- analyst-pro-prd.md §2.3 目录结构（electron/ 和 src/ 的完整结构）
- analyst-pro-prd.md §8.2 Electron 安全最佳实践（contextIsolation, sandbox, CSP）
- AGENTS.md "Electron Security" 和 "Project Structure" 章节

目标：
1. package.json + electron-vite + TypeScript strict mode
2. electron/main.ts（contextIsolation: true, nodeIntegration: false, sandbox: true）
3. electron/preload.ts + contextBridge 桥接骨架
4. src/App.tsx + TailwindCSS + shadcn/ui 基础主题
5. CSP header 配置
6. npm run check:all 脚本（eslint + vitest + electron-vite build）

关键约束：
- Electron 安全三件套不可省略：contextIsolation + nodeIntegration:false + sandbox
- 所有 Renderer→Main 通信必须走 contextBridge
```

**Planner 产出示例**（`docs/plans/YYYYMMDD-HHMM-electron-scaffold.md`）：

```markdown
## Outcomes
- Electron 34+ 项目结构初始化
- electron-vite + React 18 + TailwindCSS + shadcn/ui 配置
- contextIsolation: true, nodeIntegration: false, sandbox: true
- 基础 preload.ts 和 contextBridge 桥接
- npm run check:all 可执行（eslint + vitest + electron-vite build）

## Work Items

### WI-001: 项目初始化和构建工具链
- Goal: package.json + electron-vite 配置 + TypeScript strict mode
- Acceptance: `electron-vite build` 成功
- Tests: vitest 配置 + 空测试通过
- Estimate: M

### WI-002: Electron Main Process 骨架
- Goal: main.ts + preload.ts + BrowserWindow 创建
- Acceptance: 启动后显示空白窗口
- Tests: main process 启动逻辑单元测试
- Estimate: S

### WI-003: Renderer React 骨架
- Goal: App.tsx + TailwindCSS + shadcn/ui 主题
- Acceptance: 渲染 "AnalystPro" 标题
- Tests: App.test.tsx 渲染测试
- Estimate: S

### WI-004: 安全配置验证
- Goal: contextIsolation + sandbox + CSP header
- Acceptance: DevTools 控制台无安全警告
- Tests: 安全配置单元测试
- Estimate: S
```

**你需要做的**：

1. Planner 产出计划后，审阅并确认

2. 等待 9 个 Agent 依次执行

3. Release Steward 提议 commit 时，审阅后说 "accept + commit"

4. 可选：`/audit-fix commit -1` 做一次快速审计

### 5.2 模块 2：SQLite Schema

```
/feature-workflow sqlite-schema

请参考以下文件：
- analyst-pro-prd.md §4.1 SQLite Schema（从 deals 表到 file_outputs 表，共 9 张核心表的完整 SQL）
- analyst-pro-prd.md §4.3.3 Skills 元数据缓存（skills 表 + 索引）
- analyst-pro-prd.md §4.3.4 MCP Server 配置（mcp_servers 表）
- analyst-pro-prd.md §4.3.6 数据表扩展（archive_documents + stage_input_bundles）
- AGENTS.md "Data Separation" 章节

目标：
1. electron/db/schema.ts — 全部表定义 + 索引
2. electron/db/deals.ts — deals CRUD
3. electron/db/intel.ts — intel_items CRUD
4. electron/db/sessions.ts — agent_sessions + conversations + chat_messages CRUD
5. Schema 版本管理（migration 机制）

关键约束：
- 使用 better-sqlite3，测试用 in-memory database
- API keys / OAuth tokens 不入 SQLite（存 macOS Keychain）
- SQLite 存结构化数据，Markdown workspace 存非结构化数据，不重复
```

Planner 会读取 PRD 中的完整 SQL 定义，生成包含以下 Work Items 的计划：

- WI-001: `electron/db/schema.ts` — 全部表定义 + 索引

- WI-002: `electron/db/deals.ts` — deals CRUD

- WI-003: `electron/db/intel.ts` — intel\_items CRUD

- WI-004: `electron/db/sessions.ts` — agent\_sessions + conversations + chat\_messages CRUD

- WI-005: Schema 迁移机制（版本管理）

每个 WI 都会先写测试（in-memory SQLite），再实现代码。

### 5.3 模块 3-17：后续模块

按照 §3.3 的表格，每个模块都带上对应的 **PRD 章节引用 + 目标 + 约束**。流程一致：

```
/feature-workflow [module-name]

请参考以下文件：
- analyst-pro-prd.md §[查表填入]
- AGENTS.md "[查表填入]" 章节

目标：
1. [查表填入]

关键约束：
- [查表填入]

→ 审阅 Planner 计划（重点核对是否遗漏 PRD 中的规格）
→ 等待 9 Agent 执行
→ 确认提交
→ /audit-fix commit -1
```

### 5.4 Phase 1 完成后的总体验证

```bash
# 1. 运行完整质量门禁
/release-gate

# 2. 验证所有计划的完成度
/plan-verify

# 3. 可选：Codex 独立审计
/codex-audit --full

# 4. 检查 PROGRESS.md 中积累的经验教训
cat PROGRESS.md
```

---

## 6. 单个模块开发示例：Agent Engine

以 `agent-engine` 模块为例，详细展示 9 个 Agent 的工作过程。

### Step 1: 启动（含上下文指引）

```
/feature-workflow agent-engine

请参考以下文件：
- analyst-pro-prd.md §3.2 Agent 引擎核心（完整 engine.ts 伪代码，包含 query() 封装、streaming、hooks 注册）
- analyst-pro-prd.md §3.5 Session 管理与上下文持久化（SessionConfig 接口）
- analyst-pro-prd.md §3.7 Hook 系统设计（7 种 Hook 事件表）
- analyst-pro-prd.md §3.4 与 OpenClaw Subagent 的对应关系（SDK 映射参考）
- AGENTS.md "Agent SDK Architecture" 章节

目标：
1. electron/agents/types.ts — AgentConfig, QueryOptions, StreamMessage 等核心类型
2. electron/agents/engine.ts — 封装 SDK query()，支持 streaming、session resume、maxBudgetUsd
3. electron/agents/hooks/hitl.hook.ts — HITL 审批 hook（canUseTool 实现）
4. electron/agents/hooks/audit.hook.ts — 文件变更审计日志
5. electron/agents/hooks/cost.hook.ts — 成本监控
6. electron/agents/secretary.ts — Secretary 主 agent 配置（系统提示合成 + subagent 注册）

关键约束：
- 使用 @anthropic-ai/claude-agent-sdk 的 query() API
- Secretary 默认 model: sonnet
- Bash 命令 / 写入 ic_memos/ 或 lp_reports/ 必须走 HITL 确认
- maxBudgetUsd 限制单次对话成本
- cwd 指向 workspace 目录
```

### Step 2: Planner Agent 工作

Planner 会：

1. 用 `Read` 工具读取 PRD §3.2（因为你明确指定了章节）

2. 读取 `AGENTS.md` 中的 Agent SDK 架构规则

3. 研究 `@anthropic-ai/claude-agent-sdk` 的 API（基于 PRD 伪代码）

4. 产出计划文件 `docs/plans/YYYYMMDD-HHMM-agent-engine.md`

```markdown
## Work Items

### WI-001: AgentEngine 核心类型定义
- Goal: 定义 AgentConfig, QueryOptions, StreamMessage 等类型
- Tests: electron/agents/types.test.ts — 类型导出验证
- Touched: electron/agents/types.ts
- Estimate: S

### WI-002: engine.ts 核心实现
- Goal: 封装 SDK query()，支持 streaming、session resume、budget control
- Tests: electron/agents/engine.test.ts — mock SDK query()
- Touched: electron/agents/engine.ts
- Dependencies: WI-001
- Estimate: M

### WI-003: Hook 系统注册
- Goal: PreToolUse/PostToolUse/Stop hook 注册框架
- Tests: electron/agents/hooks/*.test.ts
- Touched: electron/agents/hooks/
- Dependencies: WI-002
- Estimate: M

### WI-004: Secretary agent 配置
- Goal: Secretary 系统提示 + subagent 定义注册
- Tests: electron/agents/secretary.test.ts
- Touched: electron/agents/secretary.ts
- Dependencies: WI-002
- Estimate: S
```

### Step 3: Spec Guardian 验证

Spec Guardian 检查：

- ✅ 遵循 `contextIsolation` 安全规则（agent 运行在 main process）

- ✅ 文件在 `electron/agents/` 目录下（符合目录结构规范）

- ✅ 使用 `@anthropic-ai/claude-agent-sdk`（符合技术栈要求）

- ✅ 每个文件 < 300 行

- ✅ TDD 流程已规划

### Step 4: Impact Analyst 映射

```
WI-001 → 新建 electron/agents/types.ts
WI-002 → 新建 electron/agents/engine.ts
         依赖: electron/agents/types.ts, @anthropic-ai/claude-agent-sdk
WI-003 → 新建 electron/agents/hooks/hitl.hook.ts
         新建 electron/agents/hooks/audit.hook.ts
         新建 electron/agents/hooks/cost.hook.ts
WI-004 → 新建 electron/agents/secretary.ts
         依赖: engine.ts, hooks/
```

### Step 5: Implementer 执行 TDD

对于 WI-001，Implementer 会：

```
1. Preflight: 确认 types.ts 不存在，需要新建
2. RED: 创建 electron/agents/types.test.ts
   - 测试 AgentConfig 类型导出
   - 测试 QueryOptions 类型导出
   - 运行测试 → 失败 ❌
3. GREEN: 创建 electron/agents/types.ts
   - 定义 AgentConfig, QueryOptions, StreamMessage 等
   - 运行测试 → 通过 ✅
4. REFACTOR: 清理，确认类型定义简洁
```

### Step 6-9: 质量保障链

```
Test Runner  → npx vitest run + npm run check:all
Auditor      → 检查代码质量，如有问题循环回 Implementer
Manual Test  → 更新 docs/testing/manual-testing-guide.md
Verifier     → 最终验证所有门禁
Release Steward → 提议 commit message:
  feat(agents): add Agent Engine core with SDK query wrapper
  - Define AgentConfig and QueryOptions types
  - Implement engine.ts with streaming and session resume
  - Register hook system (HITL, audit, cost)
  - Configure Secretary agent with subagent definitions
```

---

## 7. Bug 修复：/fix 命令

当开发过程中遇到 bug 时，使用 `/fix` 命令。

### 7.1 使用方式

```bash
/fix Secretary agent 在 resume session 时抛出 "session not found" 错误
```

### 7.2 执行流程

```
1. Reproduce — 读取 engine.ts，追踪 session resume 的调用链
2. Diagnose  — 发现 SQLite 中 session_id 存储格式不匹配
3. RED        — 写失败测试 engine.test.ts: "resume with invalid session"
4. GREEN      — 修复 session ID 序列化逻辑
5. REFACTOR   — 清理相关代码
6. Verify     — npm run check:all 全绿
```

### 7.3 适用场景

| 场景           | 用 `/fix` | 用 `/feature-workflow` |
| ------------ | :------: | :-------------------: |
| 单个已知 bug     |     ✅    |           —           |
| 需要修改 1-3 个文件 |     ✅    |           —           |
| 新增功能模块       |     —    |           ✅           |
| 重构涉及多个文件     |     —    |           ✅           |
| 修复后可能影响多个模块  |    看情况   |           ✅           |

---

## 8. 代码审计：/audit-fix 命令

### 8.1 使用方式

```bash
# 审计未提交的更改
/audit-fix

# 审计最近一次提交
/audit-fix commit -1

# 审计最近 3 次提交
/audit-fix commit -3

# 审计暂存区
/audit-fix staged

# 审计特定文件/目录
/audit-fix electron/agents/
```

### 8.2 审计维度（7 项）

| 维度                  | 检查内容                                    |
| ------------------- | --------------------------------------- |
| Correctness & logic | 逻辑是否正确，有无绕过症状的补丁                        |
| Edge cases          | 边界条件、null/空值、Unicode/CJK、并发             |
| Security            | 注入、XSS、路径穿越、SQL 注入                      |
| Duplicate code      | 复制粘贴、重复逻辑                               |
| Dead code           | 未使用的导入、不可达分支、孤立函数                       |
| Shortcuts & patches | 临时方案、TODO 标记、旁路标志                       |
| Project compliance  | 是否符合 `.claude/rules/*.md` 和 `AGENTS.md` |

### 8.3 推荐使用时机

```
每完成一个 /feature-workflow 模块后:
  /audit-fix commit -1

每天结束工作前:
  /audit-fix

发布前:
  /audit-fix commit -10
  /codex-audit --full    # 可选：Codex 独立审计
```

---

## 9. Planning 和 Audit 技能（Skills）

这些技能可以在任何时候手动调用，独立于 feature-workflow。

### 9.1 `/planning` — 生成结构化计划

当你需要在 feature-workflow 之外单独做规划时使用：

```bash
# 完整计划模式（默认）
/planning 重构 Provider 抽象层以支持 OpenAI 和 Gemini

# 快速计划模式
/planning quick-plan 添加 conversation 搜索功能
```

产出：`docs/plans/YYYYMMDD-HHMM-<topic>.md`，包含 Outcomes、Work Items（WI-001 格式）、Testing Procedures 等。

### 9.2 `/plan-audit` — 检查实现是否符合计划

```bash
# 审计最近的计划
/plan-audit
```

只做检查，不修改代码。产出：Findings 表格（按严重度排序）、Plan Gaps Summary、Test Coverage Gaps。

### 9.3 `/plan-verify` — 运行测试并验证验收标准

```bash
# 验证最近的计划
/plan-verify
```

会运行测试！产出：Verification Summary、Gate Results、Acceptance Matrix（每个 WI 的每个验收标准 → Pass/Fail/Blocked）。

### 9.4 `/release-gate` — 运行完整质量门禁

```bash
/release-gate
```

执行 `npm run check:all`（eslint + vitest + electron-vite build），报告每个步骤的通过/失败状态。

### 9.5 技能组合使用

```bash
# 典型的模块完成后验证流程
/plan-verify            # 验证实现符合计划
/release-gate           # 运行完整门禁
/audit-fix commit -1    # 审计最近提交
```

---

## 10. Speckit 命令套件详解（备选路径）

如果你选择 Hybrid 方案或需要处理需求模糊的新功能，以下是 Speckit 8 个命令的详细用法。

### 10.1 完整管线流程

```
speckit.specify  →  speckit.clarify  →  speckit.plan
     ↓                                      ↓
  spec.md                              plan.md + research.md
                                            ↓
                    speckit.checklist  →  speckit.tasks
                         ↓                   ↓
                    checklists/*.md       tasks.md
                                            ↓
                                     speckit.analyze (只读)
                                            ↓
                                     speckit.implement (执行)
```

### 10.2 `/speckit.specify` — 生成规格文档

```bash
/speckit.specify 添加知识库浏览和编辑功能，支持行业知识图谱和术语库
```

产出：

- 创建 git 分支 `feat/NNN-knowledge-explorer`

- 创建 `specs/NNN-knowledge-explorer/spec.md`

- 创建架构图表 `diagrams/wireframe.html`

### 10.3 `/speckit.clarify` — 澄清模糊需求

```bash
/speckit.clarify
```

- 对 spec.md 进行 10 维度歧义扫描

- 逐个提出最多 5 个澄清问题

- 每个答案立即写回 spec.md

### 10.4 `/speckit.plan` — 生成技术计划

```bash
/speckit.plan
```

两阶段执行：

- Phase 0: 调研未知技术点 → 写入 `research.md`

- Phase 1: 数据模型 + API 契约 + 快速入门 → `data-model.md`, `contracts/`, `quickstart.md`

### 10.5 `/speckit.checklist` — 需求质量清单

```bash
/speckit.checklist
```

生成分类清单文件（`checklists/ux.md`, `checklists/api.md` 等），用于验证需求质量。

### 10.6 `/speckit.tasks` — 生成任务列表

```bash
/speckit.tasks
```

产出 `tasks.md`，格式：

```markdown
## Phase 1: Setup
- [ ] T001 Initialize KnowledgeExplorer component scaffold — `src/components/knowledge/`

## Phase 2: Foundational
- [ ] T002 Create knowledge file parser utility — `electron/workspace/knowledge-parser.ts`
- [ ] T003 [P] Add knowledge-related IPC handlers — `electron/ipc/knowledge.ipc.ts`

## Phase 3: US1 — Knowledge Browsing
- [ ] T004 [US1] Implement KnowledgeExplorer tree view — `src/components/knowledge/KnowledgeExplorer.tsx`
- [ ] T005 [US1] [P] Implement GlossaryViewer component — `src/components/knowledge/GlossaryViewer.tsx`
```

### 10.7 `/speckit.analyze` — 一致性分析

```bash
/speckit.analyze
```

**只读**，不修改任何文件。检测 6 类问题：重复、歧义、规格不足、宪章对齐、覆盖缺口、不一致。

### 10.8 `/speckit.implement` — 执行任务

```bash
/speckit.implement
```

按 Phase 顺序执行 `tasks.md` 中的每个任务，遵循 TDD，完成后标记 `[X]`。

### 10.9 Speckit 的局限性

> **重要**：Speckit 的 `implement` 命令**没有 Feature Workflow 的 9 Agent 质量保障链**（没有 Spec Guardian、Impact Analyst、Auditor、Verifier）。如果你用 Speckit 实现，建议之后补充 `/audit-fix` 审计。

---

## 11. Codex 双模型工作流（可选）

Claude Code 是主力开发者，Codex CLI 作为独立审计者，双模型交叉验证可以捕获单一模型的盲点。

### 11.1 前置条件

```bash
# 安装 Codex CLI
npm install -g @openai/codex

# 登录（推荐使用 ChatGPT Plus/Pro 订阅）
codex login

# 验证连接
/codex-preflight
```

### 11.2 使用时机

| 命令                        | 使用时机           |
| ------------------------- | -------------- |
| `/codex-audit-mini`       | 每完成一个小改动后快速检查  |
| `/codex-audit --full`     | 发布前深度审计        |
| `/codex-audit-fix`        | 审计→修复→验证自动循环   |
| `/codex-bug-analyze "描述"` | 复杂 bug 的独立分析   |
| `/codex-review-plan`      | 让 Codex 审查你的计划 |

### 11.3 推荐组合

```bash
# 每个模块完成后的完整检查
/audit-fix commit -1           # Claude 审计
/codex-audit-mini commit -1    # Codex 独立审计

# 发布前
/release-gate                  # 门禁
/codex-audit --full            # Codex 全量审计
```

---

## 12. Session 上下文管理

AnalystPro 的开发周期长，单次 session 可能无法完成一个完整的 Phase。使用上下文管理技能保持连续性。

### 12.1 保存上下文

当 Claude Code 提示上下文即将压缩时（< 10%），或你主动想暂停时：

```bash
/save-context
```

产出：`.claude/contexts/YYYY-MM-DD_HH-MM-SS.md`，包含：

- 已完成的任务列表

- 进行中的任务（进度%、下一步、阻塞项）

- 待办任务

- 关键决策和模式

### 12.2 恢复上下文

新 session 开始时：

```bash
/load-context
```

自动加载最近的上下文文件，展示恢复摘要，提供继续工作的选项。

### 12.3 最佳实践

```
开始 session:
  /load-context                    # 如果有上一次的上下文

工作中（上下文 < 20% 时）:
  /save-context                    # 主动保存

结束 session:
  /save-context                    # 保存当前进度
```

---

## 13. 完整开发节奏速查表

### 13.1 日常开发节奏

```
┌── 每个模块 ──────────────────────────────────────────────┐
│                                                          │
│  /feature-workflow [module-name]                         │
│       ↓                                                  │
│  审阅 Planner 计划 → 确认                                 │
│       ↓                                                  │
│  等待 9 Agent 执行（自动）                                 │
│       ↓                                                  │
│  Release Steward 提议 commit → "accept + commit"         │
│       ↓                                                  │
│  /audit-fix commit -1          # 事后审计                 │
│       ↓                                                  │
│  更新 CHANGELOG.md + PROGRESS.md                         │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### 13.2 Bug 修复节奏

```
发现 bug → /fix [描述] → 自动 TDD → 提交 → /audit-fix commit -1
```

### 13.3 Phase 完成后的收尾

```
/plan-verify                    # 验证所有计划
/release-gate                   # 完整门禁
/codex-audit --full             # 可选：Codex 独立审计
git tag v0.1.0                  # 打版本标签
```

### 13.4 命令速查

```
开发新模块:      /feature-workflow [name]
修复 bug:        /fix [描述]
审计代码:        /audit-fix [scope]
生成计划:        /planning [描述]
检查计划执行:    /plan-audit
运行验证:        /plan-verify
运行门禁:        /release-gate
保存进度:        /save-context
恢复进度:        /load-context
Codex 审计:      /codex-audit [scope]
Codex 快审:      /codex-audit-mini [scope]
```

---

## 14. FAQ

### Q1: Feature Workflow 和 Speckit 能同时用吗？

可以，但它们不会自动衔接。Speckit 的 `tasks.md`（T001 格式）和 Feature Workflow 的计划（WI-001 格式）是独立的。如果你用 Hybrid 方案，Speckit 的 `tasks.md` 只是参考文档，Feature Workflow 的 Planner 会独立创建自己的计划。

### Q2: Feature Workflow 的 9 个 Agent 可以跳过某个吗？

不可以。9 个 Agent 是严格顺序执行的。如果某个 Agent 发现问题（如 Spec Guardian 发现违规，Auditor 发现缺陷），会循环回之前的阶段修复。

### Q3: 一个 /feature-workflow 应该包含多大的范围？

一个可独立测试、可独立提交的模块。通常对应 PRD 中的一个子系统或一个功能域。如果 Planner 产出超过 8 个 Work Items，考虑拆分为更小的 workflow。

### Q4: /audit-fix 发现问题后需要手动修吗？

不需要。`/audit-fix` 会自动修复所有发现的问题（Phase 3: Fix All），然后验证修复（Phase 4: Verify），最多循环 3 次。如果 3 次后仍有问题，会列出剩余问题让你决定。

### Q5: Codex 审计是必须的吗？

不是必须的，但推荐在发布前使用。Codex 作为独立模型，可以捕获 Claude 的盲点。需要先安装 Codex CLI 并登录。

### Q6: session 断了怎么办？

在 session 即将结束前执行 `/save-context`，下次 session 开始时执行 `/load-context`。如果忘记保存，可以通过 `git log` 和 `PROGRESS.md` 恢复进度。

### Q7: Phase 2-4 的模块也是一样的流程吗？

是的。每个 Phase 的每个模块都使用相同的流程：`/feature-workflow [name]` → 9 Agent 接力 → 提交 → 审计。唯一的区别是模块的内容和复杂度不同。

### Q8: 什么时候用 Speckit 而不是 Feature Workflow？

当需求模糊或正在探索性设计时。例如：

- Phase 4 的 Telegram 集成（用户行为待定义）

- 全新的协作功能（需求待探索）

- 对现有架构的大规模重构（需要先分析影响）

### Q9: 如何处理跨模块的依赖？

按照 §3.2 的执行顺序，基础层先完成，依赖层后完成。如果某个模块依赖另一个未完成的模块，在 feature-workflow 的 Planner 阶段会识别并提示。

### Q10: AnalystPro 内置的 Skills（/today, /deal 等）和这里的开发命令是什么关系？

完全不同的东西：

| 类别                                 | 用途                  | 在哪里定义               | 谁用                     |
| ---------------------------------- | ------------------- | ------------------- | ---------------------- |
| 开发命令（/feature-workflow, /fix 等）    | 开发 AnalystPro 的代码   | `.claude/commands/` | 开发者（你）                 |
| AnalystPro Skills（/today, /deal 等） | AnalystPro 应用内的用户功能 | `electron/skills/`  | AnalystPro 的用户（VC 合伙人） |

开发命令是你用来**构建**这个应用的工具。AnalystPro Skills 是你**构建出来**的产品功能。

---

## 附录 A：Phase 1-4 模块执行清单

### Phase 1: MVP

```
□  1. /feature-workflow electron-scaffold
□  2. /feature-workflow sqlite-schema
□  3. /feature-workflow workspace-manager
□  4. /feature-workflow provider-types
□  5. /feature-workflow agent-engine
□  6. /feature-workflow secretary-agent
□  7. /feature-workflow market-intel-agent
□  8. /feature-workflow deal-analyst-p1
□  9. /feature-workflow skill-system
□ 10. /feature-workflow ipc-bridge
□ 11. /feature-workflow chat-panel
□ 12. /feature-workflow pipeline-view
□ 13. /feature-workflow intel-feed
□ 14. /feature-workflow workspace-view
□ 15. /feature-workflow hitl-system
□ 16. /feature-workflow keychain-auth
□ 17. /feature-workflow output-router
   ── Phase 1 验收 ──
□ /release-gate
□ /codex-audit --full (可选)
□ git tag v0.1.0
```

### Phase 2: 完整分析链 + 多模型 + MCP 基础

```
□ 18. /feature-workflow deal-analyst-p2       # BP 深度分析
□ 19. /feature-workflow deal-analyst-p3       # IC Memo
□ 20. /feature-workflow hardtech-dd-agent
□ 21. /feature-workflow industry-researcher-agent
□ 22. /feature-workflow openai-provider
□ 23. /feature-workflow gemini-provider
□ 24. /feature-workflow custom-provider
□ 25. /feature-workflow conversation-history
□ 26. /feature-workflow skill-editor
□ 27. /feature-workflow kpi-tracker
□ 28. /feature-workflow cron-scheduler
□ 29. /feature-workflow heartbeat
□ 30. /feature-workflow knowledge-explorer
□ 31. /feature-workflow mcp-manager
□ 32. /feature-workflow playwright-mcp
   ── Phase 2 验收 ──
□ /release-gate
□ git tag v0.2.0
```

### Phase 3: 投后 + LP + 高级功能

```
□ 33. /feature-workflow portfolio-monitor
□ 34. /feature-workflow lp-reporter
□ 35. /feature-workflow vps-sync
□ 36. /feature-workflow optional-mcp-servers   # arxiv, financial-data, github
□ 37. /feature-workflow mcp-config-ui
□ 38. /feature-workflow file-drag-upload
□ 39. /feature-workflow export-pdf-excel
□ 40. /feature-workflow auto-update
   ── Phase 3 验收 ──
□ /release-gate
□ git tag v0.3.0
```

### Phase 4: 生态扩展

```
□ 41. /feature-workflow telegram-integration
□ 42. /feature-workflow voice-input
□ 43. /feature-workflow mcp-hot-reload
□ 44. /feature-workflow mcp-skill-bridge
□ 45. /feature-workflow skill-ecosystem
□ 46. /feature-workflow team-collaboration
   ── Phase 4 验收 ──
□ /release-gate
□ git tag v1.0.0
```

---

## 附录 B：Feature Workflow 9 Agent 速查

```
┌─ Stage ──┬── Agent ─────────────┬── 产出 ──────────────────────────────┐
│ 1. Plan  │ Planner              │ docs/plans/YYYYMMDD-HHMM-*.md      │
│ 2. Spec  │ Spec Guardian        │ 合规检查报告（pass/fail）              │
│ 3. Impact│ Impact Analyst       │ 文件影响映射 + 依赖风险                 │
│ 4. Impl  │ Implementer          │ 代码 + 测试（TDD: RED→GREEN→REFACTOR）│
│ 5. Test  │ Test Runner          │ vitest + check:all 结果              │
│ 6. Audit │ Auditor              │ 代码审查发现（→ 循环回 Impl 如有问题）    │
│ 7. Manual│ Manual Test Author   │ docs/testing/manual-testing-guide.md │
│ 8. Verify│ Verifier             │ 最终通过/失败清单                      │
│ 9. Ship  │ Release Steward      │ commit message（等用户确认）            │
└──────────┴──────────────────────┴──────────────────────────────────────┘
```

---

## 附录 C：目录结构快速参考

```
.claude/
├── commands/                     # 开发命令定义
│   ├── feature-workflow.md       # 主工作流
│   ├── fix.md                    # Bug 修复
│   ├── audit-fix.md              # 审计循环
│   └── speckit.*.md              # Speckit 命令套件
├── skills/                       # 辅助技能
│   ├── planning/                 # 计划生成
│   ├── plan-audit/               # 计划审计
│   ├── plan-verify/              # 计划验证
│   ├── release-gate/             # 质量门禁
│   ├── save-context.md           # 保存上下文
│   └── load-context.md           # 恢复上下文
├── agents/                       # 开发用 subagent 定义
│   ├── planner.md
│   ├── implementer.md
│   ├── auditor.md
│   ├── test-runner.md
│   ├── verifier.md
│   ├── spec-guardian.md
│   ├── impact-analyst.md
│   ├── release-steward.md
│   └── manual-test-author.md
├── rules/                        # 项目规则
│   ├── 00-engineering-principles.md
│   └── 10-tdd.md
└── contexts/                     # Session 上下文存档
    └── YYYY-MM-DD_HH-MM-SS.md

docs/
├── plans/                        # Feature Workflow 计划文件
│   └── YYYYMMDD-HHMM-*.md
└── testing/
    └── manual-testing-guide.md   # 手动测试指南

specs/                            # Speckit 产出（如果使用）
└── NNN-feature-name/
    ├── spec.md
    ├── plan.md
    ├── tasks.md
    ├── research.md
    ├── data-model.md
    ├── contracts/
    ├── checklists/
    └── diagrams/
```

