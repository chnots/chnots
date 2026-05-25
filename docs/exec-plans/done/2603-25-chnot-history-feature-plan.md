# Chnot History Feature Plan


## Goal

Add history browsing and version apply capability for `mdwt`, `excalidraw`, `mindmap`, and `kfile` in Chnot editing flows.

## Scope

- Add a `History` entry point in the headbar right side.
- Clicking `History` opens a version list with:
  - version time
  - a `View` action
- Clicking `View` loads selected historical data into main editor area as read-only preview.
- In preview mode, show top-right actions:
  - `Apply`: apply viewed historical version to latest
  - `Latest`: leave preview mode and switch back to latest version

## Display Rules

- Do not show history button on thread page in normal embedded mode.
- Show history-related actions only when:
  - single Chnot page, or
  - fullscreen Chnot editor
- Thread block fullscreen is allowed to show history features.

## Backend Plan

For each module (`mdwt`, `graph`, `kfile`):

1. Add history list API by `otid` (all versions from hist table, ordered by `tid` desc).
2. Add history fetch API by `otid + tid` (load specific historical version content).
3. Add history apply API by `otid + tid` (copy selected historical version as latest record).

Notes:

- Reuse existing `_hist` data model and `po_otid_commit` where possible.
- Keep response payloads minimal and consistent for frontend consumption.

## Frontend Plan

1. Extend module services (`mdwt`, `graph/excalidraw`, `graph/mind-elixir`, `kfile`) with history list/view/apply requests.
2. Add per-editor history UI state:
   - history list
   - preview version id (`tid`)
   - preview data
   - preview mode flag
3. Register headbar actions from editor components:
   - normal mode: `History`
   - preview mode: `Apply` + `Latest`
4. Ensure preview renders in read-only mode and apply writes selected version back.
5. Respect display rules for single page vs thread/fullscreen.

## Validation

- Verify buttons and behavior on:
  - single Chnot page
  - thread embedded block
  - thread block fullscreen
- Verify each kind (`mdwt`, `excalidraw`, `mindmap`, `kfile`) supports list/view/apply end-to-end.
