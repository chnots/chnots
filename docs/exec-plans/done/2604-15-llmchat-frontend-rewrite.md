# LLMChat 前端全面重构

## Background

- llmchat 前端经过多轮迭代积累了较多技术债：hooks 违规（biome-ignore-all 掩盖）、无 memo 化、组件过大、重复数据请求等
- 缺少关键功能：多模态输入、工具调用（MCP/Function Calling）、复制 record 为 markdown、历史版本
- 流式动画体验不够好，希望有 token 级淡入效果
- 用户对话通常 < 20 条，虚拟化不急迫

## Goals

1. **代码质量**：修复 hooks 违规、添加 memo/useMemo、拆分大组件、修复滚动检测逻辑
2. **架构升级**：迁移到 `useChat` + `DirectChatTransport`，获得原生 tool calling + 多模态支持
3. **流式体验**：Token 级淡入动画
4. **新功能**：多模态输入、工具调用 UI、复制 record 为 markdown

## Scope

- In scope: 前端全面重构，安装 `@ai-sdk/react`，组件拆分，新功能开发
- Out of scope: 后端 API 变更、虚拟化（对话短不需要）、UI 设计风格大改

## Technical Decisions

| 决策项 | 结论 |
|--------|------|
| 状态管理 | 保留 Zustand，优化 store 结构和 selector |
| 流式架构 | 迁移到 `useChat` (`@ai-sdk/react@3.0.162`) + `DirectChatTransport` + `ToolLoopAgent` |
| 多模态 | 扩展 ContentBlock → UIMessage `FileUIPart`，支持图片/文件/音频全类型 |
| 工具调用 | 按工具可配置（自动执行 / 需确认），利用 `ToolUIPart` 的 `approval-requested` 状态机 |
| 组件架构 | 细粒度拆分，职责单一 |
| 流式动画 | Token 级 CSS 淡入效果 |
| 数据转换 | `ContentBlock[]` ↔ `UIMessage` 双向转换层 |
| 范围约束 | 只改前端，后端 API 不变 |

## UIMessage 数据结构参考

当前 `ContentBlock` 到 `UIMessagePart` 的映射关系：

| 现有 ContentBlock | UIMessage Part |
|---|---|
| `{ type: "thinking", data: string }` | `ReasoningUIPart { type: 'reasoning', text, state: 'streaming' \| 'done' }` |
| `{ type: "content", data: string }` | `TextUIPart { type: 'text', text, state: 'streaming' \| 'done' }` |
| `{ type: "error", data: string }` | 自定义 `DataUIPart<{ error: string }>` |
| 无 | `FileUIPart` — 多模态（图片/文件/音频） |
| 无 | `ToolUIPart` — 工具调用，含 `input-streaming → input-available → approval-requested → output-available → output-error` 状态机 |
| 无 | `SourceUrlUIPart` / `SourceDocumentUIPart` — 引用来源 |
| 无 | `StepStartUIPart` — 多步骤边界 |

## Phase 1 — 代码质量治理

**目标**：不改变功能，修复技术债，为后续架构迁移打基础。

### 1.1 修复 hooks 违规

- **文件**：`web/src/krate/llmchat/component/record-response.tsx`
- **问题**：`RecordAnswering` 在 line 37-39 有 early return，之后的 `useRef`、`useLLMResponse`、`useEffect` 不满足 hooks 规则，用 `biome-ignore-all` 掩盖
- **方案**：将条件逻辑从组件内 early return 改为条件渲染子组件（`RecordAnsweringInner`），父组件调用 hooks 后再决定渲染什么

### 1.2 添加 memo 化

- `RecordUser`、`RecordAssistant`、`RecordCommon` 添加 `React.memo`
- `SessionContainer` 中 `records.toSorted(...)` 改为 `useMemo`（line 386-388）
- 内容解析等计算值添加 `useMemo`
- Zustand selector 使用 `useShallow` 精细化（部分已有，补齐缺失）

