## Why

Users currently cannot filter which files are compared, leading to noise from version control directories (like `.git`), build artifacts, or irrelevant file types. There is no way to narrow down the comparison to specific files of interest or exclude common clutter.

## What Changes

- Add `--exclude` flag to specify patterns for files/directories to ignore.
- Add `--include` flag to specify patterns for files to strictly include in the comparison.
- Implement support for respecting `.gitignore` files to automatically exclude ignored files.
- Add default excludes for common non-source directories (e.g., `.git`).

## Capabilities

### New Capabilities
- `file-filtering`: precise control over which files are included or excluded from the comparison process using glob patterns and gitignore rules.

### Modified Capabilities
<!-- No existing specs are being modified in their requirements. -->

## Impact

- CLI arguments in `src/main.rs` will be updated to accept new flags.
- `src/comparison.rs` will need logic to filter paths during traversal based on inclusion/exclusion rules.
- New dependencies might be needed for glob matching or gitignore parsing (e.g., `ignore` crate).
