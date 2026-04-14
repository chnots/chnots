# LLMChat History 功能

## Background

- llmchat 目前没有 history 功能，用户无法回溯会话到之前的某个状态
- 其他 chnot 类型（mdwt、excalidraw、kfile、mind-elixir）已有成熟的 history 模式可参考
- llmchat 的 record 提交时已自动写入 `_hist` 表（由 `impl_otid_support!` 宏支持），底层数据已就绪

## Goals

- 在 llmchat 右上角添加 history 按钮，点击后展示会话历史版本树
- 选择某个历史节点后，会话恢复为该时刻的状态（只读预览）
- 点击 Apply 从该状态继续对话，或点击 Latest 回到最新状态
- 移动端适配

## Scope

- In scope: 前端 history UI、tree 构建、预览/apply/latest 流程、dto.ts / service.ts 修改
- Out of scope: 后端改动（完全复用现有 `llmchat_session_record_fetch(include_hist=true)` 和 `llmchat_session_record_truncate`）

## Technical Plan

### 1. 修改 `web/src/krate/llmchat/dto.ts`

新增 `LLMChatSessionRecordFetchReq` 的 `include_hist` 字段支持（已有但前端未使用）。

### 2. 修改 `web/src/krate/llmchat/service.ts`

- 修改 `llmchatSessionRecordFetch` 支持传入 `include_hist: true`，获取主表 + `_hist` 表全量 records

### 3. 新增 `web/src/krate/llmchat/component/history-tree.tsx`

从全量 records（含 hist）构建树形结构并渲染：

- **树构建逻辑**：利用 `pre_record_otid` linked list 关系 + `tid` 时间排序推导分支，每次 truncate + 续写产生一条新分支
- **节点显示**：record 角色（user/assistant/system）图标 + 时间 + 内容摘要
- **布局**：桌面端用 Popover，移动端用 Sheet（底部弹出）
- 选中节点回调 `onViewVersion(tid: TID)`

### 4. 改造 `web/src/krate/chnot/component/rich-chnot/llmchat.tsx`

- 新增 state: `historyRecords`, `previewTid`, `previewRecords`
- 通过 `chnotHeadStore.registerHeaderActions` 注册 history 按钮到右上角 header
- 复用现有 `HistoryHeaderActions` 组件（与 mdwt 等一致）
- `onOpenHistory` → 调用 `llmchatSessionRecordFetch({ session_otid, include_hist: true })` 获取全量 records，构建版本列表
- `onViewVersion(tid)` → 前端过滤 tid 之前的 records 作为 `previewRecords`
- 预览模式：用 `previewRecords` 渲染只读聊天记录（传入 `readonly=true`）
- `onApply` → 调用 `llmchat_session_record_truncate` 截断到目标 record，更新 store 中的 records，退出预览
- `onLatest` → 清除预览状态回到当前

## Risks and Decisions

- Risk: linked list 分支推导逻辑可能复杂 → Mitigation: 第一版先用扁平列表（与 mdwt 一致），tree 结构作为后续增强
- Decision: 不修改后端，复用现有 API，减少改动范围
- Decision: 使用 Popover 而非 Sidebar 展示历史，与现有 mdwt/excalidraw 的 history 体验保持一致

## Acceptance Criteria

- [ ] 右上角出现 history 按钮
- [ ] 点击 history 按钮能看到历史版本列表（含时间和摘要）
- [ ] 点击某个版本，会话切换为该时刻的状态（只读）
- [ ] 点击 Apply，会话从该历史状态继续，可正常对话
- [ ] 点击 Latest，回到最新状态
- [ ] 移动端正常使用

## File Changes

| File | Change |
|---|---|
| `web/src/krate/llmchat/dto.ts` | 新增 history 相关类型 |
| `web/src/krate/llmchat/service.ts` | 修改 fetch 函数支持 include_hist |
| `web/src/krate/llmchat/component/history-tree.tsx` | **新文件** — 历史版本列表 UI |
| `web/src/krate/chnot/component/rich-chnot/llmchat.tsx` | 主要改造：state、history 逻辑、预览模式、header actions 注册 |

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2604-15/llmchat-history`
- [ ] Create first commit: `task-init: llmchat history feature`
- [ ] Implement in small commits
- [ ] Move file to `docs/done/` after completion
- [ ] Create final commit: `task-done: llmchat history feature`
