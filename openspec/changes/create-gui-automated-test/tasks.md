## 1. Setup Test Framework Infrastructure

- [x] 1.1 Add `tempfile` dev dependency to `gui/Cargo.toml`
- [x] 1.2 Create `gui/src/test_utils.rs` with test fixture helpers
- [x] 1.3 Add `create_test_dir_structure()` helper for standard comparison scenario
- [x] 1.4 Add `create_empty_dir()` helper for edge case testing
- [x] 1.5 Add `create_deeply_nested_dir()` helper for stress testing
- [x] 1.6 Add `create_unicode_dir()` helper for internationalization testing
- [x] 1.7 Add `create_permission_denied_dir()` helper for error testing
- [x] 1.8 Export test utilities from `gui/src/lib.rs` for integration tests

## 2. Refactor GUI for Testability

- [x] 2.1 Create `FileDialogProvider` trait in `gui/src/dialog.rs`
- [x] 2.2 Implement `NativeFileDialog` struct that uses `rfd::FileDialog`
- [x] 2.3 Implement `MockFileDialog` struct for testing
- [x] 2.4 Refactor `main.rs` to use `FileDialogProvider` instead of direct `rfd` calls
- [x] 2.5 Extract `validate_path()` logic from `DirCompareApp` to standalone function
- [x] 2.6 Make `AppState` fields accessible for testing (add getter methods or make public)
- [x] 2.7 Extract theme loading/saving logic to separate module for testing

## 3. Unit Tests for Core Functions

- [x] 3.1 Write unit tests for `validate_path()` covering all scenarios
- [x] 3.2 Write unit tests for `Theme::from_str()` with valid inputs
- [x] 3.3 Write unit tests for `Theme::from_str()` with invalid/empty inputs
- [x] 3.4 Write unit tests for `Theme::as_str()` roundtrip
- [x] 3.5 Write unit tests for `FileTreeNode::from_entries()` with various entry types
- [x] 3.6 Write unit tests for `MockFileDialog::pick_folder()`
- [x] 3.7 Write unit tests for `save_theme()` and `load_theme()` with temp config dir

## 4. Integration Tests for Directory Selection

- [x] 4.1 Create `gui/tests/directory_selection_tests.rs`
- [x] 4.2 Test manual path entry updates `AppState.dir_a_path` correctly
- [x] 4.3 Test manual path entry triggers validation automatically
- [x] 4.4 Test mock file dialog populates path field when "Browse" clicked
- [x] 4.5 Test valid directory shows ✅ indicator
- [x] 4.6 Test invalid directory shows ❌ indicator
- [x] 4.7 Test empty path shows ❌ indicator
- [x] 4.8 Test whitespace-only path shows ❌ indicator

## 5. Integration Tests for Comparison Methods

- [x] 5.1 Create `gui/tests/comparison_method_tests.rs`
- [x] 5.2 Test Filename comparison uses correct strategy
- [x] 5.3 Test Filename & Size comparison uses correct strategy
- [x] 5.4 Test Content Hash comparison uses correct strategy
- [x] 5.5 Test Sampled Hash comparison uses correct strategy
- [x] 5.6 Test comparison sets `is_comparing = true` during execution
- [x] 5.7 Test comparison displays spinner while running
- [x] 5.8 Test comparison results populate `tree_cache` on completion
- [x] 5.9 Test comparison displays error message on failure
- [x] 5.10 Test comparison re-enables button after completion or error

## 6. Integration Tests for Results Display

- [x] 6.1 Create `gui/tests/results_display_tests.rs`
- [x] 6.2 Test A-only entries display in correct tree section
- [x] 6.3 Test B-only entries display in correct tree section
- [x] 6.4 Test Both entries display in correct tree section
- [x] 6.5 Test entries display with correct colors (red/green/blue)
- [x] 6.6 Test directories show trailing "/" in tree
- [x] 6.7 Test nested directories render hierarchically
- [x] 6.8 Test summary bar shows correct counts after comparison
- [x] 6.9 Test summary bar shows zeros when no results

## 7. Integration Tests for Corner Cases

- [x] 7.1 Create `gui/tests/corner_case_tests.rs`
- [x] 7.2 Test empty directory A comparison shows correct results
- [x] 7.3 Test both empty directories comparison shows all zeros
- [x] 7.4 Test permission denied error displays user-friendly message
- [x] 7.5 Test 100-level deep nesting renders without stack overflow
- [x] 7.6 Test unicode filenames display correctly (中文, русский, emoji)
- [x] 7.7 Test large directory (1000+ files) comparison completes within timeout
- [x] 7.8 Test theme persistence saves to config file
- [x] 7.9 Test theme loading restores previous selection on startup
- [x] 7.10 Test invalid theme config falls back to System default

## 8. Test Infrastructure and CI

- [x] 8.1 Add `cargo test` to GUI workspace root README
- [x] 8.2 Document how to run specific test files
- [x] 8.3 Add test documentation to GUI module docs
- [x] 8.4 Verify all tests pass on local machine
- [ ] 8.5 Check test coverage with `cargo tarpaulin` if available
- [x] 8.6 Add `--lib` and `--test` flags documentation for headless testing
- [ ] 8.7 Create GitHub Actions workflow for GUI tests (future PR)

## 9. Code Quality and Cleanup

- [x] 9.1 Run `cargo fmt` on all modified files
- [x] 9.2 Run `cargo clippy` and fix any warnings
- [x] 9.3 Ensure all public functions have doc comments
- [x] 9.4 Verify no test code is included in release builds
- [x] 9.5 Check that `test_utils.rs` is behind `#[cfg(test)]` where appropriate
- [x] 9.6 Review test names for clarity and consistency
- [x] 9.7 Ensure tests clean up temp directories (use `TempDir` drop)
