# 2407-24

- table support: https://github.com/teableio/teable

# 2408-02

## 新组件

- markdown 编辑器: https://uiwjs.github.io/react-markdown-editor/待办:
  - date/backlink/tag 选择器
  - date/backlink/tag 高亮

## 数据库初始化

```sql
podman run -dt --name postgres-240221 -e POSTGRES_PASSWORD=1234 -v "/home/chin/files-ext/others/postgres:/var/lib/postgresql/data:Z" -p 5432:5432 postgres

CREATE DATABASE chnots;
create user chnots with encrypted password 'chnots';

ALTER DATABASE chnots OWNER TO chnots;
GRANT ALL PRIVILEGES ON DATABASE chnots TO chnots;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO chnots;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO chnots;
```

## 数据库结构

> 以下内容以 postgreSQL 库为例，但是鉴于数据库的多样性，所以不会采用一些 PostgreSQL 的高级特性，比如 Array 字段。

1. chnots 主表 -- 用来记录所有的日志

```sql
create table chnots (
    id VARCHAR(40) NOT NULL,
    ring_id VARCHAR(40) NOT NULL,

    content TEXT NOT NULL,
    type VARCHAR(255) NOT NULL,
    domain TEXT NOT NULL,

    delete_time timestamptz DEFAULT NULL,
    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    update_time timestamptz NOT NULL,
    primary key (id)
)
```

2. toent_defi 待办定义列表

```sql
create table toent_defi (
    id varchar(40) not null,
    chnot_id varchar(40) not null,
    active_flag tinyint default 1 comment '待办 flag',
    original_str text not null,
    toent_type tinyint comment '农历或者公历',
    start_time timestamptz,
    toent_time timestamptz,
    end_time timestamptz,
    interval timestamptz,
    toent_interval_type tinyint comment '间隔类型，农历或者公历',
    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    update_time timestamptz NOT NULL
)
```

3. toent_inst 待办实例列表

```sql
create table toent_inst (
    id varchar(40) not null,
    toent_id varchar(40) not null,
    active_flag tinyint default 1 comment '是否启用 flag',
    alert_time timestamptz,
    toent_time timestamptz,
    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    update_time timestamptz NOT NULL
)
```

# 2408-07

## 组件选择修改

之前选择的 @uiw/react-markdown-editor 不太方便使用，查看 @uiw/react-markdown-editor 源码后，决定使用内部依赖的

- @uiw/react-codemirror
- @uiw/react-markdown-view

## 0.0.1 版本

在 0.0.1 版本希望达到以下目标

模仿 memos 界面

可选项有

- [X] Chnot 的命名域
- [ ] Chnot 的修改、回复、删除功能
- [ ] 检索功能
- [ ] 粘贴图片功能
- [ ] 支持标签功能
- [ ] 支持 Toent 功能
- [ ] 支持双向链接功能
- [ ] 刷新页面后保留内容
- [ ] 看板页面
- [ ] 待办页面
- [ ] 点击页面自动保存

## WAIT Chnot 的命名域

针对可见程度，目前只分为 private, work, public 三种。

前端使用假的 store 来实现，要求这个 store 刷新页面后能返回原来的值。

## WAIT 完整使用 zustand?

目前我的场景来看，使用 react-query 貌似有点冗余，我其实只是需要其中的下拉刷新的部分，更多的可能暂时并不需要。

使用 Zustand 替代 react-query，因为我可能更需要自己管理数据。

但是这部分并不急于替换，让我先把整体功能实现完。
