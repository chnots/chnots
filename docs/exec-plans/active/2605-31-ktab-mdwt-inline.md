# KTab MDWT 内联表格渲染

## Background

- 当前 KTab 在 MDWT 中作为 thread item 嵌入，inline 只显示 preview 卡片，点击后在右侧 ThreadEditorPanel 打开编辑
- 用户希望 inline 直接展示完整可编辑表格，而非 preview 卡片
- 现有 KTabTable 组件和 ThreadEditorPanel 已完整可用，需要改造的是渲染集成层

## Goals

- KTab 在 MDWT 内以 inline 形式展示完整可编辑表格（替换 preview 卡片）
- 表格上方有 topbar（仅展开按钮），点击后右侧打开 ThreadEditorPanel，inline 折叠为 topbar 窄条
- 右侧面板关闭后 inline 恢复完整表格
- 任何时刻只有一个编辑实例（右侧面板打开时 inline 不渲染 KTabTable）

## Scope

- In scope:
  - 新建 `mdwt-thread-store.ts`（zustand store 管理 selectedItem 状态）
  - 新建 `KTabInlineWidget` 组件（topbar + 条件渲染 KTabTable）
  - 修改 `thread-widget-extension.tsx`（KTab case 渲染 KTabInlineWidget）
  - 修改 `thread-item-preview.tsx`（KTab case bypass 外层点击容器）
  - 修改 `mdwt.tsx`（KTab data fetching + 使用 store）
- Out of scope:
  - 可拖拽调整分栏宽度
  - 虚拟滚动
  - topbar 显示表格名称

## Design Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | `isSelected` 命名（非 `isExpanded`） | isExpanded 跟行为（折叠）语义相反 |
| 2 | selectedItem 通过 zustand store 传递 | React context 无法穿透 CodeMirror 的 createRoot |
| 3 | kindData 只存 KTabMeta | fetchData/onMetaChange 在组件内部构造 |
| 4 | ignoreEvent() 按事件来源判断 | 防止 CodeMirror 拦截 KTabTable 内的键盘事件 |
| 5 | eq() 只比较 otid + mdwtContent | 不比较 kindData，KTabInlineWidget 内部自行管理数据 |
| 6 | 折叠态点击 → ring 动画 | 面板已在视口中，不需要 scroll |
| 7 | KTab case bypass 外层点击容器 | 避免外层 onClick 拦截 KTabTable 交互 |
| 8 | isSelected=true 时不渲染 KTabTable | 单实例编辑，避免双视图数据不一致 |
| 9 | KTabInlineWidget 独立文件 | 比 Excalidraw/MindMap preview 复杂得多 |
| 10 | max-h-[300px] 自适应 | 小表格不浪费空间 |
| 11 | 新建 mdwt-thread-store.ts | selectedItem 是 MDWT thread 系统状态，职责清晰 |
| 12 | 关闭面板后 refreshThreadWidgets | KTabTable 全新 mount，自动 fetch 最新数据 |
| 13 | KTabMeta fetch 失败 → fallback mdwtContent | 与 Excalidraw/MindMap 行为一致 |
| 14 | 文件放 mdwt/component/ | 属于 MDWT widget 渲染层，不属于 chnot |
| 15 | store 放 mdwt-thread-store.ts | 职责清晰，提供 selectedOtid selector |
| 16 | props: { otid, kindData, onItemClick } | isSelected 从 store 内部获取 |
| 17 | eq() 不比较 kindData | KTabInlineWidget 内部自行 fetch meta，不依赖 prop 更新 |

## Technical Plan

### Step 1: 新建 mdwt-thread-store.ts

文件：`web/src/krate/mdwt/component/mdwt-thread-store.ts`

```typescript
import { create } from "zustand";
import type { ChnotKind } from "@/krate/chnot/po";
import type { TID } from "@/lib/id_util";

type SelectedItem = { otid: TID; kind: ChnotKind };

interface MdwtThreadState {
  selectedItem: SelectedItem | undefined;
  setSelectedItem: (item: SelectedItem | undefined) => void;
}

export const useMdwtThreadStore = create<MdwtThreadState>((set) => ({
  selectedItem: undefined,
  setSelectedItem: (item) => set({ selectedItem: item }),
}));
```

### Step 2: 修改 mdwt.tsx 使用 store

文件：`web/src/krate/chnot/component/rich-chnot/mdwt.tsx`

修改点：
- 删除 `useState` 的 `selectedItem`，改用 `useMdwtThreadStore`
- `handleThreadItemClick` 调用 store 的 `setSelectedItem`
- `ThreadEditorPanel` 的 `onClose` 调用 store 的 `setSelectedItem(undefined)`
- `loadThreadData` 中新增 KTab kindData fetch（`ktabMetaFetch`）
- `handleRefExisting` 中新增 KTab kindData fetch

