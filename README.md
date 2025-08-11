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

### File Page

<p align="center">
  <img src="./docs/_asset/kfile.png">
</p>

### Table View Page

<p align="center">
  <img src="./docs/_asset/ktab.png">
</p>

### LLMChat Page

<p align="center">
  <img src="./docs/_asset/llmchat.png">
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

- [Database Design(Chinese)](./docs/database.md)
- [Package And Release(Chinese)](./docs/release.md)

## Quick Start

```shell
mkdir chnots
git clone https://github.com/chnots/chnots.git
cd chnots

make init
make build-web
make run-server-sqlite
```

## Credits

- **Jolpin**: for its wonderful markdown and codemirror utils.
- **Excalidraw**: the powerful drawing tools.
- **tanstack-table**: the powerful table.
