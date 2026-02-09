## Why

Users need a way to exclude specific files and folders from the comparison to avoid irrelevant results and improve performance.

## What Changes

- Add support for a `.dir-compare-ignore` file.
- The `.dir-compare-ignore` file will contain a list of glob patterns for files and folders to ignore.
- The CLI and GUI will be updated to respect the ignore file.

## Capabilities

### New Capabilities
- `ignore-patterns`: Handle glob patterns for ignoring files and folders.

### Modified Capabilities
- None

## Impact

- The `core` library will need to be updated to read and apply the ignore patterns.
- The `cli` and `gui` applications will need to be updated to provide a way to specify the ignore file (or automatically detect it).
