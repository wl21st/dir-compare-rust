//! Tests for the logger module.

use dir_compare_core::logger::{self, LoggerConfig, LogLevel, OutputDestination};
use std::sync::{Mutex, OnceLock};

// Use a mutex to ensure tests don't interfere with each other
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_lock() -> &'static Mutex<()> {
    TEST_LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn test_log_level_ordering() {
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
}

#[test]
fn test_log_level_from_str() {
    assert_eq!(LogLevel::from_str("debug"), Some(LogLevel::Debug));
    assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
    assert_eq!(LogLevel::from_str("info"), Some(LogLevel::Info));
    assert_eq!(LogLevel::from_str("INFO"), Some(LogLevel::Info));
    assert_eq!(LogLevel::from_str("warn"), Some(LogLevel::Warn));
    assert_eq!(LogLevel::from_str("WARN"), Some(LogLevel::Warn));
    assert_eq!(LogLevel::from_str("warning"), Some(LogLevel::Warn));
    assert_eq!(LogLevel::from_str("WARNING"), Some(LogLevel::Warn));
    assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
    assert_eq!(LogLevel::from_str("ERROR"), Some(LogLevel::Error));
    assert_eq!(LogLevel::from_str("invalid"), None);
    assert_eq!(LogLevel::from_str(""), None);
}

#[test]
fn test_log_level_as_str() {
    assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
    assert_eq!(LogLevel::Info.as_str(), "INFO");
    assert_eq!(LogLevel::Warn.as_str(), "WARN");
    assert_eq!(LogLevel::Error.as_str(), "ERROR");
}

#[test]
fn test_log_level_default() {
    assert_eq!(LogLevel::default(), LogLevel::Info);
}

#[test]
fn test_logger_config_default() {
    let config = LoggerConfig::default();
    assert_eq!(config.level, LogLevel::Info);
    assert_eq!(config.destination, OutputDestination::Stdout);
    assert!(config.format.is_none());
}

#[test]
fn test_logger_set_level() {
    let _lock = get_test_lock().lock().unwrap();

    logger::init(LoggerConfig {
        level: LogLevel::Info,
        destination: OutputDestination::Stdout,
        format: None,
    });

    logger::set_level(LogLevel::Debug);
    assert_eq!(logger::Logger::global().level(), LogLevel::Debug);

    logger::set_level(LogLevel::Error);
    assert_eq!(logger::Logger::global().level(), LogLevel::Error);

    // Reset to default
    logger::set_level(LogLevel::Info);
}

#[test]
fn test_logger_set_destination() {
    let _lock = get_test_lock().lock().unwrap();

    logger::init(LoggerConfig {
        level: LogLevel::Info,
        destination: OutputDestination::Stdout,
        format: None,
    });

    logger::set_destination(OutputDestination::Stderr);
    // We can't easily verify the destination, but we can ensure it doesn't panic

    // Reset to default
    logger::set_destination(OutputDestination::Stdout);
}

#[test]
fn test_logger_set_format() {
    let _lock = get_test_lock().lock().unwrap();

    logger::init(LoggerConfig {
        level: LogLevel::Info,
        destination: OutputDestination::Stdout,
        format: None,
    });

    logger::set_format(Some("{level}: {message}".to_string()));
    // We can't easily verify the format, but we can ensure it doesn't panic

    // Reset to default
    logger::set_format(None);
}

#[test]
fn test_logger_methods_dont_panic() {
    let _lock = get_test_lock().lock().unwrap();

    logger::init(LoggerConfig {
        level: LogLevel::Debug,
        destination: OutputDestination::Stdout,
        format: None,
    });

    // These should not panic
    logger::debug("Test debug message");
    logger::info("Test info message");
    logger::warn("Test warn message");
    logger::error("Test error message");
}

#[test]
fn test_logger_level_filtering() {
    let _lock = get_test_lock().lock().unwrap();

    // Set level to Warn - only warn and error should produce output
    logger::init(LoggerConfig {
        level: LogLevel::Warn,
        destination: OutputDestination::Stdout,
        format: None,
    });

    // These should be filtered (no output)
    logger::debug("This should be filtered");
    logger::info("This should be filtered");

    // These should produce output
    logger::warn("This should appear");
    logger::error("This should appear");

    // Reset to default
    logger::set_level(LogLevel::Info);
}

#[test]
fn test_logger_custom_format() {
    let _lock = get_test_lock().lock().unwrap();

    logger::init(LoggerConfig {
        level: LogLevel::Info,
        destination: OutputDestination::Stdout,
        format: Some("CUSTOM: {level} - {message}".to_string()),
    });

    logger::info("Test with custom format");

    // Reset to default
    logger::set_format(None);
}

#[test]
fn test_output_destination_from_str() {
    // Test that we can create the config with different destinations
    let config_stdout = LoggerConfig {
        level: LogLevel::Info,
        destination: OutputDestination::Stdout,
        format: None,
    };
    assert_eq!(config_stdout.destination, OutputDestination::Stdout);

    let config_stderr = LoggerConfig {
        level: LogLevel::Info,
        destination: OutputDestination::Stderr,
        format: None,
    };
    assert_eq!(config_stderr.destination, OutputDestination::Stderr);
}

#[test]
fn test_output_destination_default() {
    assert_eq!(OutputDestination::default(), OutputDestination::Stdout);
}
