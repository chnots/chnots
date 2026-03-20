# Toent 的实现思路
## Toent 的写入
- 当更新 mdwt 时，自动更新 `toent_defi` 表
  - 计算所有 `time_event` 的开始和结束条件（时间，次数），取并集
  - 判断该 `mdwt` 有没有 `todo_event`
  - 将上面两项内容写入到 `toent_defi` 中

- 内存维持
  - 前一天晚上 23:59 或者当天启动时抓取今天的 toent (start_tid <= today_start || end_tid >= today_end) 记录到 app_state 的字段中
  - 如果 mdwt 更新，也要获取今天的 toent，并覆盖内存中该 `otid` 未持久化的事件

- 写入数据库
  - 当一个 toent 修改时，记录到数据库

## Toent 的读取
- 读取内存和搜索区间有交集的内容
- 根据 toent_defi 和内存中的进行取并集
- 读取数据库中在搜索区间中的内容，对内存中的内容进行 overwrite