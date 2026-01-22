# RailBreaker Design Document

## Overview
RailBreaker is a desktop handicapping app built with Tauri (Rust backend) and Vue 3 (frontend).
It ingests Brisnet single-file racecard data (zipped .DRF), transforms it into a
structured racecard model, and renders a detailed, printable card. The app targets offline usage
with local files and lightweight persistence.

## Goals
- Provide a responsive, information-dense racecard UI for handicapping.
- Support loading Brisnet single-file data from .zip or previously processed .json.
- Enable printing of full or selected races with consistent layout.
- Persist small bits of local state (window size/position, last directory).

## High-level Architecture
- Frontend: Vue 3 + Pinia + Vite.
- Backend: Tauri commands in Rust, plus local file I/O helpers.
- Data flow: Files are selected in the UI, processed by Rust, then rendered by Vue.

```
User -> Menu Action -> Vue (listen) -> Tauri invoke -> Rust command
     -> JSON result -> Racecard model -> UI components
```

## Key Components

### Frontend (Vue)
- `src/views/HomeView.vue`: main racecard viewer, menu event listeners, load flows.
- `src/views/PrintView.vue`: print-only view with WebviewWindow and auto-close.
- `src/components/racecard/*`: display of header, race details, horses, and side menu.
- `src/components/ui/*`: dialogs, loaders, tooltips, and modal panels.
- `src/stores/*`: Pinia stores for global state and config state.
- `src/utils/openPrintWindowEvent.ts`: opens the print window and delivers payload events.
- `src/utils/computePrimePowerComparisons.ts`: derives tiered color badges for Prime Power.

### Backend (Rust, Tauri)
- Commands live in `src-tauri/src/commands/*` and are exposed via `invoke_handler`.
- File helpers in `src-tauri/src/files.rs` provide atomic JSON writes.
- Data models in `src-tauri/src/models/*` mirror the frontend racecard structure.
- App setup in `src-tauri/src/lib.rs` configures menus, state, and window behavior.

## Data Model
The primary domain object is `Racecard` which contains metadata and a list of `Race` entries.
Each `Race` includes the fields and a list of `Horse` entries, with nested past performances and
trainer stats. Models are duplicated in:
- `src-tauri/src/models/racecard.rs` (Rust)
- `src/models/racecard.ts` (TypeScript)

The Rust parser converts Brisnet single-file fields to a structured model and then serializes to
JSON with camelCase keys for frontend alignment.

## File & State Persistence
- `Racecards/`: created in the app working directory for extracted .DRF files.
- `config.json`: stored in the app working directory, contains last directory and window geometry.
- `tracks.csv`: static track metadata loaded at startup.

Persistence is designed for local-only use with minimal state and no migrations.

## Core Flows

### Open .json Racecard
1) User chooses "Open Racecard..." from the menu.
2) `HomeView` receives `menu-open` and calls `load_racecard_file`.
3) Rust reads and parses JSON, converts keys to camelCase.
4) Frontend constructs `Racecard` and adds it to the in-memory `Racecards` list.

### Open .zip (Brisnet Single-File)
1) User chooses "Open Zip...".
2) `process_zip_file` extracts a single .DRF into `Racecards/`.
3) `process_racecard_file` parses .DRF, constructs the Racecard model, deletes the .DRF,
   then writes a .json version to disk and returns it.
4) UI renders the returned racecard.

### Print Racecard
1) User chooses "Print Racecard..." (enabled only when a racecard is loaded).
2) UI shows `PrintDialog` for race selection.
3) `openPrintWindowAndSendPayload` creates or reuses an offscreen `WebviewWindow` at `#/print`.
4) `PrintView` listens for payload, renders selected races, calls `window.print()`,
   then closes and destroys the print window.

## UI/UX Design
- Main view emphasizes dense, scannable race and horse info with strong typographic hierarchy.
- Sliding left panel (`RacecardSideMenu`) lists races and supports multiple loaded cards.
- Tooltips and hover states surface deeper data (extended comments, classifications).
- Print layout uses a fixed letter format with per-race pagination.

## Error Handling and Resilience
- Rust commands return `Result<_, String>` with contextual error messages.
- UI displays a modal error dialog on load/parse failures.
- Print window uses timeouts and localStorage caching to avoid stuck prints.
- Config writes are atomic with a mutex and temp file rename fallback.

## Security and Trust Boundaries
- File access is user-driven via file picker or local storage.
- Zip extraction enforces single-file archives and validates .DRF extension.
- No network access, no external APIs, and no untrusted remote content.

## Performance Considerations
- Parsing is done in Rust to avoid large data handling in the UI thread.
- Print rendering is in a separate window to isolate layout and avoid blocking the main UI.
- Racecards are kept in memory to allow quick switching between loaded cards.

## Observed Limitations
- Racecards are not persisted as a session list; only current in-memory list is kept.
- No built-in analytics beyond display and Prime Power comparisons.
- Print flow is tightly coupled to a Tauri offscreen window and localStorage cache.

## Future Work (from current roadmap)
- Add notes and annotations for races and horses.
- Add analytical features derived from racecard data.
- Extend print selection and formatting controls if needed.

