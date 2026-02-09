## 1. Core Library Changes

- [x] 1.1 Add the `ignore` crate as a dependency to `core/Cargo.toml`.
- [x] 1.2 Modify the `Comparison` struct in `core/src/comparison.rs` to include an optional `ignore_file_path: Option<PathBuf>`.
- [x] 1.3 Update the `Comparison::new` function in `core/src/comparison.rs` to accept the `ignore_file_path`.
- [x] 1.4 In `core/src/comparison.rs`, use `ignore::WalkBuilder` to create a directory walker that respects the ignore file.
- [x] 1.5 Write unit tests in `core/tests/unit_tests.rs` to verify the ignore logic.

## 2. CLI Application Changes

- [x] 2.1 Add the `--ignore` option to the CLI arguments in `cli/src/main.rs`.
- [x] 2.2 Update the `main` function in `cli/src/main.rs` to pass the `--ignore` file path to the `Comparison` struct.
- [x] 2.3 Add tests in `cli/tests/cli_tests.rs` to verify the `--ignore` option.

## 3. GUI Application Changes

- [x] 3.1 Add a button to the GUI in `gui/src/main.rs` to select an ignore file.
- [x] 3.2 Implement the file selection logic in `gui/src/dialog.rs` to open a file dialog.
- [x] 3.3 Store the selected ignore file path in the application state in `gui/src/main.rs`.
- [x] 3.4 Pass the ignore file path to the `Comparison` struct when starting the comparison.
- [x] 3.5 Add automated tests for the GUI in `gui/tests` to verify the ignore file selection.
