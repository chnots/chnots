# 2508-13

- `This sometimes happens because multiple instances of @codemirror/state are loaded, breaking instanceof checks.`
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
