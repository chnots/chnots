# Database Schema Reference

> Auto-generated reference. For design principles and sync logic, see `docs/RELIABILITY.md`.

## Table Categories

| Category | Primary Key | Sync | Mutable |
|---|---|---|---|
| OTID Table | One or more `OTID` fields | Yes (with `~_hist`) | No |
| SID Table | `SID` (digest of main content) | Yes (ignore on update) | No |
| LOCAL Table | Varies | No | Yes |

## Key Field Types

| Field | Type | Description |
|---|---|---|
| `TID` | 16-digit int (13 ts + 3 seq) | Insertion timestamp + sequence |
| `OTID` | Same as TID | Metadata group identifier |
| `SID` | String digest | Content hash for deduplication |

## Supported Databases

| Database | Dependency |
|---|---|
| PostgreSQL | tokio-postgres |
| SQLite | actor-sqlite + rusqlite |

## ORM Framework

- Custom `chin-sql` framework
- `GenerateTableSchema` derive macro for auto DDL
- `Curd` trait for standard CRUD operations
- `expand_mt_branch!` macro for multi-database support
