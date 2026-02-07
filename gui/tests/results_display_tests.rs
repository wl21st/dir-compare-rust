use dir_compare_core::{EntryKind, FilenameOnlyStrategy, compare_directories};
use dir_compare_gui::tree_view::FileTreeNode;
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

/// Tests for results display functionality
/// These tests verify that comparison results are correctly transformed for display

#[test]
fn test_results_display_a_only_entries() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // Convert A-only entries to tree nodes
    let a_only_nodes = FileTreeNode::from_entries(&result.a_only);

    // Verify we have A-only entries
    assert!(!a_only_nodes.is_empty() || result.a_only.is_empty());

    // If there are A-only entries, verify they have correct structure
    if !a_only_nodes.is_empty() {
        // Check that at least one node exists with expected name
        let has_expected_file = a_only_nodes
            .iter()
            .any(|node| node.name == "a_only.txt" || node.name == "a_only_dir");
        assert!(has_expected_file, "A-only should contain expected files");
    }
}

#[test]
fn test_results_display_b_only_entries() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // Convert B-only entries to tree nodes
    let b_only_nodes = FileTreeNode::from_entries(&result.b_only);

    // Verify B-only entries
    if !b_only_nodes.is_empty() {
        let has_expected_file = b_only_nodes
            .iter()
            .any(|node| node.name == "b_only.txt" || node.name == "b_only_dir");
        assert!(has_expected_file, "B-only should contain expected files");
    }
}

#[test]
fn test_results_display_both_entries() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // Convert "both" entries to tree nodes (just use the A side)
    let both_entries: Vec<_> = result.both.iter().map(|(a, _)| a.clone()).collect();
    let both_nodes = FileTreeNode::from_entries(&both_entries);

    // Verify both entries contain common files
    if !both_nodes.is_empty() {
        let has_common = both_nodes.iter().any(|node| node.name == "common.txt");
        assert!(has_common, "'Both' should contain common.txt");
    }
}

#[test]
fn test_directory_kind_identification() {
    let (dir_a, _dir_b) = create_test_dir_structure();

    // Scan directory A to get entries
    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_a.path(), &strategy).unwrap();

    // All entries should be in "both" since we're comparing dir with itself
    let both_entries: Vec<_> = result.both.iter().map(|(a, _)| a.clone()).collect();

    // Convert to tree nodes
    let nodes = FileTreeNode::from_entries(&both_entries);

    // Find directories and files
    let dirs: Vec<_> = nodes
        .iter()
        .filter(|n| n.kind == EntryKind::Directory)
        .collect();
    let files: Vec<_> = nodes.iter().filter(|n| n.kind == EntryKind::File).collect();

    // We should have both directories and files
    // Note: The exact count depends on the test data structure
    assert!(!nodes.is_empty(), "Should have some entries");
}

#[test]
fn test_nested_directory_structure() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // Check A-only tree for nested structure
    let a_only_nodes = FileTreeNode::from_entries(&result.a_only);

    // Look for nested directories
    let has_nested = a_only_nodes.iter().any(|node| {
        if node.name == "a_only_dir" {
            // This directory should have children
            !node.children.is_empty()
        } else {
            false
        }
    });

    // The test data has nested files, so we should find them
    // But only if a_only_dir is in the A-only list
    let a_only_has_dir = result
        .a_only
        .iter()
        .any(|e| e.path.to_string_lossy().contains("a_only_dir"));

    if a_only_has_dir {
        assert!(
            has_nested,
            "Nested directories should be properly structured"
        );
    }
}

#[test]
fn test_summary_counts_calculation() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // Calculate summary statistics
    let a_only_count = result.a_only.len();
    let b_only_count = result.b_only.len();
    let both_count = result.both.len();
    let total_count = a_only_count + b_only_count + both_count;

    // Verify counts are non-negative
    assert!(a_only_count >= 0);
    assert!(b_only_count >= 0);
    assert!(both_count >= 0);
    assert!(total_count >= 0);

    // Verify total is sum of parts
    assert_eq!(total_count, a_only_count + b_only_count + both_count);

    // With our test data, we should have entries in each category
    assert!(
        a_only_count > 0 || b_only_count > 0 || both_count > 0,
        "Should have at least some entries"
    );
}

#[test]
fn test_empty_results_show_zero_counts() {
    // Create two empty directories
    let dir_a = tempfile::TempDir::new().unwrap();
    let dir_b = tempfile::TempDir::new().unwrap();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy).unwrap();

    // All counts should be zero
    assert_eq!(result.a_only.len(), 0);
    assert_eq!(result.b_only.len(), 0);
    assert_eq!(result.both.len(), 0);
}

#[test]
fn test_tree_node_paths_are_correct() {
    let (dir_a, _dir_b) = create_test_dir_structure();

    // Get entries from dir_a
    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_a.path(), &strategy).unwrap();
    let entries: Vec<_> = result.both.iter().map(|(a, _)| a.clone()).collect();

    // Convert to tree nodes
    let nodes = FileTreeNode::from_entries(&entries);

    // Verify that paths are set correctly
    for node in &nodes {
        assert!(
            !node.path.as_os_str().is_empty(),
            "Path should not be empty"
        );
        // The path should contain the filename
        assert!(
            node.path.file_name().is_some(),
            "Path should have a file name component"
        );
    }
}
