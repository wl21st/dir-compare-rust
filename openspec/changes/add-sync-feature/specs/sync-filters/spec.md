## ADDED Requirements

### Requirement: Include filter restricts sync to matching files
The system SHALL support include patterns that limit sync to files matching at least one pattern.

#### Scenario: Include single pattern
- **WHEN** sync has include pattern `*.rs`
- **THEN** only files matching `*.rs` SHALL be considered for sync operations

#### Scenario: Include multiple patterns
- **WHEN** sync has include patterns `["*.rs", "*.toml"]`
- **THEN** files matching ANY of the patterns SHALL be considered for sync

#### Scenario: Include pattern with no matches
- **WHEN** sync has include pattern that matches no files
- **THEN** sync plan SHALL contain zero operations

#### Scenario: No include pattern
- **WHEN** sync has no include patterns specified
- **THEN** all files SHALL be considered for sync (subject to exclude patterns)

### Requirement: Exclude filter prevents sync of matching files
The system SHALL support exclude patterns that skip files matching any pattern.

#### Scenario: Exclude single pattern
- **WHEN** sync has exclude pattern `*.tmp`
- **THEN** files matching `*.tmp` SHALL NOT be synced

#### Scenario: Exclude multiple patterns
- **WHEN** sync has exclude patterns `["*.tmp", "*.log", ".git/**"]`
- **THEN** files matching ANY of the patterns SHALL be excluded

#### Scenario: Exclude takes precedence over include
- **WHEN** sync has include `*.txt` and exclude `secret*.txt`
- **THEN** `secret.txt` SHALL NOT be synced even though it matches include

### Requirement: Filter patterns use glob syntax
The system SHALL interpret filter patterns as glob patterns.

#### Scenario: Wildcard matches any characters
- **WHEN** pattern is `*.rs`
- **THEN** it SHALL match `main.rs`, `lib.rs`, `foo.rs`

#### Scenario: Double wildcard matches directories
- **WHEN** pattern is `src/**/*.rs`
- **THEN** it SHALL match `src/main.rs`, `src/lib/mod.rs`, `src/a/b/c.rs`

#### Scenario: Question mark matches single character
- **WHEN** pattern is `file?.txt`
- **THEN** it SHALL match `file1.txt`, `fileA.txt` but NOT `file12.txt`

#### Scenario: Patterns are case-sensitive by default
- **WHEN** pattern is `*.RS`
- **THEN** it SHALL NOT match `main.rs` (lowercase)

### Requirement: Filters apply to relative paths
The system SHALL match patterns against file paths relative to sync root.

#### Scenario: Pattern matches relative path
- **WHEN** syncing `/home/user/project` with pattern `src/*.rs`
- **THEN** pattern SHALL match against `src/main.rs` not `/home/user/project/src/main.rs`

#### Scenario: Directory patterns
- **WHEN** pattern is `target/`
- **THEN** it SHALL match the `target` directory and all its contents

### Requirement: Filter configuration in sync options
The system SHALL accept filters as part of `SyncOptions`.

#### Scenario: Empty filters
- **WHEN** both include and exclude filters are empty
- **THEN** all files SHALL be synced (no filtering)

#### Scenario: Filters affect both directions in merge mode
- **WHEN** sync runs in `Merge` mode with filters
- **THEN** filters SHALL apply to operations in both directions
