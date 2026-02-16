## ADDED Requirements

### Requirement: Identify duplicate files by content hash

The system SHALL detect and report duplicate files (files with identical content) in either directory. When flat mode is enabled, duplicates within the same directory are identified and grouped by content hash.

#### Scenario: Detect duplicates within source directory

- **WHEN** source directory contains multiple files with identical content
- **WHEN** user runs `dir-compare -a dir_a -b dir_b --flat`
- **THEN** system identifies duplicates within source and marks them in the output

#### Scenario: Detect duplicates within target directory

- **WHEN** target directory contains multiple files with identical content
- **WHEN** user runs `dir-compare -a dir_a -b dir_b --flat`
- **THEN** system identifies duplicates within target and marks them in the output

#### Scenario: Detect duplicates across both directories

- **WHEN** source and target both contain the same file content in multiple locations
- **WHEN** user runs `dir-compare -a dir_a -b dir_b --flat`
- **THEN** system reports total count of duplicates across both directories

### Requirement: Identify moved files across directories

The system SHALL detect files that exist in the source directory and appear to have been moved to different locations in the target directory, based on matching content hashes.

#### Scenario: Identify file moved with same name

- **WHEN** file exists at `source/docs/report.txt` with content hash H1
- **WHEN** same content hash H1 exists at `target/archive/report.txt`
- **THEN** flat mode identifies this as a moved file

#### Scenario: Identify file moved with different name

- **WHEN** file exists at `source/old_name.txt` with content hash H1
- **WHEN** same content hash H1 exists at `target/new_name.txt`
- **THEN** flat mode identifies this as moved with name change

#### Scenario: Identify file moved to different directory depth

- **WHEN** source has `documents/2024/report.txt` with hash H1
- **WHEN** target has `archive/report.txt` with same hash H1
- **THEN** flat mode correctly identifies movement despite different nesting levels

### Requirement: Display duplicate and move relationships in output

The system SHALL clearly indicate which files are duplicates and which files are moved, making it easy for users to understand file relationships across directories.

#### Scenario: Report duplicate count per content hash

- **WHEN** flat mode comparison completes
- **THEN** output shows each content hash with file count (indicating duplicates when count > 1)

#### Scenario: List all paths for duplicated content

- **WHEN** multiple files share the same content hash
- **WHEN** user views flat mode output
- **THEN** all paths are listed under that hash group for easy reference

#### Scenario: Mark moved files with directory relationship

- **WHEN** a file is identified as moved between source and target
- **WHEN** user views flat mode output
- **THEN** output clearly shows source path → target path relationship

### Requirement: Support configurable hash collision handling

The system SHALL provide options for handling hash collisions (rare cases where different files have same hash) in flat mode.

#### Scenario: Default sampling hash may have collisions

- **WHEN** user runs `--flat` without `--full-hash`
- **THEN** system uses fast sampling hash (potential for collisions on very large files)

#### Scenario: Verify results with full hash

- **WHEN** user runs `--flat --full-hash` for critical comparisons
- **THEN** system uses SHA-256 full-file hash with negligible collision risk

#### Scenario: Document hash collision limitation

- **WHEN** user reads documentation
- **THEN** sampling hash limitations are documented with mitigation guidance
