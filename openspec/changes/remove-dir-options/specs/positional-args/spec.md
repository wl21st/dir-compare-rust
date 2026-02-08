## ADDED Requirements

### Requirement: CLI accepts two positional arguments for directory paths
The CLI application SHALL accept two positional command-line arguments representing the directories to compare, eliminating the need for `--dir-a` and `--dir-b` named flags.

#### Scenario: User provides two directories as positional arguments
- **WHEN** user runs `dir-compare /path/to/dir1 /path/to/dir2`
- **THEN** the application compares dir1 and dir2 and outputs results

#### Scenario: User provides only one positional argument
- **WHEN** user runs `dir-compare /path/to/dir1` (missing second argument)
- **THEN** the application displays an error message requiring two directory arguments

#### Scenario: User provides more than two positional arguments
- **WHEN** user runs `dir-compare /path/to/dir1 /path/to/dir2 /path/to/dir3`
- **THEN** the application displays an error message indicating unexpected extra arguments

### Requirement: Named directory flags are removed
The `--dir-a` and `--dir-b` flags SHALL no longer be accepted by the CLI, as they are replaced by positional arguments.

#### Scenario: User attempts to use old --dir-a flag
- **WHEN** user runs `dir-compare --dir-a /path/to/dir1 --dir-b /path/to/dir2`
- **THEN** the application displays an error indicating unrecognized flags

#### Scenario: Mixed positional and named arguments
- **WHEN** user runs `dir-compare /path/to/dir1 --dir-b /path/to/dir2`
- **THEN** the application displays an error about unexpected flags or arguments

### Requirement: Help text reflects new positional argument interface
The help text (displayed via `--help` or `-h`) SHALL clearly show that the tool accepts two positional directory arguments instead of named flags.

#### Scenario: User requests help
- **WHEN** user runs `dir-compare --help`
- **THEN** help text shows the usage as `dir-compare <DIR1> <DIR2>` or similar
