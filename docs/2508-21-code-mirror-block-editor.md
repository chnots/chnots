# 2508-21 模块 codemirror 编辑器

- 引入 subchnot 逻辑
- 应该以 Heading 为区分, 一个 heading 一个 block, 方便标识 id 等内容.
- 可以在每个 block 后面追加 toent 等实例信息.
- 在新建 block 时, 自动切分 block, 并 focus 新的 block.
- 每个 block 后面展示新增按钮

## 难点

- 如何自动拆分 code-mirror
- 后台表设计

## 具体逻辑

1. 每个 chnot 的第一个 subblock 的 id 同 chnot tid
2. 之后在每次换行时检查，当前是否满足分块条件
3. 如果满足分块条件，按照分块向后台保存

## 同步时的冲突处理
