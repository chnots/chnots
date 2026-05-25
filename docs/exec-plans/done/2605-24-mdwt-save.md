# MDWT Block Save 三项改进

## Context

当前 mdwt 后端保存流程存在三个问题：
1. 新 chnot 首次保存时 `fetch_chnot_meta` 会失败（meta 尚未创建）
2. heading 前的普通段落内容被丢弃，没有 OTID 关联
3. MdwtRecord 存储了完整的 heading 行（含 `## [[OTID]]`），heading level 信息没有单独存储

目标：修复 meta 创建时序、支持 preamble 内容、将 heading level 与正文分离存储。

## 已确认的设计决策

| 决策点 | 结论 |
|---|---|
| Thread meta 创建 | 后端 ensure 替代 fetch，前端加 kspace 参数 |
| Preamble 何时创建 | 只在 heading 前有非空内容时创建，纯空白不创建 |
| Preamble 存储 | MdwtRecord(otid=thread_otid)，不进 ChnotThreadOrder |
| Body-only 定义 | 剥离 `## [[otid]] ` 前缀，保留其后的 title + body 全部内容 |
| 无 body 的 heading | content = title 文本（如 `## [[123]] A` → content = `A`） |
| heading_level 存储 | ChnotThreadOrder 新增 heading_level 字段 (i16, 0=preamble 1-6=heading) |
| 旧数据兼容 | 先迁移再部署，不做读时兼容 |
| 删除 block | 走 archive：ChnotMeta 设 archive_tid，ChnotThreadOrder 走 archive 接口，MdwtRecord 走 po_otid_commit |
| heading_level 更新 | 走 commit 逻辑（po_otid_commit：omit 旧版本 + insert 新版本，自动保留历史） |
| 缺失 OTID | 后端报错，不自动生成 |
| 空 content | 前端不触发保存 |
| 全部 heading 删除 | 整篇内容变 preamble (otid=thread_otid)，所有子 block 删除 |
| MdwtParser | 仍用重构的完整内容（heading + body）解析 tags/events |
| MdwtCommitRsp | 删除 blocks 字段，保留 title + todo_event |
| 保存接口 | 保持 `POST /api/v1/mdwt-commit` 不变 |
| 加载接口 | 新增 `POST /api/v1/mdwt-content-load`，后端组装完整文本返回 |
| Block 重构 | `#`.repeat(level) + ` [[` + otid + `]] ` + content，block 间 `\n\n` 分隔 |

---

## Change 1: 后端 ensure 替代 fetch

### 文件

- `lib/backend/src/krate/mdwt/db.rs` — mdwt_commit
- `lib/backend/src/krate/mdwt/dto.rs` — MdwtCommitReqData 加 kspace
- `web/src/krate/chnot/component/rich-chnot/mdwt.tsx` — handleContentChange 传 kspace
- `web/src/krate/chnot/component/layout/body.tsx` — 传 kspace 到 RichMdwt

### 改动

1. `MdwtCommitReqData` 增加 `pub kspace: Option<Varchar<40>>`
2. `mdwt_commit` 中替换 `fetch_chnot_meta`：
   ```rust
   let kspace = match self.fetch_chnot_meta(thread_otid).await {
       Ok(meta) => meta.kspace,
       Err(_) => {
           let ks = mdwt.kspace.unwrap_or_else(|| "default".try_into().unwrap());
           self.ensure_chnot_meta(thread_otid, ChnotKind::MarkdownWithToent, ks.clone()).await?;
           ks
       }
   };
   ```
3. 前端 handleContentChange 传 kspace 到请求体

---

## Change 2: 支持 preamble

### 文件

- `lib/backend/src/krate/mdwt/parser/mod.rs` — ParsedBlock、parse_content_into_blocks
- `lib/backend/src/krate/mdwt/db.rs` — mdwt_commit 调用处

### 改动

1. `ParsedBlock` 增加 `pub level: u8`（0=preamble, 1-6=heading level）
2. `parse_content_into_blocks` 签名变为 `(content: &str, thread_otid: TID)`
3. Preamble 逻辑：
   - heading_indices 为空但有非空内容 → preamble block `{ otid: thread_otid, level: 0 }`
   - heading_indices[0] > 0 且前方有非空文本 → preamble block
   - 纯空白前方 → 不创建 preamble
4. Heading level 从 `#` 数量提取
5. 保留 heading otid != thread_otid 校验（排除 preamble，仅检查 level > 0 的 blocks）

---

## Change 3: Body-only 存储 + heading_level 分离

### 文件

- `lib/backend/src/krate/chnot/po.rs` — ChnotThreadOrder 加 heading_level
- `lib/backend/src/krate/mdwt/parser/mod.rs` — content 变 body-only
- `lib/backend/src/krate/mdwt/db.rs` — commit_thread_order_inner、mdwt_commit、删除 block
- `lib/backend/src/krate/chnot/dto.rs` — ChnotThreadMetaFetchRspData 加 heading_level
- `lib/backend/src/krate/chnot/db.rs` — chnot_thread_meta_fetch 查询加 heading_level
- `data/db.version` — 9→10
- `data/sqls/` — v10 migration SQL
- `lib/backend/src/mapper/db/db_version/` — v10 迁移逻辑

