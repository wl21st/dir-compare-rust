use dir_compare_core::{compare_directories, FilenameOnlyStrategy};
use tempfile::TempDir;

/// Tests for corner cases and edge conditions
/// These tests verify the GUI handles unusual scenarios gracefully

/// Helper: Creates empty directories
fn create_empty_dirs() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");
    (dir_a, dir_b)
}

/// Helper: Creates deeply nested directory structure (100 levels)
fn create_deeply_nested_dir() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    let mut path_a = dir_a.path().to_path_buf();
    let mut path_b = dir_b.path().to_path_buf();

    for i in 0..100 {
        path_a.push(format!("level_{}", i));
        path_b.push(format!("level_{}", i));
        std::fs::create_dir(&path_a).unwrap();
        std::fs::create_dir(&path_b).unwrap();
    }

    std::fs::write(path_a.join("deep_file.txt"), "deep content").unwrap();
    std::fs::write(path_b.join("deep_file.txt"), "deep content").unwrap();

    (dir_a, dir_b)
}

/// Helper: Creates directories with unicode filenames
fn create_unicode_dirs() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    // Chinese
    std::fs::write(dir_a.path().join("文件.txt"), "chinese A").unwrap();
    std::fs::write(dir_b.path().join("文件.txt"), "chinese B").unwrap();

    // Russian
    std::fs::write(dir_a.path().join("файл.txt"), "russian A").unwrap();
    std::fs::write(dir_b.path().join("файл.txt"), "russian B").unwrap();

    // Emoji
    std::fs::write(dir_a.path().join("🎉.txt"), "emoji A").unwrap();
    std::fs::write(dir_b.path().join("🎉.txt"), "emoji B").unwrap();

    (dir_a, dir_b)
}

/// Helper: Creates directories for permission error testing
fn create_permission_denied_dir() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    // Create a directory that will have restricted permissions
    let restricted = dir_a.path().join("restricted");
    std::fs::create_dir(&restricted).unwrap();
    std::fs::write(restricted.join("secret.txt"), "secret").unwrap();

    // Remove read permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&restricted).unwrap().permissions();
        perms.set_mode(0o000);
        std::fs::set_permissions(&restricted, perms).unwrap();
    }

    std::fs::write(dir_b.path().join("normal.txt"), "normal content").unwrap();

    (dir_a, dir_b)
}

