## 1. Project Setup

- [x] 1.1 Create workspace Cargo.toml with cli, gui, and core members
- [x] 1.2 Create core/Cargo.toml with lib crate type
- [x] 1.3 Create cli/Cargo.toml as bin target depending on core
- [x] 1.4 Create gui/Cargo.toml as bin target depending on core and egui
- [x] 1.5 Move src/lib.rs contents to core/src/lib.rs
- [x] 1.6 Move src/main.rs contents to cli/src/main.rs
- [x] 1.7 Update Cargo.toml dependencies for workspace structure
- [x] 1.8 Verify CLI builds and runs correctly after refactoring

## 2. Core Logic Architecture

- [x] 2.1 Define ComparisonStrategy enum (Filename, Size, Hash)
- [x] 2.2 Define ComparisonResult struct with a_only, b_only, both fields
- [x] 2.3 Extract compare_directories function to core crate
- [x] 2.4 Update cli to import and use core comparison logic
- [x] 2.5 Add unit tests for core comparison functions
- [x] 2.6 Verify CLI integration tests still pass

## 3. GUI Project Scaffold

- [x] 3.1 Add egui and eframe dependencies to gui/Cargo.toml
- [x] 3.2 Add rfd dependency for native file dialogs
- [x] 3.3 Create basic eframe Application struct
- [x] 3.4 Implement eframe::App trait with update method
- [x] 3.5 Create AppState struct with dir_a_path, dir_b_path, comparison_method, results, theme
- [x] 3.6 Set up central panel with application title
- [x] 3.7 Configure window to show "dir-compare" title bar
- [x] 3.8 Test window controls (minimize, maximize, close) work natively

## 4. Input Controls

- [x] 4.1 Add Directory A text input field with label
- [x] 4.2 Add Directory B text input field with label
- [x] 4.3 Implement path validation function checking directory existence
- [x] 4.4 Add visual validation indicator for valid paths (green check)
- [x] 4.5 Add visual error indicator for invalid paths (red X)
- [x] 4.6 Add Browse button for Directory A opening native file picker
- [x] 4.7 Add Browse button for Directory B opening native file picker
- [x] 4.8 Configure file pickers to filter for directories only
- [x] 4.9 Populate input fields when user selects directory from picker

## 5. Comparison Method Selector

- [x] 5.1 Create ComparisonMethod enum (Filename, Size, Hash)
- [x] 5.2 Add combobox or radio buttons for method selection
- [x] 5.3 Wire selected method to AppState.comparison_method
- [x] 5.4 Verify each method option updates the comparison behavior

## 6. Compare Button and Execution

- [x] 6.1 Add Compare button to UI
- [x] 6.2 Implement button enabled/disabled logic based on path validation
- [x] 6.3 Create async comparison function to avoid blocking UI thread
- [x] 6.4 Add loading indicator during comparison execution
- [x] 6.5 Display results when comparison completes
- [x] 6.6 Handle comparison errors with user-friendly error messages

## 7. Results Tree View

- [x] 7.1 Create TreeNode struct representing directory tree entries
- [x] 7.2 Implement tree data structure from ComparisonResult
- [x] 7.3 Add expandable/collapsible directory functionality
- [x] 7.4 Display "A-only" section with entries from dir_a only
- [x] 7.5 Display "B-only" section with entries from dir_b only
- [x] 7.6 Display "Both" section with matching entries
- [x] 7.7 Add expand/collapse icons for directory nodes
- [x] 7.8 Implement click handlers for tree node interaction

## 8. Visual Distinction

- [x] 8.1 Add folder icon rendering for directory entries
- [x] 8.2 Add file icon rendering for file entries
- [x] 8.3 Style A-only entries (e.g., red tint or indicator)
- [x] 8.4 Style B-only entries (e.g., green tint or indicator)
- [x] 8.5 Style matching entries (e.g., blue tint or indicator)

## 9. Statistics Summary

- [x] 9.1 Count and display number of A-only entries
- [x] 9.2 Count and display number of B-only entries
- [x] 9.3 Count and display number of matching entries
- [x] 9.4 Count and display total entries scanned
- [x] 9.5 Display statistics in bottom panel of window

## 10. Theming

- [x] 10.1 Create Theme enum (Light, Dark, System)
- [x] 10.2 Implement theme selector in settings or menu
- [x] 10.3 Wire theme selection to egui::Style changes
- [x] 10.4 Define light theme colors (background, text, accents)
- [x] 10.5 Define dark theme colors (background, text, accents)
- [x] 10.6 Persist theme preference to local storage
- [x] 10.7 Apply theme on application startup

## 11. Cross-Platform Testing

- [x] 11.1 Test GUI on macOS (Intel and Apple Silicon)
- [x] 11.2 Test GUI on Windows 10/11
- [x] 11.3 Test GUI on Linux (Ubuntu or similar)
- [x] 11.4 Verify file picker works on each platform
- [x] 11.5 Verify window controls work on each platform
- [x] 11.6 Test with directories containing Unicode characters
- [x] 11.7 Test with deep directory structures (100+ levels)

## 12. Performance Optimization

- [x] 12.1 Test with large directories (10,000+ files)
- [x] 12.2 Implement virtual scrolling for tree view if needed
- [x] 12.3 Profile memory usage during comparison
- [x] 12.4 Optimize comparison algorithm if performance issues found
- [x] 12.5 Verify binary size is within acceptable range

## 13. Documentation and Release

- [x] 13.1 Update README with GUI usage instructions
- [x] 13.2 Add screenshots to documentation
- [x] 13.3 Document installation options (CLI only, GUI only, both)
- [x] 13.4 Update Cargo.toml with proper metadata
- [x] 13.5 Create release notes for v1.0
- [x] 13.6 Test cargo install for both binaries