### 3a. Schema

1. `ChnotThreadOrder` 增加 `pub heading_level: i16`
2. v10 migration：
   ```sql
   ALTER TABLE chnot_thread_order ADD COLUMN heading_level SMALLINT NOT NULL DEFAULT 0;
   ALTER TABLE chnot_thread_order_hist ADD COLUMN heading_level SMALLINT NOT NULL DEFAULT 0;
   ```
3. 迁移脚本单独处理：回填 heading_level + 剥离 MdwtRecord.content 中的 heading 前缀

### 3b. 解析器 body-only

```rust
// heading block: content = heading_line 后面的全部内容（含 title 文本）
// 如 `## [[123]] My Title\nBody` → content = `My Title\nBody`
let heading_text = caps.get(3).map(|m| m.as_str()).unwrap_or("");
let block_content = [heading_text.to_owned()]
    .into_iter()
    .chain(block_lines.iter().map(|l| l.to_string()))
    .collect::<Vec<_>>()
    .join("\n");
```

preamble block（level==0）content = 原始文本。

### 3c. 后端 commit/archive 流程

**原则：删除走 archive，更新走 commit。**

1. `mdwt_commit` 中每个 block：
   - 重构完整内容给 MdwtParser：`"#".repeat(level) + " [[otid]] " + content`
   - `overwrite_mdwt_record` 传入 body-only content（内部走 `po_otid_commit`）

2. `commit_thread_order_inner` 改为接收 `&[ParsedBlock]`：
   - 新 block：INSERT 带 heading_level
   - 已有 block heading_level 变化：走 `po_otid_commit`（omit 旧版本 → insert 新版本，自动保留 `_hist`）

3. 被删除的 blocks 走 archive：
   - `ChnotMeta`：设置 `archive_tid = Some(TID::now())`（软删除，复用 `chnot_meta_commit` 逻辑）
   - `ChnotThreadOrder`：走 `chnot_thread_order_archive` 接口（`omit_rows` copy 到 `_hist` 再删）
   - `MdwtRecord`：走 `po_otid_commit`（copy 到 `mdwt_record_hist` 再删主表记录）

4. Preamble block 存为 `MdwtRecord(otid=thread_otid, content=preamble_text)`，不进 ChnotThreadOrder

5. `MdwtCommitRsp` 删除 `blocks` 字段

### 3d. 新增加载接口

**`POST /api/v1/mdwt-content-load`**
- 入参：`{ otid: TID }`
- 返回：`{ content: String }`
- 后端逻辑：
  1. 查 ChnotThreadOrder(thread_otid) → 获取子 blocks 的 otid + heading_level + korder
  2. 查 MdwtRecord(thread_otid) → 获取 preamble（如存在）
  3. 查 MdwtRecord(child_otids) → 获取子 blocks 的 body-only content
  4. 组装：preamble 在前，然后按 korder 遍历子 blocks，每个拼接 `#`.repeat(level) + ` [[` + otid + `]] ` + content
  5. blocks 间用 `\n\n` 分隔

### 3e. 前端适配

1. 删除 `joinMdwtBlocks` 函数（后端已组装）
2. 加载流程简化：`mdwtContentLoad({ otid })` → 直接设置 content
3. 保存流程不变：`mdwtCommit({ mdwt: { otid, content, kspace } })`
4. DTO 类型适配

---

## 实施顺序

1. Phase 1 — 后端解析器：ParsedBlock 加 level、parse_content_into_blocks 支持 preamble 和 body-only content、更新测试
2. Phase 2 — DB schema：ChnotThreadOrder 加 heading_level、v10 migration SQL
3. Phase 3 — 后端 mdwt_commit：ensure 替代 fetch、body-only 存储、删除 block 逻辑、commit_thread_order_inner 改造
4. Phase 4 — 新增加载接口：mdwt-content-load
5. Phase 5 — 前端适配：DTO 类型、加载流程简化、save 传 kspace

---

## 验证

- `cargo test -p backend` — parser 单元测试
- 新建 chnot → 输入内容 → 保存 → 通过 mdwt-content-load 加载验证内容一致
- 新建 chnot → 输入 preamble + heading → 保存 → 加载验证 preamble 保留
- 编辑 chnot → 改 heading 级别（## → ###）→ 保存 → 加载验证 level 更新
- 删除所有 heading → 保存 → 加载验证变 preamble
- 删除部分 heading → 保存 → 验证被删 block 走了 archive（ChnotMeta.archive_tid 非空、ChnotThreadOrder 在 _hist 中、MdwtRecord 在 _hist 中）
