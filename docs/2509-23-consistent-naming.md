# 规范化命名

- 全部使用 Post 进行请求
- 接口命名格式为
  `/api/v-<ver>/<resource>-<operation.noun>`
- `<operation.noun>`
  - 必须为名词
  - `获取`: -fetch
  - `列表`: -list
  - `覆盖`: -commit
  - `删除`: -archive
