# Toent 逻辑修正计划

## 背景

`docs/impl-spec/toent.md` 定义了 Toent 的目标流程（写入定义、内存维持、读取合并覆盖），但当前后端实现与该流程不一致，导致查询口径和状态持久化行为存在偏差。

本计划基于当前代码现状梳理，目标是在不破坏现有 API 兼容性的前提下，逐步对齐规范。

相关代码入口：

- `lib/backend/src/krate/mdwt/db.rs`
- `lib/backend/src/krate/toent/db.rs`
- `lib/backend/src/krate/toent/mapper.rs`
- `lib/backend/src/krate/toent/dto.rs`
- `lib/backend/src/app.rs`

## 对比结论（现状 vs 规范）

### 1) 写入链路

规范要求：

- mdwt 更新时重算 `time_event` 的开始/结束条件（含时间、次数）并取并集；
- 判断是否有 `todo_event`；
- 将二者写入 `toent_defi`。

现状：

- `mdwt_commit` 已触发 `toent_defi_commit`；
- 新增了 `toent_inst_commit` 接口骨架，但未接入完整调用链；
- `toent_inst_commit` 当前存在 `ToentDefi` 查询未按 `otid` 过滤的问题，可能读错定义；
- `ToentInstCommitReq.note` 尚未落库。

### 2) 内存维持

规范要求：

- 启动时或 23:59 预抓取当天 toent；
- mdwt 更新时刷新当天 toent，并覆盖内存中该 `otid` 的未持久化事件。

现状：

- `AppState` 仍无 Toent 内存缓存字段；
- 无启动预热与跨天刷新任务；
- 无 mdwt 更新后的缓存局部刷新。

### 3) 读取合并

规范要求：

- 先取内存与查询区间交集；
- 再与 `toent_defi` 做并集；
- 最后以数据库区间结果覆盖内存结果。

现状：

- 当前仅做 `toent_defi` 生成实例 + `toent_inst` 覆盖；
- 尚未接入内存层；
- 区间判定条件仍有改进空间（需统一为标准相交判定）。

## 修正目标

1. 修复实例提交链路关键错误（`otid` 过滤、状态推进、note 落库）。
2. 将实例提交能力打通到 controller/API 与前端调用链。
3. 实现 Toent 当日内存缓存和刷新机制。
4. 将搜索改造为“内存交集 + defi 并集 + DB 覆盖”的三段式。
5. 补齐测试，保障统计和查询口径一致。

## 文件修改清单

### 已修改（本轮已存在改动）

- `lib/backend/src/krate/toent/dto.rs`
  - 新增 `ToentInstCommitReq` / `ToentInstCommitRsp`；
  - 新字段包含 `otid`、`inst_tid`、`note`、`todo_state`。
- `lib/backend/src/krate/toent/mapper.rs`
  - `ToentMapper` trait 由 `toent_commit` 调整为 `toent_inst_commit`；
  - `MapperType` 分支分发更新为 `toent_inst_commit`。
- `lib/backend/src/krate/toent/db.rs`
  - 新增 `KDb::toent_inst_commit` 事务入口；
  - 新增 `KDbTx::toent_inst_commit` 实现骨架；
  - 目前仍存在待修问题：`ToentDefi` 查询未按 `otid` 过滤、`note` 未落库。

### 待修改（按阶段执行）

- `lib/backend/src/krate/toent/db.rs`
  - 修复 `toent_inst_commit` 的 `otid` 精确查询；
  - 完善 `finished_count` 与状态推进逻辑；
  - 将 `req.note` 写入 `ToentInst.note`；
  - 重构 `toent_search` 为“内存交集 + defi 并集 + DB 覆盖”。
- `lib/backend/src/krate/toent/controller.rs`
  - 新增 `toent_inst_commit` HTTP 路由与处理函数；
  - 保持 `toent_todo_state_commit` 兼容。
- `lib/backend/src/app.rs`
  - 新增 Toent 当日缓存字段（并发安全容器）；
  - 为启动加载与跨天刷新提供状态承载。
- `lib/backend/src/krate/mdwt/db.rs`
  - 在 `mdwt_commit` 后补充实例层刷新/提交触发；
  - 保证定义层与实例层行为一致。
- `web/src/krate/toent/service.ts`（若文件不存在则创建）
  - 增加 `toentInstCommit` 请求封装；
  - 与后端新接口保持 DTO 对齐。
- `web/src/krate/toent/*` 相关调用点
  - 接入实例级提交（状态、备注）；
  - 保持原有 Toent 页面交互不回退。

## 分阶段实施

### 阶段 1：修复实例提交链路（高优先级）

- 修复 `toent_inst_commit` 中 `ToentDefi` 查询条件，按 `req.otid` 精确查询；
- 明确 `finished_count` 增量逻辑，按“本次状态变更”而非定义快照判定；
- 将 `req.note` 写入实例（若为修改已有实例，遵循覆盖规则）；
- 增加空定义、无实例、重复提交等边界保护。

验收：

- 提交同一 `otid` 不会读取到其他任务定义；
- DONE 状态推进计数符合预期；
- note 可在实例中读回。

### 阶段 2：打通 API 入口与调用链

- 在 `toent/controller.rs` 增加 `toent_inst_commit` 路由；
- 保持现有 `toent_todo_state_commit` 兼容，同时提供实例粒度提交能力；
- Web 端补充 `toentInstCommit` service（不改已有接口语义）。

验收：

- 后端接口可独立调用并成功写入；
- 前端可按实例提交状态/备注。

### 阶段 3：引入内存层（当天缓存）

- 在 `AppState` 新增 Toent 缓存容器（按 `otid` 组织，支持并发读写）；
- 启动时加载“当天区间相交”数据；
- 增加跨天刷新机制（优先实现请求时日期切换检测，后续可加定时任务）；
- mdwt 更新后对目标 `otid` 执行局部缓存刷新。

验收：

- 启动后首次查询命中缓存；
- 跨天后缓存自动切换到新日期窗口；
- mdwt 更新后目标任务缓存及时刷新。

### 阶段 4：重构搜索三段式合并

按规范实现：

1. 读取内存层与查询区间相交实例；
2. 根据 `toent_defi` 生成实例并与内存并集；
3. 读取 `toent_inst` 并覆盖前两步结果（数据库优先）。

同时统一：

- 区间相交判定函数；
- `include_completed` / `include_uncompleted` / `include_no_time_todo` 过滤顺序。

验收：

- 列表/周/月查询口径一致；
- 数据库覆盖优先级稳定。

### 阶段 5：测试与回归

- 单元测试：
  - 区间相交判定边界；
  - 状态推进与 `finished_count`；
  - note 写入读取。
- 集成测试：
  - `mdwt_commit -> toent_defi`；
  - `toent_inst_commit` 实例落库；
  - `toent_search` 三段合并优先级。
- 回归检查：
  - `toent_inst_count` 与 `toent_search` 统计口径一致。

## 风险与规避

- 风险：引入缓存后并发一致性复杂度上升。
  - 规避：先用单一缓存窗口（当天）+ 明确 overwrite 规则，避免过早泛化。
- 风险：实例提交语义与现有 TODO 切换行为冲突。
  - 规避：保留旧接口行为，新增接口渐进接管。
- 风险：三段合并导致性能回退。
  - 规避：先保证正确性，再按热点查询增加缓存索引与分页优化。

## 执行顺序

1. 阶段 1（修实例提交关键 bug）
2. 阶段 2（补 API 与调用链）
3. 阶段 3（加当日缓存）
4. 阶段 4（改搜索三段式）
5. 阶段 5（测试回归）
