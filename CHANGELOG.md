# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0]

### Breaking Changes

- **CLI Interface Simplification**: Removed `--dir-a` and `--dir-b` named arguments in favor of positional arguments.
  - **Before**: `dir-compare -a /path/to/dir1 -b /path/to/dir2`
  - **After**: `dir-compare /path/to/dir1 /path/to/dir2`
  - All other flags (`--method`, `--format`, `--output`, `--case-insensitive`, `--verify`) remain unchanged and work the same way.
  - This improves usability by aligning with standard Unix directory comparison tools like `diff`.

### Changed

- Updated help text to reflect new positional argument syntax
- Updated error messages to reference positional arguments instead of named flags
- Updated documentation (README.md) with new CLI invocation examples

## [0.1.0] - Previous Release

See RELEASE_NOTES.md for v1.0.0 features and details.
