# Refactor: 合并 md-codemirror 回 web 并清理 mdwt

## Context

`lib/md-codemirror` 是一个只有 `web/mdwt` 一个消费者的"共享库"，属于 false abstraction。同时 `web/mdwt/component/mdwt-editor.tsx`（402行）过于臃肿，混合了 React 渲染、CM 扩展组合、补全逻辑、粘贴处理。

**目标**：
1. 将 `lib/md-codemirror/src/` 合并到 `web/src/lib/md-codemirror/`，消除包边界
2. 拆分 mdwt-editor.tsx，将各关注点提取为独立模块

## Phase 1: 合并 md-codemirror 到 web

### 1.1 移动源码

```
lib/md-codemirror/src/  →  web/src/lib/md-codemirror/
```

保留原有目录结构（editor/、heading-block/ 等不变）。

### 1.2 更新 web 内的 import（共 2 个文件）

**`web/src/krate/mdwt/component/mdwt-editor.tsx`**
```diff
- import type { TableEditDetail } from "@chnots/md-codemirror";
- import { createCodemirrorTheme, generateKeybinding, Hashtag, headingBlocks, livePreview, MathConfig, TABLE_EDIT_EVENT } from "@chnots/md-codemirror";
+ import type { TableEditDetail } from "@/lib/md-codemirror";
+ import { createCodemirrorTheme, generateKeybinding, Hashtag, headingBlocks, livePreview, MathConfig, TABLE_EDIT_EVENT } from "@/lib/md-codemirror";
```

**`web/src/krate/mdwt/component/table-editor/table-editor-dialog.tsx`**
```diff
- import type { ParsedTable } from "@chnots/md-codemirror";
- import { generateMarkdownTable, parseMarkdownTable } from "@chnots/md-codemirror";
+ import type { ParsedTable } from "@/lib/md-codemirror";
+ import { generateMarkdownTable, parseMarkdownTable } from "@/lib/md-codemirror";
```

### 1.3 清理包配置

- `web/package.json`：删除 `"@chnots/md-codemirror": "workspace:*"`
- `web/tsconfig.json`：删除 md-codemirror 的 project reference（line 22-24）
- `pnpm-workspace.yaml`：删除 `- lib/md-codemirror`（line 4）
- 删除 `lib/md-codemirror/` 目录（含 dist/、node_modules/ 等）

### 1.4 移动 md-codemirror 内部的 import 路径

md-codemirror 内部使用相对路径（如 `./editor/codemirror/...`），移动后路径关系不变，无需修改。

## Phase 2: 拆分 mdwt-editor.tsx

合并完成后，拆分 mdwt-editor.tsx：

### 2.1 提取补全逻辑

**创建 `web/src/krate/mdwt/component/slash-completions.ts`**（~50 行）
- `SLASH_COMMANDS` 数组（mdwt-editor.tsx:120-145）
- `slashCommandCompletions()` 函数（mdwt-editor.tsx:147-161）

**创建 `web/src/krate/mdwt/component/chnot-completions.ts`**（~60 行）
- `chnotCompletions()` 异步函数（mdwt-editor.tsx:163-215）
- import `slashCommandCompletions` from `./slash-completions`
- 保留 `chnotSearch`、`chnotTagNameList`、`toentTodoEventGuess` 依赖

### 2.2 提取粘贴处理

**创建 `web/src/krate/mdwt/component/paste-handler.ts`**（~75 行）
- `eventHandlers` 常量（mdwt-editor.tsx:46-118）

### 2.3 提取扩展组合

**创建 `web/src/krate/mdwt/component/editor-extensions.ts`**（~90 行）
- `markdownExtension` 和 `extensions` 数组构建（mdwt-editor.tsx:317-368）
- 导出 `buildEditorExtensions(config)` 函数

### 2.4 精简后的 mdwt-editor.tsx（~150 行）

```
imports（从提取的模块导入）
EditorCustomContext 定义
MdwtEditor 组件：
  - refs、state、effects（~70 行，不变）
  - buildEditorExtensions 调用（~10 行）
  - JSX return（~30 行）
exports
```

## 不动的文件

- `mdwt-extension.ts`（Backlink、ChnotProps、todoHighlight）— 域逻辑，留在 web
- `heading-chnot-completion.ts` — 已经独立，不动
- `thread-widget-extension.tsx` — React/CM 集成复杂，不动

## Verification

每个 phase 完成后：
1. `pnpm -F web type-check`
2. 手动测试：打开编辑器 → slash 命令、hashtag 补全、backlink 补全、粘贴 HTML、表格编辑
