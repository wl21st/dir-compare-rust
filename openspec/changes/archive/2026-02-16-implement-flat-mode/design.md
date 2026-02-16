## Context

The directory comparison tool currently uses path-based hierarchy matching. This works well for comparing identical directory structures, but fails when:

- Files are moved to different folder locations
- Directory structures differ but contain identical files (duplicates)
- Users want to sync based on content equivalence rather than location

The existing comparison engine in `src/comparison.rs` builds a tree of files matched by path. Adding flat mode requires a parallel comparison strategy that:

1. Hashes file contents (using existing sampling hash or full hash)
2. Groups files by content hash instead of path
3. Identifies move/duplicate relationships
4. Returns results compatible with current output formatters

## Goals / Non-Goals

**Goals:**

- Enable content-based file matching across arbitrary directory structures
- Identify moved files and duplicates via content hashing
- Integrate flat mode as an optional comparison strategy alongside hierarchy mode
- Maintain backward compatibility with existing path-based comparison
- Support both fast sampling hash and full-file hash for flat mode
- Display flat mode results with clear indicators of moved/duplicate files

**Non-Goals:**

- Modifying the existing hierarchy-based comparison logic
- Changing the sampling hash implementation
- Automatic conflict resolution between multiple duplicate files
- Real-time or continuous monitoring of directories

## Decisions

**1. Architecture: Strategy Pattern for Comparison Modes**

- Introduce a `ComparisonStrategy` trait with implementations for `HierarchyComparison` and `FlatComparison`
- Current logic stays in `HierarchyComparison`; new `FlatComparison` handles content-based matching
- Rationale: Keeps concerns separated, makes testing easier, allows future comparison strategies
- Alternative considered: Single comparison function with mode branching (less extensible, harder to test)

**2. Content Hashing: Reuse Existing Hash Infrastructure**

- Use existing `sample_hash()` by default; add `--full-hash` option for `--flat` mode
- Flat mode performance benefit comes from comparing fewer bytes per file
- Rationale: Leverages existing code, allows users to trade speed for accuracy
- Alternative considered: Always use full hash in flat mode (slower, eliminates performance benefit)

**3. Output Format: List Files by Content Hash**

- Flat mode output groups files by content hash, showing which files are duplicates/moved
- Each group shows hash, size, count, and file list with paths
- Rationale: Clear visualization of duplication, helps users understand move relationships
- Alternative considered: Separate report format (duplicates only, moves only) - less comprehensive

**4. CLI Integration: New `--flat` Flag**

- Add `--flat` flag to enable flat mode comparison
- Mutually exclusive with hierarchy-specific options if needed
- Rationale: Intuitive UX, clearly signals different comparison behavior
- Alternative considered: `--compare-mode flat|hierarchy` (more verbose)

**5. File Grouping Algorithm**

- Phase 1: Hash all files in both directories
- Phase 2: Group files by hash → identify unique vs duplicated content
- Phase 3: Match groups across source/target to identify moved files
- Rationale: Simple two-pass approach, memory-efficient, easy to understand
- Alternative considered: Single-pass with in-memory hash map (less clear semantics)

## Risks / Trade-offs

- **[Risk] Hash collisions**: Two different files could have the same hash. **Mitigation**: Use cryptographically strong hash (SHA-256 if sampling proves unreliable); add `--verify` option for bit-perfect comparison.
- **[Risk] Large file directories**: Hashing all files is O(n); sampling hash mitigates. **Mitigation**: Implement progressive hashing, show progress bar for large scans.
- **[Risk] Ambiguous duplicates**: When multiple files have same content, which is the "original"? **Mitigation**: Document "moved" vs "copy" semantics; show all matches, let user decide.
- **[Risk] Slow initial adoption**: New feature, users need to learn the flag and output format. **Mitigation**: Clear documentation, example usage in README, intuitive flag naming.

## Migration Plan

1. Implement `ComparisonStrategy` trait and refactor existing comparison into `HierarchyComparison`
2. Create `FlatComparison` implementation with hashing and grouping logic
3. Extend CLI parser to accept `--flat` flag
4. Update main comparison logic to dispatch to correct strategy
5. Add output formatting for flat mode results (may extend existing formatters or create new ones)
6. Add integration tests for flat mode with known test directories
7. Document flat mode in README with examples

## Open Questions

- Should flat mode show which files are "new" (only in source/target)? Or just show by-hash groupings?
- How to display flat mode results in HTML/Markdown output? Same table format or specialized visualization?
- Should `--flat` mode work with inclusion/exclusion patterns like hierarchy mode?
- Performance targets: What's acceptable hash time for directories with 10K+, 100K+ files?
