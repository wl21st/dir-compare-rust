## Context

The current `dir-compare` tool is CLI-only, built as a single Rust binary with `clap` for argument parsing. The core comparison logic (`src/`) can be reused. The goal is to add a GUI frontend while maintaining the existing CLI.

**Constraints:**
- Must work on Windows, macOS, and Linux
- Binary size should remain reasonable
- GUI must support all existing comparison methods
- Core logic should be reusable from both CLI and GUI

## Goals / Non-Goals

**Goals:**
- Add a cross-platform GUI using a Rust-native framework
- Support all existing comparison methods (filename, size, hash) via the GUI
- Provide interactive tree/table view with click-to-expand folders
- Implement light/dark theme support
- Maintain CLI as the primary binary, GUI as a feature flag or separate binary

**Non-Goals:**
- Web-based or mobile GUI (desktop only)
- Changing core comparison behavior
- Real-time sync between CLI and GUI
- Cloud/multi-user features

## Decisions

### 1. GUI Framework: **egui**

**Decision:** Use `egui` with `eframe` for the GUI implementation.

**Rationale:**
- **Immediate mode** is well-suited for tools with simple state and frequent full redraws
- **Smallest binary size** (~3-5 MB) compared to iced (~8-12 MB)
- **Best cross-platform support** (Windows, macOS, Linux, WASM)
- **Fastest development velocity** - simple API, well-documented
- **WASM support** enables potential future web deployment
- No external system dependencies (unlike GTK)

**Alternatives considered:**
- **Iced**: More structured Elm-style architecture, better for complex state, but larger binaries and steeper learning curve. Better fit if the app grows to need multiple screens with complex navigation.
- **Slint**: QML-like declarative syntax, good for embedded, but less mature ecosystem.
- **Tauri**: Web-based frontend, adds complexity and security considerations.

### 2. Project Structure: **Workspace with Feature Flags**

**Decision:** Create a Cargo workspace with two binaries sharing a common library.

```
dir-compare/
├── Cargo.toml          # workspace
├── cli/Cargo.toml      # CLI binary (default)
├── gui/Cargo.toml      # GUI binary (egui-based)
└── core/Cargo.toml     # shared comparison logic
```

**Rationale:**
- Clean separation of concerns
- Users can install just CLI, just GUI, or both
- Feature flags allow conditional compilation

**Alternative:** Single binary with `--gui` flag. Rejected because it would require GUI framework as a compile-time dependency for all users, increasing binary size.

### 3. Core Logic Architecture

**Decision:** Move comparison logic to `core` crate, imported by both `cli` and `gui`.

```rust
// core/src/lib.rs
pub fn compare_directories(
    dir_a: &Path,
    dir_b: &Path,
    strategy: ComparisonStrategy
) -> ComparisonResult { ... }
```

**Rationale:**
- DRY: One implementation for both frontends
- Testing: Easier to unit test core logic in isolation
- Extensibility: Enables future frontends (HTTP API, etc.)

### 4. GUI Data Flow

**Decision:** Use egui's immediate mode with a central `AppState` struct.

```rust
struct AppState {
    dir_a_path: String,
    dir_b_path: String,
    comparison_method: ComparisonMethod,
    results: Option<ComparisonResult>,
    theme: Theme,
}
```

**Rationale:**
- Immediate mode fits egui's paradigm
- Simple state management for this app complexity
- No need for complex state machines or reducers

### 5. UI Layout

**Decision:** Three-panel layout:
1. **Top panel**: Path inputs, method selector, "Compare" button
2. **Middle panel**: Results as expandable tree view
3. **Bottom panel**: Statistics summary

**Rationale:**
- Familiar pattern (input → action → results)
- Maximizes vertical space for directory trees
- Clear separation of concerns

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| egui's immediate mode doesn't suit the UI | Medium | Test early; can switch to iced if needed |
| Binary size still larger than CLI-only | Low | Offer CLI-only build option |
| Cross-platform file picking inconsistencies | Medium | Use `rfd` (Rust File Dialogs) library |
| Accessibility not as good as native toolkits | Medium | egui has basic screen reader support; monitor upstream |
| Performance on very large directories | Medium | Add virtual scrolling to tree view |

## Migration Plan

**Phase 1: Core Extraction**
1. Create `core/` crate with shared comparison logic
2. Verify CLI still works with refactored code
3. Add integration tests

**Phase 2: GUI Scaffold**
1. Create `gui/` crate with basic egui window
2. Add path input fields and button
3. Connect to core comparison function

**Phase 3: UI Polish**
1. Implement tree/table view for results
2. Add theme switching (light/dark)
3. Add statistics panel

**Phase 4: Testing & Release**
1. Cross-platform testing (Windows, macOS, Linux)
2. Performance testing on large directories
3. Update documentation
4. Release as v1.0 with GUI support
