# _Chnots_

<p align="center">
  <img src="./web/public/static/favicon/chnots.svg" alt="chnots logo">
</p>

> Record, Sync, Plan

## Overview

### Chnot Page

<p align="center">
  <img src="./docs/_asset/chnots.png">
</p>

### Drawing Page(Excalidraw)

<p align="center">
  <img src="./docs/_asset/excalidraw.png">
</p>

### Thread Page

<p align="center">
  <img src="./docs/_asset/thread.png">
</p>


## Features

- Workspace Support
- P2P Sync
- Immutable Database
- Postgres/Sqlite Support
- Tag-based Chnot Management
- Todo/Event Management(with Chinese Calendar support)
- Multi-Platform Support(Server & Tauri App)

## Documents

- [Architecture & Build Commands](./docs/ARCHITECTURE.md)
- [Reliability & Sync Protocol](./docs/RELIABILITY.md)
- [Product Concepts](./docs/PRODUCT_SENSE.md)
- [Package And Release(Chinese)](./docs/product-specs/release.md)
- [Design Docs Index](./docs/DESIGN.md)

## Quick Start

```shell
mkdir chnots
git clone https://github.com/chnots/chnots.git
cd chnots

make init-workflow
make build-web
make run-server-sqlite
```

## Credits

- **Jolpin**: for its wonderful markdown and codemirror utils.
- **Excalidraw**: the powerful drawing tools.
- **tanstack-table**: the powerful table.
