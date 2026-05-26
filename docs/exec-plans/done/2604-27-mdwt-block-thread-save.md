# MDWT 保存时解析 Block 并使用 chnot-thread-commit 提交

## Background

- 当前 mdwt 保存有两条路径：thread 模式（按 `## [[OTID]]` 标题拆分 block 逐个保存）和 fallback 模式（整篇内容作为单条 mdwt 保存）
- fallback 模式下，整篇文档只存一条 MdwtRecord，无法利用 thread 的排序、归档、子项管理等能力
- 希望统一保存行为：无论前端是否已经带有 block 标记，后端保存时都应解析 block，拆分为多条 mdwt，通过 chnot-thread-commit 组织

## Goals

- mdwt 保存时，自动解析 markdown 内容中的 heading block
- 每个 block 保存为独立的 MdwtRecord
- 使用 chnot-thread-commit 维护父子关系和排序
- 兼容已有不含 block 标记的内容（自动为其生成 OTID 并升级为 block 格式）

## Scope

- In scope:
  - 后端 `mdwt_commit` 接口改造：接收完整内容 → 解析 block → 逐个保存 MdwtRecord → 调用 chnot-thread-commit
  - 新增 OTID 自动分配逻辑（为无 `[[OTID]]` 标记的 heading 生成 OTID）
  - 前端保存路径简化：移除 block-based / fallback 双路径，统一走新的后端接口
- Out of scope:
  - 历史数据迁移（已有单条 mdwt 不做自动迁移）
  - Toent 联动逻辑变更
  - 搜索/索引逻辑变更

## Technical Plan

### Phase 1: 后端 block 解析与 OTID 自动分配

1. 在 `lib/backend/src/krate/mdwt/` 新增 block 解析函数
   - 复用 `MdwtParser` 中已有的 heading 级别 block 识别逻辑（`parser/mod.rs`）
   - 为不含 `[[OTID]]` 的 heading 自动生成 TID 并注入到标题行
   - 返回 `Vec<Block>`，每个 block 包含 `otid`、`content`、`order`

2. 改造 `KDbTx::mdwt_commit`（`db.rs:138-161`）
   - 当前：直接 `overwrite_mdwt_record` 保存单条
   - 改为：先解析 block，逐个调用 `overwrite_mdwt_record`，最后调用 `chnot_thread_order_commit`
   - 返回值增加 `blocks: Vec<{ otid, title }>` 供前端同步状态

### Phase 2: 后端 chnot-thread-commit 联动

3. 在 `mdwt_commit` 事务内，调用已有的 `chnot_thread_order_commit`（`chnot/db.rs:103-190`）
   - `thread_otid` = 当前 chnot 的 otid
   - `orders` = 所有 block 的 otid 列表（按文档顺序）
   - `remove_others = true`（清理已删除的 block）

4. 新增 block 的 ChnotMeta 自动创建
   - 需要为每个新 block 创建 `ChnotMeta(kind=MDWT, kspace=parent.kspace)` 记录
   - 复用 `chnot_meta_commit` 逻辑

### Phase 3: 前端简化

5. 简化 `MdwtChnot.directlySave`（`web/src/krate/chnot/component/rich-chnot/mdwt.tsx:151-249`）
   - 移除 block-based save 和 fallback save 双路径
   - 统一将完整编辑器内容提交到 `mdwtCommit`，由后端负责 block 拆分
   - 前端仍需处理返回的 block 列表来更新 `lastSavedContentRef`

6. 适配 `mdwtCommit` 返回值
   - 前端 DTO 增加 `blocks` 字段
   - `handleContentChange` 中不再需要 `splitDocumentByBlocks`

### Phase 4: 验证与清理

7. 验证场景
   - 新建笔记：首个 heading 自动生成 OTID → block 保存 → thread 有序
   - 多 heading 笔记：每个 heading 成独立 block
   - 无 heading 笔记：整篇作为单个 block（自动补 OTID）
   - 编辑已有 thread 笔记：block 增删改正常

8. 清理
   - 移除前端 `splitDocumentByBlocks` 在保存流程中的调用（保留用于编辑器高亮等 UI 用途）
   - 清理 `chnotThreadOrderCommit`、`chnotThreadOrderArchive` 在前端的直接调用

## Risks and Decisions

- Risk: 后端 block 解析与前端 CodeMirror 解析不一致 → Mitigation: 统一使用 comrak AST 解析，heading 规则以 comrak 为准
- Risk: OTID 自动生成导致前端编辑器内容与后端不一致 → Mitigation: 后端返回注入了 OTID 的新内容，前端刷新编辑器
- Decision: 在后端而非前端做 block 拆分，减少前端复杂度，保证数据一致性
- Decision: 无 heading 的内容整体作为一个 block，heading 级别不限（## 到 ###### 均可）

## Acceptance Criteria

- [ ] `POST /api/v1/mdwt-commit` 接收完整内容后，后端自动按 heading 拆分为多条 MdwtRecord
- [ ] 每个 block 自动创建对应的 ChnotMeta(kind=MDWT)
- [ ] 父 chnot 的 ChnotThreadOrder 正确维护所有子 block 的顺序
- [ ] 无 heading 的内容作为单个 block 保存，自动分配 OTID
- [ ] 前端移除 block-based / fallback 双路径，统一走单次提交
- [ ] 已有的 thread 笔记加载和保存正常
- [ ] Toent 联动（todo_event、time_events）仍正常工作

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2604-27/mdwt-block-thread-save`
- [ ] Create first commit: `task-start: docs/exec-plans/active/2604-27-mdwt-block-thread-save.md`
- [ ] Implement Phase 1: backend block parsing with auto-OTID
- [ ] Implement Phase 2: chnot-thread-commit integration
- [ ] Implement Phase 3: frontend simplification
- [ ] Implement Phase 4: verification and cleanup
- [ ] Move file to `docs/done/` after completion
- [ ] Create final commit: `task-done: docs/exec-plans/active/2604-27-mdwt-block-thread-save.md`
