## Context

The `gui` crate is a recent addition and lacks comprehensive test coverage, specifically for tree construction and validation logic. Several reliability issues have been identified:
- Silent failures during file hashing in `core` (permission errors treated as empty files).
- Incomplete theme switching (System theme reset is buggy).
- Missing unit tests for the GUI logic.

This design addresses these by introducing a test suite, robust error handling, and fixed theme logic.

## Goals / Non-Goals

**Goals:**
- Establish a pattern for testing GUI-related logic (tree view, validation) without needing a full UI environment.
- Ensure file read errors are explicitly handled during comparison, preventing false positives/negatives.
- Provide a reliable theme switching experience or clearly communicate limitations.

**Non-Goals:**
- Complete UI integration testing (e.g., clicking buttons in tests). We focus on logic unit tests.
- Optimizing `TreeCache` memory usage (this is deferred to a future update).

## Decisions

### 1. Test Structure for GUI Logic
**Decision:** Create a `gui/tests/` directory with `tree_tests.rs` and `validation_tests.rs`.
**Rationale:**  Keeping tests separate from `src/` (integration-style directory structure) is standard Rust practice for testing public APIs. However, since we need to test internal logic (`FileTreeNode::from_entries`), we will expose these functions as `pub` or `pub(crate)` and might need to use `#[cfg(test)]` modules within `src/` if we want to test private implementation details.
**Refinement:** We will use `#[cfg(test)] mod tests { ... }` *inside* `gui/src/tree_view.rs` and `gui/src/main.rs` for unit tests that need access to private fields/types, or public integration tests if the API is public. Given `FileTreeNode` is likely public for the UI to use, `gui/tests/` is appropriate for high-level behavior.

### 2. Error Handling in Hashing
**Decision:** Change `compute_file_hash` signature to `Result<String, std::io::Error>`.
**Rationale:** Currently, it returns `String` (empty on error). Returning `Result` forces the caller (`FastHashStrategy`) to handle the error.
**Alternatives Considered:**
- Log error and return empty string: Keeps API simple but hides the problem (current behavior).
- Return `Option<String>`: Less informative than `Result`.

### 3. Theme Switching Strategy
**Decision:** Use `egui`'s context `set_visuals` with `Visuals::light()` / `Visuals::dark()` explicitly when overriding, and attempt to clear the override when "System" is selected. If "System" reset is flaky in `eframe`, we will document the limitation in the UI tooltips or use a restart-required banner.
**Rationale:** `eframe` (web/native) has varying support for system theme following. Explicit visual setting is reliable for Light/Dark.

## Risks / Trade-offs

- **[Risk] Test Visibility:** `FileTreeNode` fields might need to be made public for verification in integration tests.
  - **Mitigation:** Use accessors or derive `Debug`/`PartialEq` to facilitate testing without exposing internal mutability.
- **[Risk] Hashing Performance:** Adding `Result` handling might add slight overhead.
  - **Mitigation:** Negligible compared to disk I/O.
- **[Risk] Theme "System" Reset:** `eframe` might not support "un-setting" a theme easily without reload.
  - **Mitigation:** If it fails, we fall back to manual Light/Dark or warn the user.

## Migration Plan

- No data migration needed.
- `Cargo.lock` update is standard.
- Changes to `core` are source-compatible for other crates if they don't rely on the exact hashing internals (which are internal details).
