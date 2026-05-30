# KTab 表格重建（Notion 风格）

## Background

- 现有 ktab 前端组件已全部删除，需要从零重建
- 后端 API（meta fetch/commit、cell list/commit）和数据模型（po.ts、dto.ts、service.ts、vo.ts）完整可用
- 目标是 Notion 风格的可编辑表格，支持多类型单元格、转置、CSV 导出

## Goals

- 从零构建一个统一表格组件，通过 `compact` prop 区分嵌入式和全屏模式
- 支持 7 种单元格类型：文本（markdown）、整数、小数、日期、进度条、枚举多选、图片
- 实现行号、转置、CSV 导出、右键列名编辑、单击编辑等交互

## Scope

- In scope:
  - Cell 渲染器工厂 + 7 种 cell 类型（显示态 + 编辑态）
  - 统一表格组件（`KTabTable`）+ `compact` prop
  - 行号列（始终显示）
  - 转置视图切换（纯前端，不持久化）
  - CSV 导出（两种模式都有）
  - 列名右键编辑
  - 单击单元格进入编辑态
  - 新增列（末尾 "+" 按钮，两种模式都有）
  - `table.tsx` 集成层适配
- Out of scope:
  - 虚拟滚动分页（预留接口，首期全量加载）
  - 列类型修改（已有列不改类型）
  - 范围选中
  - 过滤栏
  - 完整键盘导航（Tab/Arrow keys）
  - 列拖拽排序
  - 列固定（freeze pane）
  - 撤销/重做

## Cell 类型系统

| Cell 类型 | view_kind | store_kind | 显示态 | 编辑态 | 编辑触发 |
|-----------|-----------|------------|--------|--------|----------|
| 文本 | `string` | `str` | Markdown 渲染（bold/italic/strikethrough/code/link） | textarea | 单击 |
| 整数 | `integer` | `i64` | 数字右对齐 | input 带整数验证 | 单击 |
| 小数 | `decimal` | `f64` | 数字右对齐 | input 带小数验证 | 单击 |
| 日期 | `date` | `date` | 格式化日期居中 | 日期选择器 Popover | 单击 |
| 进度 | `progress` | `f64` | 进度条（支持分数/小数） | 数字输入 | 单击 |
| 枚举多选 | `multi_select` | `str` | 彩色标签 | Popover 下拉多选 | 单击 |
| 图片 | `image` | `str` | 缩略图 + 点击预览 | 文件选择上传（kfile） | 单击 |

## Technical Plan

### Step 1: 类型扩展

文件：`web/src/krate/ktab/po.ts`

- `KTabColumnViewKind` 加 `"integer"`
- `ktabViewToStoreKind` 已有 `"integer" -> "i64"` 映射
- 确认所有 7 种 view_kind 都有对应的 store_kind 映射

### Step 2: Cell 渲染器

新建目录：`web/src/krate/ktab/component/cell-renderer/`

```
cell-renderer/
├── types.ts              # CellRendererProps, CellRenderer 接口定义
├── index.tsx             # getCellRenderer(viewKind) 工厂函数
├── text-cell.tsx         # string: Markdown 显示 + textarea 编辑
├── number-cell.tsx       # integer + decimal: 数字显示 + input 编辑
├── date-cell.tsx         # date: 日期显示 + 日期 Popover 编辑
├── progress-cell.tsx     # progress: 进度条显示 + 数字输入编辑
├── multi-select-cell.tsx # multi_select: 标签显示 + Popover 下拉多选
└── image-cell.tsx        # image: 缩略图 + kfile 上传
```

每个渲染器统一接口：
```typescript
interface CellRendererProps {
  value: unknown;
  columnMeta: KTabColumnMeta;
  onCommit: (value: unknown) => void;
  readonly: boolean;
}
```

渲染器内部自行管理编辑态（单击触发），不需要外部 `isEditing` 状态。

### Step 3: 统一表格组件

新建：`web/src/krate/ktab/component/ktab-table.tsx`

核心 props：
```typescript
interface KTabTableProps {
  tableMeta: KTabMeta;
  fetchData: (tableId: number, start: number, size: number) => Promise<KTabRowData[]>;
  readonly: boolean;
  onMetaChange: (meta: KTabMeta) => void;
  compact?: boolean; // true = 嵌入式, false/undefined = 全屏
}
```

