## Why

The current CLI requires users to specify both `--dir-a` and `--dir-b` arguments, which is overly verbose and unintuitive. Most directory comparison tools accept positional arguments for the directories being compared. This change simplifies the user interface and improves usability by supporting positional arguments instead.

## What Changes

- **BREAKING**: Remove `--dir-a` and `--dir-b` named arguments
- Add two positional arguments for the directories to compare
- Simplify CLI invocation from `dir-compare -a dir1 -b dir2` to `dir-compare dir1 dir2`
- Update help text and documentation to reflect new interface

## Capabilities

### New Capabilities
- `positional-args`: Support two positional command-line arguments for directory paths

### Modified Capabilities
<!-- No existing spec requirements are changing; this is a CLI interface simplification -->

## Impact

- **Code**: `cli/src/main.rs` - argument parsing changes
- **Tests**: Update CLI integration tests to use new positional argument syntax
- **Documentation**: README.md examples need updating
- **User-facing**: Breaking change for existing CLI users
