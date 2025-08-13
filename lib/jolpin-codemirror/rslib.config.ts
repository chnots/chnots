import { defineConfig } from '@rslib/core';

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
    {
      format: 'cjs',
      syntax: ['node 18'],
    },
  ],
});
