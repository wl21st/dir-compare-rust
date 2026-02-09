## Why

The `dir-compare` tool currently only supports **read-only comparison** of directories. Users can identify differences between directories but have no built-in way to act on those differences. This forces users to manually copy, delete, or synchronize files using external tools, which is error-prone and time-consuming. Adding a sync feature transforms the tool from a passive comparison utility into an active directory management solution.

## What Changes

- **New sync module** in `core/` with file operations (copy, delete, update)
- **New sync strategies** supporting multiple synchronization modes:
  - Source Only: Mirror source directory to target (one-way sync)
  - Target Only: Mirror target directory to source (reverse one-way sync)
  - Merge: Bidirectional synchronization (newest wins or conflict detection)
  - Newer Only: Update files only when source is newer than target
  - Orphan Removal: Delete files in target that no longer exist in source
- **Filter support** for sync operations:
  - Inclusive patterns: Only sync files matching specified patterns
  - Exclusive patterns: Skip files matching specified patterns
- **CLI integration** with new `sync` subcommand and flags
- **GUI integration** with sync action buttons and strategy selection
- **Dry-run mode** to preview sync operations before execution
- **Progress reporting** for long-running sync operations

## Capabilities

### New Capabilities

- `sync-engine`: Core synchronization logic including file operations (copy, delete, update), conflict detection, and transaction support for atomic operations
- `sync-strategies`: Implementation of sync modes (source-only, target-only, merge, newer-only) with configurable behavior
- `sync-filters`: Include/exclude pattern matching for selective synchronization
- `orphan-management`: Detection and removal of orphaned files in target directories

### Modified Capabilities

_(none - this is new functionality that builds on top of existing comparison infrastructure without changing its requirements)_

## Impact

- **Core library** (`core/src/`): New `sync.rs` module with `SyncEngine`, `SyncStrategy`, `SyncFilter`, and `SyncResult` types
- **CLI** (`cli/src/main.rs`): New `sync` subcommand with flags for strategy, filters, dry-run, and verbosity
- **GUI** (`gui/src/`): New sync panel/dialog for selecting sync options and executing operations
- **Dependencies**: May need additional crates for file operations (atomic writes, permission handling)
- **Error handling**: New error types for sync failures (permission denied, disk full, conflict)
- **Testing**: New integration tests for sync operations using temp directories
