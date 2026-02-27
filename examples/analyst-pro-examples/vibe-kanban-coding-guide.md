# Vibe Kanban 并行开发指南 — AnalystPro 实战

> 使用 [Vibe Kanban](https://github.com/BloopAI/vibe-kanban) 替代手动四窗口协调，完成 AnalystPro Phase 1 全部 17 个模块的并行开发。

---

## 1. Vibe Kanban 核心概念

```
┌──────────────────────────────────────────────────────────┐
│  Vibe Kanban = Kanban Board + Git Worktree + Agent Runner │
│                                                           │
│  你负责: Plan (拆任务) + Review (审代码)                    │
│  VK 负责: 创建 worktree + 启动 agent + 流式监控 + 合并     │
│  Agent 负责: 写代码 + 跑测试                                │
└──────────────────────────────────────────────────────────┘
```

| 概念                | 说明                                          |
| ----------------- | ------------------------------------------- |
| **Issue**         | 一个工作单元（对应我们的一个模块，如 #6 secretary-agent）      |
| **Sub-issue**     | 子任务（对应模块内的 Work Item，如 WI-001）              |
| **Workspace**     | 隔离执行环境 = 一个 git worktree + 一个 agent session |
| **Attempt**       | 同一 Issue 的多次尝试（换 agent/换 prompt/重试）         |
| **Board Columns** | Planning → In Progress → In Review → Done   |

**关键区别**：手动模式下你需要自己跑 `git worktree add`、切窗口、跑 `npm run check:all`、手动 merge。Vibe Kanban 把这些全自动化了。

---

## 2. 环境准备

### 2.1 安装前置

```bash
# 1. 确保 Node.js 18+
node -v

# 2. 确保 Claude Code 已认证
npx -y @anthropic-ai/claude-code
# 完成认证流程

# 3. 启动 Vibe Kanban
npx vibe-kanban
```

浏览器自动打开 Web UI（默认 [http\://localhost:3000）。](http://localhost:3000）。)

### 2.2 首次项目配置

1. **创建 Project**：在 VK Web UI 中创建项目 `AnalystPro`

2. **关联 Repository**：指向本地 git 仓库路径 `/Users/jackcheng/Documents/01_Coding/mac-app/analyst-pro`

3. **选择 Base Branch**：`master`（所有 worktree 从 master 分出）

4. **配置 Agent**：选择 Claude Code 作为默认 agent

### 2.3 Setup Script 配置

在 VK Settings → Repo Scripts 中配置，每个 worktree 创建后自动执行：

```bash
# Setup Script（每个 worktree 启动时自动运行）
npm install
```

```bash
# Cleanup Script（worktree 删除前自动运行）
npm run check:all
```

### 2.4 MCP Server 配置（可选增强）

在 VK Settings → MCP Servers 中添加 Vibe Kanban 自身作为 MCP server，让 Claude Code agent 能直接读写 Kanban board 状态：

```json
{
  "mcpServers": {
    "vibe-kanban": {
      "command": "npx",
      "args": ["vibe-kanban", "--mcp"]
    }
  }
}
```

这样 agent 完成任务后可以自动把 Issue 移到 "In Review" 列。

MCP Server 正确配置（官方文档格式）：

```json
{
  "mcpServers": {
    "vibe_kanban": {
      "command": "npx",
      "args": ["-y", "vibe-kanban@latest", "--mcp"]
    }
  }
}
```

可用 MCP Tools：`list_projects`, `list_tasks`, `create_task`, `get_task`, `update_task`, `delete_task`, `start_workspace_session`, `list_repos`, `get_repo`, `get_context`。

### 2.5 Agent Profile 配置（profiles.json）

在 VK Settings → Agents 中配置多个 Claude Code variant，适配不同模块复杂度：

```json
{
  "executors": {
    "CLAUDE_CODE": {
      "DEFAULT": {
        "CLAUDE_CODE": { "dangerously_skip_permissions": true }
      },
      "PLAN": {
        "CLAUDE_CODE": { "plan": true }
      }
    }
  }
}
```

| Variant   | 用途           | 适用模块                                 |
| --------- | ------------ | ------------------------------------ |
| `DEFAULT` | 自主执行（跳过权限确认） | 大部分模块                                |
| `PLAN`    | 先生成 plan 再执行 | #10 ipc-bridge, #11 chat-panel 等复杂模块 |

> `dangerously_skip_permissions: true` 等价于 `--dangerously-skip-permissions`，让 agent 自主运行不弹确认框。生产环境慎用。

### 2.6 Task Tag 模板

在 VK Settings → General → Task Tags 中创建可复用的 `@` 标签模板：

**Tag: ****`feature_workflow_prompt`**

```
## 依赖
- [列出已完成的依赖模块]

## PRD 参考
- analyst-pro-prd.md §[章节号] [章节名]
- AGENTS.md "[相关段落]" 章节

## 目标产出
1. [文件路径] — [做什么]

## 关键约束
- [不可违反的规则]

## Agent Prompt
/feature-workflow [module-slug]

请参考以下文件和章节：
- [同上 PRD 参考]

目标：
- [同上目标产出]

关键约束：
- [同上]
```

创建 Issue 时输入 `@feature_workflow_prompt` 即可自动插入模板，再填充具体内容。

### 2.7 Worktree 目录位置

VK 默认在项目根目录创建 `.vibe-kanban-workspaces/` 存放所有 worktree。可在 Settings → General → Workspace Directory 中修改。

每个 worktree 结构：

```
.vibe-kanban-workspaces/
├── vk-abc123-electron-scaffold/    # TASK-001 的隔离工作目录
├── vk-def456-sqlite-schema/        # TASK-002
└── vk-ghi789-workspace-manager/    # TASK-003
```

### 2.8 键盘快捷键速查

| 快捷键     | 操作                     |
| ------- | ---------------------- |
| `C`     | 创建新 Task               |
| `Cmd+K` | 打开 Command Bar         |
| `V C`   | 切换 Changes 面板（diff 视图） |
| `X P`   | 创建 Pull Request        |
| `X M`   | Merge 分支               |
| `X R`   | Rebase 分支              |
| `X U`   | Push 变更                |
| `R S`   | 运行 Setup Script        |
| `R C`   | 运行 Cleanup Script      |
| `T D`   | 启动/停止 Dev Server       |

---

## 3. 模块分解 — 创建 17 个 Issues

### 3.1 Issue 创建策略

按 `analystpro-feature-workflow-dev-guide.md` §3.3 的 17 个模块，在 VK Board 上创建 17 个 Issue。使用 **Parent-Child 结构**表示依赖：

```
Layer 0:  TASK-001 electron-scaffold
Layer 1:  TASK-002 sqlite-schema
          TASK-003 workspace-manager
          TASK-004 provider-types
          TASK-016 keychain-auth
Layer 2:  TASK-005 agent-engine
          TASK-009 skill-system
          TASK-017 output-router
Layer 3:  TASK-006 secretary-agent
          TASK-007 market-intel-agent
          TASK-008 deal-analyst-p1
          TASK-015 hitl-system
Layer 4:  TASK-010 ipc-bridge
Layer 5:  TASK-011 chat-panel
          TASK-012 pipeline-view
          TASK-013 intel-feed
          TASK-014 workspace-view
```

### 3.2 单个 Issue 创建模板

以 #6 secretary-agent 为例，在 VK 中创建 Issue：

**Title**: `#6 secretary-agent — Secretary 主 agent 配置`

**Priority**: High

**Tags**: `agent-layer`, `phase-1`, `layer-3`

**Description**:

```markdown
## 依赖
- #5 agent-engine ✅
- #3 workspace-manager ✅

## PRD 参考
- analyst-pro-prd.md §3.1 架构模式
- analyst-pro-prd.md §3.2 engine.ts 伪代码中 Secretary 配置
- analyst-pro-prd.md §3.3 全部 Subagent 定义
- AGENTS.md "Agent SDK Architecture" 章节

## 目标产出
1. electron/agents/secretary.ts — Secretary 主 agent 配置
2. electron/agents/secretary.test.ts — 测试
3. 系统提示模板（读取 workspace/SOUL.md + USER.md + IDENTITY.md）

## 关键约束
- model: sonnet
- systemPrompt 使用 SDK preset: "claude_code"
- allowedTools 必须包含 Task + AskUserQuestion
- workspace 文件可能不存在，需优雅处理

## Agent Prompt
/feature-workflow secretary-agent

请参考以下文件和章节：
- analyst-pro-prd.md §3.1 架构模式
- analyst-pro-prd.md §3.2 engine.ts 伪代码中 Secretary 配置
- analyst-pro-prd.md §3.3 全部 Subagent 定义
- AGENTS.md "Agent SDK Architecture" 章节

目标：
1. electron/agents/secretary.ts — secretarySystemPrompt 合成函数 + agentDefinitions 注册 + buildSecretaryOptions()
2. electron/agents/secretary.test.ts — 测试

关键约束：
- model: sonnet
- systemPrompt 使用 { type: "preset", preset: "claude_code", append: ... }
- allowedTools 包含 Task 和 AskUserQuestion
```

### 3.3 批量创建 17 个 Issues

按以下顺序在 VK Board 的 **Planning** 列创建全部 17 个 Issue：

| TASK ID  | Title                   | Priority | Tags                     | 依赖 (Description 中注明) |
| -------- | ----------------------- | -------- | ------------------------ | -------------------- |
| TASK-001 | `#1 electron-scaffold`  | Urgent   | `foundation`, `layer-0`  | 无                    |
| TASK-002 | `#2 sqlite-schema`      | High     | `data-layer`, `layer-1`  | #1                   |
| TASK-003 | `#3 workspace-manager`  | High     | `data-layer`, `layer-1`  | #1                   |
| TASK-004 | `#4 provider-types`     | High     | `provider`, `layer-1`    | #1                   |
| TASK-005 | `#5 agent-engine`       | Urgent   | `agent-layer`, `layer-2` | #4                   |
| TASK-006 | `#6 secretary-agent`    | High     | `agent-layer`, `layer-3` | #5, #3               |
| TASK-007 | `#7 market-intel-agent` | High     | `agent-layer`, `layer-3` | #5                   |
| TASK-008 | `#8 deal-analyst-p1`    | High     | `agent-layer`, `layer-3` | #5                   |
| TASK-009 | `#9 skill-system`       | Urgent   | `skill-layer`, `layer-2` | #2                   |
| TASK-010 | `#10 ipc-bridge`        | Urgent   | `integration`, `layer-4` | #2, #3, #4, #5, #9   |
| TASK-011 | `#11 chat-panel`        | High     | `ui-layer`, `layer-5`    | #10, #9              |
| TASK-012 | `#12 pipeline-view`     | High     | `ui-layer`, `layer-5`    | #10, #2              |
| TASK-013 | `#13 intel-feed`        | High     | `ui-layer`, `layer-5`    | #10, #2              |
| TASK-014 | `#14 workspace-view`    | High     | `ui-layer`, `layer-5`    | #10, #2, #3          |
| TASK-015 | `#15 hitl-system`       | High     | `agent-layer`, `layer-3` | #5, #2               |
| TASK-016 | `#16 keychain-auth`     | Medium   | `auth`, `layer-1`        | #1                   |
| TASK-017 | `#17 output-router`     | Medium   | `data-layer`, `layer-2`  | #2, #3               |

每个 Issue 的 Description 中都包含完整的 Agent Prompt（参照 §3.2 模板和 `analystpro-feature-workflow-dev-guide.md` §3.3 对应模块的上下文指引）。

---

## 4. 并行开发执行 — 分 6 个 Batch

### 4.1 Batch 0: 基础（串行）

```
┌─────────────────────────┐
│ TASK-001 electron-scaffold │ → Claude Code
└─────────────────────────┘
```

**操作**：

1. 点击 TASK-001 → 点击 **+** 创建 Task Attempt → 选择 Agent `CLAUDE_CODE`、Variant `DEFAULT`、Base Branch `master`

2. VK 自动：创建 git worktree → 创建 `vk/xxx-electron-scaffold` 分支 → 运行 Setup Script (`npm install`) → 启动 Claude Code agent

3. 在 Task Description 的 chat 中粘贴完整的 `/feature-workflow` 上下文指引，或直接用 Issue Description 中预写的 Agent Prompt

4. 实时监控：左侧 Board 显示 **In Progress**，右侧 Chat 面板流式显示 agent 思考和操作过程

5. Agent 完成 → Task 自动移到 **In Review** → 在 Changes 面板（快捷键 `V C`）审查 diff

6. 通过 → 点击 **Rebase**（确保与 master 同步）→ 点击 **Merge** → VK 自动合并到 master + 清理 worktree

7. Task 自动移到 **Done**

> **Task 生命周期**：To Do → (创建 Attempt) → In Progress → (Agent 完成) → In Review → (Merge) → Done

### 4.2 Batch 1: 基础模块（3 并行）

```
┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│ TASK-002 sqlite-schema│  │ TASK-003 workspace-mgr│  │ TASK-004 provider-types│
│ Agent: Claude Code    │  │ Agent: Claude Code    │  │ Agent: Claude Code    │
└──────────────────────┘  └──────────────────────┘  └──────────────────────┘
```

**操作**：

1. 同时将 TASK-002、TASK-003、TASK-004 拖到 **In Progress**

2. VK 为每个 Issue 创建独立 worktree（从 master 分支）

3. 3 个 Claude Code agent 并行执行

4. 在 VK Board 实时监控 3 个 agent 的状态（Running / Idle / Completed）

5. 完成一个就 Review 一个 → Merge 一个

6. **合并顺序**：先完成的先合并，后完成的 VK 自动 rebase 到最新 master

> **可选**：如果 TASK-016 keychain-auth 也想在这个 Batch 做，可以作为第 4 个并行 — 它只依赖 #1，与其他三个零冲突。

### 4.3 Batch 2: 核心引擎（2-3 并行）

等待 Batch 1 全部 merge 到 master。

```
┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│ TASK-005 agent-engine │  │ TASK-009 skill-system │  │ TASK-017 output-router│
│ 依赖: #4 ✅          │  │ 依赖: #2 ✅          │  │ 依赖: #2+#3 ✅       │
└──────────────────────┘  └──────────────────────┘  └──────────────────────┘
```

**操作**：同 Batch 1，3 个并行拖入 In Progress。

### 4.4 Batch 3: Agent 定义（3-4 并行）

等待 TASK-005 agent-engine merge 到 master。

```
┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│ TASK-006 secretary    │  │ TASK-007 market-intel │  │ TASK-008 deal-analyst│  │ TASK-015 hitl-system │
│ 依赖: #5+#3 ✅       │  │ 依赖: #5 ✅          │  │ 依赖: #5 ✅          │  │ 依赖: #5+#2 ✅      │
└──────────────────────┘  └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
```

**注意**：4 个并行是 Vibe Kanban 的优势 — 手动模式下我们限制在 3 个 Worker 以避免三方冲突，但 VK 的自动 rebase + merge 可以处理更多并行。

**冲突风险**：#6、#7、#8 都可能改 `electron/agents/engine.ts`（注册 subagent）。VK 按完成顺序自动 rebase 可以处理，但建议 Review 时特别关注这个文件。

### 4.5 Batch 4: 集成层（串行）

等待 TASK-006, 007, 009 全部 merge。

```
┌──────────────────────────────────────┐
│ TASK-010 ipc-bridge                   │
│ 依赖: #2+#3+#4+#5+#9 全部 ✅         │
│ 集成层 — 桥接所有已完成模块            │
└──────────────────────────────────────┘
```

**这个模块必须串行**：它桥接所有后端模块到前端，修改 `preload.ts`、`main.ts`、`electron-api.d.ts` 等共享文件。

### 4.6 Batch 5: UI 层（3-4 并行）

等待 TASK-010 merge。

```
┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│ TASK-011 chat-panel  │  │ TASK-012 pipeline-view│  │ TASK-013 intel-feed  │  │ TASK-014 workspace   │
│ 依赖: #10+#9 ✅      │  │ 依赖: #10+#2 ✅      │  │ 依赖: #10+#2 ✅      │  │ 依赖: #10+#2+#3 ✅  │
└──────────────────────┘  └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
```

**冲突风险**：4 个模块都可能改 `src/App.tsx`（添加路由/组件）。VK 自动 rebase 可以处理追加式修改，但如果 Batch 4 的 ipc-bridge 在 `App.tsx` 中预留了路由插槽，冲突会降到最低。

### 4.7 补漏

如果 TASK-016 keychain-auth 和 TASK-017 output-router 没有在早期 Batch 完成，可以在任何有空闲 slot 时插入 — 它们的依赖在 Batch 1 后就已满足。

---

## 5. 每个 Issue 的详细 Agent Prompt

> 以下是每个 Issue 的 Description 中应包含的完整 Agent Prompt。复制到 VK Issue 的 Description 字段中，VK 启动 agent 时会自动使用。

### TASK-001: electron-scaffold

```
/feature-workflow electron-scaffold

请参考以下文件和章节：
- analyst-pro-prd.md §2.2 技术栈
- analyst-pro-prd.md §2.3 目录结构
- analyst-pro-prd.md §8.2 Electron 安全最佳实践
- analyst-pro-prd.md §5.1 布局架构
- AGENTS.md "Electron Security" + "Project Structure" + "Tech Stack"

目标：
1. package.json — 核心依赖声明
2. electron-vite.config.ts — main/preload/renderer 三进程构建
3. tsconfig.json — strict mode
4. electron/main.ts — BrowserWindow（contextIsolation+sandbox+CSP）
5. electron/preload.ts — contextBridge 骨架
6. src/App.tsx — React + TailwindCSS + shadcn/ui
7. tailwind.config.ts
8. package.json scripts: "dev", "build", "check:all"

关键约束：
- contextIsolation: true + nodeIntegration: false + sandbox: true
- CSP header 通过 session.defaultSession.webRequest 设置
```

### TASK-002 \~ TASK-017

> 参照 `analystpro-feature-workflow-dev-guide.md` §3.3.1 \~ §3.3.17 中每个模块的完整上下文指引。每个 Issue 的 Description 直接复制对应模块的 `/feature-workflow` 命令 + 参考文件 + 目标 + 关键约束。

---

## 6. Review & Merge 最佳实践

### 6.1 Review 检查清单

每个 Issue 从 "In Progress" 移到 "In Review" 后，在 VK diff viewer 中检查：

```
□ 核心产出文件是否齐全（对照 Issue Description 中的目标列表）
□ 测试文件是否存在（*.test.ts）
□ package.json 新增依赖是否合理
□ 没有修改不属于本模块的文件（越界检查）
□ 没有 TODO/FIXME 无 task ID
□ CHANGELOG.md 条目已添加
□ 文件行数 < 300（AGENTS.md 规则）
```

### 6.2 Merge 策略

VK 提供两种 merge 方式：

| 方式              | 适用场景                                 |
| --------------- | ------------------------------------ |
| **Local Merge** | VK 内置 rebase + merge 到 master，适合日常开发 |
| **GitHub PR**   | VK 自动创建 PR，适合需要 CI/CD 或团队 review     |

**推荐**：Phase 1 开发期使用 Local Merge（快速迭代），功能稳定后切换到 GitHub PR。

### 6.3 Merge 后验证

VK merge 后自动清理 worktree。手动验证：

```bash
# 在项目主目录
git log --oneline -5        # 确认 merge commit
npm run check:all           # 全量质量门禁
```

### 6.4 Rebase 冲突解决

并行开发后 merge 时可能遇到 rebase 冲突，VK 提供三种解决方式：

| 方式       | 操作                                                                | 适用场景                          |
| -------- | ----------------------------------------------------------------- | ----------------------------- |
| **自动解决** | 点击 Resolve Conflicts → VK 生成解决指令 → agent 自动修复                     | 大部分冲突（package.json、import 追加） |
| **手动解决** | 点击 Open in Editor → 编辑冲突标记 → `git add . && git rebase --continue` | 复杂语义冲突                        |
| **放弃重来** | 点击 Abort Rebase → 创建 New Attempt（从最新 master 重新开始）                 | 冲突太多不值得修                      |

> 对于本项目最常见的 `package.json` 冲突，推荐使用自动解决 — VK 的 agent 可以智能合并依赖项列表。

### 6.5 失败处理 — Attempt 模型

每个 Task 支持多次 Attempt（1:N 关系），用于失败重试或方案对比：

1. 打开 Task → 点击右上角 **⋯** → **Create New Attempt**

2. 可以重新配置：

   - **换 Agent**：从 Claude Code 换到 Codex（对比不同模型的实现）

   - **换 Variant**：从 DEFAULT 换到 PLAN（让 agent 先规划再执行）

   - **换 Base Branch**：从旧 master 换到最新 master

3. VK 保留所有 attempt 历史，可以对比不同 attempt 的 diff

4. 每个 attempt 有独立的 worktree 和分支，互不干扰

### 6.6 Subtask 拆分（大模块适用）

对于 #10 ipc-bridge、#11 chat-panel 等大模块，可以用 Subtask 进一步拆分：

1. 创建主 Task: `#10 ipc-bridge`

2. 创建 Attempt 并开始执行

3. 在 Attempt 中点击 **⋯** → **Create Subtask**

4. Subtask 自动继承父 Attempt 的分支作为 base

5. Subtask 有独立的生命周期（To Do → In Progress → In Review → Done）

```
#10 ipc-bridge (Parent Task)
├── Subtask: agent.ipc.ts + chat.ipc.ts
├── Subtask: provider.ipc.ts + skill.ipc.ts
├── Subtask: workspace.ipc.ts + session.ipc.ts + mcp.ipc.ts
└── Subtask: preload.ts + electron-api.d.ts 类型定义
```

Git 分支结构：

```
master
└── vk/xxx-ipc-bridge (parent)
    ├── vk/yyy-subtask-agent-chat-ipc
    ├── vk/zzz-subtask-provider-skill-ipc
    └── ...
```

Subtask 完成后 merge 回 parent 分支，parent 最终 merge 回 master。

### 6.7 使用 MCP 批量创建 Tasks

对于 17 个模块的批量创建，可以利用 VK MCP Server + Claude Desktop / Raycast：

```
我需要为 AnalystPro Phase 1 创建以下模块的开发任务：

1. electron-scaffold — Electron 脚手架 (layer-0, 无依赖)
2. sqlite-schema — SQLite 表定义 (layer-1, 依赖 #1)
3. workspace-manager — Workspace 文件管理 (layer-1, 依赖 #1)
...（列出全部 17 个）

每个 task 的 description 包含依赖、PRD 参考、目标产出和关键约束。
请在 AnalystPro 项目中创建这些 tasks。
```

MCP client 会调用 `create_task` API 批量生成 17 个 Task，省去手动逐个创建的工作。

---

## 7. VK 替代手动协调的对照表

| 手动四窗口模式                                | Vibe Kanban 等价操作                               |
| -------------------------------------- | ---------------------------------------------- |
| Coordinator 窗口                         | VK Web UI Board 视图                             |
| `git worktree add ...`                 | 拖 Issue 到 In Progress（自动创建）                    |
| Worker 窗口 `cd ... && claude`           | VK 自动启动 agent session                          |
| 手动跟踪 §1 状态表                            | Board 列状态（Planning/In Progress/In Review/Done） |
| 口头通知 "窗口2: #7 完成"                      | Issue 自动移到 In Review 列                         |
| `git checkout master && git merge ...` | 点击 Merge 按钮                                    |
| `git worktree remove ...`              | VK merge 后自动清理                                 |
| `npm run check:all`                    | 配置为 Cleanup Script 自动执行                        |
| `parallel-coding-guide.md` 手动更新        | Board 实时反映状态                                   |
| 决策下一模块                                 | 查看 Planning 列 + 依赖 tag 过滤                      |

---

## 8. 高级配置

### 8.1 并行度控制

VK 没有硬性并行限制，但实际受限于：

| 限制因素                               | 建议值                    |
| ---------------------------------- | ---------------------- |
| Claude Code API 并发                 | 3-5 个 session（取决于订阅计划） |
| 本地磁盘（每个 worktree 一份 node\_modules） | \~500MB × 并行数          |
| CPU/内存（Electron build + test）      | 3-4 个并行适合 16GB Mac     |

**推荐**：M1/M2 Mac 16GB 内存，并行度 3；32GB 内存，并行度 4-5。

### 8.2 Agent Profile 配置

在 VK Settings → Agent Profiles 中为不同类型的模块配置不同的 agent profile：

| Profile   | Agent                | 适用模块                                 |
| --------- | -------------------- | ------------------------------------ |
| `default` | Claude Code (Sonnet) | 大部分模块                                |
| `complex` | Claude Code (Opus)   | #10 ipc-bridge, #11 chat-panel       |
| `simple`  | Claude Code (Haiku)  | #16 keychain-auth, #17 output-router |

### 8.3 依赖可视化

利用 VK 的 Tag 筛选功能快速查看可执行模块：

1. 给已完成的 Issue 加 tag `done`

2. 筛选 `layer-N` tag 查看当前可并行的层级

3. 在 Planning 列中，依赖未满足的 Issue 保持灰色/低优先级

---

## 9. 完整执行时间线

```
Phase 1 MVP — 17 模块 — 6 Batch

Batch 0 ──── #1 scaffold ─────────────────────── merge ✅
             │
Batch 1 ──── #2 sqlite ──────┐
             #3 workspace ────┤ 3 并行 ────────── merge ✅
             #4 provider ─────┘
             │
Batch 2 ──── #5 engine ──────┐
             #9 skill ────────┤ 3 并行 ────────── merge ✅
             #17 router ──────┘
             │
Batch 3 ──── #6 secretary ───┐
             #7 intel-agent ──┤
             #8 deal-analyst ─┤ 4 并行 ────────── merge ✅
             #15 hitl ────────┘
             │
Batch 4 ──── #10 ipc-bridge ─────────────────── merge ✅
             │
Batch 5 ──── #11 chat ───────┐
             #12 pipeline ────┤
             #13 intel-feed ──┤ 4 并行 ────────── merge ✅
             #14 workspace-ui ┘
             │
             (+#16 keychain 插入任何有空闲 slot 的 Batch)

────────────── Phase 1 MVP Done ──────────────
```

---

## 510. 从手动模式迁移到 VK（当前状态：#1-#9 ✅ 已完成）

> 截至迁移时点，手动四窗口模式已完成 9 个模块（#1-#9），剩余 8 个模块（#10-#17）切换到 Vibe Kanban 接管。

### 10.1 当前完成状态

| #      | 模块                   | 状态               | 所属层         |
| ------ | -------------------- | ---------------- | ----------- |
| 1      | `electron-scaffold`  | ✅ 已 merge master | Layer 0     |
| 2      | `sqlite-schema`      | ✅ 已 merge master | Layer 1     |
| 3      | `workspace-manager`  | ✅ 已 merge master | Layer 1     |
| 4      | `provider-types`     | ✅ 已 merge master | Layer 1     |
| 5      | `agent-engine`       | ✅ 已 merge master | Layer 2     |
| 6      | `secretary-agent`    | ✅ 已 merge master | Layer 3     |
| 7      | `market-intel-agent` | ✅ 已 merge master | Layer 3     |
| 8      | `deal-analyst-p1`    | ✅ 已 merge master | Layer 3     |
| 9      | `skill-system`       | ✅ 已 merge master | Layer 2     |
| **10** | **`ipc-bridge`**     | ⬜ 待开发            | **Layer 4** |
| **11** | **`chat-panel`**     | ⬜ 待开发            | **Layer 5** |
| **12** | **`pipeline-view`**  | ⬜ 待开发            | **Layer 5** |
| **13** | **`intel-feed`**     | ⬜ 待开发            | **Layer 5** |
| **14** | **`workspace-view`** | ⬜ 待开发            | **Layer 5** |
| **15** | **`hitl-system`**    | ⬜ 待开发            | **Layer 3** |
| **16** | **`keychain-auth`**  | ⬜ 待开发            | **Layer 1** |
| **17** | **`output-router`**  | ⬜ 待开发            | **Layer 2** |

### 10.2 剩余模块依赖分析

```
已完成的依赖全部满足 ✅，剩余 8 个模块的解锁状态：

立即可开发（依赖已满足）：
  #10 ipc-bridge       ── 需要 #2+#3+#4+#5+#9 → 全部 ✅  ⭐ 关键路径
  #15 hitl-system       ── 需要 #5+#2 → 全部 ✅
  #16 keychain-auth     ── 需要 #1 → ✅
  #17 output-router     ── 需要 #2+#3 → 全部 ✅

等待 #10 完成后可开发：
  #11 chat-panel        ── 需要 #10+#9
  #12 pipeline-view     ── 需要 #10+#2
  #13 intel-feed        ── 需要 #10+#2
  #14 workspace-view    ── 需要 #10+#2+#3
```

### 10.3 迁移步骤

#### Step 1: 清理手动 worktree

```bash
# 确认当前状态
git checkout master
git log --oneline -15   # 确认 #1-#9 全部在 master

# 查看并清理所有旧 worktree
git worktree list
git worktree remove ../analyst-pro-wt-deal-analyst    2>/dev/null
git worktree remove ../analyst-pro-wt-market-intel    2>/dev/null
git worktree remove ../analyst-pro-wt-skill-system    2>/dev/null

# 清理已合并的远程分支（可选）
git branch -d feat/market-intel-agent feat/skill-system feat/deal-analyst-p1 2>/dev/null
```

#### Step 2: 启动 Vibe Kanban

```bash
npx vibe-kanban
# 浏览器自动打开 http://localhost:3000
```

#### Step 3: 创建项目 + 关联仓库

1. VK Web UI → **New Project** → 名称 `AnalystPro`

2. **Add Repository** → 路径 `/Users/jackcheng/Documents/01_Coding/mac-app/analyst-pro`

3. **Base Branch** → `master`

4. **Setup Script** → `npm install`

5. **Cleanup Script** → `npm run check:all`

#### Step 4: 创建已完成模块的标记 Issues（可选）

为保持完整的看板视图，可以为 #1-#9 创建 Issues 并直接放入 **Done** 列：

```
标记已完成（快速创建，只需 Title）：
#1 electron-scaffold    → Done
#2 sqlite-schema        → Done
#3 workspace-manager    → Done
#4 provider-types       → Done
#5 agent-engine         → Done
#6 secretary-agent      → Done
#7 market-intel-agent   → Done
#8 deal-analyst-p1      → Done
#9 skill-system         → Done
```

> 或跳过这步，只创建剩余 8 个 Issues，Done 列留空。

#### Step 5: 创建剩余 8 个 Issues

在 VK Board 的 **To Do** 列创建以下 8 个 Issues（每个 Issue 的 Description 包含完整 Agent Prompt）：

---

**Issue 1: ****`#10 ipc-bridge — IPC 桥接层`**

Priority: **Urgent** | Tags: `integration`, `layer-4`, `critical-path`

```markdown
## 依赖
- #2 sqlite-schema ✅
- #3 workspace-manager ✅
- #4 provider-types ✅
- #5 agent-engine ✅
- #9 skill-system ✅

## PRD 参考
- analyst-pro-prd.md §2.3 目录结构中 electron/ipc/ 部分
- analyst-pro-prd.md §9.4 MCP Server 管理架构中 "IPC: mcp.ipc.ts" 段
- AGENTS.md "IPC Pattern" + "Electron Security" 章节

## 目标产出
1. electron/ipc/agent.ipc.ts — Agent IPC (agent:query, agent:abort, agent:status)
2. electron/ipc/chat.ipc.ts — Chat IPC (chat:send, chat:abort, chat:history, chat:list-conversations, chat:new-conversation)
3. electron/ipc/provider.ipc.ts — Provider IPC (provider:list, provider:get-config, provider:save-config, provider:test-connection, provider:get-models)
4. electron/ipc/skill.ipc.ts — Skill IPC (skill:list, skill:invoke, skill:toggle, skill:create, skill:delete)
5. electron/ipc/workspace.ipc.ts — Workspace IPC (workspace:read-file, workspace:write-file, workspace:list-dir, workspace:watch)
6. electron/ipc/session.ipc.ts — Session IPC (session:list, session:get, session:resume, session:delete)
7. electron/ipc/mcp.ipc.ts — MCP IPC placeholder (Phase 1 定义接口，返回空/placeholder)
8. electron/preload.ts 更新 — contextBridge.exposeInMainWorld 暴露所有 channel
9. src/types/electron-api.d.ts — window.electronAPI 完整类型

## 关键约束
- IPC handler 按功能域拆分注册在 electron/ipc/
- Renderer 调用 window.electronAPI.xxx()，不直接使用 ipcRenderer
- channel 名 "domain:action" 格式（如 "chat:send"）
- preload.ts 只做桥接，不含业务逻辑
- mcp.ipc.ts Phase 1 返回 placeholder

## Agent Prompt
/feature-workflow ipc-bridge

请参考以下文件和章节：
- analyst-pro-prd.md §2.3 目录结构中 electron/ipc/ 部分
- analyst-pro-prd.md §9.4 MCP Server 管理架构中 "IPC: mcp.ipc.ts" 段
- AGENTS.md "IPC Pattern" 章节
- AGENTS.md "Electron Security (Non-negotiable)" 章节

目标：
1. electron/ipc/agent.ipc.ts — Agent IPC handler
2. electron/ipc/chat.ipc.ts — Chat 对话 IPC
3. electron/ipc/provider.ipc.ts — Provider 配置 IPC
4. electron/ipc/skill.ipc.ts — Skill IPC
5. electron/ipc/workspace.ipc.ts — Workspace 文件操作 IPC
6. electron/ipc/session.ipc.ts — Session 管理 IPC
7. electron/ipc/mcp.ipc.ts — MCP Server IPC (placeholder)
8. electron/preload.ts 更新 — contextBridge 暴露所有 IPC channel
9. src/types/electron-api.d.ts — 完整类型定义

关键约束：
- 所有 IPC handler 按功能域拆分
- channel 名 "domain:action" 格式
- preload.ts 只做桥接，不含业务逻辑
```

---

**Issue 2: ****`#15 hitl-system — 人机审批系统`**

Priority: **High** | Tags: `agent-layer`, `layer-3`, `security`

```markdown
## 依赖
- #5 agent-engine ✅
- #2 sqlite-schema ✅

## PRD 参考
- analyst-pro-prd.md §3.6 HITL 实现（两种机制 + TypeScript 代码）
- analyst-pro-prd.md §3.7 Hook 系统设计表
- analyst-pro-prd.md §4.1 hitl_approvals 表 SQL 定义
- analyst-pro-prd.md §5.3 交互设计要点
- AGENTS.md "HITL Enforcement" 章节

## 目标产出
1. electron/agents/hooks/hitl.hook.ts 更新 — canUseTool: Bash 拦截, ic_memos/lp_reports 写入拦截
2. electron/ipc/hitl.ipc.ts — HITL IPC (hitl:approve, hitl:reject, hitl:pending-list)
3. src/components/chat/HITLApproval.tsx 更新 — 琥珀色审批卡片 + macOS 通知
4. electron/db/hitl.ts — hitl_approvals 表 CRUD
5. 桌面通知集成 — Electron Notification API

## 关键约束
- IC Memo、LP 材料、Bash 命令必须人工确认（非协商）
- 审批记录持久化到 hitl_approvals 表
- 同时显示 Chat 内联卡片 + macOS 桌面通知
- 拒绝后 agent 收到 deny message，不重试

## Agent Prompt
/feature-workflow hitl-system

请参考以下文件和章节：
- analyst-pro-prd.md §3.6 HITL 实现
- analyst-pro-prd.md §3.7 Hook 系统设计表
- analyst-pro-prd.md §4.1 hitl_approvals 表 SQL 定义
- analyst-pro-prd.md §5.3 交互设计要点
- AGENTS.md "HITL Enforcement" 章节

目标：
1. electron/agents/hooks/hitl.hook.ts — canUseTool 完善
2. electron/ipc/hitl.ipc.ts — HITL 专用 IPC
3. src/components/chat/HITLApproval.tsx — 审批卡片 UI
4. electron/db/hitl.ts — hitl_approvals 表 CRUD
5. 桌面通知集成

关键约束：
- IC Memo、LP 材料、Bash shell 命令必须人工确认
- HITL 审批记录持久化到 SQLite
- 同时在 Chat 面板和 macOS 桌面通知
```

---

**Issue 3: ****`#16 keychain-auth — Keychain 认证`**

Priority: **Medium** | Tags: `auth`, `layer-1`, `security`

```markdown
## 依赖
- #1 electron-scaffold ✅

## PRD 参考
- analyst-pro-prd.md §8.1 数据安全表
- analyst-pro-prd.md §5.2.5 系统设置 — "AI 模型配置"
- analyst-pro-prd.md §4.1 provider_configs 表注释
- AGENTS.md "Electron Security" 章节

## 目标产出
1. electron/auth/keychain.ts — KeychainService (set/get/delete/hasApiKey, 使用 safeStorage API)
2. electron/auth/oauth.ts — OAuth PKCE 骨架（Phase 1 接口定义）
3. electron/ipc/auth.ipc.ts — 认证 IPC (auth:set-key, auth:has-key, auth:delete-key, auth:test-key)
4. src/components/settings/ProviderSettings.tsx — API key 输入 + Test Connection UI

## 关键约束
- API keys 绝不存入 SQLite 或明文文件
- 优先 Electron safeStorage API（基于 macOS Keychain）
- Phase 1 只实现 Claude API key 存储

## Agent Prompt
/feature-workflow keychain-auth

请参考以下文件和章节：
- analyst-pro-prd.md §8.1 数据安全表
- analyst-pro-prd.md §5.2.5 系统设置 — "AI 模型配置"
- analyst-pro-prd.md §4.1 provider_configs 表注释
- AGENTS.md "Electron Security (Non-negotiable)" 章节

目标：
1. electron/auth/keychain.ts — KeychainService
2. electron/auth/oauth.ts — OAuth PKCE 骨架
3. electron/ipc/auth.ipc.ts — 认证 IPC
4. src/components/settings/ProviderSettings.tsx — Provider 配置 UI

关键约束：
- API keys 不入 SQLite，使用 Electron safeStorage API
- Phase 1 只实现 Claude API key
```

---

**Issue 4: ****`#17 output-router — 文件输出路由`**

Priority: **Medium** | Tags: `data-layer`, `layer-2`

```markdown
## 依赖
- #2 sqlite-schema ✅
- #3 workspace-manager ✅

## PRD 参考
- analyst-pro-prd.md §5.2.6 AI 对话面板 — "文件输出路由" 段
- analyst-pro-prd.md §4.1 file_outputs 表 SQL 定义
- analyst-pro-prd.md §4.3.1 归档目录规范
- analyst-pro-prd.md §4.3.2 上传后自动处理

## 目标产出
1. electron/workspace/output-router.ts — OutputRouter (routeOutput + addFrontmatter)
2. electron/db/file-outputs.ts — file_outputs 表 CRUD
3. electron/workspace/output-router.test.ts — 路由规则 + frontmatter 格式测试

## 关键约束
- 所有 agent 文件输出必须经过 OutputRouter
- 每个输出文件必须包含 YAML frontmatter 元数据
- 路由规则基于 agent_name + deal_id，不基于文件内容
- outputs/{date}/ 是 fallback 路径

## Agent Prompt
/feature-workflow output-router

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.6 AI 对话面板 — "文件输出路由" 段
- analyst-pro-prd.md §4.1 file_outputs 表 SQL 定义
- analyst-pro-prd.md §4.3.1 归档目录规范

目标：
1. electron/workspace/output-router.ts — OutputRouter
2. electron/db/file-outputs.ts — file_outputs 表 CRUD
3. 测试：路由规则 + frontmatter 格式

关键约束：
- 所有 agent 输出必须经过 OutputRouter
- 每个文件必须包含 YAML frontmatter
- 路由基于 agent_name + deal_id
```

---

**Issue 5: ****`#11 chat-panel — AI 对话面板`** *(blocked by #10)*

Priority: **High** | Tags: `ui-layer`, `layer-5`, `critical-path`

```markdown
## 依赖
- #10 ipc-bridge ⬜（阻塞中）
- #9 skill-system ✅

## PRD 参考
- analyst-pro-prd.md §5.2.6 AI 对话面板（9 个功能点完整说明）
- analyst-pro-prd.md §5.1 布局架构（push layout CSS grid）
- analyst-pro-prd.md §6.6 SkillPalette 交互规范
- analyst-pro-prd.md §3.6 HITL 实现 — HITLApprovalProps

## 目标产出
1. src/components/chat/ChatPanel.tsx — push layout 面板 (Cmd+J 切换, 拖拽缩放)
2. src/components/chat/ChatHeader.tsx — Provider 选择器
3. src/components/chat/ConversationArea.tsx — 消息流滚动区域
4. src/components/chat/MessageBubble.tsx — 消息渲染 (react-markdown + 语法高亮)
5. src/components/chat/ChatInput.tsx — 输入框 (Cmd+Enter, / 触发 SkillPalette)
6. src/components/chat/SkillPalette.tsx — Skill 自动补全弹窗
7. src/components/chat/HITLApproval.tsx — HITL 审批卡片
8. src/components/chat/AgentMonitorCollapsible.tsx — Agent 监控
9. src/stores/chatStore.ts — Zustand store

## 关键约束
- Push layout（Chat 打开时主内容区收缩，非 overlay）
- SkillPalette 中 agent Skill 仅 Claude 模式可用
- HITL 琥珀色边框 + macOS 通知
- 消息持久化通过 IPC

## Agent Prompt
/feature-workflow chat-panel

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.6 AI 对话面板
- analyst-pro-prd.md §5.1 布局架构
- analyst-pro-prd.md §6.6 SkillPalette 交互规范
- analyst-pro-prd.md §3.6 HITL — HITLApprovalProps

目标：
1. ChatPanel.tsx — push layout (Cmd+J, 拖拽缩放)
2. ChatHeader.tsx — Provider 选择器
3. ConversationArea.tsx — 消息流
4. MessageBubble.tsx — react-markdown + subagent badge
5. ChatInput.tsx — / 触发 SkillPalette
6. SkillPalette.tsx — 按 ap-type 分组
7. HITLApproval.tsx — 琥珀色审批卡片
8. AgentMonitorCollapsible.tsx
9. chatStore.ts — Zustand

关键约束：
- Push layout，非 overlay
- agent Skill 仅 Claude 模式可用
```

---

**Issue 6: ****`#12 pipeline-view — 项目流程看板`** *(blocked by #10)*

Priority: **High** | Tags: `ui-layer`, `layer-5`

```markdown
## 依赖
- #10 ipc-bridge ⬜（阻塞中）
- #2 sqlite-schema ✅

## PRD 参考
- analyst-pro-prd.md §5.2.1 项目流程
- analyst-pro-prd.md §5.0 信息架构
- analyst-pro-prd.md §4.1 deals 表
- analyst-pro-prd.md §4.3.5 阶段输入门禁

## 目标产出
1. src/components/pipeline/Pipeline.tsx — 看板主视图
2. src/components/pipeline/DealCard.tsx — 项目卡片 (拖拽)
3. src/components/pipeline/DealDetail.tsx — 项目详情页
4. src/components/pipeline/StageGatePanel.tsx — 阶段门禁面板
5. src/components/pipeline/ICMemoViewer.tsx — IC Memo 查看器
6. src/stores/pipelineStore.ts — Zustand store

## 关键约束
- deals.stage 8 阶段枚举
- 二级过滤标签对应 4 大阶段分组
- 拖拽改变阶段触发 subagent 工作流
- StageGatePanel 内嵌详情页

## Agent Prompt
/feature-workflow pipeline-view

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.1 项目流程
- analyst-pro-prd.md §5.0 信息架构
- analyst-pro-prd.md §4.1 deals 表
- analyst-pro-prd.md §4.3.5 阶段输入门禁

目标：
1. Pipeline.tsx — 看板主视图
2. DealCard.tsx — 项目卡片
3. DealDetail.tsx — 项目详情页
4. StageGatePanel.tsx — 阶段门禁
5. ICMemoViewer.tsx — IC Memo 查看器
6. pipelineStore.ts

关键约束：
- 8 阶段枚举，4 大分组过滤
- 拖拽触发 agent 工作流
```

---

**Issue 7: ****`#13 intel-feed — 市场情报 UI`** *(blocked by #10)*

Priority: **High** | Tags: `ui-layer`, `layer-5`

```markdown
## 依赖
- #10 ipc-bridge ⬜（阻塞中）
- #2 sqlite-schema ✅

## PRD 参考
- analyst-pro-prd.md §5.2.3 市场情报
- analyst-pro-prd.md §4.1 intel_items 表
- analyst-pro-prd.md §3.3.1 market-intel agent 输出路径

## 目标产出
1. src/components/intel/IntelFeed.tsx — 情报主页面 (Tab 切换 4 子视图)
2. src/components/intel/IntelList.tsx — 情报列表 (评分排序 + 分色 badge)
3. src/components/intel/DailyBrief.tsx — 每日简报
4. src/components/intel/PolicyAlert.tsx — 政策预警
5. src/components/intel/VCMoves.tsx — VC 动向表格
6. src/stores/intelStore.ts — Zustand store

## 关键约束
- 独立一级菜单，高频使用入口
- 评分 badge 分色：5=红, 4=橙, 3=黄, 2=灰, 1=浅灰
- 支持已读/关注/归档三种状态
- 默认按时间倒序，可切换评分排序

## Agent Prompt
/feature-workflow intel-feed

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.3 市场情报
- analyst-pro-prd.md §4.1 intel_items 表
- analyst-pro-prd.md §3.3.1 market-intel agent 输出路径

目标：
1. IntelFeed.tsx — Tab 切换 4 子视图
2. IntelList.tsx — 评分排序 + 分色 badge
3. DailyBrief.tsx — 每日简报
4. PolicyAlert.tsx — 政策预警
5. VCMoves.tsx — VC 动向
6. intelStore.ts

关键约束：
- 评分 badge 5 色
- 已读/关注/归档状态
```

---

**Issue 8: ****`#14 workspace-view — 工作台 UI`** *(blocked by #10)*

Priority: **High** | Tags: `ui-layer`, `layer-5`

```markdown
## 依赖
- #10 ipc-bridge ⬜（阻塞中）
- #2 sqlite-schema ✅
- #3 workspace-manager ✅

## PRD 参考
- analyst-pro-prd.md §5.2.4 工作台
- analyst-pro-prd.md §4.1 kpi_records 表
- analyst-pro-prd.md §5.4 AI 需求到菜单映射表

## 目标产出
1. src/components/workspace/WorkspaceView.tsx — 工作台主容器
2. src/components/workspace/KPITracker.tsx — KPI 追踪面板
3. src/components/workspace/DailyTodo.tsx — 今日待办
4. src/components/workspace/WeeklyPlan.tsx — 周计划
5. src/components/workspace/ReportCenter.tsx — 报告中心
6. src/components/knowledge/KnowledgeExplorer.tsx — 知识库浏览器
7. src/components/knowledge/GlossaryViewer.tsx — 术语库
8. src/stores/workspaceStore.ts — Zustand store

## 关键约束
- 一级菜单，含 5 个子视图
- KPI 来自 SQLite，Todo 聚合自 workspace 文件
- 报告中心按 report_type 分组
- 知识库浏览 workspace/knowledge/

## Agent Prompt
/feature-workflow workspace-view

请参考以下文件和章节：
- analyst-pro-prd.md §5.2.4 工作台
- analyst-pro-prd.md §4.1 kpi_records 表
- analyst-pro-prd.md §5.4 AI 需求到菜单映射表

目标：
1. WorkspaceView.tsx — 主容器
2. KPITracker.tsx — KPI
3. DailyTodo.tsx — 待办
4. WeeklyPlan.tsx — 周计划
5. ReportCenter.tsx — 报告中心
6. KnowledgeExplorer.tsx — 知识库
7. GlossaryViewer.tsx — 术语库
8. workspaceStore.ts

关键约束：
- 5 个子视图
- KPI 从 SQLite，Todo 从 workspace 文件
```

### 10.4 VK 并行开发顺序 — 3 个 Batch 完成剩余 8 模块

```
迁移后的执行计划（从 Batch A 开始）：

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Batch A — 立即启动（4 并行）
  依赖全部已满足，同时拖入 In Progress：

  ┌───────────────────┐  ┌───────────────────┐
  │ #10 ipc-bridge    │  │ #15 hitl-system   │
  │ ⭐ 关键路径        │  │ 独立 agent 层     │
  │ Variant: PLAN     │  │ Variant: DEFAULT  │
  │ 冲突: 低          │  │ 冲突: 无          │
  └───────────────────┘  └───────────────────┘
  ┌───────────────────┐  ┌───────────────────┐
  │ #16 keychain-auth │  │ #17 output-router │
  │ 独立 auth 层      │  │ 独立 workspace 层 │
  │ Variant: DEFAULT  │  │ Variant: DEFAULT  │
  │ 冲突: 无          │  │ 冲突: 无          │
  └───────────────────┘  └───────────────────┘

  冲突分析：
  - #10 改 electron/ipc/ + preload.ts + electron-api.d.ts
  - #15 改 electron/agents/hooks/ + electron/db/hitl.ts
  - #16 改 electron/auth/ + 新增 electron/ipc/auth.ipc.ts
  - #17 改 electron/workspace/output-router.ts + electron/db/file-outputs.ts
  → 四个模块核心文件完全不重叠，可安全 4 并行

  ⚠️ 唯一潜在冲突：#10 和 #16 都会新增 IPC handler
     #10 创建 7 个 ipc 文件，#16 新增 auth.ipc.ts
     → 冲突点在 preload.ts（都要注册 channel）
     → 解决：让 #10 先 merge，#16 rebase 后补充 auth channel

  合并顺序建议：
  1. #15 / #16 / #17（独立模块，先完成先合并）
  2. #10 最后合并（集成层，确保包含 #15/#16 的 IPC 接口）
  → 或者 #10 先合并也可以，#16 的 auth.ipc.ts 在 rebase 时追加

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Batch B — #10 merge 后启动（4 并行）
  等待 #10 ipc-bridge merge 到 master：

  ┌───────────────────┐  ┌───────────────────┐
  │ #11 chat-panel    │  │ #12 pipeline-view │
  │ ⭐ 核心 UI         │  │ 投资看板          │
  │ Variant: PLAN     │  │ Variant: DEFAULT  │
  └───────────────────┘  └───────────────────┘
  ┌───────────────────┐  ┌───────────────────┐
  │ #13 intel-feed    │  │ #14 workspace-view│
  │ 市场情报 UI        │  │ 工作台 UI         │
  │ Variant: DEFAULT  │  │ Variant: DEFAULT  │
  └───────────────────┘  └───────────────────┘

  冲突分析：
  - #11 改 src/components/chat/*
  - #12 改 src/components/pipeline/*
  - #13 改 src/components/intel/*
  - #14 改 src/components/workspace/* + src/components/knowledge/*
  → 四个模块目录完全不同，可安全 4 并行

  ⚠️ 共享冲突点：src/App.tsx（都要添加路由/组件引用）
     → 建议在 #10 ipc-bridge 完成时，预留 App.tsx 路由插槽：
        {/* ROUTE: chat */}
        {/* ROUTE: pipeline */}
        {/* ROUTE: intel */}
        {/* ROUTE: workspace */}
     → 或者按完成顺序 merge + rebase，VK 自动处理追加式修改

  合并顺序建议：先完成先合并，最后一个 rebase 时可能需要手动处理 App.tsx

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Batch C — 收尾验证
  全部 17 模块 merge 到 master 后：

  1. git checkout master
  2. npm run check:all        # 全量质量门禁
  3. npx electron-vite preview # 启动 app 验证 UI 集成
  4. 手动走一遍核心流程:
     - 打开 Chat → 发消息 → 收到回复
     - / 触发 SkillPalette → 选择 /today
     - Pipeline 看板 → 创建 Deal → 拖拽
     - Intel Feed → 查看情报列表
     - Workspace → 查看 KPI + Todo

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 10.5 VK 详细操作流程

#### Batch A 操作（4 并行）

```
Step 1: 同时启动 4 个 Tasks
─────────────────────────────
在 VK Board 中：

1. 点击 #10 ipc-bridge Issue → 点 + 创建 Attempt
   → Agent: CLAUDE_CODE
   → Variant: PLAN（复杂集成模块，先规划再执行）
   → Base Branch: master
   → VK 自动创建 worktree + 启动 agent

2. 点击 #15 hitl-system → 创建 Attempt
   → Agent: CLAUDE_CODE, Variant: DEFAULT, Base: master

3. 点击 #16 keychain-auth → 创建 Attempt
   → Agent: CLAUDE_CODE, Variant: DEFAULT, Base: master

4. 点击 #17 output-router → 创建 Attempt
   → Agent: CLAUDE_CODE, Variant: DEFAULT, Base: master

→ 4 个 Task 同时进入 In Progress，Board 实时显示运行状态

Step 2: 监控进度
─────────────────────────────
- Board 视图实时显示 4 个 Task 状态（Running / Idle / Completed）
- 点击任意 Task 查看 agent 的 Chat 输出流
- 预计 #15/#16/#17 先完成（简单模块），#10 最后完成（集成模块）

Step 3: 逐个 Review + Merge（先完成先处理）
─────────────────────────────
假设完成顺序：#17 → #16 → #15 → #10

▸ #17 output-router 完成：
  1. Task 自动移到 In Review
  2. 按 V C 打开 Changes 面板，审查 diff
  3. 检查清单：
     □ output-router.ts 路由规则正确
     □ file-outputs.ts CRUD 齐全
     □ 测试覆盖所有 agent 类型路由
     □ YAML frontmatter 格式正确
     □ CHANGELOG.md 已更新
  4. 通过 → 按 X R (Rebase) → 按 X M (Merge)
  5. Task 自动移到 Done

▸ #16 keychain-auth 完成：
  1. 同上 Review 流程
  2. 额外检查：safeStorage API 使用正确，无明文存储
  3. Rebase + Merge

▸ #15 hitl-system 完成：
  1. 同上 Review 流程
  2. 额外检查：canUseTool hook 拦截规则正确
  3. Rebase + Merge

▸ #10 ipc-bridge 完成：
  1. Review（重点检查）：
     □ preload.ts 包含 #15/#16 的 channel（如果 #15/#16 先 merge）
     □ electron-api.d.ts 类型完整
     □ 所有 7 个 ipc 文件齐全
  2. Rebase → 可能需要解决 preload.ts 冲突
     → 如果 VK 自动解决失败，用手动方式合并 channel 列表
  3. Merge

Step 4: 验证 Batch A
─────────────────────────────
在项目主目录：
  git checkout master
  npm run check:all           # 全量门禁
  → 绿灯后进入 Batch B
```

#### Batch B 操作（4 并行 UI 层）

```
Step 1: 确认 #10 已 merge
─────────────────────────────
  git log --oneline -5        # 确认 ipc-bridge merge commit

Step 2: 同时启动 4 个 UI Tasks
─────────────────────────────
  #11 chat-panel      → Variant: PLAN（最复杂的 UI 模块，9 个组件）
  #12 pipeline-view   → Variant: DEFAULT
  #13 intel-feed      → Variant: DEFAULT
  #14 workspace-view  → Variant: DEFAULT

  → 4 个 Task 同时进入 In Progress

Step 3: Review 重点
─────────────────────────────
UI 模块 Review 额外检查：
  □ 使用了 window.electronAPI.xxx() 而非直接 ipcRenderer
  □ Zustand store 使用 persist middleware
  □ TailwindCSS 类名，无内联样式
  □ 组件文件 < 300 行
  □ 无越界修改其他模块的组件

Step 4: Merge 策略
─────────────────────────────
  → 先完成先 merge
  → src/App.tsx 冲突时：合并各模块的路由/组件引用
  → 最后一个 merge 后运行 npm run check:all

Step 5: 最终验证（Batch C）
─────────────────────────────
  全部 17 模块 Done → Phase 1 MVP 完成
  → npx electron-vite preview  # 启动 app
  → 手动走核心流程验证
```

### 10.6 VK 迁移后的完整时间线

```
迁移前（手动完成）：
  #1 scaffold       ✅ ─┐
  #2 sqlite          ✅  ├─ Batch 0-3（手动四窗口）
  #3 workspace       ✅  │
  #4 provider        ✅  │
  #5 engine          ✅  │
  #6 secretary       ✅  │
  #7 market-intel    ✅  │
  #8 deal-analyst    ✅  │
  #9 skill-system    ✅ ─┘

迁移后（VK 接管）：
  ┌─ Batch A（4 并行）─────────────────────────┐
  │ #10 ipc-bridge ⭐   #15 hitl     #16 auth  │
  │                     #17 router              │
  │ → Review → Merge → master                  │
  └─────────────────────────────────────────────┘
            │
  ┌─ Batch B（4 并行）─────────────────────────┐
  │ #11 chat    #12 pipeline                    │
  │ #13 intel   #14 workspace                   │
  │ → Review → Merge → master                  │
  └─────────────────────────────────────────────┘
            │
  ┌─ Batch C（收尾验证）───────────────────────┐
  │ npm run check:all                           │
  │ npx electron-vite preview                   │
  │ 手动验收核心流程                              │
  └─────────────────────────────────────────────┘
            │
     Phase 1 MVP Done ✅
```

---

## 11. Troubleshooting

| 问题                             | 解决方案                                                   |
| ------------------------------ | ------------------------------------------------------ |
| Agent 启动后卡住不动                  | 检查 Claude Code 认证状态：`claude --version`                 |
| Merge 后 `npm run check:all` 失败 | 不要 merge 到 Done — 在 In Review 中 New Attempt 修复         |
| 磁盘空间不足                         | 减少并行度，或在 VK Settings 启用自动清理                            |
| 端口冲突（dev server）               | VK 内置 dev-manager-mcp 自动分配端口                           |
| Agent 修改了不该改的文件                | Review 时仔细检查 diff，拒绝越界修改                               |
| 语义冲突（两个模块逻辑矛盾）                 | Merge 后跑集成测试，发现问题创建新 Issue 修复                          |
| VK 本身崩溃                        | worktree 和 branch 仍在 git 中，`git worktree list` 查看，手动恢复 |

---

## 12. 参考链接

- [Vibe Kanban GitHub](https://github.com/BloopAI/vibe-kanban)

- [Vibe Kanban 官方文档](https://www.vibekanban.com/docs)

- [Vibe Kanban 实战指南 (VirtusLab)](https://virtuslab.com/blog/ai/vibe-kanban/)

- [parallel-coding-guide.md](./parallel-coding-guide.md) — 手动四窗口模式（本项目已有）

- [analystpro-feature-workflow-dev-guide.md](./analystpro-feature-workflow-dev-guide.md) — 模块分解详情 §3.3

