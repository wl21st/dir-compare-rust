use std::fs;
use tempfile::TempDir;

/// Creates a standard test directory structure with various file types
pub fn create_test_dir_structure() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    // Directory A only files
    fs::write(dir_a.path().join("a_only.txt"), "content A").unwrap();
    fs::create_dir(dir_a.path().join("a_only_dir")).unwrap();
    fs::write(dir_a.path().join("a_only_dir/nested.txt"), "nested A").unwrap();

    // Directory B only files
    fs::write(dir_b.path().join("b_only.txt"), "content B").unwrap();
    fs::create_dir(dir_b.path().join("b_only_dir")).unwrap();
    fs::write(dir_b.path().join("b_only_dir/nested.txt"), "nested B").unwrap();

    // Common files with same content
    fs::write(dir_a.path().join("common.txt"), "common content").unwrap();
    fs::write(dir_b.path().join("common.txt"), "common content").unwrap();

    // Files with same name but different sizes
    fs::write(dir_a.path().join("different_size.txt"), "short").unwrap();
    fs::write(
        dir_b.path().join("different_size.txt"),
        "this is a much longer content",
    )
    .unwrap();

    // Nested common directories
    fs::create_dir_all(dir_a.path().join("common_dir/subdir")).unwrap();
    fs::create_dir_all(dir_b.path().join("common_dir/subdir")).unwrap();
    fs::write(dir_a.path().join("common_dir/file.txt"), "dir file A").unwrap();
    fs::write(dir_b.path().join("common_dir/file.txt"), "dir file B").unwrap();

    (dir_a, dir_b)
}

/// Creates empty directories for edge case testing
pub fn create_empty_dirs() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");
    // Both directories are empty
    (dir_a, dir_b)
}

/// Creates a deeply nested directory structure (100 levels)
pub fn create_deeply_nested_dir() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    let mut path_a = dir_a.path().to_path_buf();
    let mut path_b = dir_b.path().to_path_buf();

    for i in 0..100 {
        path_a.push(format!("level_{}", i));
        path_b.push(format!("level_{}", i));
        fs::create_dir(&path_a).unwrap();
        fs::create_dir(&path_b).unwrap();
    }

    fs::write(path_a.join("deep_file.txt"), "deep content").unwrap();
    fs::write(path_b.join("deep_file.txt"), "deep content").unwrap();

    (dir_a, dir_b)
}

/// Creates directories with unicode filenames for internationalization testing
pub fn create_unicode_dirs() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    // Chinese
    fs::write(dir_a.path().join("文件.txt"), "chinese A").unwrap();
    fs::write(dir_b.path().join("文件.txt"), "chinese B").unwrap();

    // Russian
    fs::write(dir_a.path().join("файл.txt"), "russian A").unwrap();
    fs::write(dir_b.path().join("файл.txt"), "russian B").unwrap();

    // Japanese
    fs::write(dir_a.path().join("ファイル.txt"), "japanese A").unwrap();
    fs::write(dir_b.path().join("ファイル.txt"), "japanese B").unwrap();

    // Emoji
    fs::write(dir_a.path().join("🎉.txt"), "emoji A").unwrap();
    fs::write(dir_b.path().join("🎉.txt"), "emoji B").unwrap();

    // Mixed unicode directory
    fs::create_dir(dir_a.path().join("文件夹 📁")).unwrap();
    fs::create_dir(dir_b.path().join("文件夹 📁")).unwrap();
    fs::write(dir_a.path().join("文件夹 📁/внутри.txt"), "inside A").unwrap();
    fs::write(dir_b.path().join("文件夹 📁/внутри.txt"), "inside B").unwrap();

    (dir_a, dir_b)
}

/// Creates directories for permission error testing
/// Note: This only works on Unix systems. On Windows, permissions work differently.
pub fn create_permission_denied_dir() -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    // Create a directory that will have restricted permissions
    let restricted = dir_a.path().join("restricted");
    fs::create_dir(&restricted).unwrap();
    fs::write(restricted.join("secret.txt"), "secret").unwrap();

    // Remove read permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&restricted).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&restricted, perms).unwrap();
    }

    // Create a normal file in dir_b for comparison
    fs::write(dir_b.path().join("normal.txt"), "normal content").unwrap();

    (dir_a, dir_b)
}

/// Creates directories with many files for performance testing
pub fn create_many_files_dir(count: usize) -> (TempDir, TempDir) {
    let dir_a = TempDir::new().expect("Failed to create temp dir A");
    let dir_b = TempDir::new().expect("Failed to create temp dir B");

    for i in 0..count {
        // Some files only in A
        if i % 3 == 0 {
            fs::write(
                dir_a.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
        }
        // Some files only in B
        else if i % 3 == 1 {
            fs::write(
                dir_b.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
        }
        // Some files in both
        else {
            fs::write(
                dir_a.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
            fs::write(
                dir_b.path().join(format!("file_{}.txt", i)),
                format!("content {}", i),
            )
            .unwrap();
        }
    }

    (dir_a, dir_b)
}

/// Helper to restore permissions on cleanup (for permission tests)
#[cfg(unix)]
pub fn restore_permissions(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(mut perms) = fs::metadata(path).map(|m| m.permissions()) {
        perms.set_mode(0o755);
        let _ = fs::set_permissions(path, perms);
    }
}
