## Context

The GUI application uses egui/eframe framework and provides visual directory comparison. Current testing relies on manual verification. The CLI has automated tests, but GUI lacks coverage for interactive features like directory browsing, comparison workflows, and visual feedback.

Key components to test:
- `DirCompareApp`: Main application state and UI logic
- `AppState`: Directory paths, comparison settings, results
- `TreeCache`: Result transformation for tree view
- `Theme`: Light/Dark/System theme management
- `tree_view` module: Hierarchical file display

## Goals / Non-Goals

**Goals:**
- Create testable architecture for GUI components
- Add unit tests for state management and validation
- Add integration tests for end-to-end workflows
- Test all comparison methods through GUI paths
- Cover common user workflows and edge cases
- Ensure theme persistence works correctly

**Non-Goals:**
- Full visual regression testing (screenshot comparison)
- Testing actual file dialogs (use mocking)
- Performance benchmarks for large directories
- Cross-platform UI pixel-perfect testing
- Accessibility compliance testing

## Decisions

### 1. Test Architecture: Separate Test Utilities Module
**Decision**: Create `gui/src/test_utils.rs` for test helpers and fixtures.

**Rationale**: 
- Keeps production code clean
- Allows reuse across multiple test files
- Standardizes test directory creation

**Alternatives considered**:
- Inline test helpers in each test file (rejected: duplication)
- Separate test crate (rejected: overkill for this size)

### 2. Testing Strategy: Unit + Integration Tests
**Decision**: Use Rust's built-in test framework with `tempfile` for directory fixtures.

**Unit Tests** (`gui/src/` inline):
- Path validation logic
- Theme serialization/deserialization
- Tree node construction

**Integration Tests** (`gui/tests/`):
- Full comparison workflows
- Directory selection flows
- Error handling scenarios

**Rationale**: Balances coverage with maintainability. Unit tests for logic, integration for workflows.

### 3. Mocking File Dialogs
**Decision**: Abstract file dialog behind a trait for test injection.

**Rationale**: 
- Real file dialogs block and require user interaction
- Tests need to run headlessly in CI
- Trait allows mock implementation in tests

**Implementation**: 
```rust
trait FileDialogProvider {
    fn pick_folder(&self) -> Option<PathBuf>;
}

struct NativeFileDialog;
struct MockFileDialog { return_path: Option<PathBuf> }
```

### 4. Test Data Management
**Decision**: Use `tempfile::TempDir` with helper functions to create structured test directories.

**Rationale**:
- Automatic cleanup after tests
- No test pollution between runs
- Can simulate various scenarios (empty, nested, permissions)

**Helper functions**:
- `create_test_dir_structure()` - Standard comparison scenario
- `create_empty_dir()` - Edge case testing
- `create_deeply_nested_dir()` - Performance/stress testing
- `create_unicode_dir()` - Internationalization testing

### 5. Coverage Prioritization
**Decision**: Focus on state transitions and user-visible behaviors over UI rendering details.

**Priority 1 (Must have)**:
- Directory validation
- Comparison execution
- Results display
- Error handling

**Priority 2 (Should have)**:
- Theme switching
- Tree navigation
- Status bar updates

**Priority 3 (Nice to have)**:
- Animation smoothness
- Resize handling

## Risks / Trade-offs

**Risk**: GUI tests may be flaky due to async nature of comparisons  
**Mitigation**: Use channels for synchronization; add timeouts; retry on transient failures

**Risk**: egui immediate-mode architecture makes testing state changes difficult  
**Mitigation**: Test the `AppState` struct directly, not the rendering; extract logic from `update()` into testable functions

**Risk**: File system tests may fail on Windows vs Unix differences  
**Mitigation**: Use `std::path::Path` for cross-platform compatibility; avoid shell commands; test on CI for both platforms

**Risk**: Tests run slowly with real file I/O  
**Mitigation**: Keep test directories small (< 100 files); use in-memory filesystem where possible; parallel test execution

**Risk**: Theme testing requires config directory access  
**Mitigation**: Use `tempfile` for config directory in tests; set via environment variable or dependency injection

## Migration Plan

**Phase 1: Foundation** (This change)
1. Add `test_utils.rs` with fixture helpers
2. Create `FileDialogProvider` trait and refactor `main.rs`
3. Add unit tests for `validate_path()`, theme functions
4. Add first integration test for basic comparison

**Phase 2: Coverage** (Follow-up change)
1. Add tests for all comparison methods
2. Add corner case tests
3. Add CI integration

**Phase 3: CI/CD Integration** (Follow-up change)
1. Add headless GUI test support
2. Configure GitHub Actions job
3. Add coverage reporting

**Rollback**: Tests are additive - simply don't run them if issues arise. No production code changes except minor refactorings for testability.

## Open Questions

1. **Headless Testing**: Can eframe run headlessly in CI? Need to verify if `eframe` supports offscreen rendering or if we need feature flags to disable UI in tests.

2. **Platform Coverage**: Should tests run on Windows, macOS, and Linux in CI? Likely yes, but may be slow.

3. **Test Duration**: What's acceptable test duration? Target: < 30 seconds for full GUI test suite.
