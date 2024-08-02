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
