# Design: Port `dir-compare` GUI to Tauri + TypeScript

## 1. Objective

Replace the current `eframe/egui` GUI crate with a cross-platform Tauri desktop app while preserving existing user-visible behavior and reusing `dir-compare-core` for comparison logic.

## 2. Current State Summary

Current GUI implementation:
- UI stack: `eframe/egui` in `gui/src/main.rs`
- Core integration: direct calls to `dir_compare_core::compare_directories`
- Async model: background thread + `std::sync::mpsc::channel()`
- Features:
  - Directory A/B selection and validation
  - Ignore file selection
  - Comparison method selection (Filename, Filename+Size, FastHash, SampledHash)
  - SampledHash verify-on-match enabled in GUI flow
  - Result tree rendering (A-only, B-only, both)
  - Theme persistence via config file (`dirs` crate)

## 3. Scope

In scope:
- New Tauri app with TypeScript frontend
- Rust command layer that bridges frontend to `dir-compare-core`
- Feature parity with existing GUI
- Test strategy migration
- Packaging/build updates

Out of scope (initial migration):
- New comparison algorithms
- Sync feature or other OpenSpec in-progress changes
- Major UX redesign beyond parity

## 4. High-Level Architecture

```text
+-----------------------------------------------+
|                 Tauri Frontend                |
|        (TypeScript + framework/vanilla)       |
|                                               |
| - Form state                                  |
| - Compare action                              |
| - Tree rendering                              |
| - Theme selection                             |
+------------------------+----------------------+
                         |
                         | invoke() / events
                         v
+-----------------------------------------------+
|              Tauri Rust Backend               |
|                                               |
| Commands:                                      |
| - compare_directories                          |
| - validate_directory                           |
| - pick_folder / pick_file                      |
| - load_theme / save_theme                      |
|                                               |
| Adapters: DTO <-> dir-compare-core types      |
+------------------------+----------------------+
                         |
                         v
+-----------------------------------------------+
|                dir-compare-core               |
| - compare_directories                         |
| - strategy implementations                    |
| - ignore file support                         |
+-----------------------------------------------+
```

## 5. Repository Layout Proposal

Add new crate and frontend assets:

- `tauri-app/`
  - `src-tauri/` (Rust backend)
  - `src/` (TypeScript frontend)
  - `package.json`
  - `tauri.conf.json`

Workspace update:
- Add `tauri-app/src-tauri` as a Cargo workspace member.

Legacy GUI status:
- Keep `gui/` during parity validation.
- Deprecate and remove after acceptance.

## 6. Behavioral Parity Requirements

The Tauri app must preserve these behaviors from existing GUI:

1. Compare button enabled only when:
   - `dir_a` valid directory
   - `dir_b` valid directory
   - no comparison currently in progress
2. `SampledHash` uses `verify_on_match = true` (GUI behavior parity).
3. Ignore file path optional and passed through to compare.
4. Errors displayed as user-facing message.
5. Result sections:
   - Only in A
   - Only in B
   - In Both
6. Footer summary counts:
   - A only, B only, both, total
7. Theme persisted across sessions.

## 7. Rust Backend API Contract (Tauri Commands)

All command payloads are `serde` serializable.

### 7.1 Command: `compare_directories`

Request:
- `dir_a: String`
- `dir_b: String`
- `method: ComparisonMethodDto`
- `ignore_file_path: Option<String>`
- `case_insensitive: bool` (default false)
- `verify_on_match: Option<bool>`

`ComparisonMethodDto`:
- `"filename"`
- `"filename_size"`
- `"fast_hash"`
- `"sampled_hash"`

Response:
- `ComparisonResultDto`
  - `a_only: Vec<EntryDto>`
  - `b_only: Vec<EntryDto>`
  - `both: Vec<EntryPairDto>`
  - `summary: SummaryDto`

`EntryDto`:
- `path: String` (relative)
- `kind: "file" | "directory"`
- `size: Option<u64>`

`EntryPairDto`:
- `a: EntryDto`
- `b: EntryDto`

`SummaryDto`:
- `a_only_count: usize`
- `b_only_count: usize`
- `both_count: usize`
- `total_count: usize`

Errors:
- Return `Result<..., String>` with sanitized message.

### 7.2 Command: `validate_directory`

Request:
- `path: String`

Response:
- `bool`

### 7.3 Commands: `pick_folder`, `pick_file`

Request:
- none

Response:
- `Option<String>` absolute path

### 7.4 Commands: `load_theme`, `save_theme`

`ThemeDto`:
- `"light" | "dark" | "system"`

