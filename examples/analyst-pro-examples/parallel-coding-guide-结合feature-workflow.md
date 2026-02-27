# AnalystPro 并行开发指南

> 基于 `analystpro-feature-workflow-dev-guide.md` §3.3 模块分解策略，分析 17 个模块的依赖关系、并行开发可行性和冲突解决方案。

---

## 1. 模块完成状态

| #  | 模块 | 状态 | 说明 |
|----|------|------|------|
| 1  | `electron-scaffold` | ✅ 已完成 | 基础脚手架 |
| 2  | `sqlite-schema` | ✅ 已完成 | SQLite 表定义 + CRUD |
| 3  | `workspace-manager` | ✅ 已完成 | Workspace 文件管理 |
| 4  | `provider-types` | ✅ 已完成 | Provider 接口定义 |
| 5  | `agent-engine` | ✅ 已完成 | Agent 引擎核心 |
| 6  | `secretary-agent` | ✅ 已完成 | Secretary 主 agent 配置 |
| 7  | `market-intel-agent` | ✅ 已完成 | 市场情报 agent |
| 8  | `deal-analyst-p1` | 🔧 开发中 (窗口1) | 项目分析 agent (Phase 1) |
| 9  | `skill-system` | 🔧 开发中 (窗口3) | Skill 加载/调用系统 |
| 10 | `ipc-bridge` | ⬜ 待开发 | IPC 桥接层 |
| 11 | `chat-panel` | ⬜ 待开发 | AI 对话面板 |
| 12 | `pipeline-view` | ⬜ 待开发 | 项目流程看板 |
| 13 | `intel-feed` | ⬜ 待开发 | 市场情报 UI |
| 14 | `workspace-view` | ⬜ 待开发 | 工作台 UI |
| 15 | `hitl-system` | ⬜ 待开发 | 人机审批系统 |
| 16 | `keychain-auth` | ⬜ 待开发 | Keychain 认证 |
| 17 | `output-router` | ⬜ 待开发 | 文件输出路由 |

---

## 2. 依赖图

```
Layer 0 (基础)
  #1 electron-scaffold ✅ ──────────────────────────────────────

Layer 1 (可并行 — 独立基础模块)
  #2 sqlite-schema      ✅ ── 需要 #1
  #3 workspace-manager   ✅ ── 需要 #1
  #4 provider-types      ✅ ── 需要 #1
  #16 keychain-auth      ⬜ ── 需要 #1

Layer 2 (可并行 — 依赖 Layer 1 子集)
  #5 agent-engine        ✅ ── 需要 #4
  #9 skill-system        🔧 ── 需要 #2
  #17 output-router      ⬜ ── 需要 #2 + #3

Layer 3 (可并行 — 依赖 agent-engine)
  #6 secretary-agent     ✅ ── 需要 #5 + #3
  #7 market-intel-agent  ✅ ── 需要 #5
  #8 deal-analyst-p1     🔧 ── 需要 #5
  #15 hitl-system        ⬜ ── 需要 #5 + #2

Layer 4 (集成层)
  #10 ipc-bridge         ⬜ ── 需要 #2 + #3 + #4 + #5 + #9

Layer 5 (可并行 — UI 层)
  #11 chat-panel         ⬜ ── 需要 #10 + #9
  #12 pipeline-view      ⬜ ── 需要 #10 + #2
  #13 intel-feed         ⬜ ── 需要 #10 + #2
  #14 workspace-view     ⬜ ── 需要 #10 + #2 + #3
```

---

## 3. 当前可并行开发分析

**当前窗口**：#6 secretary-agent（开发中）

**已解锁的模块**（依赖全部满足）：

| 模块 | 依赖 | 与 #6 文件冲突 | 战略价值 |
|------|------|---------------|---------|
| #7 market-intel-agent | #5 ✅ | 无（独立文件 `definitions/market-intel.ts`） | 高：与 #6 同属 agent 层，逻辑相关 |
| #8 deal-analyst-p1 | #5 ✅ | 无（独立文件 `definitions/deal-analyst.ts`） | 高：与 #6 同属 agent 层 |
| #9 skill-system | #2 ✅ | 无（独立目录 `electron/skills/`） | **最高**：解锁 #10 → #11 关键路径 |
| #15 hitl-system | #5 ✅ + #2 ✅ | 低（可能共改 `hooks/`） | 中：可推迟到 #10 之前 |
| #16 keychain-auth | #1 ✅ | 无（独立目录 `electron/auth/`） | 低：非关键路径 |
| #17 output-router | #2 ✅ + #3 ✅ | 无（独立文件 `workspace/output-router.ts`） | 低：非关键路径 |

