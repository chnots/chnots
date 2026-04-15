# Architecture

## Project Overview

Chnots is a note-taking application with Rust backend and React frontend, supporting both browser and Tauri desktop/mobile environments.

### Tech Stack

- **Backend**: Rust with Axum web framework (lib/backend, server)
- **Frontend**: React 19 + TypeScript + Rsbuild (web)
- **Desktop**: Tauri v2 with Rust core (tauri, tauri/src-tauri)
- **Libraries**: chin-tools (Rust utilities), mind-elixir-core (mind maps), @chnots/md-codemirror (editor)

## Project Structure

### Backend (lib/backend)

- `src/krate/<domain>/` - Domain modules (chnot, mdwt, kfile, etc.)
  - `po.rs` - Persistent objects (database entities)
  - `dto.rs` - Request/response DTOs
  - `controller.rs` - Axum route handlers
  - `mapper.rs` - Database operations trait
  - `db.rs` - SQL implementation
  - `sync.rs` - Data sync logic
- `src/mapper/` - Database mapper infrastructure
- `src/model/` - Shared models (KReq, PageRsp, etc.)

### Frontend (web)

- `src/krate/<domain>/` - Domain modules (mirrors backend)
  - `dto.ts` - Request/response types
  - `po.ts` - Persistent object types
  - `service.ts` - API service functions
  - `store.ts` - Zustand store
  - `component/` - React components
- `src/common/component/ui/` - UI component library
- `src/lib/` - Utilities (request, utils, id_util, types)

### Workspace Pattern

- DTOs sync via `make sync-struct` (Rust -> TypeScript)
- Shared types: `TID`, `Varchar<N>`, `DbText`
- Domain isolation: each domain self-contained

### Platform Matrix

|           | Server   | App   |
| --------- | -------- | ----- |
| GNU/Linux | Vanilla  | Tauri |
| Windows   | Vanilla  | Tauri |
| Android   | via App  | Tauri |

## Build & Development Commands

### Root Make Commands

- `make run-web` - Start web dev server (pnpm install && pnpm dev)
- `make run-server-sqlite` - Run server with SQLite
- `make run-server-postgres` - Run server with PostgreSQL
- `make run-tauri-desktop` - Run Tauri desktop app
- `make build-web` - Build web assets (includes lib builds)
- `make build-server` - Build server release
- `make build-tauri-desktop` - Build desktop app
- `make build-tauri-android` - Build Android app
- `make sync-struct` - Sync Rust structs to TypeScript DTOs

## Rust Code Style

**Edition**: 2024 (workspace), 2021 (tauri)

### Imports

- External crates first, then workspace deps, then local modules
- Group imports logically
- Use `use crate::*` for prelude patterns
- Use `pub(crate)` for internal module APIs

### Types & Structs

```rust
// PO (Persistent Objects) - database entities
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotMeta {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,
    pub kspace: Varchar<40>,
    #[gts_type = "i64"]
    pub tid: TID,
}

// DTO - request/response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetaCommitReq {
    pub metas: Vec<ChnotMetaCommitReqData>,
}

// Enum with derive macros
#[derive(Debug, Clone, Sequence)]
pub enum ChnotKind {
    MarkdownWithToent,
    ExcalidrawV1,
}
```

### Naming Conventions

- Structs: PascalCase (e.g., `ChnotMeta`, `ShareAppState`)
- Functions/Methods: snake_case (e.g., `chnot_meta_commit`, `chnot_search`)
- Constants: UPPER_SNAKE_CASE (e.g., `MAX_PAGE_SIZE`)
- Type aliases: PascalCase (e.g., `MapperType`, `KResponse`)

### Error Handling

- Use `anyhow::Error` with `anyhow!` and `bail!` macros
- `AResult<T>` = `anyhow::Result<T>`
- `EResult` = `anyhow::Result<()>`
- Use `?` operator for propagation

### Async Patterns

- Use `#[tokio::main]` for async entry points
- All handler/controller functions are async
- Use `State<ShareAppState>` for dependency injection
- Axum extractors: `Json<Req>`, `HeaderMap`, `State`

### Database Patterns

- Use `chin-sql` ORM-like framework
- Implement `Curd` trait for POs
- Use `GenerateTableSchema` derive macro
- Mapper trait defines domain operations
- Use `expand_mt_branch!` macro for multi-database support

### Controller Conventions

- All endpoints use POST method
- Naming format: `/api/v-<ver>/<resource>-<operation.noun>`
- Operation nouns:
  - Fetch one: `-fetch`
  - Fetch many: `-list`
  - Overwrite: `-commit`
  - Delete: `-archive`

### DAO Layer Conventions

- `po_` prefix: direct PO read/write operations
  - query: must search by primary key, return a flat container
  - upsert: must write complete PO objects
- `*_` prefix: non-PO data operations

## Testing

### Rust Tests

- `cargo test` - Run all tests
- `cargo test test_name` - Run specific test
- Test files: `tests/*.rs` or `mod.rs` with `#[cfg(test)]`
- Use `#[tokio::test]` for async tests

## Important Notes

- **No root package.json** - Run commands in respective directories
- **Tauri compatibility** - Code works in both browser and Tauri
- **Multi-database** - SQLite and PostgreSQL supported
- **Git hooks** - Pre-commit/post-commit hooks configured
- **Rust 2024 edition** - Use modern Rust features
- **Tailwind 4.0** - Utility-first CSS with Emotion
- **Biome** - Linter/formatter (replaces ESLint/Prettier)
