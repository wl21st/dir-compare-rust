## ADDED Requirements

### Requirement: Tree node construction from entries
The system SHALL be able to construct a `FileTreeNode` structure from a flat list of `Entry` objects.

#### Scenario: Single directory with file
- **WHEN** a list of entries containing one file "dir/file.txt" is provided
- **THEN** a root node with one child "dir" is created
- **AND** "dir" node has one child "file.txt"
- **AND** the structure correctly reflects the hierarchy

#### Scenario: Empty entry list
- **WHEN** an empty list of entries is provided
- **THEN** an empty tree or root node with no children is created

### Requirement: Path validation
The application SHALL validate input paths before attempting to scan them.

#### Scenario: Valid path
- **WHEN** a valid, existing directory path is provided (e.g., "/tmp")
- **THEN** validation returns true

#### Scenario: Empty path
- **WHEN** an empty string is provided as a path
- **THEN** validation returns false

#### Scenario: Non-existent path
- **WHEN** a path to a non-existent directory is provided
- **THEN** validation returns false (if checking existence) or handles it gracefully
