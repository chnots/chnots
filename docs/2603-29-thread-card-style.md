# 优化 ChnotThread 展示效果

## Background

- ChnotThread 当前的展示是简单的纵向堆叠，每个 block 之间仅用水平分割线分隔，视觉层次不够清晰
- 用户希望参考 Twitter 的卡片风格，让每个 chnot block 有更好的视觉边界和交互体验
- 需要在不改变数据流和业务逻辑的前提下，仅优化 UI 呈现

## Goals

- 每个 chnot block 使用全包裹的圆角边框卡片样式
- 操作按钮（Eye、Trash2 等）改为 hover 时才显示，减少视觉噪音
- block 内部的添加按钮（Link / Plus）移到外部（thread 层级），不污染单个 block
- 使用主题变量（`border-border`、`bg-card`、`text-muted-foreground` 等）替代硬编码灰色
- 保持 drag-and-drop 排序功能正常工作

## Scope

- In scope:
  - `thread/block.tsx` — 单个 block 的卡片样式和工具栏布局
  - `thread/index.tsx` — thread 容器宽度、标题区样式、底部操作按钮
- Out of scope:
  - block 内部的子组件渲染（RichMdwt、Excalidraw 等）
  - 数据流和 API 调用逻辑
  - fullscreen 模式的展示

## Technical Plan

1. **block.tsx 卡片化**：外层 div 从 `border-b border-border` 改为 `rounded-lg border border-border bg-card my-1`，形成独立卡片
2. **block.tsx 工具栏优化**：Eye、Trash2 等操作按钮加 `opacity-0 group-hover:opacity-100`，hover 时才显示
3. **block.tsx 移除内部添加按钮**：删除 block 内的 Link/Plus 按钮，相关 props 和 import 一并清理
4. **index.tsx 容器样式**：保持 `max-w-2xl` 紧凑宽度，标题区 `border-b border-border` 分隔，底部保留 Link/Add 操作按钮

## Risks and Decisions

- Risk: 卡片 `my-1` 间距在 drag 时可能产生视觉跳动 → Mitigation: drag overlay 已有独立样式，间距固定为 4px 影响不大
- Decision: 使用 `bg-card` 而非 `bg-background`，让卡片与底色有微妙的层次区分
- Decision: 不添加左侧头像/时间线 rail 布局，保持原始纵向 flex 结构

## Acceptance Criteria

- [ ] 每个 chnot block 呈现为独立的圆角卡片，有边框和背景色
- [ ] 操作按钮（Eye、Trash2）在鼠标悬停时才出现
- [ ] 每个 block 内部不再有 Link/Plus 添加按钮
- [ ] thread 底部保留 Link / Add 操作按钮
- [ ] drag-and-drop 排序功能正常
- [ ] `npm run check` 和 `npm run build` 通过

## Execution Checklist

- [x] 确认方案并实现 block.tsx 卡片样式
- [x] 确认方案并实现 index.tsx 容器样式
- [x] 清理未使用的 import 和参数
- [x] lint 和 build 通过
