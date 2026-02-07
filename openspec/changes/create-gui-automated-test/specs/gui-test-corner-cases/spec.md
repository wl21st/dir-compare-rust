## ADDED Requirements

### Requirement: Handle empty directories
The GUI SHALL gracefully handle empty directories.

#### Scenario: Compare empty directory A
- **WHEN** Directory A is empty and Directory B has files
- **AND** comparison completes
- **THEN** "A Only" SHALL show 0
- **AND** "B Only" SHALL show N entries
- **AND** "Both" SHALL show 0

#### Scenario: Compare both empty directories
- **WHEN** both directories are empty
- **AND** comparison completes
- **THEN** all categories SHALL show 0

### Requirement: Handle permission errors
The GUI SHALL handle permission-denied errors gracefully.

#### Scenario: Permission denied shows error
- **WHEN** the user selects a directory without read permission
- **AND** clicks "Compare"
- **THEN** an error message SHALL display
- **AND** the message SHALL contain "permission" or "access"

### Requirement: Handle deeply nested directories
The GUI SHALL handle deeply nested directory structures.

#### Scenario: 100 levels deep renders correctly
- **WHEN** directories contain 100 levels of nesting
- **AND** comparison completes
- **THEN** the tree view SHALL render all levels
- **AND** the UI SHALL remain responsive

### Requirement: Handle unicode filenames
The GUI SHALL handle filenames containing unicode characters.

#### Scenario: Unicode filenames display correctly
- **WHEN** directories contain files with names like "文件.txt", "файл.txt", "🎉.txt"
- **AND** comparison completes
- **THEN** all filenames SHALL display correctly in the tree
- **AND** matching SHALL work based on selected method

### Requirement: Handle large file counts
The GUI SHALL handle directories with many files.

#### Scenario: 10000 files comparison completes
- **WHEN** directories contain 10000 files each
- **AND** method is "Filename"
- **THEN** comparison SHALL complete within 30 seconds
- **AND** results SHALL be paginated or virtualized

### Requirement: Theme persistence
The GUI SHALL persist theme selection across sessions.

#### Scenario: Theme saves on change
- **WHEN** the user switches to "Dark" theme
- **AND** closes the application
- **AND** reopens the application
- **THEN** the theme SHALL be "Dark"

#### Scenario: Invalid theme config falls back
- **WHEN** the theme config file contains invalid data
- **AND** the application starts
- **THEN** the theme SHALL default to "System"
