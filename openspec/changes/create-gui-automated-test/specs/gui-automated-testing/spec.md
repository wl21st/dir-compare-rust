## ADDED Requirements

### Requirement: GUI test framework exists
The GUI SHALL have an automated testing framework enabling unit and integration tests.

#### Scenario: Test framework is importable
- **WHEN** a developer writes `use dir_compare_gui::test_utils;`
- **THEN** the test utilities module SHALL be available

#### Scenario: Test utilities create directories
- **WHEN** a test calls `test_utils::create_test_dir_structure()`
- **THEN** temporary directories with test files SHALL be created
- **AND** the directories SHALL be automatically cleaned up after the test

### Requirement: File dialog abstraction exists
The GUI SHALL use a `FileDialogProvider` trait to enable mocking in tests.

#### Scenario: Mock dialog returns path
- **WHEN** a test creates `MockFileDialog { return_path: Some("/test/path") }`
- **AND** calls `pick_folder()` on the mock
- **THEN** the mock SHALL return `Some("/test/path")`

#### Scenario: Native dialog integration
- **WHEN** the GUI runs in production
- **THEN** it SHALL use `NativeFileDialog` implementation
- **AND** the native file dialog SHALL open for the user
