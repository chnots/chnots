# KTab 交互改进

## Background

- 当前 ktab 表格无键盘导航（Tab/Enter 不跨格移动），编辑体验差
- 时间列默认值在 `makeNewRow()` 中填入本地 state 但未 commit 到后端，刷新后丢失
- 行增删操作（添加行/删除行）只改本地 state，不调后端 API
- mdwt 内联 widget 在侧边栏/全屏编辑后数据不同步
- 无文本换行（Wrap）控制，长文本被截断

## Goals

- Tab → 右移一格，行末 → 下一行第一格，表末 → 新建行
- Enter → commit 并跳到下一行同列
- Shift+Tab → 反向导航
- 新行 date/datetime 列填入当前时间，用户编辑该行其他列时连带 commit
- 新增后端行删除 API（软删除，archive 到 hist 表）
- KTabMeta 增加 `wrap_enabled` 字段，工具栏 Wrap 按钮持久化
- mdwt 关闭侧边栏/全屏后自动刷新内联 widget
- 编辑态单元格蓝色边框高亮

## Scope

- In scope:
  - `KTabTable` 增加 `activeCell` 协调状态，统一管理跨单元格导航
  - CellRendererProps 增加 `isActive`/`onActivate`/`onNavigate` props
  - 全部 5 种 cell renderer 适配协调编辑，加 `React.memo`
  - Text/Number Cell: Tab/Enter/Escape 键盘处理
  - Checkbox Cell: Space 切换，Tab 离开
  - Date/MultiSelect Cell: Tab 关闭 popover（不 commit 未确认变更）
  - 新行默认值填入 + 延迟 commit（pending rows 追踪）
  - KTabMeta 加 `wrap_enabled` 字段（前端 + 后端）
  - 工具栏 Wrap 切换按钮
  - 后端 `POST /api/v1/ktab-row-delete` 软删除端点
  - 前端 `handleDeleteRow` 调后端 API
  - mdwt 全屏 close 时调 `refreshThreadWidgets()`
  - 活跃格 `ring-2 ring-primary` 蓝色边框

- Out of scope:
  - Arrow 键跨格导航（输入框内 Arrow 用于光标移动）
  - 撤销/重做
  - 列拖拽排序、范围选中、虚拟滚动、过滤栏

## Decisions

| # | 决策 | 结论 |
|---|------|------|
| 1 | 编辑触发 | 单击即编辑 |
| 2 | Checkbox + Tab | 停留，Space 切换 |
| 3 | Date/MultiSelect Popover + Tab | 关闭 popover，不 commit |
| 4 | 新行日期默认值 | 填本地，修改其他列时连带 commit |
| 5 | 编辑协调方案 | isActive prop + React.memo |
| 6 | Enter 行为 | commit 并跳到下一行同列 |
| 7 | 删除行 | 新增后端 API，archive 到 hist 表（`omit_rows`） |
| 8 | Wrap 按钮 | 持久化到 KTabMeta.wrap_enabled |
| 9 | 活跃格视觉 | ring-2 ring-primary ring-inset + bg-primary/5 |

## Technical Plan

### Step 1: 数据模型扩展

**前端** `web/src/krate/ktab/po.ts`:
- `KTabMeta` 加 `wrap_enabled?: boolean`

**后端** `lib/backend/src/krate/ktab/po.rs`:
- `KTabMeta` 加 `pub wrap_enabled: Option<bool>`

### Step 2: CellRendererProps 重构

`web/src/krate/ktab/component/cell-renderer/types.ts`:
```typescript
interface CellRendererProps {
  value: unknown;
  columnMeta: KTabColumnMeta;
  readonly: boolean;
  onCommit: (value: unknown) => void;
  onColumnChange?: (patch: Partial<KTabColumnMeta>) => void;
  // 新增 — 协调编辑
  isActive: boolean;
  onActivate: () => void;
  onNavigate: (dir: 'next' | 'prev' | 'up' | 'down') => void;
}
```

### Step 3: KTabTable 核心改造

`web/src/krate/ktab/component/ktab-table.tsx`:

