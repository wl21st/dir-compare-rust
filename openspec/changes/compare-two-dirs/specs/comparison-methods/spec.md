## ADDED Requirements

### Requirement: Pluggable comparison strategy interface
The system SHALL support multiple comparison methods through a pluggable interface, allowing users to select the appropriate method based on their accuracy and performance needs.

#### Scenario: Initialize comparison with selected method
- **WHEN** user specifies a comparison method (filename, size, or hash)
- **THEN** system initializes the appropriate comparison strategy for that operation

#### Scenario: Switch between methods without code changes
- **WHEN** system is designed with pluggable strategies
- **THEN** new comparison methods can be added without modifying existing strategy implementations

### Requirement: Filename-only comparison
The system SHALL support comparison based solely on file/directory names, matching entries that have identical names regardless of content or properties.

#### Scenario: Match by filename
- **WHEN** user selects filename-only comparison method
- **THEN** system considers two entries matching if they have identical names and path relative to root

#### Scenario: Case sensitivity in filename comparison
- **WHEN** system performs filename comparison on case-sensitive filesystem
- **THEN** system treats "File.txt" and "file.txt" as different entries

#### Scenario: Case insensitivity option
- **WHEN** user specifies case-insensitive filename comparison
- **THEN** system treats "File.txt" and "file.txt" as matching entries

### Requirement: Filename plus size comparison
The system SHALL support comparison based on filename and file size, matching entries only if both name and size are identical.

#### Scenario: Match by filename and size
- **WHEN** user selects filename+size comparison method
- **THEN** system considers two files matching only if names are identical AND sizes are equal

#### Scenario: Handle directories with size comparison
- **WHEN** system applies filename+size method to directories
- **THEN** system matches directories by name only (size is not applicable to directories)

#### Scenario: Detect content changes via size
- **WHEN** file content is modified, changing its size
- **THEN** system reports files with same name but different sizes as unmatched

### Requirement: Fast-hash based comparison
The system SHALL support comparison using a fast (non-cryptographic) hash algorithm, matching entries with identical names and file hash.

#### Scenario: Compute file hash
- **WHEN** user selects hash-based comparison method
- **THEN** system computes a fast hash (e.g., xxHash or similar) for each file content

#### Scenario: Match by filename and hash
- **WHEN** user selects hash-based comparison method
- **THEN** system considers two files matching only if names are identical AND content hashes are equal

#### Scenario: Handle large files efficiently
- **WHEN** system processes large files with hash method
- **THEN** system uses streaming hash computation to avoid loading entire file into memory

#### Scenario: Handle directories with hash comparison
- **WHEN** system applies hash method to directories
- **THEN** system matches directories by name only (hash is not applicable to directories)

#### Scenario: Identical content with different names
- **WHEN** two files have identical content but different names
- **THEN** system reports them as unmatched (hash method matches by name first, then verifies hash)

### Requirement: Performance characteristics documented
The system SHALL document expected performance characteristics of each comparison method to help users choose appropriately.

#### Scenario: Recommend method selection
- **WHEN** user seeks guidance on which method to use
- **THEN** documentation clearly states that filename is fastest, hash is slowest, size is middle ground for performance
