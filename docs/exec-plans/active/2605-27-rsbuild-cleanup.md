# Rsbuild 构建工具调研与清理

## Background

- 经过 Rsbuild vs Vite 对比调研，决定保持 Rsbuild（Rust-based 更快，已稳定运行，匹配项目 Rust 技术栈）
- 当前存在残留配置需要清理：未使用的 plugin-less、tauri 残留 Vite 脚本

## Goals

- 清理未使用的构建依赖和残留配置
- 统一文档中对构建工具的描述

## Scope

- In scope: 依赖清理、tauri/package.json 清理、CI workflow 注释修正
- Out of scope: 构建工具迁移、rsbuild.config.ts 改动

## Technical Plan

1. `web/package.json`: 移除 `@rsbuild/plugin-less` 依赖（无 .less 文件）
2. `tauri/package.json`: 清除残留 Vite 脚本（dev/build/lint/preview），保留 `tauri` 相关命令
3. `.github/workflows/build.yml`: 修正 L61 注释 `webkitgtk 4.0 is for Tauri v1` → `Tauri v2 uses webkitgtk 4.1`
4. `pnpm install` 锁文件更新

## Risks and Decisions

- Decision: 保持 Rsbuild 不迁移 Vite — Rsbuild 更快、已稳定、匹配 Rust 技术栈
- Risk: tauri/package.json 清理后 tauri dev 可能受影响 → Mitigation: tauri 通过 `make run-tauri-desktop` 调用 `pnpm tauri dev`，不依赖 package.json 的 dev script

## Acceptance Criteria

- [ ] `@rsbuild/plugin-less` 已从 devDependencies 移除
- [ ] tauri/package.json 不再包含 Vite 相关 script
- [ ] CI workflow 注释准确反映 Tauri v2
- [ ] `pnpm install` 成功，`pnpm build` 成功
- [ ] TypeScript 编译无错误

## Execution Checklist

- [ ] Create branch `feat/2605-27/rsbuild-cleanup`
- [ ] Create first commit: `task-start: docs/exec-plans/active/2605-27-rsbuild-cleanup.md`
- [ ] Implement in small commits
- [ ] Move file to `docs/exec-plans/done/` after completion
- [ ] Create final commit: `task-done: docs/exec-plans/done/2605-27-rsbuild-cleanup.md`
