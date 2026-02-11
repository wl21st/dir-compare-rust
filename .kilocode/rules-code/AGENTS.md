# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Non-Obvious Coding Patterns

- **SampledHashStrategy constants**: Uses 7 samples of 431 bytes each (prime number to avoid filesystem block alignment)
- **File size in hash**: SampledHashStrategy includes file size (8 bytes big-endian) to prevent false positives when one file is a prefix of another
- **Small vs large files**: Files < 3017 bytes are fully hashed; larger files use 7 distributed samples
- **GUI async pattern**: Comparison runs in separate thread with mpsc channel (`std::sync::mpsc::channel()`) to avoid blocking UI
- **Ignore file support**: Use `.dir-compare-ignore` files with gitignore-style patterns via the `ignore` crate
- **GUI theme persistence**: Theme state saved to config file; tests must use `--test-threads=1` to avoid conflicts
- **Test helpers**: `gui/src/test_utils.rs` contains mock implementations for file dialogs (FileDialogProvider trait)
