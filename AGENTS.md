# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Project Structure (Non-Obvious)

- **Workspace structure**: This is a Cargo workspace with 3 crates (`cli/`, `core/`, `gui/`) - NOT a single-crate project
- **Rust 2024 edition**: Uses Rust 2024 edition (newer than typical 2021), specified in workspace-level `Cargo.toml`
- **Positional CLI args**: CLI uses positional arguments (DIR1 DIR2), not `-a`/`-b` flags as might be expected
- **Method aliases**: CLI accepts multiple aliases for methods: `hash|fxhash|fasthash`, `sampled|sampled-hash`, `filename|name`

## Build & Test Commands (Non-Obvious)

- **Run single test**: `cargo test test_name` (e.g., `cargo test test_cli_invalid_format`)
- **Run single package tests**: `cargo test --package dir-compare-core` (workspace has 3 packages)
- **Run ignored tests**: `cargo test -- --ignored` (slow tests marked with `#[ignore]`)
- **Coverage with tarpaulin**: Requires LLVM engine (nightly Rust) - see `.tarpaulin.toml`
- **Single package coverage**: `cargo tarpaulin --package dir-compare-core --out Html`
- **CLI binary name**: `dir-compare` (not `dir-compare-cli` despite package name)
- **GUI binary name**: `dir-compare-gui` (matches package name)

## Comparison Strategy Implementation Details (Non-Obvious)

- **SampledHashStrategy constants**: Uses 7 samples of 431 bytes each (prime number to avoid filesystem block alignment)
- **File size in hash**: SampledHashStrategy includes file size (8 bytes big-endian) to prevent false positives when one file is a prefix of another
- **Small vs large files**: Files < 3017 bytes (7*431) are fully hashed; larger files use 7 distributed samples
- **Hash error handling**: Hash functions return `"ERROR:{path}"` on failure (unique per path to avoid false matches)
- **FastHash uses FxHash**: Not SHA-256 - uses `fxhash` crate for speed (non-cryptographic)
- **SampledHash uses SHA-256**: For sampled hashing, uses `sha2` crate (cryptographic)
- **Verify flag**: `--verify` flag on CLI makes SampledHashStrategy perform full hash check after sampled match

## GUI Implementation Details (Non-Obvious)

- **GUI async pattern**: Comparison runs in separate thread with `std::sync::mpsc::channel()` to avoid blocking UI
- **GUI always uses verify**: GUI hardcodes `verify_on_match: true` for SampledHashStrategy (line 319 in `gui/src/main.rs`)
- **Theme persistence**: Theme state saved to config file via `dirs` crate; tests must use `--test-threads=1` to avoid conflicts
- **Test mocking**: `gui/src/test_utils.rs` contains `FileDialogProvider` trait for mocking file dialogs in tests
- **GUI tests run headlessly**: Use mocked dependencies to avoid opening actual windows

## Ignore File Support (Non-Obvious)

- **Ignore file format**: Uses gitignore-style patterns via the `ignore` crate
- **Default ignore file**: `.dir-compare-ignore` (not `.gitignore`)
- **Ignore file location**: Can be specified via `--ignore` flag or passed to `compare_directories()` function

## Testing Patterns (Non-Obvious)

- **CLI tests use `cargo_bin_cmd!` macro**: From `assert_cmd` crate to get binary path
- **Slow tests marked with `#[ignore]`**: Deep nesting (100 levels) and performance tests
- **GUI theme tests need isolation**: Must run with `--test-threads=1` due to shared config file state
- **Test helpers in production code**: `gui/src/test_utils.rs` is in `src/` not `tests/` (needed for trait visibility)

## Coverage Configuration (Non-Obvious)

- **Requires nightly Rust**: `.tarpaulin.toml` specifies LLVM engine which requires nightly
- **Excludes test files**: Coverage excludes `target/*`, `**/tests/*`, `**/test_*` patterns
- **Multiple output formats**: Generates HTML, XML (Cobertura), and LCOV in `target/coverage/`
- **5-minute timeout**: Tests have 300-second timeout configured in `.tarpaulin.toml`
