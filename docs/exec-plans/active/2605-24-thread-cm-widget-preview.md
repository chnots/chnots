# Thread CM Widget Preview + Sheet Editor

## Context

mdwt 页面中，当 chnot 有 `chnot_thread_order` 子项时，自动在 CodeMirror 编辑器内容末尾渲染子项预览。每个子项按 `korder` 排序，以 `heading_level` 控制的 heading 层级显示，包含 mdwt 文本内容 + kind-specific 预览。点击子项打开右侧 50% 宽 Sheet 进行编辑，支持全屏。

Thread kind 是过时设计，未来任何 mdwt chnot 都可以有 thread 子项，不限于 ThreadV1。

## Design Decisions

| # | 决策 | 选择 | 理由 |
|---|------|------|------|
| Q1 | 触发条件 | 任何 mdwt chnot 自动查 `chnotThreadMetaFetch` | 不限 ThreadV1 |
| Q3 | 渲染方式 | CM Block Widget（方案 A） | 视觉统一，内容流一体 |
| Q3a | Widget 位置 | 追加在文档末尾 | C2 生成式 heading，按 korder 排序 |
| Q4 | React 挂载 | `createRoot` 挂载完整 React 组件 | 复用现有组件 |
| Q6 | 数据加载 | React 层加载，通过 compartment 传给 CM | D2，避免 CM 内管理 React 生命周期 |
| Q7 | 点击通信 | 回调函数 `onItemClick(otid, kind)` | E2，简单直接 |
| Q8 | 保存刷新 | Sheet 保存后立即刷新 widget 预览 | F1，用户体验一致 |
| Q9 | Widget 结构 | 单个 React root per thread item | G1，状态管理简单 |
| Q10 | Heading 内容 | `# [[otid]] mdwt_content` | 从 mdwtRecordList 批量加载取第一行 |
| Q11 | CM 插入方式 | `Decoration.widget({ block: true })` 追加文档末尾 | I1，标准 CM 块级装饰 |
| Q12 | heading_level | 控制渲染的 heading 层级 | J1，1→`#`，2→`##`... |
| Q13 | Extension 位置 | `web/src/krate/mdwt/component/` | K2，避免 lib 包依赖业务 |
| Q14 | Sheet 编辑器 | 所有 kind 完整编辑器 + 全屏按钮 | L1，保持一致性 |

## Architecture

### New Files

**1. `web/src/krate/mdwt/component/thread-widget-extension.ts`**

CM extension，职责：
- 接收 thread 数据（通过 CM Compartment/StateField）
- 为每个 thread 子项创建 `Decoration.widget({ block: true })`
- Widget class `ThreadItemWidget extends WidgetType`：
  - `toDOM()` 创建容器 div
  - `createRoot` 挂载 React 组件
  - `ignoreEvent()` 返回 `false`（允许点击事件穿透）
  - `destroy()` 调用 `root.unmount()`
- 所有 decoration 定位在文档末尾 line，按 korder 依次排列

**2. `web/src/krate/mdwt/component/thread-item-preview.tsx`**

Widget 内的 React 组件，每个 thread 子项渲染：
- Heading：`<h{level}>[[otid]] title_line</h{level}>`
- mdwt 文本：`ReactMarkdown` 渲染 mdwt content（去掉第一行 title 后的部分）
- Kind-specific 预览（在 mdwt 文本下方）：
  - `mdwt` / `llmchat` / `threadv1`：无额外内容
  - `excalidraw`：`ExcalidrawPreview`
  - `ktab`：`DataTable`（readonly）
  - `kfile`：`ImageKFile` 或 `CommonKFile`
  - `mindmap`：`MindElixirPreview`
- 整个容器可点击 → 调用 `onItemClick(otid, kind)`

Props: `{ item: ChnotThreadMetaFetchRspData, mdwtContent: string, kindData?: any, onClick }`

**3. `web/src/krate/mdwt/component/thread-editor-sheet.tsx`**

编辑 Sheet：
- `Sheet` + `SheetContent`，className 覆盖 `sm:max-w-[50vw]`
- 根据 kind 渲染编辑器：
  - `mdwt` / `llmchat`：`MdwtChnot`
  - `excalidraw`：`ExcalidrawChnot`（带 fullscreen）
  - `ktab`：`TableChnot`
  - `kfile`：`KFileChnot`
  - `mindmap`：`MindMapChnot`（带 fullscreen）
- `onPostSave` 实现：`chnotMetaCommit` + 回调通知父组件刷新
- 全屏按钮复用 `Fullscreen` 组件模式

### Modified Files

