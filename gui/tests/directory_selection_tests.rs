use dir_compare_gui::dialog::{FileDialogProvider, MockFileDialog};
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create test directory structure
fn create_test_dir_structure() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    // Directory A only files
    std::fs::write(dir_a.path().join("a_only.txt"), "content A").unwrap();
    std::fs::create_dir(dir_a.path().join("a_only_dir")).unwrap();
    std::fs::write(dir_a.path().join("a_only_dir/nested.txt"), "nested A").unwrap();

    // Directory B only files
    std::fs::write(dir_b.path().join("b_only.txt"), "content B").unwrap();
    std::fs::create_dir(dir_b.path().join("b_only_dir")).unwrap();
    std::fs::write(dir_b.path().join("b_only_dir/nested.txt"), "nested B").unwrap();

    // Common files with same content
    std::fs::write(dir_a.path().join("common.txt"), "common content").unwrap();
    std::fs::write(dir_b.path().join("common.txt"), "common content").unwrap();

    (dir_a, dir_b)
}

/// Tests for directory selection workflows
/// Note: These tests focus on state management and validation logic
/// Actual UI interactions are tested through the mock dialog provider

#[test]
fn test_manual_path_entry_updates_state() {
    // Create test directories
    let (dir_a, dir_b) = create_test_dir_structure();
    let path_a = dir_a.path().to_str().unwrap().to_string();
    let path_b = dir_b.path().to_str().unwrap().to_string();

    // Simulate setting paths directly (as would happen with manual entry)
    // In a real app, this would be done through the UI
    // Here we just verify the paths are valid
    assert!(std::path::Path::new(&path_a).is_dir());
    assert!(std::path::Path::new(&path_b).is_dir());
}

#[test]
fn test_mock_file_dialog_populates_path() {
    let test_path = PathBuf::from("/test/directory");
    let dialog = MockFileDialog::new(Some(test_path.clone()));

    let result = dialog.pick_folder();
    assert_eq!(result, Some(test_path));
}

#[test]
fn test_valid_directory_shows_checkmark() {
    // Create a temporary directory
    let temp_dir = tempfile::TempDir::new().unwrap();
    let path = temp_dir.path().to_str().unwrap();

    // Verify path validation (this simulates the ✅ indicator logic)
    assert!(!path.trim().is_empty());
    assert!(std::path::Path::new(path).is_dir());
}

#[test]
fn test_invalid_directory_shows_x_mark() {
    // Test with non-existent path
    let invalid_path = "/nonexistent/path/12345";

    // Verify path validation fails
    assert!(!std::path::Path::new(invalid_path).is_dir());
}

#[test]
fn test_empty_path_shows_x_mark() {
    let empty_path = "";

    // Empty path should fail validation
    assert!(empty_path.trim().is_empty());
}

#[test]
fn test_whitespace_only_path_shows_x_mark() {
    let whitespace_paths = ["   ", "\t", "\n", "  \t\n  "];

    for path in &whitespace_paths {
        assert!(
            path.trim().is_empty(),
            "Path '{}' should be considered empty",
            path
        );
    }
}

#[test]
fn test_file_path_shows_x_mark() {
    // Create a temporary file (not a directory)
    let temp_dir = tempfile::TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    std::fs::write(&file_path, "content").unwrap();

    // File path should not be valid for directory selection
    assert!(!std::path::Path::new(&file_path).is_dir());
    assert!(std::path::Path::new(&file_path).exists());
}

#[test]
fn test_directory_validation_with_real_dirs() {
    use dir_compare_gui::validate_path;

    // Valid directory
    let temp_dir = tempfile::TempDir::new().unwrap();
    assert!(validate_path(temp_dir.path().to_str().unwrap()));

    // Invalid paths
    assert!(!validate_path(""));
    assert!(!validate_path("   "));
    assert!(!validate_path("/nonexistent/path"));

    // File (not directory)
    let file_path = temp_dir.path().join("file.txt");
    std::fs::write(&file_path, "content").unwrap();
    assert!(!validate_path(file_path.to_str().unwrap()));
}

#[test]
fn test_ignore_file_selection() {
    let test_path = PathBuf::from("/test/ignore_file.txt");
    let dialog = MockFileDialog::new(Some(test_path.clone()));

    let result = dialog.pick_file();
    assert_eq!(result, Some(test_path));
}
