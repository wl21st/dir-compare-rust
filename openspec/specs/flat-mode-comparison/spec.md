# flat-mode-comparison Specification

## Purpose
TBD - created by archiving change implement-flat-mode. Update Purpose after archive.
## Requirements
### Requirement: Flat mode comparison groups files by content hash

The system SHALL support a flat mode that compares directories by matching files based on content hash rather than path structure. Users can enable flat mode via the `--flat` CLI flag.

#### Scenario: Enable flat mode with sampling hash

- **WHEN** user runs `dir-compare -a dir_a -b dir_b --flat`
- **THEN** system compares files by content hash using fast sampling hash by default

#### Scenario: Enable flat mode with full file hash

- **WHEN** user runs `dir-compare -a dir_a -b dir_b --flat --full-hash`
- **THEN** system compares files by content hash using full-file hash for bit-perfect accuracy

#### Scenario: Flat mode produces hash-grouped output

- **WHEN** flat mode comparison completes
- **THEN** results are organized by content hash with file counts and path listings per hash group

### Requirement: Flat mode matches files across different directory structures

When comparing directories with different folder hierarchies, flat mode SHALL identify files with identical content regardless of their location in the tree.

#### Scenario: Match identical files in different structures

- **WHEN** source directory has `documents/report.txt` with content hash H1
- **WHEN** target directory has `archive/2024/report.txt` with same content hash H1
- **THEN** flat mode identifies them as the same file (moved/copied)

#### Scenario: Handle files-only vs nested structures

- **WHEN** source contains flat file list and target has nested subdirectories
- **WHEN** both contain identical files by content
- **THEN** flat mode successfully matches all files and reports them as equivalent

### Requirement: Flat mode performance with sampling hash

The system SHALL use fast sampling hash by default in flat mode to enable rapid comparison of large files without reading entire file contents.

#### Scenario: Fast hash comparison on large files

- **WHEN** comparing directories with large files (>100MB)
- **WHEN** user runs with `--flat` (default sampling hash)
- **THEN** comparison completes faster than reading full file contents

#### Scenario: User can opt into full hash verification

- **WHEN** user needs guaranteed accuracy for critical files
- **WHEN** user runs `--flat --full-hash`
- **THEN** system uses full-file hash despite longer comparison time

### Requirement: Flat mode works with inclusion and exclusion patterns

The system SHALL support the same `--include` and `--exclude` patterns in flat mode as in hierarchy mode.

#### Scenario: Exclude files in flat mode

- **WHEN** user runs `dir-compare -a dir_a -b dir_b --flat --exclude "*.log"`
- **THEN** `.log` files are not included in flat mode comparison

#### Scenario: Include specific files in flat mode

- **WHEN** user runs `dir-compare -a dir_a -b dir_b --flat --include "*.txt"`
- **THEN** only `.txt` files are included in flat mode comparison