### 推荐并行方案

```
窗口 1 (当前): #6 secretary-agent
窗口 2 (新开):  #7 market-intel-agent
窗口 3 (新开):  #9 skill-system
```

**选择理由**：

- **#7 market-intel-agent**：与 #6 同属 Layer 3 agent 定义层，文件零冲突（#6 改 `secretary.ts`，#7 创建 `definitions/market-intel.ts`），逻辑上紧密关联——Secretary 注册 market-intel 为 subagent，两者同时开发可以更好地对齐接口
- **#9 skill-system**：完全独立的子系统（`electron/skills/` 目录），文件零冲突。**战略价值最高**——它是 #10 ipc-bridge 的前置依赖，#10 又是全部 UI 层（#11-#14）的前置依赖。现在开发 #9 能最大程度缩短关键路径

**未选 #8 的原因**：#8 deal-analyst 虽然也无冲突，但 #7 + #8 同时开发意味着 agent 层 3 个模块全并行，合并时 `electron/agents/engine.ts`（注册 subagent）可能三方冲突。保留 #8 在 #6 + #7 合并后再做更稳妥。

---

## 4. 并行开发操作指南

### 4.1 使用 git worktree 创建独立工作目录

```bash
# 确保当前分支干净
git status -sb

# 为 #7 创建 worktree
git worktree add ../analyst-pro-wt-market-intel \
  -b feat/market-intel-agent feat/secretary-agent

# 为 #9 创建 worktree
git worktree add ../analyst-pro-wt-skill-system \
  -b feat/skill-system feat/secretary-agent
```

> 基于当前开发分支（`feat/secretary-agent`）创建，确保包含 #1-#5 的全部代码。

### 4.2 各窗口启动命令

```bash
# 窗口 2: market-intel-agent
cd ../analyst-pro-wt-market-intel
claude   # 启动 Claude Code
# 输入: /feature-workflow market-intel-agent（参照 §3.3.7 的上下文指引）

# 窗口 3: skill-system
cd ../analyst-pro-wt-skill-system
claude
# 输入: /feature-workflow skill-system（参照 §3.3.9 的上下文指引）
```

### 4.3 合并顺序

```
1. #6 secretary-agent   → merge 到 main（或开发主干）
2. #7 market-intel-agent → rebase 到 main → merge
3. #9 skill-system       → rebase 到 main → merge
每次 merge 后运行: npm run check:all
```

---

## 5. 冲突热点与预防

### 5.1 高冲突风险文件

| 文件 | 哪些模块会改 | 冲突类型 |
|------|------------|---------|
| `package.json` | 几乎全部（加依赖） | 依赖项追加 |
| `electron/main.ts` | #2, #3, #5, #9, #10, #15, #16 | import + 初始化代码 |
| `electron/preload.ts` | #10 主要 + 各 IPC 模块 | contextBridge 方法 |
| `src/App.tsx` | #11, #12, #13, #14 | JSX 路由/布局 |
| `src/types/electron-api.d.ts` | 所有需要 IPC 的模块 | 类型定义追加 |
| `CHANGELOG.md` | 全部 | 条目追加 |

### 5.2 低冲突文件（模块独占）

| 文件/目录 | 独占模块 |
|----------|---------|
| `electron/db/*` | #2 ✅ |
| `electron/workspace/manager.ts` | #3 ✅ |
| `electron/providers/types.ts` | #4 ✅ |
| `electron/agents/secretary.ts` | #6 ✅ |
| `electron/agents/definitions/market-intel.ts` | #7 ✅ |
| `electron/agents/definitions/deal-analyst.ts` | #8 |
| `electron/skills/*` | #9 🔧 |
| `electron/auth/*` | #16 |
| `electron/workspace/output-router.ts` | #17 |
| `src/components/chat/*` | #11 |
| `src/components/pipeline/*` | #12 |
| `src/components/intel/*` | #13 |
| `src/components/workspace/*` | #14 |

### 5.3 当前三窗口冲突预估

