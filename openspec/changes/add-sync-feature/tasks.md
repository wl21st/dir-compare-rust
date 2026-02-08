## 1. Core Types and Module Setup

- [ ] 1.1 Create `core/src/sync.rs` module with public exports in `lib.rs`
- [ ] 1.2 Define `SyncMode` enum with variants: `SourceToTarget`, `TargetToSource`, `Merge`, `NewerOnly`
- [ ] 1.3 Define `SyncOperation` enum with variants: `Copy`, `Delete`, `CreateDir`, `RemoveDir`
- [ ] 1.4 Define `SyncOptions` struct with fields: `mode`, `remove_orphans`, `dry_run`, `filter`
- [ ] 1.5 Define `SyncPlan` struct with operations list and statistics
- [ ] 1.6 Define `SyncResult` struct with completed/failed operations and stats
- [ ] 1.7 Define `SyncError` enum with variants: `PermissionDenied`, `DiskFull`, `IoError`, `Conflict`

## 2. Filter Implementation

- [ ] 2.1 Add `globset` dependency to `core/Cargo.toml`
- [ ] 2.2 Define `SyncFilter` struct with `include` and `exclude` `GlobSet` fields
- [ ] 2.3 Implement `SyncFilter::new()` constructor from pattern vectors
- [ ] 2.4 Implement `SyncFilter::matches()` method for path matching
- [ ] 2.5 Add unit tests for glob pattern matching (wildcards, double-star, exclusions)

## 3. Sync Engine - Planning Phase

- [ ] 3.1 Implement `SyncEngine` struct with comparison strategy reference
- [ ] 3.2 Implement `plan()` method that takes `ComparisonResult` and `SyncOptions`
- [ ] 3.3 Implement `SourceToTarget` planning logic (copy a_only, optionally delete b_only)
- [ ] 3.4 Implement `TargetToSource` planning logic (copy b_only, optionally delete a_only)
- [ ] 3.5 Implement `Merge` planning logic (copy both directions, newest-wins for conflicts)
- [ ] 3.6 Implement `NewerOnly` planning logic (copy newer source files only)
- [ ] 3.7 Apply filter to operations during planning
- [ ] 3.8 Implement operation ordering (dirs before files for create, files before dirs for delete)
- [ ] 3.9 Calculate plan statistics (file counts, byte totals)

## 4. Sync Engine - Execution Phase

- [ ] 4.1 Implement `execute()` method that takes a `SyncPlan` and optional progress callback
- [ ] 4.2 Implement copy operation with parent directory creation
- [ ] 4.3 Implement file delete operation
- [ ] 4.4 Implement directory delete operation (recursive)
- [ ] 4.5 Implement directory create operation
- [ ] 4.6 Handle errors gracefully, collect failures without aborting
- [ ] 4.7 Preserve modification time on copied files
- [ ] 4.8 Build `SyncResult` with completed/failed operations

## 5. Orphan Management

- [ ] 5.1 Implement orphan detection based on sync direction
- [ ] 5.2 Implement orphan filtering (respect include/exclude patterns)
- [ ] 5.3 Order orphan deletions correctly (files before parent directories)
- [ ] 5.4 Add orphan statistics to plan and result (separate counts)
- [ ] 5.5 Disable orphan removal for Merge and NewerOnly modes

## 6. CLI Integration

- [ ] 6.1 Add `clap` subcommand for `sync` with shared directory args
- [ ] 6.2 Add `--mode` flag with valid values: source-to-target, target-to-source, merge, newer-only
- [ ] 6.3 Add `--include` flag (repeatable) for include patterns
- [ ] 6.4 Add `--exclude` flag (repeatable) for exclude patterns
- [ ] 6.5 Add `--remove-orphans` flag (boolean)
- [ ] 6.6 Add `--dry-run` flag (default behavior, preview only)
- [ ] 6.7 Add `--execute` flag to actually perform sync operations
- [ ] 6.8 Implement sync plan display (tabular format showing operations)
- [ ] 6.9 Implement sync execution with progress output
- [ ] 6.10 Add exit codes for success, partial failure, complete failure

## 7. GUI Integration

- [ ] 7.1 Add sync button to comparison results view
- [ ] 7.2 Create sync options dialog with mode dropdown
- [ ] 7.3 Add include/exclude pattern input fields
- [ ] 7.4 Add orphan removal checkbox
- [ ] 7.5 Implement dry-run preview display
- [ ] 7.6 Add confirmation dialog before actual execution
- [ ] 7.7 Implement progress bar for sync execution
- [ ] 7.8 Display sync result summary with success/failure counts

## 8. Testing

- [ ] 8.1 Add unit tests for `SyncMode` planning logic
- [ ] 8.2 Add unit tests for `SyncFilter` pattern matching
- [ ] 8.3 Add unit tests for orphan detection and ordering
- [ ] 8.4 Add integration tests for copy operations using tempdir
- [ ] 8.5 Add integration tests for delete operations using tempdir
- [ ] 8.6 Add integration tests for full sync workflows (each mode)
- [ ] 8.7 Add CLI integration tests for sync subcommand
- [ ] 8.8 Add tests for error handling (permission denied, missing files)

## 9. Documentation

- [ ] 9.1 Add rustdoc comments to all public types and methods
- [ ] 9.2 Update README with sync feature usage examples
- [ ] 9.3 Add sync CLI examples to help text
- [ ] 9.4 Document sync modes and their behavior differences
