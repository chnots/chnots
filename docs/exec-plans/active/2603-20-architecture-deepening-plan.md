# 代码架构深化计划

## Background

- 当前 `Chnot`、`Mdwt`、`Toent`、`Kspace` 几个核心域在 DTO、保存流程和上下文传递上存在明显交叉依赖。
- 一些高频业务动作依赖隐式上下文或分散的编排逻辑，导致理解成本高、边界测试困难、后续改动风险偏大。
- 这次计划优先收敛最影响稳定性与可测试性的架构点，同时把其余候选项纳入后续阶段。

## Goals

- 降低笔记保存链路在前后端之间的跨模块跳转和隐式副作用。
- 将 `kspace` 从 transport/header 隐式注入改为更显式的上下文边界。
- 为后续 DTO 解耦、页面拆分、状态拆分建立可执行的分阶段路线。

## Scope

- In scope: 盘点并重构 `mdwt`/`toent` 保存编排边界，梳理 `kspace` 请求上下文传递，明确共享 contract 的归属，补充关键边界测试。
- In scope: 输出分阶段模块拆分方案，标注优先级、影响面和验证方式。
- Out of scope: 一次性重写全部 domain；视觉 UI 调整；与本次架构调整无关的功能新增。

## Technical Plan

1. 梳理笔记保存链路，覆盖前端触发点、后端 controller、db、cache、parser 的调用关系，识别必须收口到统一 facade/application service 的步骤。
2. 设计并落地保存流程 facade，统一处理 markdown 持久化、标题提取、todo/time 解析、toent 更新、cache 刷新等副作用，把 controller 降为薄入口。
3. 梳理 `web/src/lib/axios-fetch.ts`、`web/src/lib/tauri-fetch.ts` 与后端 `KReq` 的 `kspace` 注入路径，定义显式 request/session context 的传递方式。
4. 调整前后端 service/handler 签名，逐步减少对全局 store、header 和 transport 魔法的依赖，并为核心调用路径提供兼容层。
5. 归类跨 domain DTO，优先抽离搜索过滤、摘要结果、todo/time 相关共享 contract，避免 `chnot`、`mdwt`、`toent` 相互反向引用。
6. 为保存 facade 和显式上下文边界补充测试，优先验证成功路径、失败传播、缓存更新与空间隔离行为。
7. 在上述主线稳定后，再拆分 `web/src/krate/toent/component/toent-page.tsx` 与 `web/src/krate/chnot/store.ts`，把复杂 UI 编排迁移到更深的 hook/store 模块。

## Risks and Decisions

- Risk: 保存流程横跨多个 domain，改动时容易打破现有副作用顺序 -> Mitigation: 先补调用链清单和边界测试，再逐步收口到 facade。
- Risk: `kspace` 依赖当前 transport 约定，直接切换可能影响大量现有接口 -> Mitigation: 先引入显式 context 模型和兼容层，再分批替换调用方。
- Risk: DTO 抽离可能影响 `sync-struct` 和前后端类型同步 -> Mitigation: 优先抽离稳定 contract，并在同步后检查 TS/Rust 两侧引用闭环。
- Decision: 第一阶段优先做“保存流程 facade + 显式 `kspace` context”，因为这两项同时覆盖稳定性、可测试性和后续拆分的公共基础。
- Decision: `toent-page` 和 `chnot store` 暂列第二阶段，避免首轮同时改业务流程和大型前端状态结构。

## Acceptance Criteria

- [ ] 有一份清晰的保存流程边界定义，说明哪些步骤归 facade/application service 负责。
- [ ] `kspace` 关键调用路径不再仅依赖隐式 header 注入，存在可复用的显式上下文传递方案。
- [ ] 至少一类跨 domain DTO 被重新归类到更清晰的 shared contract 边界。
- [ ] 保存链路的核心边界具备可运行测试，能覆盖成功和失败传播场景。
- [ ] 第二阶段的 `toent-page` 与 `chnot store` 拆分方向有明确输入输出边界，而不是继续直接堆逻辑到页面和 store 中。

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2603-20/architecture-deepening-plan`
- [ ] Create first commit: `task-start: docs/2603-20-architecture-deepening-plan.md architecture deepening`
- [ ] Implement in small commits
- [ ] Move file to `docs/done/` after completion
- [ ] Create final commit: `task-done: docs/2603-20-architecture-deepening-plan.md architecture deepening`
