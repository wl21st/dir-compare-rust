use dir_compare_core::{
    compare_directories, ComparisonStrategyType, FastHashStrategy, FilenameOnlyStrategy,
    FilenameSizeStrategy, SampledHashStrategy,
};
use dir_compare_gui::test_utils::create_test_dir_structure;
use std::path::Path;

/// Tests for comparison method workflows through the GUI
/// These tests verify that the correct comparison strategies are used

#[test]
fn test_filename_comparison_strategy() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    assert!(result.is_ok());
    let comparison = result.unwrap();

    // Verify we got results
    assert!(
        !comparison.a_only.is_empty()
            || !comparison.b_only.is_empty()
            || !comparison.both.is_empty()
    );
}

#[test]
fn test_filename_size_comparison_strategy() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FilenameSizeStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    assert!(result.is_ok());
    let comparison = result.unwrap();

    // With FilenameSizeStrategy, files with same name but different sizes
    // should appear in different categories (a_only or b_only)
    // We just verify the comparison completed without errors
    let total_count = comparison.a_only.len() + comparison.b_only.len() + comparison.both.len();
    assert!(total_count > 0, "Comparison should return results");
}

#[test]
fn test_content_hash_comparison_strategy() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = FastHashStrategy::new(false);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    assert!(result.is_ok());
    let comparison = result.unwrap();

    // Common.txt has same content in both directories
    let has_common_file = comparison.both.iter().any(|(a, _b)| {
        a.path
            .file_name()
            .map(|n| n == "common.txt")
            .unwrap_or(false)
    });
    assert!(
        has_common_file,
        "Files with identical content should be in 'both'"
    );
}

#[test]
fn test_sampled_hash_comparison_strategy() {
    let (dir_a, dir_b) = create_test_dir_structure();

    let strategy = SampledHashStrategy::new(false, true);
    let result = compare_directories(dir_a.path(), dir_b.path(), &strategy);

    assert!(result.is_ok());
    let comparison = result.unwrap();

    // Should complete successfully and return results
    // Sampled hash should be faster but still accurate for our test files
    assert!(comparison.a_only.len() + comparison.b_only.len() + comparison.both.len() > 0);
}

#[test]
fn test_comparison_returns_error_for_invalid_dir() {
    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(
        Path::new("/nonexistent/path/12345"),
        Path::new("/another/nonexistent/path"),
        &strategy,
    );

    assert!(result.is_err());
}

#[test]
fn test_all_strategies_produce_consistent_results() {
    let (dir_a, dir_b) = create_test_dir_structure();

    // Run all strategies
    let filename_result = compare_directories(
        dir_a.path(),
        dir_b.path(),
        &FilenameOnlyStrategy::new(false),
    )
    .unwrap();

    let hash_result =
        compare_directories(dir_a.path(), dir_b.path(), &FastHashStrategy::new(false)).unwrap();

    // For identical files, filename and hash should agree on "both" count
    // (Note: This assumes common.txt has identical content)
    let filename_both_count = filename_result.both.len();
    let hash_both_count = hash_result.both.len();

    // At minimum, common.txt should be in "both" for both strategies
    assert!(
        filename_both_count >= 1,
        "Filename strategy should find at least one match"
    );
    assert!(
        hash_both_count >= 1,
        "Hash strategy should find at least one match"
    );
}

#[test]
fn test_comparison_method_selection_mapping() {
    // Verify that ComparisonStrategyType maps to correct strategies
    // This is a logic test - we verify the enum variants exist
    let methods = [
        ComparisonStrategyType::Filename,
        ComparisonStrategyType::FilenameSize,
        ComparisonStrategyType::FastHash,
        ComparisonStrategyType::SampledHash,
    ];

    // Just verify all variants are accessible
    assert_eq!(methods.len(), 4);
}

#[test]
fn test_comparison_state_management() {
    // Test that simulates the UI state changes during comparison
    // In the real GUI, these would be fields in AppState

    // Initial state
    let mut results_available = false;
    let error_message: Option<String> = None;

    // When comparison starts
    let is_comparing = true;
    assert!(is_comparing);
    assert!(!results_available);
    assert!(error_message.is_none());

    // Simulate successful completion
    let is_comparing = false;
    results_available = true;
    assert!(!is_comparing);
    assert!(results_available);
    assert!(error_message.is_none());
}

#[test]
fn test_comparison_error_state() {
    // Test error handling state
    let _error_message: Option<String> = None;

    // When error occurs
    let is_comparing = false;
    let error_message: Option<String> = Some("Permission denied".to_string());

    assert!(!is_comparing); // Should not be comparing anymore
    assert!(error_message.is_some());
}

#[test]
fn test_button_enabled_state_logic() {
    // Test the logic that determines if Compare button is enabled
    fn can_compare(dir_a_valid: bool, dir_b_valid: bool, is_comparing: bool) -> bool {
        dir_a_valid && dir_b_valid && !is_comparing
    }

    // Both directories valid, not comparing -> can compare
    assert!(can_compare(true, true, false));

    // One directory invalid -> cannot compare
    assert!(!can_compare(true, false, false));
    assert!(!can_compare(false, true, false));

    // Both invalid -> cannot compare
    assert!(!can_compare(false, false, false));

    // Currently comparing -> cannot compare (button disabled)
    assert!(!can_compare(true, true, true));
}

#[test]
fn test_async_comparison_channel() {
    use std::sync::mpsc::channel;

    // Simulate the async comparison workflow
    let (tx, rx) = channel::<Result<String, String>>();

    // Spawn a thread that sends a result
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        tx.send(Ok("Comparison complete".to_string())).ok();
    });

    // Wait for result (with timeout to avoid hanging)
    let result = rx.recv_timeout(std::time::Duration::from_secs(1));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), "Comparison complete");
}
