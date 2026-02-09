## ADDED Requirements

### Requirement: Ignore default version control directories
The application SHALL ignore `.git` directories during directory comparison by default.

#### Scenario: Comparing directories with git history
- **WHEN** user compares two directories containing `.git` folders without specifying exclude options
- **THEN** the `.git` folders and their contents are not included in the comparison output

### Requirement: Ignore files matching user patterns
The application SHALL allow users to specify patterns to exclude from the comparison using the `--exclude` flag.

#### Scenario: Excluding a specific directory
- **WHEN** user runs comparison with `--exclude "node_modules"`
- **THEN** any directory named `node_modules` is skipped

#### Scenario: Excluding by file extension
- **WHEN** user runs comparison with `--exclude "*.log"`
- **THEN** all files ending in `.log` are skipped

### Requirement: Include only files matching user patterns
The application SHALL allow users to specify patterns to exclusively include in the comparison using the `--include` flag.

#### Scenario: Including only source files
- **WHEN** user runs comparison with `--include "*.rs"`
- **THEN** only files ending in `.rs` are compared; all other files are ignored

#### Scenario: Including multiple patterns
- **WHEN** user runs comparison with `--include "*.rs"` and `--include "*.toml"`
- **THEN** only `.rs` and `.toml` files are compared

### Requirement: Respect .gitignore files
The application SHALL respect ignore rules defined in `.gitignore` files found within the compared directories.

#### Scenario: Ignoring build artifacts
- **WHEN** a `.gitignore` file contains `/target`
- **THEN** the `target` directory is excluded from comparison
