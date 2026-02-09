## ADDED Requirements

### Requirement: Ignore files and folders based on glob patterns
The system SHALL ignore files and folders that match the glob patterns specified in the ignore file.

#### Scenario: Ignore a specific file
- **WHEN** the ignore file contains the pattern "file.txt"
- **AND** a file named "file.txt" exists in the directory being compared
- **THEN** the file "file.txt" SHALL be excluded from the comparison.

#### Scenario: Ignore a directory
- **WHEN** the ignore file contains the pattern "node_modules/"
- **AND** a directory named "node_modules" exists in the directory being compared
- **THEN** the "node_modules" directory and all its contents SHALL be excluded from the comparison.

#### Scenario: Ignore files by extension
- **WHEN** the ignore file contains the pattern "*.log"
- **AND** there are files with the ".log" extension in the directory being compared
- **THEN** all files with the ".log" extension SHALL be excluded from the comparison.

#### Scenario: No ignore file
- **WHEN** no ignore file is provided
- **THEN** all files and folders SHALL be included in the comparison.
