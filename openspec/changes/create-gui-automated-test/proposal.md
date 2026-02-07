## Why

The GUI component provides visual directory comparison with interactive features like directory selection, comparison method selection, and tree views. Currently, there's no automated testing for the GUI, making it difficult to ensure reliability when adding new features or fixing bugs. Automated GUI tests will catch regressions early and provide confidence when refactoring.

## What Changes

- Add automated GUI testing framework using native Rust testing capabilities
- Create test suite covering common user workflows:
  - Directory selection validation
  - Comparison execution with different methods (filename, size, hash, sampled)
  - Results display in tree view
  - Theme switching
- Create test suite covering corner cases:
  - Empty directories
  - Permission-denied paths
  - Very deep directory structures
  - Unicode filenames
  - Large file counts
  - Cancelled comparisons
  - Invalid directory paths
- Add integration between GUI tests and CI/CD pipeline
- Document GUI testing approach for future developers

## Capabilities

### New Capabilities
- `gui-automated-testing`: Automated end-to-end testing framework for GUI components
- `gui-test-directory-selection`: Test directory picker validation and error handling
- `gui-test-comparison-methods`: Test all comparison methods through GUI
- `gui-test-results-display`: Test tree view rendering and navigation
- `gui-test-corner-cases`: Test edge cases like empty dirs, permissions, unicode

### Modified Capabilities
- (none - this is a new testing infrastructure addition)

## Impact

- **gui/**: New test files in `gui/tests/` directory, test utilities in `gui/src/test_utils.rs`
- **CI/CD**: New test job for GUI tests
- **Dependencies**: May add testing-focused dependencies to `gui/Cargo.toml`
- **Developer workflow**: Tests run with `cargo test` in gui workspace
