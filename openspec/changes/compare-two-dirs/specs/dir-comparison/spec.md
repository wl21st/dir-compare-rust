## ADDED Requirements

### Requirement: Recursive directory traversal
The system SHALL recursively scan two directory paths and enumerate all files and subdirectories, respecting directory structure and handling permission errors gracefully.

#### Scenario: Successful traversal of both directories
- **WHEN** user provides two valid directory paths
- **THEN** system traverses both directory trees completely, collecting all file and directory entries

#### Scenario: Handle permission denied errors
- **WHEN** system encounters a file or directory it cannot read
- **THEN** system logs a warning, skips the inaccessible entry, and continues traversal

#### Scenario: Handle non-existent directories
- **WHEN** user provides a path that does not exist
- **THEN** system reports an error with clear message indicating which path is invalid

### Requirement: Categorize entries into three lists
The system SHALL compare the two directory trees and produce three categorized result lists: entries only in directory A, entries only in directory B, and entries present in both directories.

#### Scenario: Identify entries only in A
- **WHEN** system completes traversal of both directories
- **THEN** system produces a list of all files/folders that exist in A but not in B

#### Scenario: Identify entries only in B
- **WHEN** system completes traversal of both directories
- **THEN** system produces a list of all files/folders that exist in B but not in A

#### Scenario: Identify entries in both
- **WHEN** system completes traversal of both directories
- **THEN** system produces a list of all files/folders that exist in both A and B (matched according to comparison method)

#### Scenario: Handle empty directories
- **WHEN** one or both directories are empty
- **THEN** system correctly categorizes entries: empty directory is treated as A-only or B-only or both with zero entries

### Requirement: Preserve directory structure information
The system SHALL retain relative path information for all discovered entries, preserving directory hierarchy in results.

#### Scenario: Maintain relative paths
- **WHEN** system traverses subdirectories
- **THEN** results include relative paths (e.g., "subdir/file.txt") that reflect the original directory structure

#### Scenario: Distinguish files from directories
- **WHEN** system reports results
- **THEN** each entry is clearly marked as either a file or directory

### Requirement: Support normalized path comparison
The system SHALL normalize paths to handle cross-platform compatibility (Windows backslashes vs Unix forward slashes).

#### Scenario: Compare paths across platforms
- **WHEN** directories contain subdirectories with multiple levels
- **THEN** paths are normalized consistently regardless of platform for accurate comparison

### Requirement: Handle symlinks appropriately
The system SHALL treat symlinks as regular file entries (not follow them) to prevent infinite loops and duplicate results.

#### Scenario: Encounter symbolic link
- **WHEN** system encounters a symbolic link during traversal
- **THEN** system treats it as a regular file entry and does not follow the link
