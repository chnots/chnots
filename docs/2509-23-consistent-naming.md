# 规范化命名

## Controller 层

- 全部使用 Post 进行请求
- 接口命名格式为
  `/api/v-<ver>/<resource>-<operation.noun>`
- `<operation.noun>`
  - 必须为名词
  - `获取（一个）`: -fetch
  - `获取（多个）`: -list
  - `覆盖`: -commit
  - `删除`: -archive

## DAO 层

- `po_` 代表直接查写 po 数据
  - query 方法必须是以主键进行搜索，返回一个容器，不能再套更多的层
  - upsert 方法必须是以完整的 po 对象进行写入（一个或者多个）
- `*_` 代表写非 po 数据
