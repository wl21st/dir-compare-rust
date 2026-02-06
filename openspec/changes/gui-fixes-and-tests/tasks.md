## 1. Core Error Handling

- [ ] 1.1 Update `compute_file_hash` in `core/src/comparison.rs` to return `Result<String, std::io::Error>` instead of silently swallowing errors.
- [ ] 1.2 Update `FastHashStrategy` usage of `compute_file_hash` to handle the `Result`. Ensure permission errors are distinguished from successful empty file hashes.
- [ ] 1.3 Verify `core` crate builds and tests pass with new error handling.

## 2. GUI Tests Implementation

- [ ] 2.1 Create `gui/tests/tree_tests.rs` and verify `FileTreeNode::from_entries` logic.
- [ ] 2.2 Add unit tests for `DirCompareApp::validate_path` (either in `gui/src/main.rs` via `#[cfg(test)]` or exposed to integration tests).
- [ ] 2.3 Expose necessary fields or methods in `FileTreeNode` (e.g., via `pub` or test-only accessors) to support verification.
- [ ] 2.4 Run `cargo test -p dir_compare_gui` to verify new tests pass.

## 3. GUI Theme Fixes

- [ ] 3.1 Refactor theme selection logic in `gui/src/main.rs` to use explicit `ctx.set_visuals` for Light/Dark.
- [ ] 3.2 Implement logic for "System" theme selection: attempt to reset visuals to default or respect system preference.
- [ ] 3.3 Add tooltip or warning UI if "System" reset requires a restart or is limited by `eframe` capabilities.
- [ ] 3.4 Verify theme switching works at runtime without inconsistent states.

## 4. Final Polish & Cleanup

- [ ] 4.1 Commit the updated `Cargo.lock` with the new GUI dependencies properly.
- [ ] 4.2 Run full project test suite (`cargo test --workspace`) to ensure no regressions.
- [ ] 4.3 Run `cargo clippy` and address any lints introduced by new tests or changes.
