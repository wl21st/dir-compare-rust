## Why

The new GUI crate currently lacks critical test coverage, leaving core logic like tree building and path validation unverified. Additionally, there are reliability and UX issues regarding silent file read errors during hashing and incomplete theme switching logic that need to be addressed to ensure a stable and user-friendly experience.

## What Changes

- **Add GUI Test Coverage**: Implement unit tests for `FileTreeNode::from_entries` and `DirCompareApp::validate_path`.
- **Improve Error Handling**: Modify hash computation to handle file read errors explicitly instead of silently returning empty strings.
- **Fix Theme Switching**: Correct the system theme selection logic or document its limitations to prevent inconsistent UI states.
- **Optimize Memory (Low Priority)**: Address memory doubling in `TreeCache` by documenting the issue or exploring `Rc<Entry>` usage.
- **Dependency Management**: Acknowledge and commit the updated `Cargo.lock` reflecting the new GUI dependencies.

## Capabilities

### New Capabilities
- `gui-test-suite`: Core unit testing infrastructure for the GUI crate, covering tree construction and input validation.
- `robust-file-hashing`: Resilient file hash computation that distinguishes between empty files and read errors.
- `gui-theme-control`: Reliable theme switching mechanism handling system defaults and manual overrides.

### Modified Capabilities
- None

## Impact

- **Codebase**:
    - `gui/src/tree_view.rs`
    - `gui/src/main.rs`
    - `core/src/comparison.rs`
    - `gui/tests/` (new directory/files)
- **Dependencies**: `Cargo.lock` updated with `egui`, `winit`, `wgpu` and others.