### 1.3 拆分大组件

将 `session.tsx` (~400行) 拆分为：
- `session-container.tsx` — 主容器，组合各子模块
- `record-list.tsx` — record 列表渲染与排序
- `scroll-manager.tsx` — 滚动检测与自动滚动逻辑
- `persistence-manager.tsx` — record 持久化逻辑（当前散落在 useEffect 中）

将 `record-frame.tsx` / `record-assistant.tsx` 拆分为：
- `thinking-block.tsx` — 思考过程折叠展示
- `content-renderer.tsx` — Markdown/代码/数学公式渲染（封装 streamdown）
- `action-toolbar.tsx` — 复制、重新生成等操作按钮栏
- `record-frame.tsx` — 保留为布局容器（logo、名称、时间戳）

### 1.4 修复滚动检测

- **文件**：`session.tsx` line 357-362
- **问题**：`onScroll` 中的 atBottom 计算逻辑 `Math.abs(rect.y - rect.height - 50) > contentRef.current.scrollHeight` 可能反了
- **方案**：改为标准 at-bottom 检测：`container.scrollHeight - container.scrollTop - container.clientHeight < threshold`

### 1.5 消除重复请求

- `refreshBots` / `refreshTemplates` 在 `LLMChatChnot` 和 `SessionContainer` 中各调用一次
- 统一在 `LLMChatChnot`（最外层）调用一次，通过 props 或 context 传递

## Phase 2 — useChat 迁移

**目标**：用 AI SDK 的 `useChat` 替换自定义 `useLLMResponse`，获得原生 tool calling 和多模态支持。

### 2.1 安装依赖

```bash
pnpm add @ai-sdk/react@3.0.162
```

### 2.2 创建 DirectChatTransport 配置层

- **新文件**：`web/src/krate/llmchat/transport.ts`
- 封装 `createLLMProvider()` → `ToolLoopAgent` → `DirectChatTransport` 的创建流程
- 当 bot 切换时重建 transport
- 注册 MCP 工具到 agent 的 `tools` 参数

### 2.3 构建数据转换层

- **新文件**：`web/src/krate/llmchat/message-adapter.ts`
- `recordVOToUIMessages(records: LLMChatRecordVO[]): UIMessage[]`
  - user record → `{ role: 'user', parts: [TextUIPart, FileUIPart...] }`
  - assistant record → `{ role: 'assistant', parts: [ReasoningUIPart, TextUIPart, ToolUIPart...] }`
  - system record → `{ role: 'system', parts: [TextUIPart] }`
  - ContentBlock `thinking` → `ReasoningUIPart`
  - ContentBlock `content` → `TextUIPart`
  - ContentBlock `error` → `DataUIPart<{ error: string }>`
- `uiMessageToRecordVO(msg: UIMessage, sessionOtid: TID, preRecordOtid?: TID): LLMChatRecordVO`
  - 反向转换，用于持久化
- `contentBlocksToParts(blocks: ContentBlock[]): UIMessagePart[]`
- `partsToContentBlocks(parts: UIMessagePart[]): ContentBlock[]`

### 2.4 重写 RecordAnswering

- **文件**：`web/src/krate/llmchat/component/record-response.tsx`
- 用 `useChat` 替换 `useLLMResponse`
- 通过 `useChat` 的 `initialMessages` 加载历史 records（经转换层）
- 通过 `onFinish` 回调将完成的 UIMessage 转换回 record 并同步到 Zustand store
- 保留 unmount 时的 in-flight 响应持久化（通过 `useChat` 的 `messages` 状态在 cleanup 中读取）
- 保留 abort 能力（`useChat` 的 `stop()`）

### 2.5 Token 级淡入动画

