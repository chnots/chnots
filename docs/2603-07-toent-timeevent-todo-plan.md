# Toent TimeEvent Todo Completion Plan

## Background

- `docs/features/toent.md` defines TODO + EVENT syntax and expected behavior for recurring events, alerts, and end conditions.
- `lib/backend/src/krate/toent/logic/timeevent/` still has multiple `todo!()` placeholders and compile blockers.
- Current decision: do not use `TimeEventSegs`; `TimeEvent::guess` should parse directly from the full `Words` input.

## Goals

- Remove all `todo!()` in `lib/backend/src/krate/toent/` related to time event parsing and timestamp conversion.
- Make `TimeEvent::guess` infer segments directly from `Words` without introducing `TimeEventSegs`.
- Restore compilability of the toent logic path and keep behavior aligned with `docs/features/toent.md`.

## Scope

- In scope:
  - `lib/backend/src/krate/toent/logic/timeevent/mod.rs`
  - `lib/backend/src/krate/toent/logic/timeevent/timeenum/base.rs`
  - `lib/backend/src/krate/toent/logic/timeevent/timeenum/westen.rs`
  - `lib/backend/src/krate/toent/logic/timeevent/timeenum/chinese.rs`
  - Existing unit tests under the above modules (adjust/add minimal tests if needed).
- Out of scope:
  - Frontend behavior/UI changes.
  - Database schema and `toent` query API contract changes.
  - Non-toent module refactors.

## Technical Plan

1. Replace `TimeEvent::guess` segmentation flow with direct token scanning on full `Words`.
   - Use marker-driven state machine to split into base/alert/interval/interval_end/end slices.
   - Parse each slice with existing `TimeEnum`, `TimeInterval`, `EndCondition`, and `RepeatType` logic.
2. Complete `TimeEvent::try_from_standard` with the same marker strategy (without `TimeEventSegs`) and map all fields (`base`, `interval`, `interval_end`, `alert`, `end_cond`).
3. Fix `TimeEvent::standard_string` to serialize current struct fields consistently (remove stale `reminder` usage).
4. Implement missing timeenum logic:
   - `WesTime::is_valid`
   - `WesTime::to_utc_timestamp`
   - `ChnTime::to_utc_timestamp`
   - `convert_time_to_secs` remaining units.
5. Validate via `cargo check` and focused tests for toent timeevent parsing.

## Risks and Decisions

- Risk: marker parsing ambiguity between `*`, `*=`, and `=` can mis-segment tokens.
  - Mitigation: enforce deterministic precedence (`*=` before `*`, and exact prefix checks).
- Risk: Chinese lunar conversion boundary conditions may fail for out-of-range dates.
  - Mitigation: propagate conversion errors with clear `anyhow` context and keep fallback behavior explicit.
- Decision: keep existing data structures and parser entry points; only fill missing logic and remove stale references.
- Decision: avoid introducing `TimeEventSegs`; use local helper functions returning tuple/slices only.

## Acceptance Criteria

- [ ] No `todo!()` remains under `lib/backend/src/krate/toent/logic/timeevent/`.
- [ ] `TimeEvent::guess` no longer references or depends on `TimeEventSegs`.
- [ ] `cargo check` passes for workspace/backend build path.
- [ ] Existing toent timeevent tests pass; if gaps exist, add minimal tests for marker segmentation and UTC conversion.
- [ ] Parsing/serialization path supports examples from `docs/features/toent.md` for TODO + EVENT core syntax.

## Execution Checklist

- [ ] Confirm the plan is executable with user.
- [ ] Create branch `feat/2603-07/toent-timeevent-todo-plan`.
- [ ] Create first commit: `task-start: docs/2603-07-toent-timeevent-todo-plan.md toent timeevent todo completion plan`.
- [ ] Implement in small commits.
- [ ] Move file to `docs/done/` after completion.
- [ ] Create final commit: `task-done: docs/done/2603-07-toent-timeevent-todo-plan.md toent timeevent todo completion plan`.
