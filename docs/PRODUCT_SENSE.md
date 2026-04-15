# Product Sense

## Core Concepts

### Chnot

`Knot` 的改写，本意是指本软件中所有的笔记内容都是一个节点，但是 `Knot` 实在太普遍了，所以做了一个变体。

### Toent

**To**do and Ev**ent** 的统一体。

标记格式：

```
# [TODO] 买点东西
; ID: item-id
; EVENT: 2025-12-02 12:00:00 ,12d **12d =2025-12-30
; STATE: TODO @ 2025-05-05 12:00:00 +8:00
;; NOTE: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
; STATE: DOING @ 2025-05-05 12:00:00 +8:00
; STATE: DONE @ 2025-05-05 15:00:00 +8:00
```

### Kspace

Workspace 在 `Chnots` 中的表达。

### KTab

Table 在 `Chnots` 中的表达。

## Feature Domains

See `docs/features/` for detailed feature specs:

- `chnot` - Core note entities
- `toent` - Todo/event management
- `mdwt` - Markdown writing
- `kfile` - File management
- `ktab` - Table management
- `graph` - Graph visualization
- `llmchat` - LLM conversation

## Implementation Specs

See `docs/impl-spec/` for implementation details.