**新增状态**:
- `activeCell: {rowIdx: number; colName: string} | null` — 当前编辑格
- `wrapEnabled: boolean` — 初始值从 `tableMeta.wrap_enabled` 读取
- `pendingRowTids: Set<number>` — 有未提交默认值的新行

**新增方法**:
- `navigateCell(from, dir)` — 计算目标格，commit 当前格，设置新的 activeCell；表末则新建行
- `createNewRow(afterRowIdx?)` — 新建行并加入 `pendingRowTids`
- `commitCell` 增强 — 若行在 `pendingRowTids` 中，批量提交所有默认值列 + 当前列

**Wrap 按钮** (toolbar):
```tsx
<Button variant="ghost" size="sm" onClick={toggleWrap}>
  <WrapTextIcon className="h-3.5 w-3.5 mr-1" />
  {wrapEnabled ? "No Wrap" : "Wrap"}
</Button>
```

**Wrap 效果**: `<td>` 上条件替换 `overflow-hidden text-ellipsis whitespace-nowrap` 为 `whitespace-pre-wrap break-all`。

**活跃格样式**:
```tsx
className={isActive ? "border ring-2 ring-primary ring-inset bg-primary/5" : "border"}
```

### Step 4: Cell Renderer 逐个改造

| Renderer | 改动 |
|----------|------|
| **TextCell** | `isActive`→进入编辑态；Enter→`onNavigate('down')`；Tab→`onNavigate('next')`；Escape→deactivate；onClick→`onActivate()`；`React.memo` |
| **NumberCell** | 同 TextCell |
| **CheckboxCell** | `isActive`→聚焦 input；Space 切换（浏览器默认）；Tab→`onNavigate`；`React.memo` |
| **DateCell** | `isActive`→打开 Popover；`isActive`变 false→关闭；Tab→阻止冒泡+关闭+`onNavigate`；Enter→commit 日期+`onNavigate('down')`；`React.memo` |
| **MultiSelectCell** | 同 DateCell（Popover 型） |

### Step 5: 行删除后端 API

**DTO** `lib/backend/src/krate/ktab/dto.rs`:
```rust
pub struct KTabRowDeleteReq { pub table_id: TID, pub row_tid: TID }
pub struct KTabRowDeleteRsp {}
```

**DB** `lib/backend/src/krate/ktab/db.rs`:
- `ktab_row_delete`: 遍历所有列，对三种 cell 类型各调 `omit_rows`（自动 copy to hist + delete from main），在事务中执行

**Mapper** `lib/backend/src/krate/ktab/mapper.rs`:
- `ktab_row_delete` trait 方法

**Controller** `lib/backend/src/krate/ktab/controller.rs`:
- 注册 `POST /api/v1/ktab-row-delete` 路由

**前端** `web/src/krate/ktab/dto.ts` + `service.ts`:
- `KTabRowDeleteReq` 类型 + `ktabRowDelete` 函数

**ktab-table.tsx** `handleDeleteRow`:
```typescript
await ktabRowDelete({ table_id: tableMeta.otid, row_tid: row.row_tid });
setRows(prev => prev.filter(r => r.row_tid !== row.row_tid));
```

### Step 6: mdwt 同步刷新

`web/src/krate/chnot/component/rich-chnot/mdwt.tsx`:
- 全屏 dismiss 时调用 `refreshThreadWidgets()`（当前只清 state）

`web/src/krate/mdwt/component/ktab-inline-widget.tsx`:
- 监听 `kindData` prop 变化，用 `useEffect` 同步到内部 state
- 或改用 `useRef` + 强制刷新

### Step 7: 模块导出更新

`web/src/krate/ktab/index.ts` — 确认导出新增的类型和函数。

## Files to Modify

