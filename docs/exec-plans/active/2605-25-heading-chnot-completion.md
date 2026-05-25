# Plan: Heading Completion for Child Chnot Creation

## Context

Currently in mdwt editor, typing `## ` triggers `otidInjector` which auto-injects `[[otid]]` into headings. We want to intercept this moment with a completion popup that lets users create new chnots of various kinds or reference existing ones. If dismissed, the existing otidInjector behavior remains.

## Trigger Flow

1. User types `#` through `######` + space
2. Completion popup appears (custom rendered, using `@codemirror/autocomplete`)
3. User navigates with arrow keys / enter → completion interaction
4. User types anything else → completion dismissed, otidInjector injects `[[otid]]` as before

## Popup Layout

Two groups with section headers:

```
── Create New ──
  📝 Markdown (MDWT)
  🎨 Excalidraw
  📊 Table
  📁 File
  🤖 LLM Chat
  🧠 Mind Map

── Reference Existing ──
  🔍 Search existing chnots...
```

### "Create New" Selection

1. Inject `[[newOtid]]` at heading position (`## [[newOtid]] `)
2. Call `chnotMetaCommit` async to create meta with correct kind
3. Open `ThreadEditorPanel` (right panel) with the new chnot's editor
4. Backend handles `chnotThreadOrderCommit` when parent mdwt is saved

### "Reference Existing" Flow

1. User selects "Search existing chnots..." → popup content replaces with search input + results
2. Search uses `chnotSearch` API, excludes:
   - Current mdwt's own otid
   - Otids already referenced via `[[otid]]` in current content
3. User selects a result:
   - Inject `[[existingOtid]]` at heading position
   - Render the chnot's mdwt content and kind-specific preview (like thread-item-preview)
   - No ThreadEditorPanel opens
   - No `chnotThreadOrderCommit` call (backend handles on save)

## Implementation

### File Changes

#### 1. `lib/md-codemirror/src/heading-block/heading-completion.ts` (NEW)

Core completion source:

- `headingCompletion(config)` returns a completion function
- Pattern match: `/^#{1,6} $/` at line start, only when no otid present
- Returns two groups of `Completion` options:
  - "Create New": one option per ChnotKind (except ThreadV1), each with `apply` callback
  - "Reference Existing": single option that triggers search mode
- Custom `optionRender` for rich UI (icon + label + section headers)
- State management for search mode: when active, shows search input + results from `chnotSearch`
- Coordination with otidInjector via a shared `StateField` flag

#### 2. `lib/md-codemirror/src/heading-block/otid-inject.ts` (MODIFY)

- Add check for heading completion flag: if a heading completion popup is currently active for a heading, skip injection
- The flag is set by the completion source when it shows options, cleared on dismiss
- This ensures: completion active → otidInjector waits; completion dismissed → next keystroke triggers otidInjector normally

#### 3. `lib/md-codemirror/src/heading-block/index.ts` (MODIFY)

- Export new `headingCompletion` function
- Wire into `headingBlocks()` config

#### 4. `web/src/krate/mdwt/component/mdwt-editor.tsx` (MODIFY)

- Add `headingCompletion` to the `autocompletion({ override: [...] })` array
- Pass necessary context: parent otid, existing otids in content, kind icons
- Pass callbacks: `onCreateChnot` (creates meta + opens panel), `onRenderPreview` (for exists)

#### 5. `web/src/krate/chnot/component/rich-chnot/mdwt.tsx` (MODIFY)

- Provide `onCreateChnot` callback to the editor:
  - `genTID()` for new otid
  - `chnotMetaCommit` to create meta
  - Set `selectedItem` to open `ThreadEditorPanel`
- Provide parent otid and content ref for exclude list

### Key Reuse

| Existing code | Used for |
|---|---|
| `ChnotKindIcon` (`web/src/krate/chnot/component/kind-icon.tsx`) | Icons in popup options |
| `chnotMetaCommit` (`web/src/krate/chnot/service.ts`) | Create new chnot meta |
| `chnotSearch` (`web/src/krate/chnot/service.ts`) | Global search for exists |
| `ThreadEditorPanel` (`web/src/krate/mdwt/component/thread-editor-sheet.tsx`) | Open editor for new chnots |
| `ThreadItemPreview` (`web/src/krate/mdwt/component/thread-item-preview.tsx`) | Render preview for exists |
| `HEADING_OTID_RE` (`lib/md-codemirror/src/heading-block/block-model.ts`) | Detect existing otid references |
| `otidInjector` (`lib/md-codemirror/src/heading-block/otid-inject.ts`) | Fallback injection |

### Completion ↔ otidInjector Coordination

```
User types "## "
  → docChanged fires
  → otidInjector.update() runs, finds heading without otid
  → checks headingCompletionActive flag → TRUE (completion showing) → SKIP
  → completion source fires, shows popup

User presses arrow/enter:
  → completion.apply() fires → inject otid, create chnot, open panel
  → heading already has otid, otidInjector ignores it on next pass

User types a letter:
  → completion dismissed
  → headingCompletionActive flag cleared
  → docChanged fires again
  → otidInjector finds heading without otid, flag is FALSE → INJECTS [[otid]]
```

## Verification

1. Type `## ` → popup appears with two groups
2. Arrow down to "Excalidraw", Enter → `[[otid]]` injected, right panel opens excalidraw editor
3. Type `## `, type a letter → popup dismissed, `[[otid]]` auto-injected as before
4. Select "Search existing chnots" → search input appears, type query → results show
5. Select an existing chnot → `[[existingOtid]]` injected, preview rendered below heading
6. All heading levels (`#` through `######`) behave the same
7. ThreadV1 does NOT appear in "Create New" list
8. Self-otid and already-referenced otids excluded from search results
