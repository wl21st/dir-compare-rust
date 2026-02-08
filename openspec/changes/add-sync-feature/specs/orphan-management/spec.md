## ADDED Requirements

### Requirement: Orphan detection identifies target-only files
The system SHALL identify orphaned files as those existing in target but not in source.

#### Scenario: Detect orphaned files
- **WHEN** comparison result contains files in `b_only`
- **THEN** these files SHALL be identified as orphans when source-to-target sync is configured

#### Scenario: Detect orphaned directories
- **WHEN** comparison result contains directories in `b_only`
- **THEN** these directories SHALL be identified as orphans

#### Scenario: Orphan detection respects sync direction
- **WHEN** sync mode is `TargetToSource`
- **THEN** orphans SHALL be files in `a_only` (source-only) instead

### Requirement: Orphan removal is opt-in
The system SHALL only remove orphans when explicitly enabled via `remove_orphans` option.

#### Scenario: Orphan removal disabled by default
- **WHEN** sync runs without explicit `remove_orphans` setting
- **THEN** orphaned files SHALL NOT be deleted

#### Scenario: Orphan removal enabled
- **WHEN** sync runs with `remove_orphans: true`
- **THEN** orphaned files SHALL be queued for deletion in sync plan

### Requirement: Orphan removal respects filters
The system SHALL apply include/exclude filters to orphan removal.

#### Scenario: Excluded orphans preserved
- **WHEN** orphan removal is enabled and exclude pattern matches orphan file
- **THEN** that orphan SHALL NOT be deleted

#### Scenario: Non-matching orphans preserved with include filter
- **WHEN** orphan removal is enabled and orphan does not match include pattern
- **THEN** that orphan SHALL NOT be deleted

### Requirement: Orphan removal deletes directories after contents
The system SHALL order orphan deletions so files are deleted before their parent directories.

#### Scenario: Delete orphaned directory tree
- **WHEN** orphan removal targets a directory with contents
- **THEN** child files SHALL be deleted before child directories, and child directories before parent

#### Scenario: Preserve non-empty directories with filtered contents
- **WHEN** orphan directory contains files excluded by filter
- **THEN** directory SHALL NOT be deleted (it still has contents)

### Requirement: Orphan removal reports deleted files
The system SHALL include orphan deletions in sync plan and result.

#### Scenario: Orphan operations in plan
- **WHEN** sync plan is generated with orphan removal
- **THEN** plan SHALL contain `Delete` operations for each orphan with file size

#### Scenario: Orphan deletion statistics
- **WHEN** sync result is produced with orphan removal
- **THEN** statistics SHALL include count and total size of deleted orphans separately from other deletions

### Requirement: Orphan removal is disabled in merge mode
The system SHALL NOT remove orphans when sync mode is `Merge`.

#### Scenario: Merge mode ignores orphan setting
- **WHEN** sync runs in `Merge` mode with `remove_orphans: true`
- **THEN** no files SHALL be deleted (merge only copies, never deletes)

### Requirement: Orphan removal is disabled in newer-only mode
The system SHALL NOT remove orphans when sync mode is `NewerOnly`.

#### Scenario: Newer-only ignores orphan setting
- **WHEN** sync runs in `NewerOnly` mode with `remove_orphans: true`
- **THEN** no files SHALL be deleted (newer-only never deletes)
