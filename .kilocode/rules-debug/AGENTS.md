# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Non-Obvious Debugging Patterns

- **GUI theme tests**: Must run with `--test-threads=1` to avoid config file conflicts between tests
- **Slow tests**: Marked with `#[ignore]` attribute; run with `-- --ignored` flag
  - Deep nesting test: Creates 100-level directory structure
  - Performance test: Creates 100 files and measures comparison time
  - Theme persistence tests: Share config state
- **GUI tests run headlessly**: Use mocked dependencies in `gui/src/test_utils.rs` (FileDialogProvider trait)
- **Test isolation**: GUI tests use temporary directories and mock file dialogs to avoid opening actual windows
- **Coverage tool**: Uses `cargo-tarpaulin` with LLVM engine for accurate coverage (requires nightly Rust)
- **Coverage config**: `.tarpaulin.toml` excludes test files and generates HTML/XML/LCOV reports in `target/coverage/`
