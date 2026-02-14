## ADDED Requirements

### Requirement: Three-category result structure
The system SHALL produce comparison results organized into three distinct categories, clearly separating entries that exist only in directory A, only in directory B, and in both directories.

#### Scenario: Produce three separate result lists
- **WHEN** comparison completes
- **THEN** system outputs three organized result sets: A-only, B-only, and both

#### Scenario: Empty category handling
- **WHEN** one or more result categories contains no entries
- **THEN** system still produces the category section, clearly indicating it is empty

### Requirement: Display relative paths in results
The system SHALL display file and directory paths relative to the root directories provided by the user, making results human-readable and preserving directory hierarchy.

#### Scenario: Show relative paths
- **WHEN** system outputs results
- **THEN** entries are displayed with paths relative to their respective root directory (e.g., "subdir/file.txt" not absolute path)

#### Scenario: Indicate file vs directory
- **WHEN** system displays an entry
- **THEN** output clearly distinguishes between files and directories (e.g., via notation like trailing slash or prefix)

### Requirement: File size information in results
The system SHALL include file size information in output when size is relevant to the comparison or when explicitly requested.

#### Scenario: Include sizes in output
- **WHEN** user requests size information or uses size-based comparison
- **THEN** system displays file sizes alongside entries for reference

#### Scenario: Size discrepancies in both category
- **WHEN** files match by name but have different sizes in A and B
- **THEN** system reports sizes for both entries so user can identify the discrepancy

### Requirement: Count summaries
The system SHALL provide summary counts for each category to give users a quick overview of comparison results.

#### Scenario: Display category counts
- **WHEN** comparison completes
- **WHEN** system outputs results
- **THEN** system displays count of entries in each category (A-only: N, B-only: M, both: K)

### Requirement: Structured output format
The system SHALL support at minimum a human-readable text format, with provision for JSON output in future enhancements.

#### Scenario: Human-readable text output
- **WHEN** user does not specify output format
- **THEN** system produces text output with clear section headers and organized entry lists

#### Scenario: Sorted results
- **WHEN** system produces output
- **THEN** entries within each category are sorted (e.g., alphabetically by path) for consistency

### Requirement: Handle special characters in paths
The system SHALL properly display and handle paths containing special characters (spaces, unicode, etc.) in output without corruption or confusion.

#### Scenario: Display paths with spaces
- **WHEN** entries contain spaces in filenames or directory names
- **THEN** system displays them correctly, either via quoting, escaping, or other clear notation

#### Scenario: Handle unicode characters
- **WHEN** entries contain unicode characters (accents, non-Latin scripts)
- **THEN** system displays them correctly in output
