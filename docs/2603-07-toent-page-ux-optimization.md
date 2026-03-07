# Toent Page UX Optimization Plan

## Background

- `web/src/krate/toent/component/toent-page.tsx` currently mixes data fetching, state control, and all three views, which raises interaction iteration cost.
- List/week/month views show inconsistent information density, and month view relies on hover preview that is weak on mobile.
- Toent semantics (state/priority/event definitions) already exist in dto/po and backend responses, but are not fully surfaced in UI.

## Goals

- Keep the same feel with Chnot Page
- Improve Toent page interaction flow first, then keep code structure ready for later performance optimization.
- Make list/week/month views consistent in core task information and actions.
- Improve mobile usability for month and day-level schedule operations.

## Scope

- In scope: `web/src/krate/toent/component/toent-page.tsx`, extracted subcomponents under `web/src/krate/toent/component/`, minor style and UX behavior changes.
- In scope: preserve existing APIs (`toentSearch`, `toentInstCount`, `toentTodoStateCommit`) and current backend contracts.
- Out of scope: backend API schema change, new Toent parsing rules, non-Toent module redesign.

## Technical Plan

1. Split `toent-page.tsx` into view-focused components (`toolbar`, `list-view`, `week-view`, `month-view`, `editor-sheet`) while keeping behavior equivalent.
2. Add a filter summary bar and unify card information across list/week/month: title, time, state, priority.
3. Replace month hover preview with click-to-open day agenda panel to support desktop and mobile consistently.
4. Improve state transition feedback and empty states (no data / no search result / load failure) with clearer UI feedback.
5. Keep editor sheet workflow, and add context summary for the selected Toent item to reduce navigation friction.

## Risks and Decisions

- Risk: component splitting can introduce state sync regressions -> Mitigation: keep data source in page-level container and pass typed props down.
- Risk: UX changes may affect users' habits -> Mitigation: preserve existing core operations (open editor, change state, date navigation) and only improve flow.
- Decision: interaction-first delivery in this phase, avoid backend and protocol changes to reduce integration risk.

## Acceptance Criteria

- [ ] List/week/month views expose consistent core fields: title, time, state, and priority.
- [ ] Month view supports click-based day detail interaction usable on mobile and desktop.
- [ ] Status updates provide clear success/failure feedback and do not break current data refresh behavior.
- [ ] Empty and error states are visually explicit instead of only header text hints.
- [ ] Existing Toent APIs and data contracts remain unchanged.

## Execution Checklist

- [ ] Confirm the plan is executable with user.
- [ ] Create branch `feat/2603-07-toent-page-ux-optimization`.
- [ ] Create first commit: `task-init: toent page ux optimization plan.`
- [ ] Implement in small commits.
- [ ] Move file to `docs/done/` after completion.
- [ ] Create final commit: `task-done: toent page ux optimization plan.`
