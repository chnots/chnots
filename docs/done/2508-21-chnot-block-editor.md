# 2508-21 chnot block editor

- 新增新的 `chnot` 类型，为 `Thread`
- 每个 `chnot-thread` 可以支持多种类型的 `chnot` 线性排列

## 问题

- 如何自动拆分 mdwt
  不自动拆分 mdwt，暂时使用手动添加的方式
- 后台表设计
  引入 `chnot_thread_order` 表，记录了 `thread_id` 和 `chnot` 的关系，同时 `chnot-thread` 作为普通 `chnot` 存储在 `chnot_meta` 表中
- 如何追加/插入 chnot
  - 在 thread 页面内编辑
    仅允许直接编辑 mdwt 及 table，其他类型需要全屏后在进行编辑，thread 页面中只支持查看
  - 插入 chnot 逻辑
    可以搜索其他 chnot 插入到当前 thread 中

## 具体逻辑
1. 每个 chnot 的第一个 subblock 的 id 同 chnot tid
2. 之后在每次换行时检查，当前是否满足分块条件
3. 如果满足分块条件，按照分块向后台保存


## 同步时的冲突处理
和 chnot 一样，使用 otid 逻辑进行处理
