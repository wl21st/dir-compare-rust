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

        let strategy = dir_compare::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare::compare_directories(&dir_a, &dir_b, &strategy);

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

        let strategy = dir_compare::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare::compare_directories(&dir_a, &dir_b, &strategy);

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

        let strategy = dir_compare::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare::compare_directories(&dir_a, &dir_b, &strategy);

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

        let strategy = dir_compare::comparison::FilenameSizeStrategy::new(false);
        let result = dir_compare::compare_directories(&dir_a, &dir_b, &strategy);

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

        let strategy = dir_compare::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare::compare_directories(&dir_a, &dir_b, &strategy);

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

        let strategy = dir_compare::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare::compare_directories(&dir_a, &dir_b, &strategy);

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

    #[test]
    fn test_sampled_hash_strategy() {
        let temp_dir = tempfile::tempdir().unwrap();
        // Create large files
        // size = 5000.
        // S = 431. 2S = 862.
        // interior = 5000 - 862 = 4138.
        // step = 4138 / 6 = 689.
        // Sample ranges start at:
        // 1: 0 (end 431)
        // 2: 431 + 689 = 1120 (start)
        // Modification at 500 should be ignored by sampling-only.

        let mut content_a = vec![b'x'; 5000];
        let mut content_b = vec![b'x'; 5000];
        content_b[500] = b'y'; // Modification in gap between Sample 1 and 2

        let dir_a =
            create_test_dir_with_files(temp_dir.path(), "dir_a", &[("large.bin", &content_a)]);
        let dir_b =
            create_test_dir_with_files(temp_dir.path(), "dir_b", &[("large.bin", &content_b)]);

        // 1. Sampling only -> Should Match
        let strategy = dir_compare::comparison::SampledHashStrategy::new(false, false);
        let result = dir_compare::compare_directories(&dir_a, &dir_b, &strategy).unwrap();
        assert_eq!(
            result.both.len(),
            1,
            "Sampling only should match despite difference in unsampled region"
        );

        // 2. Verify on match -> Should NOT Match
        let strategy_verify = dir_compare::comparison::SampledHashStrategy::new(false, true);
        let result_verify =
            dir_compare::compare_directories(&dir_a, &dir_b, &strategy_verify).unwrap();
        assert_eq!(
            result_verify.both.len(),
            0,
            "Verify on match should detect difference"
        );
        assert_eq!(result_verify.a_only.len(), 1);
        assert_eq!(result_verify.b_only.len(), 1);

        // 3. Small files -> Full read always
        let dir_c = create_test_dir_with_files(temp_dir.path(), "dir_c", &[("small.txt", b"abc")]);
        let dir_d = create_test_dir_with_files(
            temp_dir.path(),
            "dir_d",
            &[("small.txt", b"abd")], // diff
        );
        let result_small = dir_compare::compare_directories(&dir_c, &dir_d, &strategy).unwrap();
        assert_eq!(result_small.both.len(), 0, "Small files should differ");
    }
}
