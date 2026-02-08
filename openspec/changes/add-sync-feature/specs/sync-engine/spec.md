## ADDED Requirements

### Requirement: Sync engine produces execution plan from comparison result
The system SHALL generate a `SyncPlan` containing ordered operations from a `ComparisonResult` and `SyncOptions`.

#### Scenario: Generate plan from comparison result
- **WHEN** sync engine receives a `ComparisonResult` and `SyncOptions`
- **THEN** it SHALL produce a `SyncPlan` with operations ordered by: directories before files for creation, files before directories for deletion

#### Scenario: Plan includes operation statistics
- **WHEN** a sync plan is generated
- **THEN** it SHALL include statistics: total files to copy, total files to delete, total bytes to transfer

### Requirement: Sync engine executes plan with progress reporting
The system SHALL execute a `SyncPlan` and report progress for each operation.

#### Scenario: Execute plan with callback
- **WHEN** sync engine executes a plan with a progress callback
- **THEN** it SHALL invoke the callback before and after each operation with operation details and status

#### Scenario: Execution produces result summary
- **WHEN** plan execution completes
- **THEN** system SHALL return a `SyncResult` containing: completed operations, failed operations with errors, and aggregate statistics

### Requirement: Sync engine supports copy operation
The system SHALL copy files from source to target, creating parent directories as needed.

#### Scenario: Copy file to existing directory
- **WHEN** copy operation targets an existing directory
- **THEN** file SHALL be copied preserving content and modification time

#### Scenario: Copy file to non-existent directory
- **WHEN** copy operation targets a path where parent directory does not exist
- **THEN** system SHALL create parent directories before copying file

#### Scenario: Copy handles permission error
- **WHEN** copy operation fails due to permission denied
- **THEN** operation SHALL be recorded in failed list with `PermissionDenied` error

### Requirement: Sync engine supports delete operation
The system SHALL delete files and directories from the target.

#### Scenario: Delete file
- **WHEN** delete operation targets a file
- **THEN** file SHALL be removed from filesystem

#### Scenario: Delete directory
- **WHEN** delete operation targets an empty directory
- **THEN** directory SHALL be removed from filesystem

#### Scenario: Delete non-empty directory
- **WHEN** delete operation targets a non-empty directory
- **THEN** system SHALL recursively delete contents before removing directory

### Requirement: Sync engine supports dry-run mode
The system SHALL support a dry-run mode that generates a plan without executing operations.

#### Scenario: Dry-run generates plan only
- **WHEN** sync is invoked with `dry_run: true`
- **THEN** system SHALL return the `SyncPlan` without modifying any files

#### Scenario: Dry-run plan matches execution plan
- **WHEN** dry-run is performed followed by actual execution with same inputs
- **THEN** both plans SHALL contain identical operations (assuming no filesystem changes between calls)

### Requirement: Sync engine handles partial failures gracefully
The system SHALL continue execution after individual operation failures and report all errors.

#### Scenario: Continue after copy failure
- **WHEN** a copy operation fails
- **THEN** subsequent operations SHALL still be attempted

#### Scenario: Aggregate errors in result
- **WHEN** multiple operations fail during execution
- **THEN** all failures SHALL be collected in `SyncResult.failed` with corresponding error details
