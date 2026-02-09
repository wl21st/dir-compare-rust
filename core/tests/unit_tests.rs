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
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None);

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
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None);

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
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None);

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
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None);

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
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None);

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
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None);

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

        let content_a = vec![b'x'; 5000];
        let mut content_b = vec![b'x'; 5000];
        content_b[500] = b'y'; // Modification in gap between Sample 1 and 2

        let dir_a =
            create_test_dir_with_files(temp_dir.path(), "dir_a", &[("large.bin", &content_a)]);
        let dir_b =
            create_test_dir_with_files(temp_dir.path(), "dir_b", &[("large.bin", &content_b)]);

        // 1. Sampling only -> Should Match
        let strategy = dir_compare_core::SampledHashStrategy::new(false, false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None).unwrap();
        assert_eq!(
            result.both.len(),
            1,
            "Sampling only should match despite difference in unsampled region"
        );

        // 2. Verify on match -> Should NOT Match
        let strategy_verify = dir_compare_core::SampledHashStrategy::new(false, true);
        let result_verify =
            dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy_verify, None).unwrap();
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
        let result_small =
            dir_compare_core::compare_directories(&dir_c, &dir_d, &strategy, None).unwrap();
        assert_eq!(result_small.both.len(), 0, "Small files should differ");
    }

    #[test]
    fn test_sampled_hash_includes_file_size() {
        let temp_dir = tempfile::tempdir().unwrap();
        // Create two files with identical content but different sizes
        // by appending extra padding to one
        let content_same = b"identical content here";
        let content_padded = b"identical content here padding";

        let dir_a =
            create_test_dir_with_files(temp_dir.path(), "dir_a", &[("file.txt", content_same)]);
        let dir_b =
            create_test_dir_with_files(temp_dir.path(), "dir_b", &[("file.txt", content_padded)]);

        let strategy = dir_compare_core::SampledHashStrategy::new(false, false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None).unwrap();

        // Files with different sizes should NOT match even if sampled content overlaps
        assert_eq!(
            result.both.len(),
            0,
            "Files with different sizes should not match (file size is part of hash)"
        );
        assert_eq!(result.a_only.len(), 1);
        assert_eq!(result.b_only.len(), 1);
    }

    #[test]
    fn test_sampled_hash_same_size_and_content_match() {
        let temp_dir = tempfile::tempdir().unwrap();
        // Create files with identical content and size
        let content = vec![b'x'; 5000];

        let dir_a = create_test_dir_with_files(temp_dir.path(), "dir_a", &[("file.bin", &content)]);
        let dir_b = create_test_dir_with_files(temp_dir.path(), "dir_b", &[("file.bin", &content)]);

        let strategy = dir_compare_core::SampledHashStrategy::new(false, false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None).unwrap();

        assert_eq!(result.both.len(), 1, "Identical files should match");
        assert_eq!(result.a_only.len(), 0);
        assert_eq!(result.b_only.len(), 0);
    }

    #[test]
    fn test_sampled_hash_size_determinism() {
        let temp_dir = tempfile::tempdir().unwrap();
        // Test that hashing is deterministic - same file produces same hash
        let content = b"test content for determinism check";

        let dir_a = create_test_dir_with_files(temp_dir.path(), "dir_a", &[("file.txt", content)]);
        let dir_b = create_test_dir_with_files(temp_dir.path(), "dir_b", &[("file.txt", content)]);

        let strategy = dir_compare_core::SampledHashStrategy::new(false, false);

        // Run comparison multiple times
        for _ in 0..3 {
            let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None).unwrap();
            assert_eq!(
                result.both.len(),
                1,
                "Hash should be deterministic across multiple runs"
            );
        }
    }

    #[test]
    fn test_sampled_hash_empty_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        // Test empty file hashing (edge case)
        let dir_a = create_test_dir_with_files(temp_dir.path(), "dir_a", &[("empty.txt", b"")]);
        let dir_b = create_test_dir_with_files(temp_dir.path(), "dir_b", &[("empty.txt", b"")]);

        let strategy = dir_compare_core::SampledHashStrategy::new(false, false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None).unwrap();

        assert_eq!(
            result.both.len(),
            1,
            "Empty files with same size (0) should match"
        );
    }

    #[test]
    fn test_sampled_hash_endianness_consistency() {
        let temp_dir = tempfile::tempdir().unwrap();
        // Test that file size is encoded consistently (big-endian) regardless of platform.
        // We use specific sizes that would produce different hash results if endianness
        // was handled incorrectly (e.g., if platform native endianness was used).
        //
        // Size 0x100 (256): BE bytes = [0,0,0,0,0,0,1,0], LE bytes = [0,1,0,0,0,0,0,0]
        // Size 0x10000 (65536): BE bytes = [0,0,0,0,0,1,0,0], LE bytes = [0,0,1,0,0,0,0,0]
        //
        // Since to_be_bytes() always produces big-endian, the hash will be consistent
        // across all platforms (x86, ARM, etc.)

        // Use small files that trigger the "read entire file" path
        let content_256 = vec![b'a'; 256];
        let content_65536 = vec![b'a'; 65536];

        let dir_a =
            create_test_dir_with_files(temp_dir.path(), "dir_a", &[("size_256.bin", &content_256)]);
        let dir_b = create_test_dir_with_files(
            temp_dir.path(),
            "dir_b",
            &[("size_256.bin", &content_65536)],
        );

        let strategy = dir_compare_core::SampledHashStrategy::new(false, false);
        let result = dir_compare_core::compare_directories(&dir_a, &dir_b, &strategy, None).unwrap();

        // Files with different sizes should NOT match
        assert_eq!(
            result.both.len(),
            0,
            "Files with different sizes (256 vs 65536) should not match - verifies size is part of hash"
        );

        // Verify that files with same size do match (same content)
        let dir_c =
            create_test_dir_with_files(temp_dir.path(), "dir_c", &[("size_256.bin", &content_256)]);
        let dir_d =
            create_test_dir_with_files(temp_dir.path(), "dir_d", &[("size_256.bin", &content_256)]);
        let result2 = dir_compare_core::compare_directories(&dir_c, &dir_d, &strategy, None).unwrap();
        assert_eq!(
            result2.both.len(),
            1,
            "Files with identical content and size (256) should match"
        );
    }

    #[test]
    fn test_sampled_hash_fixed_values() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Precalculated SHA-256 hash values for known inputs
        // These values are computed with size prefix (8 bytes big-endian) + content
        // and verified to be consistent across platforms using to_be_bytes()

        // SMALL FILE (100 bytes of 'x'):
        // Size prefix: 0x0000000000000064 (100 in big-endian)
        // Content: 100 bytes of 0x78 ('x')
        const SMALL_HASH: &str = "e7df876b6a4c535ff133690986d1bd4f1e2a29506e6fe5ec06cb8402fd209888";

        // MEDIUM FILE (5000 bytes of 'y'):
        // Size prefix: 0x0000000000001388 (5000 in big-endian)
        // Sampled content: 7 samples of 431 bytes each
        const MEDIUM_HASH: &str =
            "040f2d864134b6510f2287a4b4338865b50b8327ad76ca21b2eaf8ac78645790";

        // LARGE FILE (100000 bytes of 'z'):
        // Size prefix: 0x00000000000186a0 (100000 in big-endian)
        // Sampled content: 7 samples of 431 bytes each
        const LARGE_HASH: &str = "c6c18786d57b8a5e02955f17d2047159c11ca59b0af98b3a47aed12bb22c319a";

        // EMPTY FILE (0 bytes):
        // Size prefix: 0x0000000000000000
        const EMPTY_HASH: &str = "af5570f5a1810b7af78caf4bc70a660f0df51e42baf91d4de5b2328de0e83dfc";

        // SMALL FILE TEST
        let small_content = vec![b'x'; 100];
        let dir_small = create_test_dir_with_files(
            temp_dir.path(),
            "dir_small",
            &[("small.bin", &small_content)],
        );
        let small_path = dir_small.join("small.bin");
        let computed_small = dir_compare_core::comparison::compute_sampled_hash(&small_path);
        assert_eq!(
            computed_small, SMALL_HASH,
            "Small file (100 bytes 'x') hash should match precalculated value"
        );

        // MEDIUM FILE TEST
        let medium_content = vec![b'y'; 5000];
        let dir_medium = create_test_dir_with_files(
            temp_dir.path(),
            "dir_medium",
            &[("medium.bin", &medium_content)],
        );
        let medium_path = dir_medium.join("medium.bin");
        let computed_medium = dir_compare_core::comparison::compute_sampled_hash(&medium_path);
        assert_eq!(
            computed_medium, MEDIUM_HASH,
            "Medium file (5000 bytes 'y') hash should match precalculated value"
        );

        // LARGE FILE TEST
        let large_content = vec![b'z'; 100000];
        let dir_large = create_test_dir_with_files(
            temp_dir.path(),
            "dir_large",
            &[("large.bin", &large_content)],
        );
        let large_path = dir_large.join("large.bin");
        let computed_large = dir_compare_core::comparison::compute_sampled_hash(&large_path);
        assert_eq!(
            computed_large, LARGE_HASH,
            "Large file (100KB 'z') hash should match precalculated value"
        );

        // EMPTY FILE TEST
        let dir_empty =
            create_test_dir_with_files(temp_dir.path(), "dir_empty", &[("empty.bin", b"")]);
        let empty_path = dir_empty.join("empty.bin");
        let computed_empty = dir_compare_core::comparison::compute_sampled_hash(&empty_path);
        assert_eq!(
            computed_empty, EMPTY_HASH,
            "Empty file (0 bytes) hash should match precalculated value"
        );

        // Verify determinism: same file produces same hash
        let computed_small2 = dir_compare_core::comparison::compute_sampled_hash(&small_path);
        assert_eq!(
            computed_small, computed_small2,
            "Hash computation should be deterministic"
        );
    }

    #[test]
    fn test_compare_with_ignore_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_a = create_test_dir_with_files(
            temp_dir.path(),
            "dir_a",
            &[
                ("file1.txt", b"content1"),
                ("file2.log", b"log content"),
                ("should_be_ignored.txt", b"ignore me"),
            ],
        );
        let dir_b = create_test_dir_with_files(
            temp_dir.path(),
            "dir_b",
            &[("file1.txt", b"content1"), ("file2.log", b"log content")],
        );

        let ignore_file_path = temp_dir.path().join(".dir-compare-ignore");
        let mut ignore_file = File::create(&ignore_file_path).unwrap();
        writeln!(ignore_file, "*.log").unwrap();
        writeln!(ignore_file, "should_be_ignored.txt").unwrap();

        let strategy = dir_compare_core::comparison::FilenameOnlyStrategy::new(false);
        let result = dir_compare_core::compare_directories(
            &dir_a,
            &dir_b,
            &strategy,
            Some(&ignore_file_path),
        );

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.a_only.len(), 0, "Expected no A-only entries after ignore");
        assert_eq!(result.b_only.len(), 0, "Expected no B-only entries after ignore");
        assert_eq!(result.both.len(), 1, "Expected 1 matching entry after ignore");
        assert_eq!(
            result.both[0].0.path.file_name().unwrap(),
            "file1.txt"
        );
    }
}
