# 2603-04 Toent 解析与三表落地计划

## Background

- 当前 Toent 功能未闭环：`mdwt_commit` 只写 `MdwtRecord`，未稳定写入 toent 定义与实例。
- 旧设计里存在 `chnot_block` 语义，现已不适用；本任务统一为 `otid`（即 `chnot.otid`）单层模型。
- 已确认目标表结构：`toent_todo`、`toent_event`、`toent_inst`。

## Goals

- 在后端实现 `mdwt_commit -> 解析 toent -> 落定义表 -> 生成实例` 的最小闭环。
- 新增并纳管三张 toent 表，支持后续调度与动作执行扩展。
- 确保重复提交幂等，不产生重复实例。

## Scope

- In scope:
  - 新增表结构与迁移脚本：
    - `toent_todo(otid, todo_priority, todo_state, todo_closed, tid)`
    - `toent_event(otid, event_defi, tid)`，其中 `event_defi` 为 JSON 文本，使用对象数组：`{"events": [{...}]}`
    - `toent_inst(otid, timezone, naive_time, target_status, note, alert_tid, target_tid, tid)`
  - `mdwt_commit` 链路接入 toent 解析与写入。
  - 按 `otid` 重建未来窗口实例（MVP 默认 30 天）。
  - `toent_inst.naive_time` 使用字符串格式 `YYYY-MM-DD HH:MM:SS`。
  - `toent_inst` 增加 `alert_tid`、`target_tid` 用于提醒触发与目标执行追踪。
  - 前端 Toent MVP 页面：
    - `Toent` 日程页支持 `列表/周/月` 三视图切换。
    - 周视图以周一作为一周起始。
    - 月视图展示按天聚合的 `toent_inst` 摘要（超出折叠为 `+N`）。
    - 解析并展示 `toent_inst.naive_time(YYYY-MM-DD HH:MM:SS)`，完成按天分桶与排序。
- Out of scope:
  - Toent 前端高级交互（拖拽改期、批量编辑、复杂筛选持久化）。
  - 完整动作系统（通知、多类型执行器）与复杂调度策略。
  - 多 definition 模型（本期固定 1 otid 1 份 todo/event 定义）。

## Technical Plan

1. 数据模型与迁移
   - 重构 `lib/backend/src/krate/toent/po.rs` 为 `ToentTodo`、`ToentEvent`、`ToentInst`。
   - 为三个 PO 补齐 `GenerateTableSchema`、`TryFrom<&KDbRow>`、`Curd`、`impl_otid_support!`。
   - 新增 `data/sqls/v7-all.sql` 创建主表与 hist 表。
   - 更新 `data/db.version` 到 `7`。

2. Toent 持久层能力
   - 扩展 `lib/backend/src/krate/toent/mapper.rs`：upsert todo/event、重建 toent_inst。
   - 实现 `lib/backend/src/krate/toent/db.rs` 对应逻辑。
   - 幂等策略：按 `otid` 删除未来窗口实例后重建。

3. MDWT 接入与解析
   - 在 `lib/backend/src/krate/mdwt/db.rs` 的 `mdwt_commit` 里接入 toent 处理。
   - 解析规则：
     - `todo_priority` 来自 `[TODO !A]`。
     - `todo_state` 优先 `; STATE:` 最后一条，否则取 `[TODO ...]` 状态。
     - `todo_closed` 由状态推导（`DONE/CANCEL` 为 `true`）。
     - 一个 mdwt 可提取多个 event 定义，统一写入 `event_defi` JSON：`{"events": [{...}]}`。
     - `events` 元素使用对象格式，序列化直接使用 `serde_json`。
     - 推荐对象字段：`raw`（原始文本）、`standard`（标准化文本）、`timezone`（可空）。

4. 实例生成
   - 从 `toent_event.event_defi.events` 展开未来窗口（优先使用对象内 `standard` 字段）。
   - 写入 `toent_inst`：
     - `timezone`：从事件偏移提取（如 `+8:00`）。
     - `naive_time`：无时区字符串时间。
     - `target_status`：初始取 `toent_todo.todo_state`。
     - `note`：可空。
     - `alert_tid`：提醒触发时间（`TID`，可空）。
     - `target_tid`：目标执行时间（`TID`，不可空）。

5. 前端日程视图（MVP）
   - 在 `web/src/common/pages/toent-page.tsx` 落地 `列表/周/月` 视图切换。
   - 周视图：周一开周，支持上一周/下一周/回到今天。
   - 月视图：标准月历网格，按天聚合实例并展示状态/优先级标签。
   - 时间处理：为 `YYYY-MM-DD HH:MM:SS` 提供前端 parse 适配，保证排序和分组一致。

6. 枚举与同步纳管
   - 更新 `lib/backend/src/model/otid_table.rs`，加入新表映射。
   - 校正旧 `MdwtToent` 占位映射，避免误同步。

## Risks and Decisions

- Risk: 旧 parser 仍包含 block 时代逻辑，可能导致提取结果与新模型冲突 -> Mitigation: 在 `mdwt_commit` 中增加 otid 级提取路径，先保证主流程正确。
- Risk: `naive_time` 为文本，若格式不统一会影响排序和去重 -> Mitigation: 统一写入 `YYYY-MM-DD HH:MM:SS`，并在入库前格式化。
- Decision: 本期使用 1 `otid` 对应 1 份 `toent_todo` + 1 份 `toent_event`；多 event 通过 `event_defi.events` 承载。
- Decision: 解析失败不增加 `parse_ok/parse_error` 字段，仅日志记录并保留旧定义。

## Acceptance Criteria

- [ ] 提交包含 `[TODO]` 的 mdwt 内容后，可写入或更新 `toent_todo`。
- [ ] 提交包含多个 `; EVENT:` 的 mdwt 内容后，`toent_event.event_defi` 可写入对象数组 JSON：`{"events": [{"raw":"...","standard":"...","timezone":"+8:00"}]}`。
- [ ] 提交后可为该 `otid` 生成 `toent_inst`，且 `naive_time` 格式统一为 `YYYY-MM-DD HH:MM:SS`。
- [ ] 同一内容重复提交不产生重复实例。
- [ ] 修改 EVENT 定义后，实例按新定义重建。
- [ ] Toent 页面提供 `列表/周/月` 三视图切换，周视图以周一为起始。
- [ ] 月视图可按天看到 `toent_inst` 聚合结果，且同日实例按时间升序展示。

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2603-04/toent-tables-and-inst`
- [ ] Create first commit: `task-init: toent tables and inst plan`
- [ ] Implement in small commits
- [ ] Move file to `docs/done/` after completion
- [ ] Create final commit: `task-done: 2603-04-toent-tables-and-inst`
