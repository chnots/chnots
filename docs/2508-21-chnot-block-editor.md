# 2508-21 chnot block editor

- 将每个 `chnot` 提升为 `chnot-thread`
- 将 `chnot-record` 抽象为 `mdwt-record`
- 每个 `chnot-thread` 可以支持多种类型的 `chnot`

## 问题

- 如何自动拆分 code-mirror
- 后台表设计
- 如何追加/插入 chnot

## 具体逻辑

1. 每个 chnot 的第一个 subblock 的 id 同 chnot tid
2. 之后在每次换行时检查，当前是否满足分块条件
3. 如果满足分块条件，按照分块向后台保存

## SQL 升级脚本

```sql
insert into mdwt_record(otid, tid, content, archor) select meta_otid, tid, content, archor from chnot_record;

insert into mdwt_record(otid, tid, content, archor) select otid , tid, '<PLACEHOLDER>' as content, true from chnot_metadata on conflict do nothing;

insert into chnot_meta(otid, chnot_otid, kind, kind_id, korder, tid) select tid, otid, kind, otid, 0, tid from chnot_metadata where kind = 'mdwt';

```

## 同步时的冲突处理
