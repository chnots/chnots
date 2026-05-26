# Thread Ctrl+Enter 快速添加 TODO Block

## Background

- chnot-thread 支持线性排列多个 chnot block，但添加新 block 需要鼠标点击底部 `+` 按钮，打断输入流
- 用户需要一个快捷方式：在 mdwt block 中按 `Ctrl+Enter` 快速创建下一个 TODO 项

## Goals

- 在 thread 的 mdwt block 中按 `Ctrl+Enter`：
  - 若当前 mdwt 为空 → 在编辑器中插入 `## [TODO]`
  - 若当前 mdwt 有内容 → 在当前 block 后追加新的 mdwt block，内容为 `## [TODO]`，并自动 focus 到新 block

## Scope

- In scope: thread 中 mdwt block 的 Ctrl+Enter 行为修改
- Out of scope: 非 thread 场景（独立 mdwt、fullscreen 等）保持原有 `insertLineAfter` 行为

## Technical Plan

### 调用链路（6 个文件，自底向上）

#### 1. `web/src/krate/mdwt/component/codemirror/keybinding.ts`

- `generateKeybinding` 增加可选参数 `onCtrlEnter?: (view: EditorView) => boolean`
- `Mod-Enter` handler：若 `onCtrlEnter` 存在且返回 `true`，跳过默认 `insertLineAfter`；否则保持原行为

#### 2. `web/src/krate/mdwt/component/mdwt-editor.tsx`

- `MdwtEditor` 增加 `onCtrlEnter` prop，透传给 `generateKeybinding`

#### 3. `web/src/krate/chnot/component/rich-chnot/mdwt.tsx`

- `MdwtChnot` 增加 `onCtrlEnter` prop，透传给 `MdwtEditorMemo`

#### 4. `web/src/krate/chnot/component/rich-chnot/rich-mdwt.tsx`

- `RichMdwt` 增加 `onCtrlEnter` prop，透传给内部 `MdwtChnot`

#### 5. `web/src/krate/chnot/component/rich-chnot/thread/block.tsx`

- 增加 `onAppendBlock` 回调 prop
- 构造 `onCtrlEnter` handler：
  - 空 content → `view.dispatch` 插入 `## [TODO]`，返回 `true`
  - 非空 content → 调用 `onAppendBlock()`，返回 `true`
- 透传 `onCtrlEnter` 到 `RichChnotMemo` → `RichMdwt`

#### 6. `web/src/krate/chnot/component/rich-chnot/thread/index.tsx`

- 新增 `handleAppendBlock(afterIndex: number)` 方法
- 新增 `focusOtid` state，新 block 创建后设置，触发自动 focus
- 将 `onAppendBlock` 和 `focusOtid` 传给 `SortableRichMdwtMemo`

## Risks and Decisions

- Risk: 非 thread 场景误触发 → Mitigation: `onCtrlEnter` 仅在 thread block 中传入，其他场景为 `undefined`，走默认行为
- Decision: 新 block 默认为 `ChnotKind.MDWT`，因为 TODO 快速记录场景以 markdown 为主

## Acceptance Criteria

- [ ] thread 中空的 mdwt block 按 Ctrl+Enter 插入 `## [TODO]`
- [ ] thread 中非空 mdwt block 按 Ctrl+Enter 追加新 mdwt block（内容 `## [TODO]`）并自动 focus
- [ ] 非 thread 场景 Ctrl+Enter 行为不变（insertLineAfter）

## Execution Checklist

- [ ] 确认计划可执行
- [ ] 创建分支 `feat/2603-28/thread-ctrl-enter-todo`
- [ ] 创建 task-init commit
- [ ] 按顺序实现 6 个文件的改动
- [ ] `npm run lint` 和 `npm run build` 通过
- [ ] 移动到 `docs/done/` 并创建 task-done commit
