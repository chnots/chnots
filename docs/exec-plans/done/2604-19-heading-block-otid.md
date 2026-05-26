# Heading Block OTID 系统

## Background

- Thread（`ChnotKind.ThreadV1`）目前使用多个独立 CodeMirror 编辑器渲染多个 mdwt block，需要改为单编辑器模式
- 需要在 markdown 文档中为每个 heading section 分配持久化的 OTID（= mdwt 的 otid）
- OTID 用于定位/滚动、per-block 元数据、外部引用链接

## Goals

- 在单个 CodeMirror 编辑器中，以 `# [[OTID]] 标题` 格式标记 block
- 光标不在 heading 行时隐藏 `[[otid]]`，呈现为普通 markdown
- 用户通过 `/tid` slash command 显式添加 block OTID
- 保存时按 block 拆分为多个 mdwt record，加载时拼接回单文档
- 提供 block 级 API：按位置/otid 查询 block、滚动到 block、拆分文档

## Scope

- In scope:
  - `lib/md-codemirror` 新增 heading-block 扩展模块
  - heading 行 `[[otid]]` 的隐藏 decoration
  - `/tid` autocompletion（仅 heading 行触发）
  - `StateField<HeadingBlock[]>` 解析和存储
  - 公开 API（getBlockAtPos, splitDocumentByBlocks 等）
  - thread 组件改为单编辑器 + 加载拼接/保存拆分
- Out of scope:
  - 自动注入 OTID（不做，用户显式添加）
  - 无 `[[otid]]` 的 heading 的 block 管理
  - block 级折叠 UI（后续 feature）
  - 拖拽排序 block（后续 feature）

## Technical Plan

### 1. 新增 `lib/md-codemirror/src/heading-block/` 模块

文件结构：

```
src/heading-block/
  block-model.ts        # HeadingBlock 类型 + HEADING_OTID_RE 正则
  block-parser.ts       # 遍历 syntaxTree 提取有 otid 的 heading
  block-field.ts        # StateField<HeadingBlock[]> + block facet
  block-decorations.ts  # 隐藏 [[otid]] 的 replace decoration
  block-completion.ts   # /tid completion source（heading 行限定）
  index.ts              # headingBlocks() 扩展入口 + 公开 API
```

#### 1.1 `block-model.ts`

```typescript
export interface HeadingBlock {
  otid: TID
  from: number            // heading 行首位置
  to: number              // block 结束位置（下一个同级/更高级 heading 之前或文档末尾）
  level: number           // 1-6
  headingContent: string  // "我的标题"（不含 # 和 [[otid]]）
}

// 匹配 heading 中的 [[otid]]
export const HEADING_OTID_RE = /^(\s{0,3}#{1,6}\s+)\[\[(\d{13,16})\]\]\s+(.*)/
```

#### 1.2 `block-parser.ts`

- `parseHeadings(state: EditorState): HeadingBlock[]`
- 遍历 `syntaxTree(state)`，查找 `ATXHeading1`~`ATXHeading6` 节点
- 对每个 heading 行文本执行 `HEADING_OTID_RE`
- 只返回有 `[[otid]]` 的 heading（无 otid 的跳过）
- block 边界：heading 起始 → 下一个同级/更高级 heading 之前 或 文档末尾

#### 1.3 `block-field.ts`

- `StateField.define<HeadingBlock[]>`
- `create(state)`: 调用 `parseHeadings(state)`
- `update(blocks, tr)`: 仅在 `syntaxTree` 变化时重新 parse
- 提供 `EditorView.decorations.from(f)` 用于 line decoration
- 定义 `blockFacet` 供工具函数访问 block 数据

#### 1.4 `block-decorations.ts`

复用 livePreview 的 "active line awareness" 模式：

- 遍历 blocks，为每个 block 范围内的行添加 `Decoration.line({ attributes: { "data-block-otid": String(otid) } })`
- 对于 heading 行的 `[[otid]]` 部分，用 `Decoration.replace` 隐藏（光标不在该行时）
- 光标在 heading 行时显示完整 `# [[otid]] 标题`

#### 1.5 `block-completion.ts`

`/tid` completion source：

- 检测光标所在行是否为 heading 行（`#` 开头）
- 检测光标前是否输入了 `/tid`
- 替换 `/tid` 为 `[[genTID()]] `
- 使用 `@codemirror/autocomplete` 的 `CompletionContext`

```typescript
export function tidCompletion(config: { genTID: () => TID }): Extension
```

#### 1.6 `index.ts` — 扩展入口 + 公开 API

