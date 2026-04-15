# Design Documents

## Index

This directory contains design documentation for Chnots.

### Core Design Decisions

- **Database Design**: See `docs/RELIABILITY.md` and `docs/generated/db-schema.md`
- **API Conventions**: See `docs/ARCHITECTURE.md` (Controller Conventions, DAO Layer)
- **Sync Protocol**: See `docs/RELIABILITY.md` (Network Sync)
- **Frontend Architecture**: See `docs/FRONTEND.md`

### Feature Design Docs

Feature-level design docs are in `docs/features/`:

- `chnot.md` - Core note entity design
- `toent.md` - Todo/Event unified model
- `mdwt.md` - Markdown writing
- `kfile.md` - File management
- `ktab.md` - Table management
- `graph.md` - Graph visualization
- `llmchat.md` - LLM conversation

### Implementation Specs

Detailed implementation specs are in `docs/impl-spec/`.

## Core Beliefs (Agent-First)

1. **Immutability by default** - Syncable data is insert-only; mutations produce new records.
2. **Domain isolation** - Each krate domain is self-contained (po, dto, controller, mapper, db, sync).
3. **Type sync** - Rust structs are the source of truth; TypeScript DTOs are generated.
4. **LAN-first sync** - No central server; AP-oriented eventual consistency.
5. **History retention** - All edits are preserved in `hist` tables.
