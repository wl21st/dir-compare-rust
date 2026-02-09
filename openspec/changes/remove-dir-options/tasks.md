## 1. Update CLI Argument Parser

- [ ] 1.1 Replace `--dir-a` and `--dir-b` named arguments with two positional arguments in Args struct
- [ ] 1.2 Refactor Args struct to use `Vec<PathBuf>` or two separate positional fields with correct clap attributes
- [ ] 1.3 Validate that exactly 2 positional arguments are provided
- [ ] 1.4 Update main.rs error messages to reference positional arguments instead of named flags

## 2. Update CLI Help and Documentation

- [ ] 2.1 Update the `#[command(about = ...)]` or add examples showing new syntax
- [ ] 2.2 Verify `cargo run -- --help` displays correct usage with positional args
- [ ] 2.3 Update README.md with new CLI invocation syntax and examples

## 3. Update Tests

- [ ] 3.1 Update `tests/cli_tests.rs` to use positional arguments in all assert_cmd invocations
- [ ] 3.2 Add test case for missing positional arguments (error scenario)
- [ ] 3.3 Add test case for extra positional arguments (error scenario)
- [ ] 3.4 Verify all CLI integration tests pass with new syntax

## 4. Verification and Release Prep

- [ ] 4.1 Run full test suite (`cargo test`) and confirm all tests pass
- [ ] 4.2 Run `cargo clippy` to check for linting issues
- [ ] 4.3 Verify the tool works end-to-end with new syntax: `cargo run -- dir1 dir2`
- [ ] 4.4 Update CHANGELOG with breaking change note
- [ ] 4.5 Verify no leftover `--dir-a` or `--dir-b` references in code or docs
