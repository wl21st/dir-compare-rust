## Context

The `dir-compare` tool provides directory comparison with multiple strategies (filename, size, hash, sampled-hash). The architecture follows a clean separation:
- `core/` crate: Library with `comparison.rs` (strategies, traversal, results) and `output.rs` (formatters)
- `cli/` crate: Command-line interface using clap
- `gui/` crate: Desktop GUI using eframe/egui

The comparison infrastructure produces a `ComparisonResult` with three categories:
- `a_only`: Files only in source directory
- `b_only`: Files only in target directory  
- `both`: Files matching in both directories

The sync feature will consume this comparison result to determine what operations to perform.

## Goals / Non-Goals

**Goals:**
- Add file synchronization capabilities that integrate naturally with existing comparison infrastructure
- Support multiple sync strategies covering common use cases (mirror, merge, selective)
- Provide a safe, predictable sync experience with dry-run preview and clear operation reporting
- Maintain the existing comparison-only workflow as the default behavior
- Enable both CLI and GUI sync operations

**Non-Goals:**
- Real-time continuous sync (daemon mode) - out of scope for initial implementation
- Network/remote sync (SSH, S3, cloud) - focus on local filesystem only
- File versioning or backup history - sync is destructive by design
- Conflict resolution UI for merge mode - initial merge will use newest-wins heuristic
- Symlink special handling - treat symlinks as regular files for now

## Decisions

### 1. Sync Module Architecture

**Decision**: Create a new `sync.rs` module in `core/` that consumes `ComparisonResult` and produces `SyncPlan` + `SyncResult`.

**Rationale**: This maintains separation of concerns - comparison logic remains unchanged, sync is a consumer of comparison results. Alternatives considered:
- Embedding sync in comparison: Rejected because it would complicate the comparison module and couple unrelated concerns
- Separate crate: Rejected as overkill for current scope; can refactor later if sync grows significantly

```
ComparisonResult → SyncEngine::plan() → SyncPlan → SyncEngine::execute() → SyncResult
```

### 2. Sync Strategy Enum

**Decision**: Use an enum-based strategy selection rather than a trait-based approach.

```rust
pub enum SyncMode {
    SourceToTarget,    // Mirror source → target (delete target-only, copy source-only)
    TargetToSource,    // Mirror target → source (reverse)
    Merge,             // Copy missing files both ways (newest wins for conflicts)
    NewerOnly,         // Only copy if source is newer (no deletes)
}
```

**Rationale**: The sync modes are well-defined and finite, unlike comparison strategies which benefit from extensibility. Enums provide:
- Exhaustive match checking
- Simpler serialization for CLI/config
- Clear documentation of all supported modes

### 3. Two-Phase Sync: Plan then Execute

**Decision**: Separate sync into planning (dry-run) and execution phases.

```rust
pub struct SyncPlan {
    operations: Vec<SyncOperation>,
    stats: PlanStats,
}

pub enum SyncOperation {
    Copy { from: PathBuf, to: PathBuf, size: u64 },
    Delete { path: PathBuf, size: u64 },
    CreateDir { path: PathBuf },
    RemoveDir { path: PathBuf },
}
```

**Rationale**: 
- Dry-run is essential for safety - users must preview destructive operations
- Enables progress calculation (total bytes to copy/delete known upfront)
- Plan can be serialized for review, logging, or confirmation dialogs

### 4. Filter Implementation

**Decision**: Use glob patterns with `globset` crate for include/exclude filters.

```rust
pub struct SyncFilter {
    include: Option<GlobSet>,  // If set, only matching files are synced
    exclude: Option<GlobSet>,  // Matching files are skipped (applied after include)
}
```

**Rationale**: Glob patterns are familiar to users (`.gitignore` style). The `globset` crate is well-maintained and fast. Alternative regex was considered but globs are more intuitive for file paths.

### 5. Orphan Handling as Flag, Not Mode

**Decision**: Make orphan removal a boolean flag that modifies behavior, not a separate mode.

```rust
pub struct SyncOptions {
    mode: SyncMode,
    remove_orphans: bool,  // Delete target files not in source
    dry_run: bool,
    filter: SyncFilter,
}
```

**Rationale**: Orphan removal is orthogonal to sync direction. `SourceToTarget` with `remove_orphans: true` creates a true mirror. This is more flexible than separate modes.

### 6. CLI Subcommand Structure

**Decision**: Add `sync` as a subcommand to preserve backward compatibility.

```bash
# Existing compare behavior (unchanged)
dir-compare -a dir1 -b dir2

# New sync subcommand
dir-compare sync -a source -b target --mode source-to-target --dry-run
dir-compare sync -a dir1 -b dir2 --mode merge --exclude "*.tmp"
```

**Rationale**: Subcommand keeps sync clearly separated from comparison. Users won't accidentally sync when they meant to compare.

### 7. Error Handling Strategy

**Decision**: Use a custom `SyncError` enum with per-operation granularity.

```rust
pub enum SyncError {
    PermissionDenied { path: PathBuf, operation: &'static str },
    DiskFull { path: PathBuf, needed: u64, available: u64 },
    IoError { path: PathBuf, source: std::io::Error },
    Conflict { path: PathBuf, reason: String },
}

pub struct SyncResult {
    completed: Vec<SyncOperation>,
    failed: Vec<(SyncOperation, SyncError)>,
    stats: SyncStats,
}
```

**Rationale**: Partial success is expected in sync operations. One permission error shouldn't abort the entire sync. Collect all errors and report at the end.

### 8. Modification Time Comparison

**Decision**: Use file modification time (mtime) for newer-only and merge conflict resolution.

**Rationale**: mtime is universally available and fast to read. Alternatives:
- Content hash: Too slow for determining "newer" 
- Creation time: Not reliably available on all platforms
- Both mtime and size: Considered but mtime alone is sufficient for initial implementation

## Risks / Trade-offs

**[Risk] Data loss from incorrect sync direction** → Mitigation: Dry-run is default in GUI; CLI requires explicit `--execute` flag to actually perform operations. Clear operation preview showing exactly what will be deleted/overwritten.

**[Risk] Incomplete sync on error** → Mitigation: Partial success is acceptable. Return detailed `SyncResult` showing completed vs failed operations. User can retry failed operations.

**[Risk] Race conditions during sync** → Mitigation: Document that directories should not be modified during sync. Future enhancement could add file locking.

**[Risk] Large directory performance** → Mitigation: Reuse existing walkdir traversal which handles large directories well. Progress reporting keeps users informed during long operations.

**[Trade-off] No atomic transactions** → We don't have rollback capability. If sync is interrupted, state may be inconsistent. This is acceptable for v1; users can re-run sync to converge.

**[Trade-off] Newest-wins for merge conflicts** → Simple heuristic may not always be correct. Future enhancement could add interactive conflict resolution.

## Open Questions

1. **Should we preserve file permissions and metadata during copy?** Initial implementation will use standard `fs::copy` which preserves some metadata. May need platform-specific handling later.

2. **Should empty directories be synced?** Leaning toward yes - create empty dirs in target if they exist in source. Needs confirmation.

3. **How to handle files that become directories (or vice versa)?** Proposed: treat as delete + create. Need to confirm this edge case behavior.
