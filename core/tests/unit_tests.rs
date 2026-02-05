#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn create_test_dir_with_files(base: &Path, name: &str, files: &[(&str, &[u8])]) -> PathBuf {
        let dir = base.join(name);
        fs::create_dir_all(&dir).unwrap();
        for (filename, content) in files {
            File::create(dir.join(filename))
                .unwrap()
                .write_all(content)
                .unwrap();
        }
        dir
    }

    #[test]
    fn test_compare_identical_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a = create_test_dir_with_files(
            temp_dir.path(),
            "dir_a",
            &[("file1.txt", b"content1"), ("file2.txt", b"content2")],
        );
        let dir_b = create_test_dir_with_files(
            temp_dir.path(),
            "dir_b",
            &[("file1.txt", b"content1"), ("file2.txt", b"content2")],
        );

        let strategy = dir_compare_core::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.a_only.len(), 0, "Expected no A-only entries");
        assert_eq!(result.b_only.len(), 0, "Expected no B-only entries");
        assert_eq!(result.both.len(), 2, "Expected 2 matching entries");
    }

    #[test]
    fn test_compare_directories_with_differences() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a = create_test_dir_with_files(
            temp_dir.path(),
            "dir_a",
            &[("file1.txt", b"content1"), ("unique_to_a.txt", b"unique")],
        );
        let dir_b = create_test_dir_with_files(
            temp_dir.path(),
            "dir_b",
            &[("file1.txt", b"content1"), ("file5.txt", b"content5")],
        );

        let strategy = dir_compare_core::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result
            .a_only
            .iter()
            .any(|e| e.path.to_string_lossy().contains("unique_to_a")));
        assert!(result
            .b_only
            .iter()
            .any(|e| e.path.to_string_lossy().contains("file5")));
    }

    #[test]
    fn test_compare_empty_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a = temp_dir.path().join("empty_a");
        let dir_b = temp_dir.path().join("empty_b");
        fs::create_dir_all(&dir_a).unwrap();
        fs::create_dir_all(&dir_b).unwrap();

        let strategy = dir_compare_core::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result.a_only.is_empty());
        assert!(result.b_only.is_empty());
        assert!(result.both.is_empty());
    }

    #[test]
    fn test_filename_size_strategy() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a =
            create_test_dir_with_files(temp_dir.path(), "dir_a", &[("file.txt", b"content")]);
        let dir_b = create_test_dir_with_files(
            temp_dir.path(),
            "dir_b",
            &[("file.txt", b"different content")],
        );

        let strategy = dir_compare_core::comparison::FilenameSizeStrategy::new(false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(
            result.both.len(),
            0,
            "Files with different sizes should not match"
        );
        assert_eq!(result.a_only.len(), 1, "Expected 1 A-only entry");
        assert_eq!(result.b_only.len(), 1, "Expected 1 B-only entry");
    }

    #[test]
    fn test_directory_entries_sorted() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a = create_test_dir_with_files(
            temp_dir.path(),
            "dir_a",
            &[
                ("file3.txt", b"c"),
                ("file1.txt", b"a"),
                ("file2.txt", b"b"),
            ],
        );
        let dir_b = create_test_dir_with_files(
            temp_dir.path(),
            "dir_b",
            &[
                ("file3.txt", b"c"),
                ("file1.txt", b"a"),
                ("file2.txt", b"b"),
            ],
        );

        let strategy = dir_compare_core::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy);

        assert!(result.is_ok());
        let result = result.unwrap();

        for i in 1..result.both.len() {
            assert!(
                result.both[i - 1].0.path <= result.both[i].0.path,
                "Entries should be sorted"
            );
        }
    }

    #[test]
    fn test_nested_directory_comparison() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a = temp_dir.path().join("dir_a");
        let dir_b = temp_dir.path().join("dir_b");

        fs::create_dir_all(&dir_a.join("level1").join("level2")).unwrap();
        fs::create_dir_all(&dir_b.join("level1").join("level2")).unwrap();
        fs::write(
            dir_a.join("level1").join("level2").join("nested.txt"),
            b"content",
        )
        .unwrap();
        fs::write(
            dir_b.join("level1").join("level2").join("nested.txt"),
            b"content",
        )
        .unwrap();

        let strategy = dir_compare_core::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(
            result
                .both
                .iter()
                .any(|(a, _)| a.path.to_string_lossy().contains("nested")),
            "Expected nested.txt in both directories"
        );
    }
}
