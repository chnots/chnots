# Reliability

## Database Design Principles

- **History**: All edit history is retained.
- **Immutability**: Syncable data cannot be updated (UPDATE) after insertion, only moved (MOVE).

## Table Categories

| | OTID Table | SID Table | LOCAL Table |
|---|---|---|---|
| Characteristic | Has one or more `OTID` fields as primary key | Has an `SID` field as digest ID of main content | Works locally only |
| Sync | Yes, with `~_hist` table | Yes, ignore on update | No |
| Mutability | Immutable (sync constraint) | Immutable (primary key constraint) | Mutable |

## Database Support

- Top-level via `KDb` for database access
- ORM: Custom `chin-sql`, including struct-to-CRUD derive macros
- Supported databases:

| Database | Dependency |
|---|---|
| PostgreSQL | tokio-postgres |
| SQLite | actor-sqlite + rusqlite |

## Key Field Definitions

- **TID**: Records insertion time. 13-digit timestamp + 3-digit serial. Assumed no more than 1000 records per millisecond for a personal knowledge base.
- **OTID**: Records which metadata group this record belongs to. Same type as TID.
- **SID**: Digest ID of the record's main content.

## Sync Logic

### File Export

Export by TID ordering, relatively straightforward.

### Network Sync (LAN P2P)

Chnots uses LAN instance-to-instance point-to-point sync without a central server. The overall goal is AP-oriented: prioritize independent operation per node, then converge to eventual consistency via sync.

#### 1. Remote Nodes

- Local maintains a set of sync endpoints (`ip:port`).
- Sync executes serially per endpoint.

#### 2. Sync Scope

- `LOCAL` tables do not participate.
- `OTID` tables are the primary sync target, incremental merge per table.
- `SID` tables are synced as needed, driven by related `OTID` records.

#### 3. Handshake

Handshake confirms whether both parties allow sync and negotiates the sync window start point.

- Same `instance_id` -> reject (avoid self-sync).
- `DB_VERSION` must match exactly (strict equality, not semver patch).
- Both sides query `SyncLogTransient` for the latest sync log per `table_name + remote_id`.
- Never synced -> start = 0.
- Has log -> use latest log, check for overlapping sync records after it:
  - Overlap exists -> rewind to earliest `start_tid_ex` of overlapping records.
  - No overlap -> use last `end_sync_in`.
- Initiator takes `min(local_sync_time, remote_sync_time)` as final start.

#### 4. Time Window

- Per round, per `OTID` table.
- Start: `start_ex` from handshake. End: `TID::now()` at round start, recorded as `end_in`.
- Window semantics: `(start_ex, end_in]`.

#### 5. OTID Comparison Table

Local temporary table with fields: `TID`, `LSTATE`, `RSTATE`.

States:
- `0 = Absent`
- `1 = Cur`
- `2 = Hist`

Build process:
1. Write local main table `(tid, 1, 0)` and history table `(tid, 2, 0)` within window.
2. Paginate remote main + history `tid` lists, upsert to temp table (update `RSTATE` only).
3. Page order: main first, then history; each page sorted by `tid asc`.

#### 6. Sync Actions by State

| LSTATE | RSTATE | Action |
|---|---|---|
| 0 | 1 | Pull main record from remote |
| 0 | 2 | Pull history record from remote |
| 1 | 0 | Push main record to remote |
| 2 | 0 | Push history record to remote |
| 1 | 2 | Local omit: move current to history |
| 2 | 1 | Notify remote omit |

- **Push**: Send complete record directly.
- **Pull**: Send `tid` request, receive complete record.
- **Omit**: Move current version out of main table (not physical delete).

#### 7. Merge Rules

- Target missing primary key -> direct insert.
- History table sync -> `insert ignore` (skip if exists).
- Main table with existing primary key -> compare `tid`:
  - Remote `tid` smaller -> remote record as old version to history.
  - Remote `tid` larger -> remote record to main, original to history via `po_otid_commit`.
  - `tid` equal -> ignore.
- Protocol: last-writer-wins by `tid`, preserve old versions to `hist` table.

#### 8. SID Table Sync

- Not part of LSTATE/RSTATE comparison.
- Handled by associated workers during OTID sync.
- `SID` data: insert-primary, ignore on conflict.

#### 9. Sync Log

After each table sync, both sides write `SyncLogTransient` with:
- `remote_id`, `table_name`, `start_tid_ex`, `end_sync_in`, `sync_finish_tid`

Used for next handshake start point calculation.

#### 10. Current Implementation Characteristics

- Per-table serial sync, not a global transaction.
- Window fixed at round start `end_in`; new data during sync goes to next round.
- Temp comparison table recreated via `drop table if exists` before each sync.
