# Design Docs Index

## Verified Design Documents

| Document | Location | Status |
|---|---|---|
| Architecture | `docs/ARCHITECTURE.md` | Active |
| Frontend Guide | `docs/FRONTEND.md` | Active |
| Product Concepts | `docs/PRODUCT_SENSE.md` | Active |
| Reliability & Sync | `docs/RELIABILITY.md` | Active |
| Quality & Testing | `docs/QUALITY_SCORE.md` | Active |
| Security | `docs/SECURITY.md` | Active |
| DB Schema | `docs/generated/db-schema.md` | Active |
| Release & Deploy | `docs/product-specs/release.md` | Active |
| Consistent Naming | `docs/product-specs/consistent-naming.md` | Active |

## Feature Specs

All feature specs are in `docs/features/`:

- `chnot.md`, `toent.md`, `mdwt.md`, `kfile.md`, `ktab.md`, `graph.md`, `llmchat.md`

## Implementation Specs

All impl specs are in `docs/impl-spec/`.

## Agent-First Operating Principles

1. **Immutability by default** - Syncable data is insert-only; mutations produce new records.
2. **Domain isolation** - Each krate domain is self-contained (po, dto, controller, mapper, db, sync).
3. **Type sync** - Rust structs are the source of truth; TypeScript DTOs are generated.
4. **LAN-first sync** - No central server; AP-oriented eventual consistency.
5. **History retention** - All edits are preserved in `hist` tables.