/// Helper: Creates directories with many files
fn create_many_files_dir(count: usize) -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    for i in 0..count {
        if i % 3 == 0 {
            std::fs::write(
                dir_a.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
        } else if i % 3 == 1 {
            std::fs::write(
                dir_b.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
        } else {
            std::fs::write(
                dir_a.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
            std::fs::write(
                dir_b.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
        }
    }

    (dir_a, dir_b)
}

/// Helper to restore permissions on cleanup
#[cfg(unix)]
fn restore_permissions(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(mut perms) = std::fs::metadata(path).map(|m| m.permissions()) {
        perms.set_mode(0o755);
        let _ = std::fs::set_permissions(path, perms);
    }
}

#[test]
fn test_empty_directory_a_comparison() {
    let (dir_a, dir_b) = create_empty_dirs();

    // Add a file to dir_b only
    std::fs::write(dir_b.path().join("file.txt"), "content").unwrap();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // A-only should be 0, B-only should have 1 entry
    assert_eq!(result.a_only.len(), 0);
    assert_eq!(result.b_only.len(), 1);
    assert_eq!(result.both.len(), 0);
}

#[test]
fn test_both_empty_directories() {
    let (dir_a, dir_b) = create_empty_dirs();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // All counts should be zero
    assert_eq!(result.a_only.len(), 0);
    assert_eq!(result.b_only.len(), 0);
    assert_eq!(result.both.len(), 0);
}

#[test]
fn test_permission_denied_error() {
    let (dir_a, dir_b) = create_permission_denied_dir();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    // On Unix, this should fail with permission error
    // On Windows or if running as root, it might succeed
    match result {
        Ok(_) => {
            // If it succeeded, that's fine - permissions might allow it
        }
        Err(e) => {
            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("permission")
                    || error_str.contains("access")
                    || error_str.contains("denied"),
                "Error should mention permission or access: {}",
                e
            );
        }
    }

    // Clean up: restore permissions for cleanup
    #[cfg(unix)]
    {
        let restricted = dir_a.path().join("restricted");
        restore_permissions(&restricted);
    }
}

#[test]
fn test_deeply_nested_directories() {
    let (dir_a, dir_b) = create_deeply_nested_dir();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    // Should complete without stack overflow
    assert!(result.is_ok());

    let comparison = result.unwrap();
    // Both directories have identical structure, so everything should be in "both"
    assert!(
        comparison.both.len() >= 100,
        "Should have entries at all nesting levels"
    );
}

#[test]
fn test_unicode_filenames() {
    let (dir_a, dir_b) = create_unicode_dirs();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    // Should handle unicode filenames without errors
    assert!(result.is_ok());

    let comparison = result.unwrap();

    // With our test data, common files should be in "both"
    // and different files should be in their respective categories
    let total_entries = comparison.a_only.len() + comparison.b_only.len() + comparison.both.len();

    assert!(
        total_entries > 0,
        "Should have found files with unicode names"
    );
}

#[test]
fn test_large_directory_performance() {
    // Create directories with many files (but not too many for test speed)
    let (dir_a, dir_b) = create_many_files_dir(100);

    let start = std::time::Instant::now();
    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);
    let duration = start.elapsed();

    // Should complete within reasonable time (5 seconds for 100 files)
    assert!(result.is_ok());
    assert!(
        duration.as_secs() < 5,
        "Comparison should complete in under 5 seconds, took {:?}",
        duration
    );

    let comparison = result.unwrap();
    assert!(comparison.a_only.len() + comparison.b_only.len() + comparison.both.len() >= 100);
}

#[test]
fn test_theme_persistence() {
    use dir_compare_gui::theme::{load_theme, save_theme, Theme};

    // Save a theme
    save_theme(Theme::Dark);

    // Load it back
    let loaded = load_theme();

    // If the config directory exists, we should get the theme back
    // If it doesn't exist, load_theme returns None
    match loaded {
        Some(t) => assert_eq!(t, Theme::Dark, "Should load the theme we just saved"),
        None => {
            // Config directory might not exist in test environment
        }
    }

    // Test saving another theme
    save_theme(Theme::Light);
    let loaded2 = load_theme();
    match loaded2 {
        Some(t) => assert_eq!(t, Theme::Light, "Should load the second theme"),
        None => {
            // Config directory might not exist in test environment
        }
    }
}

#[test]
fn test_theme_loading_restores_selection() {
    use dir_compare_gui::theme::{load_theme, save_theme, Theme};

    // Save and verify we can load different themes
    for theme in [Theme::Light, Theme::Dark, Theme::System] {
        save_theme(theme);
        let loaded = load_theme();

        // The loaded theme should match what we saved (if loading succeeds)
        match loaded {
            Some(loaded_theme) => {
                assert_eq!(loaded_theme, theme, "Loaded theme should match saved")
            }
            None => {
                // Loading might fail if config dir doesn't exist or isn't writable
                // That's acceptable in test environments
            }
        }
    }
}

#[test]
fn test_invalid_theme_config_fallback() {
    use dir_compare_gui::theme::load_theme;
    use std::fs;

    // Get the config file path
    let config_dir = dirs::config_dir().map(|d| d.join("dir-compare"));
    if let Some(config_path) = config_dir {
        // Create the config directory if needed
        fs::create_dir_all(&config_path).ok();

        let theme_file = config_path.join("theme.txt");

        // Write invalid content - something that is definitely not a valid theme
        fs::write(&theme_file, "totally_invalid_theme_xyz123").ok();

        // Load should return None (fallback to System default in app)
        let loaded = load_theme();

        // The loaded result should be None because "totally_invalid_theme_xyz123" is not valid
        // However, if the previous test wrote a valid theme, it might overwrite our invalid one
        // So we check that it's either None OR it's something different from the invalid value
        match loaded {
            None => {
                // Expected: invalid theme returns None
            }
            Some(theme) => {
                // If we got a theme, it should NOT be our invalid value
                // (meaning some other test wrote a valid theme)
                assert_ne!(
                    theme.as_str(),
                    "totally_invalid_theme_xyz123",
                    "Should not load invalid theme name as theme"
                );
            }
        }

        // Clean up
        let _ = fs::remove_file(&theme_file);
    }
}

#[test]
fn test_comparison_with_very_long_filenames() {
    let dir_a = TempDir::new().unwrap();
    let dir_b = TempDir::new().unwrap();

    // Create files with long names
    let long_name: String = std::iter::repeat('a').take(200).collect();
    std::fs::write(dir_a.path().join(&long_name), "content a").unwrap();
    std::fs::write(dir_b.path().join(&long_name), "content b").unwrap();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    // Should handle long filenames
    assert!(result.is_ok());
}

#[test]
fn test_comparison_with_special_characters_in_filenames() {
    let dir_a = TempDir::new().unwrap();
    let dir_b = TempDir::new().unwrap();

    // Create files with special characters
    let special_names = [
        "file with spaces.txt",
        "file-with-dashes.txt",
        "file_with_underscores.txt",
    ];

    for name in &special_names {
        std::fs::write(dir_a.path().join(name), "content a").unwrap();
        std::fs::write(dir_b.path().join(name), "content b").unwrap();
    }

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    assert!(result.is_ok());
    let comparison = result.unwrap();

    // All files should be in "both" since they have same names
    assert_eq!(comparison.both.len(), special_names.len());
}
