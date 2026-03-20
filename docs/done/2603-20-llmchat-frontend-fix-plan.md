# LLMChat 前端修复整理（自动命名 + 重新生成）

## 背景

当前 `LLMChat` 前端有两个用户可见问题：

- 会话首次产生内容后，块标题没有自动保存。
- assistant 消息的“重新生成”在部分场景下会失效或表现不稳定。

本次修复目标是在不改后端接口的前提下，提升会话命名与重生成功率。

## 问题定位

### 1) 自动命名未落盘

- `LLMChat` 会话首次保存时，`onPostSave` 只上报 `saveState/kind`，未传 `title`。
- 上层 `chnot` 标题保存依赖 `onPostSave.title`，因此无法自动更新标题。

### 2) 重新生成偶发失效

- UI 展示前对消息做了排序，但 `regenrate` 截断逻辑基于原始数组顺序遍历。
- 当两者顺序不一致时，可能出现截断点判断异常，导致“有时不好用”。
- 另外，流式输出中的临时 assistant 记录也显示了重生按钮，容易误触到未持久化记录。

## 修复方案

### A. 自动生成并传递会话标题

- 在 `session.tsx` 新增 `buildSessionTitle(records)`：
  - 取第一条 user 消息（按 `otid` 排序后取 first）。
  - 压缩多空白为单空格并 `trim`。
  - 截断到 80 字符，作为会话标题。
- 在首次 `llmchatSessionCommit` 成功后，调用 `onPostSave(session, title)`。
- 在 `llmchat.tsx` 将该 `title` 继续透传给 `chnot` 的 `onPostSave`，触发标题落盘。

### B. 统一重生截断口径

- 在 `regenrate(recordOtid)` 内先对记录按 `otid` 排序。
- 再基于排序后的列表定位目标记录 index，并 `slice(0, index)` 截断。
- 保留原有后端截断请求 `llmchatSessionRecordTruncate`。

### C. 避免流式消息误触重生

- `record-assistant.tsx` 中，仅在 `!isAnimating` 时显示“重新生成”按钮。
- 生成中那条临时记录不提供重生入口，避免点击无效 ID。

## 修改文件

- `web/src/krate/llmchat/component/session.tsx`
  - 新增标题构造方法。
  - 调整 `onPostSave` 类型签名为 `(session, title?)`。
  - 重构 `regenrate` 的截断逻辑。
- `web/src/krate/chnot/component/rich-chnot/llmchat.tsx`
  - 接收并透传 `title` 给上层 `onPostSave`。
- `web/src/krate/llmchat/component/record-assistant.tsx`
  - 生成中隐藏重生按钮。

## 验收点

- 新建 LLMChat，会话第一条 user 消息发送后，块标题自动更新为消息摘要。
- 对已完成 assistant 消息反复点击“重新生成”，行为稳定可复现。
- 正在流式生成时，不展示“重新生成”按钮。

## 兼容性与影响

- 不涉及后端 API 变更。
- 不改数据库结构。
- 仅调整前端会话保存与重生交互逻辑，影响范围限定在 LLMChat 相关组件。
