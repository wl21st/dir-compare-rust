## 1. Refactor Comparison Architecture

- [ ] 1.1 Create `ComparisonStrategy` trait in `src/comparison.rs` with methods for comparison logic
- [ ] 1.2 Extract existing hierarchy-based comparison into `HierarchyComparison` struct implementing trait
- [ ] 1.3 Update comparison module to support strategy dispatch based on CLI flags
- [ ] 1.4 Ensure existing tests pass with refactored architecture

## 2. Implement Flat Mode Core Logic

- [ ] 2.1 Create `FlatComparison` struct implementing `ComparisonStrategy` trait
- [ ] 2.2 Implement file hashing function using existing `sample_hash()` utility
- [ ] 2.3 Implement content hash grouping algorithm (Phase 1: hash all files)
- [ ] 2.4 Implement duplicate detection within hash groups (Phase 2: identify duplicates)
- [ ] 2.5 Implement file movement matching across source/target (Phase 3: match groups)
- [ ] 2.6 Create data structure to represent flat mode comparison results (hash groups with file paths)

## 3. Extend CLI and Main Logic

- [ ] 3.1 Add `--flat` flag to CLI argument parser in `src/main.rs`
- [ ] 3.2 Add `--full-hash` flag to enable full-file hash verification in flat mode
- [ ] 3.3 Update main comparison dispatch logic to select `FlatComparison` when `--flat` is specified
- [ ] 3.4 Ensure `--flat` mode works with existing `--include` and `--exclude` pattern flags
- [ ] 3.5 Add validation to prevent incompatible flag combinations if needed

## 4. Output Formatting for Flat Mode

- [ ] 4.1 Extend output formatter to handle flat mode results (hash-grouped display)
- [ ] 4.2 Implement text format output for flat mode (group by hash with file counts and paths)
- [ ] 4.3 Implement HTML format output for flat mode with visual duplicate highlighting
- [ ] 4.4 Implement Markdown format output for flat mode reports
- [ ] 4.5 Update output module to clearly mark moved files and duplicates in all formats

## 5. Testing

- [ ] 5.1 Add unit tests for content hashing functions
- [ ] 5.2 Add unit tests for hash grouping algorithm
- [ ] 5.3 Add unit tests for duplicate detection logic
- [ ] 5.4 Add unit tests for file movement matching
- [ ] 5.5 Create integration test directories with known duplicate/moved file scenarios
- [ ] 5.6 Add integration tests for `--flat` flag with text output
- [ ] 5.7 Add integration tests for `--flat --full-hash` with text output
- [ ] 5.8 Add integration tests for flat mode with HTML and Markdown output
- [ ] 5.9 Add integration tests for flat mode with `--include` and `--exclude` patterns
- [ ] 5.10 Run full test suite (`cargo test`) and verify all tests pass

## 6. Documentation and Examples

- [ ] 6.1 Update README.md with flat mode feature description
- [ ] 6.2 Add CLI usage examples for flat mode in README
- [ ] 6.3 Add example showing flat mode with duplicate detection
- [ ] 6.4 Add example showing flat mode with file movement across structures
- [ ] 6.5 Document sampling hash vs full-file hash trade-offs in README
- [ ] 6.6 Add comparison of flat mode vs hierarchy mode use cases

## 7. Code Quality and Performance

- [ ] 7.1 Run `cargo clippy` and address any warnings
- [ ] 7.2 Run `cargo fmt` to ensure code formatting compliance
- [ ] 7.3 Profile flat mode performance with large test directories (1000+ files)
- [ ] 7.4 Verify memory usage is reasonable during hashing phase
- [ ] 7.5 Test edge cases: empty directories, single files, symbolic links if applicable

## 8. Final Verification

- [ ] 8.1 Run `cargo build` and `cargo build --release` successfully
- [ ] 8.2 Verify no compiler warnings or errors
- [ ] 8.3 Test flat mode with real-world directory structures
- [ ] 8.4 Verify output is clear and actionable for duplicate/moved file detection
- [ ] 8.5 Review all changes against design decisions and spec requirements