Behavior:
- Reuse config persistence logic (currently in `gui/src/theme.rs`) with app-specific config directory.

## 8. Frontend Design

## 8.1 State Model

Single source of UI state:
- `dirAPath: string`
- `dirBPath: string`
- `ignoreFilePath: string | null`
- `method: ComparisonMethodDto`
- `isComparing: boolean`
- `errorMessage: string | null`
- `results: ComparisonResultDto | null`
- `treeCache: TreeCache | null`
- `theme: ThemeDto`

`TreeCache`:
- `aOnly: FileTreeNode[]`
- `bOnly: FileTreeNode[]`
- `both: FileTreeNode[]`

## 8.2 Tree Construction

Port the `gui/src/tree_view.rs` transformation logic into frontend utility:
- Input: flat relative paths from result entries
- Output: nested tree nodes sorted by path components

Tree sections rendered as collapsible groups:
- Only in A
- Only in B
- In Both

## 8.3 Async Execution

On compare click:
1. Validate button preconditions in frontend.
2. Set `isComparing = true`, clear prior error/results.
3. `await invoke("compare_directories", payload)`.
4. On success: compute tree cache and set results.
5. On failure: set `errorMessage`.
6. Always set `isComparing = false`.

Optional enhancement:
- add backend progress events for long-running comparisons.

## 9. Threading and Performance

Backend must avoid blocking UI thread:
- Tauri command executes comparison on a background context if needed.
- For very large directories, consider streaming progress as events.

Performance objective for parity:
- No visible regression vs existing GUI for same method selection.

## 10. Cross-Platform Considerations

1. Paths:
- Convert `PathBuf` to display-safe strings at API boundary.
- Preserve relative paths for result entries.

2. File dialogs:
- Use Tauri dialog APIs or Rust plugin APIs consistently across platforms.

3. Packaging:
- Validate build/install on macOS, Windows, Linux early.
- Add CI matrix for at least smoke build of Tauri targets.

## 11. Migration Phases

## Phase 0: Foundation
- Create `tauri-app` scaffold.
- Add workspace wiring and dev scripts.

## Phase 1: Backend Commands
- Implement DTOs.
- Implement `compare_directories` command with strategy mapping.
- Implement validate and dialog commands.
- Implement theme load/save commands.

## Phase 2: Frontend Parity UI
- Build form, selectors, compare button/spinner, error area.
- Implement result summary/footer.
- Implement tree rendering utilities and sections.

## Phase 3: Tests and Hardening
- Add Rust command tests:
  - method mapping
  - error propagation
  - summary counts
- Add frontend unit tests:
  - button enabled logic
  - tree builder behavior
- Add E2E smoke tests:
  - compare valid dirs
  - invalid dirs
  - theme persistence

## Phase 4: Rollout
- Mark old `gui` crate as deprecated in README.
- Keep dual GUI for one release (optional).
- Remove `gui` crate after acceptance criteria met.

## 12. Testing Strategy

Retain and migrate intent from current GUI tests:
- Directory validation logic
- Comparison method selection correctness
- Result transformation correctness
- Async state transitions (loading/error/success)

Test layers:
1. `core` tests: unchanged.
2. Tauri backend tests: command and DTO mapping.
3. Frontend tests: state and tree utility.
4. E2E tests: user workflow parity.

## 13. Acceptance Criteria

1. All current GUI functional behaviors are present in Tauri app.
2. Existing `core` tests pass unchanged.
3. New backend and frontend parity tests pass.
4. Tauri builds successfully on target platforms.
5. README updated with new run/build instructions.

## 14. Risks and Mitigations

1. Risk: path encoding/platform differences.
- Mitigation: normalize at boundary; add Windows path tests.

2. Risk: UI freeze on large comparisons.
- Mitigation: background execution + optional progress events.

3. Risk: packaging/signing complexity.
- Mitigation: early CI smoke builds and documented release steps.

4. Risk: divergence in behavior during transition.
- Mitigation: explicit parity checklist and side-by-side validation.

## 15. Implementation Checklist (Actionable)

1. Scaffold `tauri-app` and add to workspace.
2. Add Rust DTOs and command handlers.
3. Add strategy mapper preserving SampledHash verify default.
4. Add TypeScript invoke client wrappers.
5. Build parity UI controls and comparison flow.
6. Port tree builder from Rust GUI logic.
7. Implement theme load/save wiring.
8. Add backend/unit/E2E tests.
9. Update docs and release instructions.
10. Plan deprecation/removal of `gui/`.
