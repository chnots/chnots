# 2508-13 将前端转为 monorepo

## 转为 monorepo 的好处

## 工具

pnpm workspace。

## 处理 `@codemirror/state` 的冲突问题。

`This sometimes happens because multiple instances of @codemirror/state are loaded, breaking instanceof checks.`
当把 jolpin 的 codemirror 工具抽出来作为一个包提供时，会出现上面的问题。

尝试把 @codemirror/state 作为 peerDependencies 或 devDependencies 提供时，都有问题。

如果使用 rslib 的话，还应该添加 `autoExternal` 内容。

```typescript
export default defineConfig({
lib: [
  {
    format: 'esm',
    syntax: ['node 18'],
    dts: true,
    autoExternal: {
      dependencies: true,
      optionalDependencies: true,
      peerDependencies: true,
      devDependencies: false,
    },
  },
```

## 引入 `lib` 触发 HMR 的工具

import `@rsbuild/plugin-source-build`

With `@rsbuild/plugin-source-build`, monorepo source code referencing is possible.

So even we modified the dependency code, rsbuild will refresh the page.

Link: https://www.npmjs.com/package/@rsbuild/plugin-source-build
