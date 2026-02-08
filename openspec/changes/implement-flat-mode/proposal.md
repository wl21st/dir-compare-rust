## Why

Directory comparison and synchronization often fails when files exist in different folder structures, even though the content is identical. Users need a way to match and sync files based purely on content, enabling duplicate detection and file movement tracking across arbitrary directory hierarchies.

## What Changes

- Introduces a new "Flat View" comparison mode that matches files by content hash rather than path
- Enables detection of moved files and duplicates regardless of their location in the directory tree
- Allows users to synchronize files across different folder structures based on content equivalence
- Maintains compatibility with existing path-based comparison strategies

## Capabilities

### New Capabilities

- `flat-mode-comparison`: Structure-agnostic file matching using content hashing to identify equivalent files across different directory hierarchies
- `flat-mode-duplicate-detection`: Identify moved and duplicate files regardless of their location within the directory tree

### Modified Capabilities

- `comparison-strategies`: Extended to include flat mode as an alternative comparison strategy alongside existing hierarchy-based comparison

## Impact

- Affects `src/comparison.rs`: Core comparison logic needs to support content-hash-based matching
- Affects `src/main.rs`: CLI argument parsing to accept flat mode flag/option
- Affects `src/lib.rs`: Public API surface to expose flat mode comparison functionality
- Affects `src/output.rs`: May need to display flat mode results (e.g., showing moved/duplicate files)
- New tests in `tests/`: Integration tests for flat mode functionality
