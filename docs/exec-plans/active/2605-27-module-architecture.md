# 前后端模块化架构整理

## Background

- 后端 10 个 krate 域模块已按 controller/db/dto/mapper/po 组织，但跨模块引用分散（如 `chnot` 直接 `use crate::krate::mdwt::dto::MdwtTagSearchType`）
- 前端 11 个 krate 域模块缺少 barrel exports（index.ts），消费方直接引用内部文件路径
- 架构文档缺少模块依赖关系图和 API 边界说明

## Goals

- 后端：每个 krate 域模块建立清晰的公开 API 边界
- 前端：每个 krate 域模块添加 index.ts barrel export
- 文档：更新 ARCHITECTURE.md 添加模块依赖图

## Scope

- In scope: 后端 mod.rs barrel exports、前端 index.ts barrel exports、ARCHITECTURE.md 更新
- Out of scope: 拆分为独立 crate/package、重构模块间依赖关系、改变现有功能

## Technical Plan

### Phase 1: 后端 — krate 域模块公开 API 整理

对每个 krate 模块（chnot, mdwt, kfile, ktab, kspace, kkv, graph, llmchat, sync, toent）：

1. 审计所有 `pub` 项，确认哪些是内部使用的（改为 `pub(crate)`）哪些是跨模块 API（保持 `pub` 或提升到 mod.rs re-export）
2. 在 `mod.rs` 中添加 `// Public API` 注释段，将跨模块需要的类型通过 re-export 暴露
3. 补充模块级 `//!` 文档注释说明模块职责

文件涉及（每个模块）：
- `lib/backend/src/krate/<domain>/mod.rs`
- `lib/backend/src/krate/<domain>/po.rs`
- `lib/backend/src/krate/<domain>/dto.rs`

### Phase 2: 前端 — krate 域模块 barrel exports

为每个 krate 模块创建 `index.ts`：

1. `web/src/krate/chnot/index.ts` — re-export dto types, po types, service functions, store hooks
2. `web/src/krate/mdwt/index.ts`
3. `web/src/krate/kfile/index.ts`
4. `web/src/krate/ktab/index.ts`
5. `web/src/krate/kspace/index.ts`
6. `web/src/krate/kkv/index.ts`
7. `web/src/krate/graph/index.ts`
8. `web/src/krate/llmchat/index.ts`
9. `web/src/krate/toent/index.ts`
10. `web/src/krate/sync/index.ts`

每个 index.ts 只 re-export 跨模块消费的类型和函数，不 re-export 内部实现。

### Phase 3: 文档 — 更新 ARCHITECTURE.md

1. 添加模块依赖关系图（ASCII art 或 Mermaid）
2. 添加每个模块的公开 API 边界说明
3. 添加前端模块对应说明

## Risks and Decisions

- Risk: 后端 re-export 可能导致 ambiguous import → Mitigation: 保持 re-export 路径明确，逐步验证 cargo check
- Risk: 前端 barrel export 增加打包体积（tree-shaking 可能失效）→ Mitigation: Rsbuild/Rspack 支持 tree-shaking barrel exports
- Decision: 不拆独立 crate/package — 当前规模下单 crate 更简单，通过 API 边界文档约束即可

## Acceptance Criteria

- [ ] 后端每个 krate 模块有清晰的 mod.rs 公开 API 段
- [ ] 后端 cargo check 无 warning 无 error
- [ ] 前端每个 krate 模块有 index.ts barrel export
- [ ] 前端 pnpm build 成功，TypeScript 编译无错误
- [ ] ARCHITECTURE.md 包含模块依赖关系图

## Execution Checklist

- [ ] Create branch `feat/2605-27/module-architecture`
- [ ] Create first commit: `task-start: docs/exec-plans/active/2605-27-module-architecture.md`
- [ ] Phase 1: 后端模块 API 整理（按模块逐个提交）
- [ ] Phase 2: 前端 barrel exports（按模块逐个提交）
- [ ] Phase 3: 文档更新
- [ ] Move file to `docs/exec-plans/done/` after completion
- [ ] Create final commit: `task-done: docs/exec-plans/done/2605-27-module-architecture.md`
