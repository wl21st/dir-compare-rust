## 1. Setup and Dependencies

- [ ] 1.1 Add `ignore` crate to `Cargo.toml`
- [ ] 1.2 Update CLI arguments in `src/main.rs` to support `--exclude`, `--include`, and `--no-ignore`

## 2. Core Implementation

- [ ] 2.1 Refactor `src/comparison.rs` to use `ignore::WalkBuilder` instead of `walkdir`
- [ ] 2.2 Implement logic to apply `--exclude` patterns to the walker
- [ ] 2.3 Implement logic to apply `--include` patterns (as overrides or whitelists)
- [ ] 2.4 Ensure default ignore rules (like `.git`) are respected unless disabled

## 3. Testing and Verification

- [ ] 3.1 Create integration tests for basic file exclusion
- [ ] 3.2 Create integration tests for file inclusion
- [ ] 3.3 Create integration tests for `.gitignore` support
- [ ] 3.4 Verify performance and correctness with existing test suite