- 在流式 `TextUIPart` 的 `state === 'streaming'` 时，对新追加的 token 应用 CSS `opacity` 淡入
- 实现方案：在 `content-renderer.tsx` 中，对 streaming 状态的文本节点，用 CSS `@keyframes fadeIn { from { opacity: 0 } to { opacity: 1 } }` + `animation: fadeIn 150ms ease-out`
- 需要追踪上次渲染的文本长度，只对新增部分应用动画（避免全文本重复动画）

### 2.6 清理

- 移除 `web/src/hooks/use-llm-response.ts`（被 `useChat` 替代）
- 移除 `record-response.tsx` 的 `biome-ignore-all`

## Phase 3 — 新功能开发

### 3.1 复制 record 为 markdown

- 在 `action-toolbar.tsx` 添加"复制为 Markdown"按钮
- 实现 `recordToMarkdown(record: LLMChatRecordVO): string`
  - 遍历 ContentBlock，`content` 类型原样输出，`thinking` 用 `<details><summary>Thinking</summary>...</details>` 包裹，`error` 用 `> ❌ ...` 引用格式
- 使用 `navigator.clipboard.writeText()` 复制

### 3.2 多模态输入

- **改造**：`user-input.tsx`
  - 添加拖拽区域 + 粘贴监听（paste event）
  - 文件预览：缩略图列表（可删除单个附件）
  - 转为 `FileUIPart`（Data URL 或上传到后端获取 URL）
- **后端不变**：`ContentBlock` 新增 `image` / `file` / `audio` 类型（前端渲染层处理，后端只做 JSON 存储）

### 3.3 工具调用 UI

- **新文件**：`web/src/krate/llmchat/component/tool-call-block.tsx`
- 可展开/收起的工具调用展示组件：
  - `input-streaming`：显示工具名称 + 加载动画
  - `input-available`：显示工具名称 + 参数 JSON（可展开）
  - `approval-requested`：显示确认/拒绝按钮 + 参数编辑（如果需要确认的工具）
  - `output-available`：显示结果（可折叠）
  - `output-error`：显示错误信息
  - `output-denied`：显示已拒绝标记
- 工具配置：在 `BotForm` 中添加工具管理面板（哪些自动执行、哪些需要确认）

### 3.4 历史版本

- 沿用已有计划 `docs/exec-plans/active/2604-15-llmchat-history.md`，在本阶段实施

## Risks and Decisions

- **Risk**：`useChat` + `DirectChatTransport` 是 AI SDK v6 较新的 API，文档可能不完善 → Mitigation：保留 `useLLMResponse` 作为 fallback，分步替换
- **Risk**：`ContentBlock` ↔ `UIMessage` 转换层可能有边界情况（legacy `<aisse>` thinking 解析）→ Mitigation：转换层独立模块，可单测
- **Risk**：多模态文件体积大，Data URL 可能导致存储膨胀 → Mitigation：先支持 Data URL，后续可改为后端上传
- **Decision**：不使用虚拟化（对话 < 20 条，不值得引入复杂度）
- **Decision**：`error` 内容块用 `DataUIPart` 扩展，不修改 AI SDK 原生类型
- **Decision**：Phase 1 不改数据流，只改代码结构；Phase 2 改数据流但不改后端；Phase 3 加新功能

## Acceptance Criteria

Phase 1：
- [ ] `biome-ignore-all` 移除，所有 hooks 调用在组件顶层
- [ ] `session.tsx` 拆分为 ≤ 200 行的独立模块
- [ ] 所有 record 组件使用 `React.memo`
- [ ] `npm run lint` 和 `npm run check` 通过
- [ ] 功能无回归（手动验证对话流程正常）

Phase 2：
- [ ] `useChat` 成功替换 `useLLMResponse`，对话流程正常
- [ ] 思考过程、内容、错误正常显示
- [ ] Token 级淡入动画生效
- [ ] 中止生成功能正常
- [ ] unmount 时 in-flight 响应正确持久化
- [ ] `use-llm-response.ts` 已移除