| 冲突文件 | #6 vs #7 | #6 vs #9 | #7 vs #9 |
|---------|----------|----------|----------|
| `package.json` | 可能（gray-matter 等） | 可能 | 可能 |
| `electron/main.ts` | 无 | 低（#9 可能注册 loader） | 无 |
| `electron/agents/engine.ts` | 低（注册 subagent） | 无 | 无 |
| 其他 | 无 | 无 | 无 |

**冲突等级：低**。三个模块的核心产出文件完全不重叠。

---

## 6. 后续批次规划

| 批次 | 模块 | 并行度 | 前置条件 |
|------|------|--------|---------|
| **当前** | #6 + #7 + #9 | 3 | #1-#5 ✅ |
| Batch A | #8 deal-analyst + #15 hitl + #16 keychain + #17 output-router | 2-3 | #6 完成 |
| Batch B | #10 ipc-bridge | 1（集成层串行） | #6 + #7 + #9 完成 |
| Batch C | #11 chat + #12 pipeline + #13 intel + #14 workspace-view | 2-3 | #10 完成 |

**预计总批次**：当前 + 3 批 = 4 个批次完成 Phase 1 全部 17 模块。

---

## 7. 四窗口并行开发模式（Coordinator + 3 Workers）

### 7.1 窗口角色分工

```
┌─────────────────────────────────────────────────────────┐
│ 窗口 0: Coordinator (协调窗口)                            │
│   目录: 项目主目录 (analyst-pro/)                          │
│   分支: master                                           │
│   职责: 状态追踪、合并、冲突解决、分配下一模块               │
│   不做: 功能开发                                          │
├─────────────────────────────────────────────────────────┤
│ 窗口 1-3: Worker (开发窗口)                               │
│   目录: 各自的 git worktree (analyst-pro-wt-xxx/)         │
│   分支: 各自的 feat/ 分支                                  │
│   职责: 执行 /feature-workflow，专注单模块开发               │
│   不做: 合并到 master、决定下一个模块                       │
└─────────────────────────────────────────────────────────┘
```

### 7.2 Coordinator 职责清单

#### 1) 维护本文档状态

每当 Worker 窗口完成或启动模块时，Coordinator 更新 §1 模块完成状态表：

```
告诉 Coordinator: "#7 已完成" 或 "#15 开始开发"
Coordinator 更新表格状态标记并记录窗口号
```

#### 2) 合并已完成模块到 master

Worker 完成模块后，Coordinator 在主目录执行合并：

```bash
# 在 Coordinator 窗口（项目主目录）
git checkout master
git merge feat/xxx-module
npm run check:all          # 必须绿灯
```

合并顺序原则：
- **先合并无冲突的**——文件独占的模块优先
- **后合并可能冲突的**——共享文件（package.json, main.ts）的模块后合并
- 每次合并后必须跑 `npm run check:all`

#### 3) 分配下一模块

Worker 窗口空闲时，Coordinator 根据以下决策树推荐下一个模块：

```
Worker 窗口空闲
│
├─ 检查 §2 依赖图：哪些模块已解锁？（依赖全部 ✅）
│
├─ 排除：已完成 ✅ / 其他窗口开发中 🔧 的模块
│
├─ 从剩余候选中选择：
│   │
│   ├─ 优先选关键路径上的模块
│   │   （解锁下游最多的模块优先，如 #9 解锁 #10 → #11-#14）
│   │
│   ├─ 次选与当前并行窗口文件冲突最小的模块
│   │   （参考 §5 冲突热点表）
│   │
│   └─ 同等条件下，选编号小的（保持 §3.3 推荐顺序）
│
└─ 输出: 模块编号 + worktree 创建命令 + /feature-workflow 启动指引
```

#### 4) 清理已合并的 worktree

模块合并到 master 后，及时清理：

```bash
git worktree remove ../analyst-pro-wt-xxx
git branch -d feat/xxx-module
```

### 7.3 Worker 窗口工作规范

#### 启动流程

```bash
# 1. Coordinator 给出 worktree 命令，Worker 执行：
cd ../analyst-pro-wt-xxx

# 2. 启动 Claude Code
claude

# 3. 执行 feature-workflow（参照 analystpro-feature-workflow-dev-guide.md §3.3 对应模块的上下文指引）
/feature-workflow module-name
# [粘贴完整上下文指引]
```

#### 完成流程