表格结构：
- **行号列**：第一列，固定宽度，灰色背景，不可编辑
- **列头**：显示列名 + 类型图标，右键弹出编辑菜单（重命名）
- **数据列**：通过 cell 渲染器工厂按 view_kind 渲染
- **末尾 "+" 列**：点击弹出新增列对话框（选列名 + 类型）

compact 模式行为：
- 全量渲染行，不用虚拟化
- 不渲染工具栏
- 显示 CSV 导出小图标按钮
- 右键列名编辑仍然可用

full 模式行为：
- 预留虚拟滚动（useVirtualizer 接口，首期全量渲染）
- 工具栏：转置按钮 + CSV 导出按钮
- 右键菜单：列名编辑

### Step 4: 转置

纯前端视图切换。在 `ktab-table.tsx` 内：

- `const [transposed, setTransposed] = useState(false)`
- 转置时：行列数据互换，列头变为第一列的值，行号变为原列名
- 不改变传入的 `tableMeta` 和数据，仅影响渲染

### Step 5: CSV 导出

新建：`web/src/krate/ktab/component/csv-export.ts`

- 导出全量数据（调 `ktabCellList` 拉全量）
- Header 用列 `name`
- 多选值用逗号分隔
- 图片值用 URL
- `Blob` + `URL.createObjectURL` 触发下载

### Step 6: 集成层适配

文件：`web/src/krate/chnot/component/rich-chnot/table.tsx`

- 删除 `DataTable` / `ComplexDataTable` 分别引用
- 统一用 `KTabTable` + `compact` prop
- `standalone` 和 `fullscreen` → `compact=false`
- 嵌入式 → `compact=true`

### Step 7: 模块导出

文件：`web/src/krate/ktab/index.ts`

- 更新导出，删除旧引用，导出 `KTabTable` 和 `KTabRowData`

## 文件清单

```
web/src/krate/ktab/
├── index.ts                          # 更新导出
├── po.ts                             # 加 integer view_kind（如缺失）
├── dto.ts                            # 不变
├── vo.ts                             # 不变
├── service.ts                        # 不变
└── component/
    ├── ktab-table.tsx                 # 统一表格组件（核心）
    ├── csv-export.ts                  # CSV 导出工具
    └── cell-renderer/
        ├── types.ts                   # 渲染器接口
        ├── index.tsx                  # 工厂函数
        ├── text-cell.tsx
        ├── number-cell.tsx
        ├── date-cell.tsx
        ├── progress-cell.tsx
        ├── multi-select-cell.tsx
        └── image-cell.tsx
```

## Risks and Decisions

- Risk: 单击编辑与行选中/列头点击事件冲突 → Mitigation: 事件只在 `<td>` 数据区域触发，列头和行号列不触发
- Risk: 转置后列数可能很大（行多时） → Mitigation: 转置仅适合小表格，大数据量时提示用户
- Risk: 图片缩略图行高与固定行高不兼容 → Mitigation: image 类型行高自适应，最小 56px
- Risk: 进度条分数解析（1/5、6/4）需要正则 → Mitigation: `^(\d+)/(\d+)$` 解析分数，fallback 为小数
- Decision: 单一组件 + compact prop，不拆两个组件，最小代码量
- Decision: 编辑态由 cell 渲染器内部管理，不需要全局 editing 状态
- Decision: CSV 导出调一次全量 API，不从当前渲染数据导出
- Decision: 不参考旧代码，从零构建

## Acceptance Criteria

- [ ] 7 种 cell 类型各自显示态渲染正确
- [ ] 7 种 cell 类型单击进入编辑态，编辑后保存到后端
- [ ] 文本 cell 支持 bold/italic/strikethrough/code/link Markdown 渲染
- [ ] 进度条 cell 支持小数和分数格式
- [ ] 枚举多选 cell 标签显示 + Popover 下拉多选
- [ ] 图片 cell 缩略图显示 + 点击预览 + kfile 上传
- [ ] compact 模式嵌入 chnot 正常，有 CSV 导出按钮
- [ ] full 模式全屏正常，有工具栏（转置 + 导出）
- [ ] 行号始终显示
- [ ] 右键列头可编辑列名
- [ ] 末尾 "+" 按钮可新增列
- [ ] 转置视图切换正确，切回恢复
- [ ] CSV 导出生成正确文件（全量数据）
- [ ] `pnpm run lint && pnpm run build` 通过
