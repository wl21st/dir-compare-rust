## 1. Project Setup

- [ ] 1.1 Create workspace Cargo.toml with cli, gui, and core members
- [ ] 1.2 Create core/Cargo.toml with lib crate type
- [ ] 1.3 Create cli/Cargo.toml as bin target depending on core
- [ ] 1.4 Create gui/Cargo.toml as bin target depending on core and egui
- [ ] 1.5 Move src/lib.rs contents to core/src/lib.rs
- [ ] 1.6 Move src/main.rs contents to cli/src/main.rs
- [ ] 1.7 Update Cargo.toml dependencies for workspace structure
- [ ] 1.8 Verify CLI builds and runs correctly after refactoring

## 2. Core Logic Architecture

- [ ] 2.1 Define ComparisonStrategy enum (Filename, Size, Hash)
- [ ] 2.2 Define ComparisonResult struct with a_only, b_only, both fields
- [ ] 2.3 Extract compare_directories function to core crate
- [ ] 2.4 Update cli to import and use core comparison logic
- [ ] 2.5 Add unit tests for core comparison functions
- [ ] 2.6 Verify CLI integration tests still pass

## 3. GUI Project Scaffold

- [ ] 3.1 Add egui and eframe dependencies to gui/Cargo.toml
- [ ] 3.2 Add rfd dependency for native file dialogs
- [ ] 3.3 Create basic eframe Application struct
- [ ] 3.4 Implement eframe::App trait with update method
- [ ] 3.5 Create AppState struct with dir_a_path, dir_b_path, comparison_method, results, theme
- [ ] 3.6 Set up central panel with application title
- [ ] 3.7 Configure window to show "dir-compare" title bar
- [ ] 3.8 Test window controls (minimize, maximize, close) work natively

## 4. Input Controls

- [ ] 4.1 Add Directory A text input field with label
- [ ] 4.2 Add Directory B text input field with label
- [ ] 4.3 Implement path validation function checking directory existence
- [ ] 4.4 Add visual validation indicator for valid paths (green check)
- [ ] 4.5 Add visual error indicator for invalid paths (red X)
- [ ] 4.6 Add Browse button for Directory A opening native file picker
- [ ] 4.7 Add Browse button for Directory B opening native file picker
- [ ] 4.8 Configure file pickers to filter for directories only
- [ ] 4.9 Populate input fields when user selects directory from picker

## 5. Comparison Method Selector

- [ ] 5.1 Create ComparisonMethod enum (Filename, Size, Hash)
- [ ] 5.2 Add combobox or radio buttons for method selection
- [ ] 5.3 Wire selected method to AppState.comparison_method
- [ ] 5.4 Verify each method option updates the comparison behavior

## 6. Compare Button and Execution

- [ ] 6.1 Add Compare button to UI
- [ ] 6.2 Implement button enabled/disabled logic based on path validation
- [ ] 6.3 Create async comparison function to avoid blocking UI thread
- [ ] 6.4 Add loading indicator during comparison execution
- [ ] 6.5 Display results when comparison completes
- [ ] 6.6 Handle comparison errors with user-friendly error messages

## 7. Results Tree View

- [ ] 7.1 Create TreeNode struct representing directory tree entries
- [ ] 7.2 Implement tree data structure from ComparisonResult
- [ ] 7.3 Add expandable/collapsible directory functionality
- [ ] 7.4 Display "A-only" section with entries from dir_a only
- [ ] 7.5 Display "B-only" section with entries from dir_b only
- [ ] 7.6 Display "Both" section with matching entries
- [ ] 7.7 Add expand/collapse icons for directory nodes
- [ ] 7.8 Implement click handlers for tree node interaction

## 8. Visual Distinction

- [ ] 8.1 Add folder icon rendering for directory entries
- [ ] 8.2 Add file icon rendering for file entries
- [ ] 8.3 Style A-only entries (e.g., red tint or indicator)
- [ ] 8.4 Style B-only entries (e.g., green tint or indicator)
- [ ] 8.5 Style matching entries (e.g., blue tint or indicator)

## 9. Statistics Summary

- [ ] 9.1 Count and display number of A-only entries
- [ ] 9.2 Count and display number of B-only entries
- [ ] 9.3 Count and display number of matching entries
- [ ] 9.4 Count and display total entries scanned
- [ ] 9.5 Display statistics in bottom panel of window

## 10. Theming

- [ ] 10.1 Create Theme enum (Light, Dark, System)
- [ ] 10.2 Implement theme selector in settings or menu
- [ ] 10.3 Wire theme selection to egui::Style changes
- [ ] 10.4 Define light theme colors (background, text, accents)
- [ ] 10.5 Define dark theme colors (background, text, accents)
- [ ] 10.6 Persist theme preference to local storage
- [ ] 10.7 Apply theme on application startup

## 11. Cross-Platform Testing

- [ ] 11.1 Test GUI on macOS (Intel and Apple Silicon)
- [ ] 11.2 Test GUI on Windows 10/11
- [ ] 11.3 Test GUI on Linux (Ubuntu or similar)
- [ ] 11.4 Verify file picker works on each platform
- [ ] 11.5 Verify window controls work on each platform
- [ ] 11.6 Test with directories containing Unicode characters
- [ ] 11.7 Test with deep directory structures (100+ levels)

## 12. Performance Optimization

- [ ] 12.1 Test with large directories (10,000+ files)
- [ ] 12.2 Implement virtual scrolling for tree view if needed
- [ ] 12.3 Profile memory usage during comparison
- [ ] 12.4 Optimize comparison algorithm if performance issues found
- [ ] 12.5 Verify binary size is within acceptable range

## 13. Documentation and Release

- [ ] 13.1 Update README with GUI usage instructions
- [ ] 13.2 Add screenshots to documentation
- [ ] 13.3 Document installation options (CLI only, GUI only, both)
- [ ] 13.4 Update Cargo.toml with proper metadata
- [ ] 13.5 Create release notes for v1.0
- [ ] 13.6 Test cargo install for both binaries
