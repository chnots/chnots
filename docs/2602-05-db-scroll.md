# 数据库维护方案

去掉目前使用的每次启动时使用程序创建表的方案，使用 `v<x>.<y>.sql` 脚本的方案。

在数据库中维护一个 `db_version_transient` 表，用来维护当前的数据库版本。

在启动时查询库中的最大版本与程序中的版本比较，如果小的话，升级。如果大的话拒绝启动。

## 实现细节

### 表结构

`db_version_transient` 表用于记录数据库的当前版本：

```rust
pub struct DbVersionTransient {
    #[gts_primary]
    pub version: i64,    // 数据库版本号
    #[gts_type = "i64"]
    pub tid: TID,        // 时间戳
}
```

### 嵌入式 SQL 文件

使用 `rust-embed` 将 SQL 迁移文件编译到二进制文件中，避免运行时依赖外部文件系统：

```rust
#[derive(RustEmbed)]
#[folder = "../../data/sqls"]
struct MigrationSqls;
```

### API 接口

- `ensure_table_db_version()` - 确保 `db_version_transient` 表存在
- `get_current_version()` - 获取数据库当前版本
- `set_version(version)` - 设置数据库版本
- `migrate_db(program_version)` - 执行数据库迁移
- `read_migration_sqls()` - 从嵌入文件中读取迁移 SQL

### 使用方式

```rust
let program_version = 1;

kdb.migrate_db(program_version).await?;
```

### 迁移文件规范

- 文件位置：`data/sqls/`
- 文件命名：`v<x>.<y>.sql`（如 `v1.0.sql`, `v1.1.sql`）
- 文件内容：SQL 语句，多个语句用 `;` 分隔
- **文件会在编译时嵌入到二进制文件中**

### 迁移逻辑

1. 检查 `db_version_transient` 表是否存在，不存在则创建
2. 读取数据库当前版本，默认为 0
3. 比较数据库版本与程序版本：
   - 如果 **数据库版本 > 程序版本**：拒绝启动，报错提示需要升级应用
   - 如果 **数据库版本 == 程序版本**：无需迁移
   - 如果 **数据库版本 < 程序版本**：执行迁移升级
4. 执行迁移：
   - 从嵌入的 SQL 文件中读取迁移脚本
   - 按版本号升序执行迁移脚本
   - 每执行一个版本脚本后，更新 `db_version_transient` 表
5. 验证迁移后的版本是否匹配程序版本

### 版本号格式

- 使用 `i64` 类型存储版本号