```typescript
export function headingBlocks(config: {
  genTID: () => TID
}): Extension

// 工具函数
export function getBlockAtPos(state: EditorState, pos: number): HeadingBlock | null
export function getBlockByOtid(state: EditorState, otid: TID): HeadingBlock | null
export function getAllBlocks(state: EditorState): HeadingBlock[]
export function splitDocumentByBlocks(state: EditorState): Map<TID, string>
export function scrollToBlock(view: EditorView, otid: TID): void
```

### 2. 修改 `lib/md-codemirror/src/editor/codemirror/livePreview/decoration-builder.ts`

在现有的 heading decoration 逻辑中集成 `[[otid]]` 隐藏：
- 当 `node.name.startsWith("ATXHeading")` 且光标不在该行时
- 用 `HEADING_OTID_RE` 匹配 heading 文本
- 对 `[[otid]]` 部分应用 `Decoration.replace`

### 3. 修改 `lib/md-codemirror/src/index.ts`

导出 heading-block 模块的所有公开 API。

### 4. 修改 `web/src/krate/mdwt/component/mdwt-editor.tsx`

- 引入 `headingBlocks` 扩展
- 将 `tidCompletion` 加入 `autocompletion({ override: [...] })`
- 传入 `genTID` 配置

### 5. 修改 `web/src/krate/chnot/component/rich-chnot/thread/index.tsx`

#### 5.1 加载流程

```
chnotThreadMetaFetch(otid) → 获取子 chnot 列表（含 korder）
→ mdwtRecordList(otids) → 批量获取 mdwt 内容（每个 content 已含 "# [[otid]] Title"）
→ joinMdwtBlocks(blocks) → 按 korder 拼接为单文档，块间 \n\n 分隔
→ 传入单个 <MdwtEditor doc={joinedDoc} />
```

#### 5.2 保存流程（debounce 500ms）

```
splitDocumentByBlocks(state) → Map<otid, content>
→ 对比上一次保存的 block 集合
→ 新增 block: mdwtCommit(otid, content) + 加入 thread order
→ 变更 block: mdwtCommit(otid, content)
→ 删除 block: chnotThreadOrderArchive(otids)
→ 顺序变化: chnotThreadOrderCommit(new orders)
```

#### 5.3 辅助函数

- `joinMdwtBlocks(blocks: Array<{ otid: TID; content: string }>): string`
  - 按 korder 排序，用 `\n\n` 连接各 block content
  - 每个 block content 首行已是 `# [[otid]] Title` 格式

## Risks and Decisions

- Risk: `[[otid]]` 语法与现有 Backlink 解析器冲突
  -> Mitigation: heading 行的 `[[otid]]` 由 block-parser 优先处理；Backlink 解析器不会在 heading 上下文中被触发（lezer parser 的节点隔离）
- Risk: 用户误删 `[[otid]]` 导致 block 身份丢失
  -> Mitigation: 保存时检测，缺少 otid 的 heading 只是普通文本，不会破坏已有 block；用户可通过 `/tid` 重新添加
- Decision: 不自动注入 OTID — 用户通过 `/tid` 显式添加，避免文档被自动修改
- Decision: 只有带 `[[otid]]` 的 heading 才算 block — 简化解析逻辑，明确区分
- Decision: OTID 存储在文本中而非外部映射 — 自包含、undo/redo 自然、无需同步

## Acceptance Criteria

- [ ] heading 行有 `# [[otid]] 标题` 格式时，光标不在该行时 `[[otid]]` 被隐藏
- [ ] 光标移到 heading 行时，`[[otid]]` 完整显示可编辑
- [ ] 在 heading 行输入 `/tid` 触发 completion，替换为 `[[genTID()]] `
- [ ] 非 heading 行输入 `/tid` 不触发 completion
- [ ] `getAllBlocks()` 正确返回所有有 otid 的 heading block
- [ ] `splitDocumentByBlocks()` 正确按 block 拆分文档内容
- [ ] Thread 加载时多个 mdwt 拼接为单文档正确显示
- [ ] Thread 保存时正确拆分为多个 mdwt record 并持久化
- [ ] Undo/Redo 后 block otid 保持正确

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2604-19/heading-block-otid`
- [ ] Create first commit: `task-start: docs/2604-19-heading-block-otid.md`
- [ ] Implement block-model + block-parser
- [ ] Implement block-field
- [ ] Implement block-decorations (integrate into livePreview)
- [ ] Implement block-completion (/tid)
- [ ] Implement index.ts (public API)
- [ ] Integrate into mdwt-editor.tsx
- [ ] Modify thread/index.tsx (join/split logic)
- [ ] Test end-to-end
- [ ] Move file to `docs/done/` after completion
- [ ] Create final commit: `task-done: docs/2604-19-heading-block-otid.md`
