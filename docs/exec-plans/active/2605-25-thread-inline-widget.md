# Thread Widget 内联渲染

## Context

当前 `thread-widget-extension.tsx` 的 `buildThreadDecorations` 将所有子 chnot 预览 widget 堆叠在文档末尾 (`endPos + i`)。正确行为：每个 `[[otid]]` heading 行下方内联渲染预览。

基于 `2605-24-thread-cm-widget-preview.md` 已有实现的改动。

## 期望效果

```
## [[otid123]] heading name     ← 编辑器已有 heading 行
child mdwt content here         ← 子 chnot 的 MDWT 内容
..........                      ← 子 chnot 剩余文本
[MindMap Preview Image]         ← 内联插入的预览 widget，可点击

### [[otid456]] another child   ← 下一个 heading
another content
[Excalidraw Preview]            ← 内联预览
```

## 修改文件

### 1. `web/src/krate/mdwt/component/thread-widget-extension.tsx`

改 `buildThreadDecorations`：

- 不再 `endPos + i`
- 遍历 `data.items`，对每个 item 的 `otid` 在 `state.doc` 中搜索 `[[otid]]`
- 找到 heading 行后，向下扫描到 section 结束（下一个 heading / 空行 / 文档末尾）
- 在 section 结束位置放 `Decoration.widget`（`side: 1`, `block: true`）
- 找不到 `[[otid]]` 的 item 跳过

### 2. `web/src/krate/mdwt/component/thread-item-preview.tsx`

移除 heading 渲染（`createElement(Tag, ...)`），heading 行已在编辑器中可见。只保留：
- 子 chnot 的 MDWT 内容（`MarkdownViewer`）
- Kind-specific preview（mindmap / excalidraw 等）

### 3. 不需要改的文件

- `mdwt.tsx`（`MdwtChnot`）— 数据加载逻辑不变
- `thread-editor-sheet.tsx` — 编辑 Sheet 不变
- `mdwt-editor.tsx` — 编辑器不变

## 验证

- heading completion 创建子 chnot 后，预览出现在对应 heading 下方
- 点击预览弹出 ThreadEditorPanel
- 多个子 chnot 各自独立定位，不堆叠
- 找不到 `[[otid]]` 的 item 不报错、不渲染
