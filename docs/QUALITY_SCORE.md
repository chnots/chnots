# Quality & Testing

## Testing Commands

### Rust

- `cargo test` - Run all tests
- `cargo test test_name` - Run specific test
- Test files: `tests/*.rs` or `mod.rs` with `#[cfg(test)]`
- Use `#[tokio::test]` for async tests

### Frontend

- TBD

## Code Quality Tools

- **Biome** - Linter/formatter (replaces ESLint/Prettier)
- **rustfmt** - Rust code formatting
- **Pre-commit hooks** - Auto-format before commit (prettier + rustfmt)

## Development Workflow

1. Create `docs/exec-plans/active/yymm-dd-task-desc.md` with task description.
2. Branch: `feat/yymm-dd-task-desc` with `task-init` commit.
3. Update plan file as implementation progresses.
4. On completion: move plan to `docs/done/`, create `task-done` commit.

## Type Safety

- DTOs synced from Rust -> TypeScript via `make sync-struct`
- Shared types: `TID`, `Varchar<N>`, `DbText`
- Zod for runtime validation on frontend
