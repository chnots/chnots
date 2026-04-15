# Security

## API Security

- All endpoints require appropriate authentication (via Axum middleware)
- Request validation through typed DTOs (Deserialize + validation)

## Data Safety

- Syncable data is immutable after insertion (no UPDATE, only MOVE)
- History retained for all edits
- LAN P2P sync only (no central server exposure)
- Instance ID check prevents self-sync

## Build Security

- Pre-commit hooks enforce code formatting
- No secrets in repository (use environment variables)
- `DB_VERSION` strict matching during sync handshake