Phase 3：
- [ ] 复制 record 为 markdown 功能正常
- [ ] 拖拽/粘贴图片、文件、音频正常发送给 LLM
- [ ] 工具调用 UI 正确展示各状态
- [ ] 需确认的工具弹出确认对话框
- [ ] 历史版本功能正常（沿用验收标准）

## File Changes

| Phase | File | Change |
|-------|------|--------|
| 1 | `web/src/krate/llmchat/component/record-response.tsx` | 修复 hooks 违规，拆分为条件渲染 |
| 1 | `web/src/krate/llmchat/component/record-user.tsx` | 添加 React.memo |
| 1 | `web/src/krate/llmchat/component/record-assistant.tsx` | 添加 React.memo，拆分子组件 |
| 1 | `web/src/krate/llmchat/component/session.tsx` | 拆分为多个模块 |
| 1 | `web/src/krate/llmchat/component/session-container.tsx` | **新文件** — 主容器 |
| 1 | `web/src/krate/llmchat/component/record-list.tsx` | **新文件** — record 列表 |
| 1 | `web/src/krate/llmchat/component/scroll-manager.tsx` | **新文件** — 滚动管理 |
| 1 | `web/src/krate/llmchat/component/persistence-manager.tsx` | **新文件** — 持久化逻辑 |
| 1 | `web/src/krate/llmchat/component/thinking-block.tsx` | **新文件** — 思考过程展示 |
| 1 | `web/src/krate/llmchat/component/content-renderer.tsx` | **新文件** — 内容渲染 |
| 1 | `web/src/krate/llmchat/component/action-toolbar.tsx` | **新文件** — 操作按钮栏 |
| 2 | `web/src/krate/llmchat/transport.ts` | **新文件** — DirectChatTransport 配置 |
| 2 | `web/src/krate/llmchat/message-adapter.ts` | **新文件** — ContentBlock ↔ UIMessage 转换 |
| 2 | `web/src/krate/llmchat/component/record-response.tsx` | 用 useChat 重写 |
| 2 | `web/src/hooks/use-llm-response.ts` | **删除** |
| 2 | `web/package.json` | 添加 `@ai-sdk/react@3.0.162` |
| 3 | `web/src/krate/llmchat/component/user-input.tsx` | 多模态拖拽/粘贴 |
| 3 | `web/src/krate/llmchat/component/tool-call-block.tsx` | **新文件** — 工具调用 UI |
| 3 | `web/src/krate/llmchat/component/bot-form.tsx` | 工具管理面板 |
| 3 | `web/src/krate/llmchat/component/history-tree.tsx` | **新文件**（沿用已有计划） |

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2604-15/llmchat-frontend-rewrite`
- [ ] Phase 1: 代码质量治理（小步提交）
  - [ ] task-init: llmchat frontend rewrite phase 1
  - [ ] 修复 hooks 违规
  - [ ] 添加 memo 化
  - [ ] 拆分大组件
  - [ ] 修复滚动检测
  - [ ] 消除重复请求
  - [ ] task-done: llmchat frontend rewrite phase 1
- [ ] Phase 2: useChat 迁移（小步提交）
  - [ ] task-init: llmchat frontend rewrite phase 2
  - [ ] 安装 @ai-sdk/react
  - [ ] 创建 transport 配置层
  - [ ] 构建消息转换层
  - [ ] 重写 RecordAnswering
  - [ ] 实现 token 级淡入动画
  - [ ] 清理旧代码
  - [ ] task-done: llmchat frontend rewrite phase 2
- [ ] Phase 3: 新功能开发（小步提交）
  - [ ] task-init: llmchat frontend rewrite phase 3
  - [ ] 复制 record 为 markdown
  - [ ] 多模态输入
  - [ ] 工具调用 UI
  - [ ] 历史版本
  - [ ] task-done: llmchat frontend rewrite phase 3
- [ ] Move file to `docs/done/` after completion