### 前端 (web/src/)
| 文件 | 改动 |
|------|------|
| `krate/ktab/po.ts` | KTabMeta 加 wrap_enabled |
| `krate/ktab/dto.ts` | 加 KTabRowDeleteReq 类型 |
| `krate/ktab/service.ts` | 加 ktabRowDelete 函数 |
| `krate/ktab/component/ktab-table.tsx` | activeCell、导航、wrap、pending rows、commitCell 增强、边框样式 |
| `krate/ktab/component/cell-renderer/types.ts` | 加 isActive/onActivate/onNavigate |
| `krate/ktab/component/cell-renderer/text-cell.tsx` | 协调编辑 + React.memo |
| `krate/ktab/component/cell-renderer/number-cell.tsx` | 同上 |
| `krate/ktab/component/cell-renderer/checkbox-cell.tsx` | isActive/Space/Tab + React.memo |
| `krate/ktab/component/cell-renderer/date-cell.tsx` | popover + Tab/Enter + React.memo |
| `krate/ktab/component/cell-renderer/multi-select-cell.tsx` | 同上 |
| `krate/chnot/component/rich-chnot/mdwt.tsx` | 全屏 dismiss → refreshThreadWidgets |
| `krate/mdwt/component/ktab-inline-widget.tsx` | kindData prop sync |
| `krate/mdwt/component/thread-widget-extension.tsx` | eq() 加 version/kinkData 比较 |

### 后端 (lib/backend/src/)
| 文件 | 改动 |
|------|------|
| `krate/ktab/po.rs` | KTabMeta 加 wrap_enabled |
| `krate/ktab/dto.rs` | 加 KTabRowDeleteReq/Rsp |
| `krate/ktab/db.rs` | ktab_row_delete 实现 |
| `krate/ktab/mapper.rs` | ktab_row_delete trait 方法 |
| `krate/ktab/controller.rs` | 注册路由 |

## Risks and Decisions

- Risk: `activeCell` 用 rowIdx 索引，会在插入/删除行后错位 → Mitigation: 用 `row_tid` 作为 key（已存在，唯一）
- Risk: Popover 型 cell（date/multi_select）的 Tab 事件可能被 Popover 消费 → Mitigation: `onKeyDown` capture phase + `stopPropagation`
- Risk: `omit_rows` 需要完整 pkey（table_otid + col_otid + row_otid），删除整行需遍历所有列 → Mitigation: 先从 meta 获取列列表，再逐列调用 `omit_rows`
- Risk: mdwt `refreshThreadWidgets` 会重新请求全部数据，大数据量可能慢 → Mitigation: 当前 PAGE_SIZE=500 上限，可接受
- Decision: 用 `activeCell` 状态而非 ref imperative handle（见 grill 决策 #5）
- Decision: 后端行删除用 hist 表软删除而非真正 DELETE（见 grill 决策 #7）
- Decision: 新行默认值延迟 commit 而非立即 commit（见 grill 决策 #4）

## Acceptance Criteria

- [ ] Tab 在 text/number/date/multi_select 格 → 右移一格并自动进入编辑
- [ ] 行末 Tab → 下一行第一格；表末 Tab → 新建行
- [ ] Enter 在 text/number → commit 并跳到下一行同列
- [ ] Shift+Tab → 反向导航
- [ ] Checkbox 格 Space 切换勾选，Tab 离开
- [ ] Date/MultiSelect Popover 打开时 Tab → 关闭 Popover 并移到下一格
- [ ] 新行 date 列显示今天日期，编辑其他列后日期值持久化到后端
- [ ] Wrap 按钮切换后单元格换行/截断变化，刷新保持状态
- [ ] 删除行后刷新页面行不再出现
- [ ] 活跃格显示蓝色 ring 边框
- [ ] mdwt 全屏编辑 ktab → 关闭全屏 → 内联 widget 显示最新数据
- [ ] `pnpm run lint && pnpm run build` 通过
- [ ] `cargo check -p backend` 通过

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2606-02/ktab-interaction-improvements`
- [ ] Create first commit: `task-start: docs/exec-plans/active/2606-02-ktab-interaction-improvements.md ktab interaction improvements`
- [ ] Implement in small commits
- [ ] Move file to `docs/exec-plans/done/` after completion
- [ ] Create final commit: `task-done: docs/exec-plans/done/2606-02-ktab-interaction-improvements.md ktab interaction improvements`
