# KTab MDWT 内联表格渲染

## Background

- 当前 KTab 在 MDWT 中作为 thread item 嵌入，inline 只显示 preview 卡片，点击后在右侧 ThreadEditorPanel 打开编辑
- 用户希望 inline 直接展示完整可编辑表格，而非 preview 卡片
- 现有 KTabTable 组件和 ThreadEditorPanel 已完整可用，需要改造的是渲染集成层

## Goals

- KTab 在 MDWT 内以 inline 形式展示完整可编辑表格（替换 preview 卡片）
- 表格上方有 topbar（仅展开按钮），点击后右侧打开 ThreadEditorPanel，inline 折叠为 topbar 窄条
- 右侧面板关闭后 inline 恢复完整表格
- 两个视图共享同一个 selectedItem 状态流，无数据同步问题

## Scope

- In scope:
  - `thread-widget-extension.tsx` 中替换 KTab 的 preview 渲染为 inline KTabTable
  - 新建 `KTabInlineWidget` 组件（topbar + KTabTable），管理展开/折叠状态
  - 折叠态仅显示 topbar 窄条（32px），展开态显示 topbar + 固定高度表格（300px）
  - 展开按钮通过 `onItemClick` 设置 selectedItem，触发右侧面板
  - 折叠态点击窄条聚焦右侧面板
- Out of scope:
  - 可拖拽调整分栏宽度
  - 双视图同时编辑（折叠态避免此问题）
  - 虚拟滚动
  - topbar 显示表格名称

## Technical Plan

### Step 1: 新建 KTabInlineWidget 组件

文件：`web/src/krate/chnot/component/rich-chnot/ktab-inline-widget.tsx`

```typescript
interface KTabInlineWidgetProps {
  otid: TID;
  isExpanded: boolean;        // selectedItem 是否指向此 otid
  onExpand: () => void;       // 设置 selectedItem = 此 otid
  onFocusPanel: () => void;   // 聚焦右侧面板
}
```

布局结构：
- 折叠态（`isExpanded=true`）：32px 窄条，含表格图标 + 展开按钮（高亮态），点击调用 `onFocusPanel`
- 展开态（`isExpanded=false`）：topbar（32px）+ KTabTable（300px 固定高度），点击 topbar 展开按钮调用 `onExpand`

topbar 样式：
- 高度 32px，flex row，左侧表格图标，右侧展开按钮（ArrowRight 或 Maximize2）
- 背景色区分于 MDWT 编辑区域（`bg-muted/50` 或 `border`）

KTabTable 集成：
- 复用现有 `table.tsx` 中的数据获取逻辑（`ktabMetaFetch` + `ktabCellList`）
- 或直接内联渲染 `KTabTable`，传入 `compact` prop
- 表格容器 `h-[300px] overflow-hidden`，KTabTable 内部自行滚动
- `readonly=false`，inline 可编辑

### Step 2: 修改 thread-widget-extension.tsx

文件：`web/src/krate/mdwt/component/thread-widget-extension.tsx`

修改点：
- `ThreadWidgetItem` 的 `kind` 已包含 `ChnotKind.KTab`，数据流不变
- 在 `ThreadItemWidget.toDOM()` 中，对 `ChnotKind.KTab` 类型渲染 `KTabInlineWidget` 而非 preview
- 需要传入 `isExpanded`（比较 `selectedItem?.otid === item.otid`）和 `onExpand`（调用 `onItemClick`）
- 当前 `onItemClick(otid, kind)` 已能设置 selectedItem 触发右侧面板，可直接复用

关键改动：
- `ThreadWidgetData` 接口可能需要新增 `selectedOtid?: TID` 字段，用于判断 inline 是否折叠
- 或者在 widget 渲染时从外部 context 获取 selectedItem 状态

### Step 3: 修改 mdwt.tsx 传递 selectedItem 状态

文件：`web/src/krate/chnot/component/rich-chnot/mdwt.tsx`

修改点：
- 当前 `selectedItem` 状态已在 mdwt.tsx 中管理（`useState`）
- 需要将 `selectedItem` 传递给 `ThreadWidgetData`，使 widget 知道当前展开的是哪个 KTab
- `onItemClick` 已设置 selectedItem，无需修改

传递方式：
- `threadWidgetExtension` 的 `ThreadWidgetData` 新增 `selectedOtid?: TID`
- 在 mdwt.tsx 中传入 `selectedOtid: selectedItem?.otid`

### Step 4: 修改 ThreadEditorPanel 关闭逻辑

文件：`web/src/krate/mdwt/component/thread-editor-sheet.tsx`

- 现有关闭逻辑（`onClose` → `setSelectedItem(undefined)`）已正确，关闭后 inline 自动恢复
- 无需额外修改，验证行为即可

### Step 5: 聚焦右侧面板

- inline 折叠态点击窄条时，需要滚动或聚焦右侧面板
- 实现方式：通过 ref 或 DOM query 获取右侧面板元素，调用 `scrollIntoView` 或 `focus`
- 简单实现：折叠态点击直接再次调用 `onExpand`（设置 selectedItem），如果右侧已打开则聚焦

## Risks and Decisions

- Risk: CodeMirror block widget 内渲染 React 组件（createRoot）在状态更新时可能导致性能问题 → Mitigation: KTab 数据获取在 widget 内部管理，避免频繁 re-creation
- Risk: inline KTabTable 高度固定 300px，列数多时横向滚动可能不友好 → Mitigation: 表格 `overflow-x-auto`，横向可滚动
- Risk: CodeMirror widget 的 React root 生命周期管理（mount/unmount）→ Mitigation: 复用现有的 `createRoot` 模式，在 `destroy()` 中 unmount
- Decision: 折叠态只有一个编辑实例（右侧面板），避免数据同步问题
- Decision: 不使用表格名称，topbar 仅展示图标 + 展开按钮
- Decision: 固定 300px 高度，不在 CodeMirror 中做自适应高度

## Acceptance Criteria

- [ ] KTab 在 MDWT inline 展示完整可编辑表格（非 preview 卡片）
- [ ] 表格上方有 topbar 窄条（32px），含展开按钮
- [ ] inline 表格固定 300px 高度，内部可滚动
- [ ] 点击 topbar 展开按钮 → 右侧 ThreadEditorPanel 打开 → inline 折叠为 topbar 窄条
- [ ] 右侧面板关闭 → inline 恢复完整表格
- [ ] inline 折叠态点击窄条 → 聚焦右侧面板
- [ ] inline 表格可编辑（单元格单击编辑、新增列等）
- [ ] 其他 thread item 类型（exdrv1、mindmapv1 等）渲染不受影响
- [ ] `pnpm run lint && pnpm run build` 通过

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2605-31/ktab-mdwt-inline`
- [ ] Create first commit: `task-start: docs/exec-plans/active/2605-31-ktab-mdwt-inline.md ktab mdwt inline table rendering`
- [ ] Implement in small commits
- [ ] Move file to `docs/exec-plans/done/` after completion
- [ ] Create final commit: `task-done: docs/exec-plans/done/2605-31-ktab-mdwt-inline.md ktab mdwt inline table rendering`
