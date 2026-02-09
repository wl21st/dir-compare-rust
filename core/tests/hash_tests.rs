#[cfg(test)]
mod tests {
    use dir_compare_core::compare_directories;
    use dir_compare_core::comparison::FastHashStrategy;
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
    fn test_fast_hash_strategy_match() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a =
            create_test_dir_with_files(temp_dir.path(), "dir_a", &[("file.txt", b"same content")]);
        let dir_b =
            create_test_dir_with_files(temp_dir.path(), "dir_b", &[("file.txt", b"same content")]);

        let strategy = FastHashStrategy::new(false);
        let result = compare_directories(&dir_a, &dir_b, &strategy, None);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.both.len(), 1, "Expected matching content to match");
        assert_eq!(result.a_only.len(), 0);
        assert_eq!(result.b_only.len(), 0);
    }

    #[test]
    fn test_fast_hash_strategy_mismatch() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a =
            create_test_dir_with_files(temp_dir.path(), "dir_a", &[("file.txt", b"content A")]);
        let dir_b =
            create_test_dir_with_files(temp_dir.path(), "dir_b", &[("file.txt", b"content B")]);

        let strategy = FastHashStrategy::new(false);
        let result = compare_directories(&dir_a, &dir_b, &strategy, None);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(
            result.both.len(),
            0,
            "Expected different content to NOT match"
        );
        assert_eq!(result.a_only.len(), 1);
        assert_eq!(result.b_only.len(), 1);
    }
}