```bash
# 1. feature-workflow 9 个阶段全部通过
# 2. npm run check:all 绿灯
# 3. 代码已 commit 到 feat/ 分支
# 4. 通知 Coordinator: "窗口 N: #XX module-name 已完成"
# 5. 等待 Coordinator 指令（合并 + 分配下一模块）
```

#### Worker 不应该做的事

- 不要自己 merge 到 master（由 Coordinator 统一合并）
- 不要自己决定下一个模块（由 Coordinator 根据依赖图分配）
- 不要修改本文档（由 Coordinator 维护状态）
- 不要修改其他 worktree 的文件

### 7.4 Coordinator 与 Worker 沟通协议

采用简短的状态消息，在各窗口间通过口头或文字传递：

| 方向 | 消息格式 | 示例 |
|------|---------|------|
| Worker → Coordinator | `窗口N: #XX 已完成` | `窗口2: #7 market-intel-agent 已完成` |
| Worker → Coordinator | `窗口N: #XX 遇到问题 [描述]` | `窗口1: #8 test 失败，engine.ts 类型不匹配` |
| Coordinator → Worker | `窗口N: 合并完成，接 #XX` | `窗口2: #7 已合并 master，接 #15 hitl-system` |
| Coordinator → Worker | `窗口N: 请 rebase master` | `窗口3: master 更新了，请 git rebase master` |

### 7.5 Best Practices

#### 合并节奏

- **即完即合**：Worker 完成一个模块就立刻合并到 master，不要攒多个模块一起合并
- **合并后广播**：合并后通知所有正在开发的 Worker "master 已更新"，但 Worker 不需要立刻 rebase——只在自己模块完成后 rebase 即可
- **绿灯才合**：`npm run check:all` 不通过不合并，由 Coordinator 在主目录排查问题

#### 冲突预防

- **package.json 冲突**：最常见的冲突源。各 Worker 添加的依赖不同，合并时手动合并 dependencies 列表，然后 `npm install` 重新生成 lock file
- **CHANGELOG.md 冲突**：每个模块的条目写在 `[Unreleased]` 下方，合并时按模块编号排序即可
- **共享文件预留插槽**：如果 Coordinator 预见到多个模块会改同一个文件（如 `electron/main.ts`），可以提前在该文件中预留注释占位符，各模块只填充自己的区块

#### 关键路径优先

- 始终优先分配关键路径上的模块（解锁下游最多的）
- 当前关键路径：`#9 skill-system → #10 ipc-bridge → #11-#14 全部 UI`
- 非关键路径模块（#16 keychain, #17 output-router）在 Worker 空闲且关键路径模块无法开始时再分配

#### 避免三方冲突

- 同一目录下的模块尽量不要三个同时开发（如 #6 + #7 + #8 都在 `electron/agents/`）
- 两个同目录 + 一个不同目录是安全组合（如 #6 + #7 在 agents/ + #9 在 skills/）

#### Coordinator 自身纪律

- Coordinator 窗口**只做协调，不做开发**——避免 Coordinator 分支与 Worker 分支混乱
- 每次状态变更后立刻更新本文档 §1 表格
- 每个 Batch 结束后回顾 §6 批次规划，根据实际情况调整后续安排
- 如果 Worker 遇到阻塞问题（test 失败、依赖缺失），Coordinator 优先协助排障，必要时暂停该 Worker 窗口，把资源（窗口）分配给其他可开发模块

### 7.6 典型工作流示例

```
时间线：

[Coordinator]  创建 3 个 worktree，分配 #8, #9, #15
     │
     ├── [Worker 1] /feature-workflow deal-analyst-p1 ...
     ├── [Worker 2] /feature-workflow skill-system ...
     └── [Worker 3] /feature-workflow hitl-system ...
     │
[Worker 2]     "#9 完成" → 通知 Coordinator
     │
[Coordinator]  合并 #9 到 master → npm run check:all ✅
               更新本文档 #9 → ✅
               清理 worktree
               分配 Worker 2 接 #17 output-router（或 #16）
     │
[Worker 1]     "#8 完成" → 通知 Coordinator
     │
[Coordinator]  合并 #8 到 master → npm run check:all ✅
               更新本文档 #8 → ✅
               检查：#10 ipc-bridge 的依赖（#2+#3+#4+#5+#9）是否全部 ✅？
               → 是 → 分配 Worker 1 接 #10 ipc-bridge
     │
     ... 循环直到 17 个模块全部完成
```
