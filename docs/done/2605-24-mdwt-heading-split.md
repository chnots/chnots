# MDWT Heading OTID Validation

## Context

当前 mdwt 写入时，后端 `parse_content_into_blocks` 会自动为缺少 OTID 的 heading 注入 `[[<OTID>]]`。需要改为：后端严格校验所有 heading 必须有 OTID，没有则报错；且第一个 heading 的 OTID 不能等于 chnot 自身的 OTID。OTID 注入由前端 CodeMirror extension 层自动完成。

## Decisions

| 分支 | 决策 |
|---|---|
| 光标问题 | 方案 2：在 `headingBlocks()` extension 层通过 transaction 注入，光标不跳。备注：如果方案 2 不好使，退到方案 1（view.dispatch in handleContentChange） |
| 无 heading 内容 | 直接报错，不自动生成 OTID。每个 mdwt 文档必须至少有一个带 OTID 的 heading |
| 返回类型简化 | `parse_content_into_blocks` 返回 `Result<Vec<ParsedBlock>>`，去掉 String。`rsp.content` 删掉 |
| 首 heading OTID 校验 | 只在后端 `mdwt_commit` 校验，前端不防 |
| `/tid` 斜杠命令 | 去掉，extension 自动注入 |
| OTID 被删 | extension 补回去，不做特殊处理 |

## Changes

### 1. 后端：`parse_content_into_blocks` 改为纯校验

**File**: `lib/backend/src/krate/mdwt/parser/mod.rs`

- 返回类型从 `(Vec<ParsedBlock>, String)` 改为 `Result<Vec<ParsedBlock>>`
- heading 没有 OTID → `Err("heading '{}' is missing OTID")`
- OTID 格式无效（`[[abc]]`） → `Err("invalid OTID in heading: {}")`
- 无 heading 的内容 → `Err` (不再自动生成单 block)
- 去掉 `updated_lines` / `modified_heading` 相关逻辑
- 去掉返回值中的 String（不再修改内容）

### 2. 后端：`mdwt_commit` 适配

**File**: `lib/backend/src/krate/mdwt/db.rs` (L142-221)

- `parse_content_into_blocks` 调用加 `?` 处理错误
- 新增首 heading OTID != thread_otid 校验
- 删除 `content_changed` / `rsp.content` 相关逻辑
- `MdwtCommitRsp` 删除 `content` 字段（前端不再需要）

### 3. 前端：CodeMirror extension 层自动注入 OTID

**File**: `lib/md-codemirror/src/heading-block/index.ts` + 新建逻辑文件

- `headingBlocks({ genTID })` 扩展：接受 genTID 参数
- 使用 CodeMirror ViewPlugin 或 StateField 监听文档变化
- 检测没有 OTID 的 heading 行，通过 transaction 插入 `[[<genTID()>]] `
- OTID 被删除时自动补回

**File**: `web/src/krate/mdwt/component/mdwt-editor.tsx`

- `headingBlocks()` → `headingBlocks({ genTID })`
- 删除 `tidCompletion({ genTID })` from autocompletion override

### 4. 前端：清理

- 删除 `tidCompletion` 函数 (`lib/md-codemirror/src/heading-block/block-completion.ts`)
- 删除 `ensureHeadingOTIDs` 函数（不需要了，extension 层已处理）
- `mdwt.tsx` 删除 `rsp.content` 相关更新逻辑
- `MdwtCommitRsp` DTO 删除 `content` 字段

### 5. 更新测试

**File**: `lib/backend/src/krate/mdwt/parser/mod.rs` (tests section)

- 修改现有测试：所有 heading 都带 OTID
- 新增测试：
  - 无 heading 内容 → 报错
  - heading 缺 OTID → 报错
  - OTID 格式无效 → 报错
  - 部分_heading 缺 OTID → 报错

## Files to Modify

| File | Change |
|---|---|
| `lib/backend/src/krate/mdwt/parser/mod.rs` | 返回 `Result<Vec<ParsedBlock>>`，纯校验 |
| `lib/backend/src/krate/mdwt/db.rs` | 处理 Result + 首 heading 校验 + 删 rsp.content |
| `lib/md-codemirror/src/heading-block/index.ts` | `headingBlocks({ genTID })` 扩展 |
| `lib/md-codemirror/src/heading-block/block-completion.ts` | 删除 `tidCompletion` |
| `web/src/krate/mdwt/component/mdwt-editor.tsx` | 传 genTID，删 tidCompletion |
| `web/src/krate/chnot/component/rich-chnot/mdwt.tsx` | 删 rsp.content 逻辑 |
| `web/src/krate/mdwt/dto.ts` | 删 MdwtCommitRsp.content |

## Verification

1. `cargo test -p backend mdwt` — 后端单元测试通过
2. `pnpm --filter web build` — 前端编译通过
3. 手动测试：新建 heading → OTID 自动注入 → 光标不跳 → 保存成功
4. 手动测试：用 API 发送无 OTID heading → 返回错误