KTab data fetch：
```typescript
if (threadMeta.meta.kind === ChnotKind.KTab) {
  try {
    const metaRsp = await ktabMetaFetch({ table_id: childOtid });
    if (metaRsp.meta) item.kindData = metaRsp.meta;
  } catch {}
}
```

### Step 3: 新建 KTabInlineWidget 组件

文件：`web/src/krate/mdwt/component/ktab-inline-widget.tsx`

```typescript
interface KTabInlineWidgetProps {
  otid: TID;
  kindData: KTabMeta;
  onItemClick: (otid: TID, kind: ChnotKind) => void;
}
```

布局结构：
- 从 store 获取 `selectedItem`，比较 `selectedItem?.otid === otid` 得到 `isSelected`
- **isSelected=true**（面板已打开）：只渲染 topbar 窄条（32px），含表格图标 + 展开按钮（高亮态），点击 → ring 动画（通过事件或 ref 通知面板）
- **isSelected=false**（面板关闭）：渲染 topbar（32px）+ KTabTable（max-h-[300px]），点击 topbar 展开按钮调用 `onItemClick`

topbar 样式：
- 高度 32px，flex row，左侧表格图标（TableIcon），右侧展开按钮（Maximize2）
- 背景色 `bg-muted/50 border`

KTabTable 集成：
- 内部构造 `fetchData`（直接调用 `ktabCellList` service）
- 内部构造 `onMetaChange`（直接调用 `ktabMetaCommit` service）
- 容器 `max-h-[300px] overflow-hidden`，KTabTable 内部自行滚动
- `readonly=false`，`compact=true`

### Step 4: 修改 thread-widget-extension.tsx

文件：`web/src/krate/mdwt/component/thread-widget-extension.tsx`

修改点：
- `ThreadItemWidget.eq()` 改为只比较 `otid` + `mdwtContent`，不比较 `kindData`
- `ThreadItemWidget.ignoreEvent()` 根据事件 target 判断：如果 target 在 KTabTable 内部返回 `true`

### Step 5: 修改 thread-item-preview.tsx

文件：`web/src/krate/mdwt/component/thread-item-preview.tsx`

修改点：
- import `KTabInlineWidget`
- `renderKindPreview()` 中新增 KTab case：
  ```typescript
  case "ktabv1":
    if (!item.kindData) return null;
    return (
      <KTabInlineWidget
        otid={item.otid}
        kindData={item.kindData as KTabMeta}
        onItemClick={onClick}
      />
    );
  ```
- KTab case 的 return **不包裹**在外层可点击 div 中（在 switch 外层判断，KTab 直接 return 整个组件）

### Step 6: 验证 ThreadEditorPanel 和面板关闭流程

- 现有关闭逻辑已正确（`setSelectedItem(undefined)` + `refreshThreadWidgets`）
- 验证关闭面板后 inline KTabTable 全新 mount 并 fetch 最新数据
- 验证 ring 动画在折叠态点击时触发

## Risks

- Risk: CodeMirror block widget 内渲染 React 组件（createRoot）在状态更新时可能导致性能问题 → Mitigation: KTab 数据获取在 widget 内部管理，selectedItem 通过 store 传递不触发 decoration 重建
- Risk: inline KTabTable 横向滚动可能不友好 → Mitigation: 表格 `overflow-x-auto`
- Risk: CodeMirror widget 的 React root 生命周期管理 → Mitigation: 复用现有 createRoot 模式，destroy() 中 unmount

## Acceptance Criteria

- [ ] KTab 在 MDWT inline 展示完整可编辑表格（非 preview 卡片）
- [ ] 表格上方有 topbar 窄条（32px），含展开按钮
- [ ] inline 表格 max-h-[300px]，小表格自适应高度
- [ ] 点击 topbar 展开按钮 → 右侧 ThreadEditorPanel 打开 → inline 折叠为仅 topbar
- [ ] 右侧面板关闭 → inline 恢复完整表格（fetch 最新数据）
- [ ] 折叠态点击窄条 → ring 动画提示
- [ ] inline 表格可编辑（单元格编辑、新增列等）
- [ ] 其他 thread item 类型渲染不受影响
- [ ] selectedItem 状态通过 zustand store 传递，不触发 CodeMirror decoration 重建
- [ ] `pnpm run lint && pnpm run build` 通过

## Execution Checklist

- [x] Confirm the plan is executable with user
- [x] Create branch `feat/2605-31/ktab-mdwt-inline`
- [x] Create first commit: `task-start: docs/exec-plans/active/2605-31-ktab-mdwt-inline.md ktab mdwt inline table rendering`
- [ ] Implement in small commits
- [ ] Move file to `docs/exec-plans/done/` after completion
- [ ] Create final commit: `task-done: docs/exec-plans/done/2605-31-ktab-mdwt-inline.md ktab mdwt inline table rendering`
