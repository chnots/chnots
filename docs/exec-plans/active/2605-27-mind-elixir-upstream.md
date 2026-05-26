# Mind-Elixir: Extract Plugins, Drop Fork, Switch to Upstream

## Background

- `lib/mind-elixir-core` is a fork of `SSShooter/mind-elixir-core` (v5.9.1) maintained in-tree
- The fork added imageControls plugin (image size/style/fullscreen/remove + background color setter) and refactored mouse/gesture handling
- Web only uses `reshapeNode` from the fork-specific additions, and the imageControls plugin is not yet wired into web but should be preserved
- Upstream v5.12.2 now includes `reshapeNode` natively, making the fork unnecessary
- The imageControls plugin only depends on public API (`reshapeNode`, `container`, `map`, types), so it can be extracted as an external plugin in web

## Goals

- Extract imageControls plugin (including background color setter) from fork into `web/src/krate/graph/mind-elixir/` as an external plugin
- Replace workspace dependency on forked `lib/mind-elixir-core` with upstream npm package `mind-elixir@^5.12.2`
- Remove fork directory and workspace config entry
- Migrate deprecated `draggable` option to `editable` and remove unused `locale` option

## Scope

- In scope:
  - Extract `imageControls.ts` + `imageControls.less` from fork to `web/src/krate/graph/mind-elixir/plugins/image-controls.ts` (convert less to CSS module or inline styles)
  - Inline `isTopic` utility as `hackIsTopic` (simple type guard, 1 line) since it's not exported by upstream
  - Import types (`Topic`, `NodeObj`, `MindElixirInstance`) from `mind-elixir` package
  - Wire plugin into `web/src/krate/graph/mind-elixir/index.tsx` via plugins prop
  - `web/package.json`: `mind-elixir` workspace -> npm `^5.12.2`
  - `pnpm-workspace.yaml`: remove `lib/mind-elixir-core`
  - Delete `lib/mind-elixir-core/` directory
  - Remove `draggable` and `locale` from both component files
  - `pnpm install` to resolve new dependency
  - Manual testing of mind map interactions
- Out of scope:
  - Any mind-elixir feature changes or new functionality
  - Updating mind-elixir usage in other parts of the codebase beyond the plugin extraction

## Technical Plan

### Phase 1: Extract Plugin

1. Create `web/src/krate/graph/mind-elixir/plugins/image-controls.ts`:
   - Copy `ImageControls` class and plugin function from fork's `src/plugin/imageControls.ts`
   - Replace `import { isTopic } from '../utils'` with inline `hackIsTopic` type guard
   - Replace internal type imports with `import type { Topic, NodeObj, MindElixirInstance } from 'mind-elixir'`
   - Convert `.less` styles to inline styles or a CSS module (`image-controls.css`)

2. Wire plugin into `web/src/krate/graph/mind-elixir/index.tsx`:
   - Import `imageControls` plugin
   - Pass it in the `plugins` array prop

### Phase 2: Switch to Upstream

3. Edit `web/package.json`: change `"mind-elixir": "workspace:*"` to `"mind-elixir": "^5.12.2"`
4. Edit `pnpm-workspace.yaml`: remove `- lib/mind-elixir-core` entry
5. Delete `lib/mind-elixir-core/` directory
6. Edit `web/src/krate/graph/mind-elixir/index.tsx`:
   - Remove `draggable: true` line (editable already handles this)
   - Remove `locale: "en" as const` line
7. Edit `web/src/krate/graph/mind-elixir/preview.tsx`:
   - Remove `draggable: false` line (editable: false already handles this)
   - Remove `locale: "en" as const` line
8. Run `pnpm install` to install upstream package

### Phase 3: Verify

9. Run TypeScript type check (`pnpm build` or `tsc`)
10. Run dev server and manually test: node creation, drag, zoom, image paste, image controls, background color, preview rendering

## Risks and Decisions

- Risk: Upstream 5.12.2 mouse/gesture behavior differs from fork's rewrite -> Mitigation: manual testing of drag, pinch zoom, node interaction
- Risk: Type changes between 5.9.1 and 5.12.2 may cause TS errors -> Mitigation: run type check after install
- Risk: `isTopic` internal utility not exported by upstream -> Mitigation: inline as `hackIsTopic` type guard
- Decision: Use `^5.12.2` semver range to allow patch updates
- Decision: Convert .less to CSS module (.css) to keep web build simple, no need for less preprocessor

## Acceptance Criteria

- [ ] imageControls plugin works as external plugin (image size, fit style, fullscreen, remove, background color)
- [ ] `mind-elixir` resolves to upstream npm package (not workspace)
- [ ] `lib/mind-elixir-core/` directory no longer exists
- [ ] No `draggable` or `locale` props in mind-elixir component usage
- [ ] TypeScript compiles without errors
- [ ] Mind map renders correctly (create, drag, zoom nodes)
- [ ] Image paste still works (reshapeNode path)
- [ ] Image controls appear on click for nodes with images
- [ ] Background color setter works
- [ ] Mind map preview renders correctly

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/2605-27/mind-elixir-upstream`
- [ ] Create first commit: `task-start: docs/exec-plans/active/2605-27-mind-elixir-upstream.md`
- [ ] Implement in small commits
- [ ] Move file to `docs/done/` after completion
- [ ] Create final commit: `task-done: docs/exec-plans/active/2605-27-mind-elixir-upstream.md`
