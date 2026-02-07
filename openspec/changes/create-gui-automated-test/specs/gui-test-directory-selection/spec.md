## ADDED Requirements

### Requirement: Validate directory path
The GUI SHALL validate that a directory path exists and is accessible.

#### Scenario: Valid directory passes validation
- **WHEN** the user enters a path that exists and is a directory
- **THEN** `validate_path()` SHALL return `true`
- **AND** the UI SHALL display a valid indicator (✅)

#### Scenario: Non-existent directory fails validation
- **WHEN** the user enters a path that does not exist
- **THEN** `validate_path()` SHALL return `false`
- **AND** the UI SHALL display an invalid indicator (❌)

#### Scenario: File path fails validation
- **WHEN** the user enters a path that exists but is a file
- **THEN** `validate_path()` SHALL return `false`
- **AND** the UI SHALL display an invalid indicator (❌)

#### Scenario: Empty path fails validation
- **WHEN** the user enters an empty path
- **THEN** `validate_path()` SHALL return `false`

#### Scenario: Whitespace-only path fails validation
- **WHEN** the user enters a path containing only whitespace
- **THEN** `validate_path()` SHALL return `false`

### Requirement: Directory selection workflow
The GUI SHALL allow users to select directories through a file dialog or manual entry.

#### Scenario: Browse button opens file dialog
- **WHEN** the user clicks the "Browse..." button for Directory A
- **THEN** a file dialog SHALL open
- **AND** the selected path SHALL populate Directory A field

#### Scenario: Manual path entry works
- **WHEN** the user types a valid path into the Directory A field
- **THEN** the path SHALL be stored in `AppState.dir_a_path`
- **AND** validation SHALL run automatically
