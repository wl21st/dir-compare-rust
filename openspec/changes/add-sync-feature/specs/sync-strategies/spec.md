## ADDED Requirements

### Requirement: Source-to-target sync mode
The system SHALL support a `SourceToTarget` mode that mirrors source directory contents to target.

#### Scenario: Copy source-only files to target
- **WHEN** sync runs in `SourceToTarget` mode
- **THEN** files in `a_only` (source-only) SHALL be queued for copy to target

#### Scenario: Source-to-target does not modify source
- **WHEN** sync runs in `SourceToTarget` mode
- **THEN** no files in source directory SHALL be modified or deleted

#### Scenario: Source-to-target with orphan removal
- **WHEN** sync runs in `SourceToTarget` mode with `remove_orphans: true`
- **THEN** files in `b_only` (target-only) SHALL be queued for deletion

### Requirement: Target-to-source sync mode
The system SHALL support a `TargetToSource` mode that mirrors target directory contents to source.

#### Scenario: Copy target-only files to source
- **WHEN** sync runs in `TargetToSource` mode
- **THEN** files in `b_only` (target-only) SHALL be queued for copy to source

#### Scenario: Target-to-source does not modify target
- **WHEN** sync runs in `TargetToSource` mode
- **THEN** no files in target directory SHALL be modified or deleted

#### Scenario: Target-to-source with orphan removal
- **WHEN** sync runs in `TargetToSource` mode with `remove_orphans: true`
- **THEN** files in `a_only` (source-only) SHALL be queued for deletion

### Requirement: Merge sync mode
The system SHALL support a `Merge` mode that performs bidirectional synchronization.

#### Scenario: Copy missing files in both directions
- **WHEN** sync runs in `Merge` mode
- **THEN** files in `a_only` SHALL be copied to target AND files in `b_only` SHALL be copied to source

#### Scenario: Merge resolves conflicts using modification time
- **WHEN** sync runs in `Merge` mode and files exist in both directories with different content
- **THEN** the file with the newer modification time SHALL overwrite the older file

#### Scenario: Merge with identical modification times
- **WHEN** conflicting files have identical modification times
- **THEN** source file SHALL take precedence (deterministic resolution)

### Requirement: Newer-only sync mode
The system SHALL support a `NewerOnly` mode that only copies files when source is newer.

#### Scenario: Copy newer source files
- **WHEN** sync runs in `NewerOnly` mode and source file is newer than target
- **THEN** source file SHALL be copied to target

#### Scenario: Skip older source files
- **WHEN** sync runs in `NewerOnly` mode and source file is older than or same age as target
- **THEN** source file SHALL NOT be copied

#### Scenario: Copy source-only files
- **WHEN** sync runs in `NewerOnly` mode and file exists only in source
- **THEN** file SHALL be copied to target (no target to compare against)

#### Scenario: Newer-only never deletes
- **WHEN** sync runs in `NewerOnly` mode
- **THEN** no files SHALL be deleted from either directory regardless of orphan settings

### Requirement: Sync mode is selectable via options
The system SHALL accept sync mode as part of `SyncOptions`.

#### Scenario: Default sync mode
- **WHEN** sync is invoked without explicit mode
- **THEN** `SourceToTarget` SHALL be used as default

#### Scenario: Mode specified in options
- **WHEN** `SyncOptions.mode` is set to a valid mode
- **THEN** sync SHALL use the specified mode for planning operations