**4. `web/src/krate/chnot/component/rich-chnot/mdwt.tsx`（MdwtChnot）**

- 新增 thread 数据加载：
  - mount 时 `chnotThreadMetaFetch({ otid })`
  - 有子项 → `mdwtRecordList({ mdwt_otids: childOtid[] })` 批量加载
  - 非 mdwt kind 子项并行加载 kind-specific 数据
- 新增 `threadData` compartment，通过 `view.dispatch` 注入 CM
- 新增 `selectedItem` state 控制 Sheet
- 渲染 `ThreadEditorSheet`

**5. `web/src/krate/mdwt/component/mdwt-editor.tsx`（MdwtEditor）**

- extensions 数组加入 `threadWidgetExtension`（通过 compartment）
- 新增可选 prop 传入 thread extension 或 compartment

## Data Flow

```
MdwtChnot mount
  ├─ mdwtContentLoad({ otid }) → 加载自身 mdwt
  └─ chnotThreadMetaFetch({ otid }) → 检查 thread 子项
       ├─ 无子项 → 正常渲染
       └─ 有子项：
            ├─ mdwtRecordList({ mdwt_otids }) → 批量 mdwt content
            ├─ 并行加载 kind-specific 数据（per item）
            └─ dispatch to CM compartment → widgets 创建

点击 widget
  → onItemClick(otid, kind) → setSelectedItem
  → ThreadEditorSheet 打开
  → 编辑保存
  → onPostSave → 重载子项数据 → dispatch CM → widget 刷新
```

## Reusable Components

| Component | File |
|-----------|------|
| `ExcalidrawPreview` | `web/src/krate/graph/excalidraw/component/excalidraw-preview.tsx` |
| `MindElixirPreview` | `web/src/krate/graph/mind-elixir/preview.tsx` |
| `ImageKFile` / `CommonKFile` | `web/src/krate/kfile/components/` |
| `DataTable` | `web/src/krate/ktab/component/data-table.tsx` |
| `MarkdownViewer` | 从 `rich-chnot/mdwt.tsx` 提取 |
| `ExcalidrawChnot` | `web/src/krate/chnot/component/rich-chnot/excalidraw.tsx` |
| `MindMapChnot` | `web/src/krate/chnot/component/rich-chnot/mindmap.tsx` |
| `TableChnot` | `web/src/krate/chnot/component/rich-chnot/table.tsx` |
| `KFileChnot` | `web/src/krate/chnot/component/rich-chnot/kfile.tsx` |
| `Sheet` / `SheetContent` | `web/src/common/component/ui/sheet.tsx` |
| `Fullscreen` | `web/src/krate/chnot/component/rich-chnot/fullscreen.tsx` |

## API

| API | Source |
|-----|--------|
| `chnotThreadMetaFetch` | `web/src/krate/chnot/service.ts` |
| `mdwtRecordList` | `web/src/krate/mdwt/service.ts` |
| `mdwtContentLoad` | `web/src/krate/mdwt/service.ts` |
| `mdwtCommit` | `web/src/krate/mdwt/service.ts` |
| `fetchExcalidraw` | `web/src/krate/graph/excalidraw/service.ts` |
| `fetchMindExilir` | `web/src/krate/graph/mind-elixir/service.ts` |
| `ktabMetaFetch` + `ktabCellList` | `web/src/krate/ktab/service.ts` |
| `kfileMetaFetch` | `web/src/krate/kfile/service.ts` |
| `chnotMetaCommit` | `web/src/krate/chnot/service.ts` |

## Steps

1. 提取 `MarkdownViewer` 为共享组件
2. 创建 `thread-widget-extension.ts`（CM extension + WidgetType）
3. 创建 `thread-item-preview.tsx`（widget 内 React 组件）
4. 创建 `thread-editor-sheet.tsx`（Sheet + per-kind 编辑器）
5. 修改 `MdwtChnot`（mdwt.tsx）加入 thread 数据加载 + Sheet 状态
6. 修改 `MdwtEditor`（mdwt-editor.tsx）接入 thread extension
7. 端到端测试

## Verification

- 打开有 thread 子项的 mdwt chnot → 子项以 heading block widget 渲染在文档末尾
- heading_level 正确控制 `#`/`##`/`###` 层级
- mdwt 文本 + kind 预览正确显示（excalidraw SVG、ktab 表格、kfile 图片/卡片、mindmap 图片）
- 点击子项 → Sheet 右侧滑出 50% 宽
- Sheet 内编辑器可正常使用
- 全屏按钮可用
- 保存后 widget 预览立即刷新
