//! Logger module for structured logging with level-based filtering.
//!
//! This module provides a global logger instance with configurable log levels,
//! output formatting, and destination configuration.

use std::io::{self, Write};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

/// Log levels for filtering output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LogLevel {
    /// Debug level - verbose diagnostic information
    Debug = 0,
    /// Info level - general informational messages
    #[default]
    Info = 1,
    /// Warn level - warning messages for potentially problematic situations
    Warn = 2,
    /// Error level - error messages for failures
    Error = 3,
}

impl LogLevel {
    /// Returns the string representation of the log level.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    /// Parses a log level from a string (case-insensitive).
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" | "WARNING" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            _ => None,
        }
    }
}

/// Output destination for log messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputDestination {
    /// Output to standard output (stdout)
    Stdout,
    /// Output to standard error (stderr)
    #[default]
    Stderr,
}

/// Configuration for the logger.
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Minimum log level to output
    pub level: LogLevel,
    /// Output destination (stdout or stderr)
    pub destination: OutputDestination,
    /// Custom format string (supports {level}, {timestamp}, {message})
    pub format: Option<String>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            destination: OutputDestination::Stderr,
            format: None,
        }
    }
}

/// Global logger instance for structured logging.
pub struct Logger {
    config: Mutex<LoggerConfig>,
}

impl Logger {
    /// Creates a new logger with the given configuration.
    fn new(config: LoggerConfig) -> Self {
        Self {
            config: Mutex::new(config),
        }
    }

    /// Returns the global logger instance, initializing it if necessary.
    pub fn global() -> &'static Logger {
        static LOGGER: OnceLock<Logger> = OnceLock::new();
        LOGGER.get_or_init(|| Logger::new(LoggerConfig::default()))
    }

    /// Initializes the global logger with custom configuration.
    ///
    /// # Important
    ///
    /// This should be called **early in the program**, ideally as one of the first
    /// operations in `main()`, before any logging functions are called.
    ///
    /// If any logging function (debug, info, warn, error) is called before `init()`,
    /// the logger will be initialized with default configuration and this function
    /// will update the existing configuration. However, it's best practice to call
    /// `init()` first to ensure consistent behavior.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dir_compare_core::logger::{LoggerConfig, LogLevel, OutputDestination};
    ///
    /// fn main() {
    ///     // Initialize logger FIRST
    ///     dir_compare_core::logger::init(LoggerConfig {
    ///         level: LogLevel::Info,
    ///         destination: OutputDestination::Stderr,
    ///         format: None,
    ///     });
    ///
    ///     // Now use logging functions
    ///     dir_compare_core::logger::info("Application started");
    /// }
    /// ```
    pub fn init(config: LoggerConfig) {
        let logger = Self::global();
        if let Ok(mut cfg) = logger.config.lock() {
            *cfg = config;
        }
    }

    /// Sets the log level.
    pub fn set_level(&self, level: LogLevel) {
        if let Ok(mut config) = self.config.lock() {
            config.level = level;
        }
    }

    /// Sets the output destination.
    pub fn set_destination(&self, destination: OutputDestination) {
        if let Ok(mut config) = self.config.lock() {
            config.destination = destination;
        }
    }

    /// Sets a custom format string.
    pub fn set_format(&self, format: Option<String>) {
        if let Ok(mut config) = self.config.lock() {
            config.format = format;
        }
    }

    /// Returns the current log level.
    pub fn level(&self) -> LogLevel {
        self.config.lock().map(|c| c.level).unwrap_or_default()
    }

    /// Logs a message at the specified level.
    pub fn log(&self, level: LogLevel, message: &str) {
        let config = match self.config.lock() {
            Ok(cfg) => cfg.clone(),
            Err(_) => return,
        };

        // Filter by level
        if level < config.level {
            return;
        }

        // Format the message
        let formatted = self.format_message(&config, level, message);

        // Write to the appropriate destination
        let result = match config.destination {
            OutputDestination::Stdout => {
                let mut stdout = io::stdout().lock();
                writeln!(stdout, "{}", formatted)
            }
            OutputDestination::Stderr => {
                let mut stderr = io::stderr().lock();
                writeln!(stderr, "{}", formatted)
            }
        };

        // Ignore write errors (logging should not crash the program)
        let _ = result;
    }

    /// Formats a log message according to the configuration.
    fn format_message(&self, config: &LoggerConfig, level: LogLevel, message: &str) -> String {
        let timestamp = self.get_timestamp();

        if let Some(ref format) = config.format {
            format
                .replace("{level}", level.as_str())
                .replace("{timestamp}", &timestamp)
                .replace("{message}", message)
        } else {
            // Default format: [LEVEL] timestamp message
            format!("[{}] {} {}", level.as_str(), timestamp, message)
        }
    }

    /// Gets the current timestamp as a Unix timestamp with microsecond precision.
    ///
    /// Format: `seconds.microseconds` (e.g., "1708099200.123456")
    fn get_timestamp(&self) -> String {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                let secs = duration.as_secs();
                let micros = duration.subsec_micros();
                format!("{}.{:06}", secs, micros)
            }
            Err(_) => "0.000000".to_string(),
        }
    }

    /// Logs a debug message.
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Logs an info message.
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Logs a warning message.
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Logs an error message.
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

// ============================================================================
// Convenience functions for global logger
// ============================================================================

/// Initializes the global logger with the given configuration.
pub fn init(config: LoggerConfig) {
    Logger::init(config);
}

/// Sets the global log level.
pub fn set_level(level: LogLevel) {
    Logger::global().set_level(level);
}

/// Sets the global output destination.
pub fn set_destination(destination: OutputDestination) {
    Logger::global().set_destination(destination);
}

/// Sets a custom format string for the global logger.
pub fn set_format(format: Option<String>) {
    Logger::global().set_format(format);
}

/// Logs a debug message using the global logger.
pub fn debug(message: &str) {
    Logger::global().debug(message);
}

/// Logs an info message using the global logger.
pub fn info(message: &str) {
    Logger::global().info(message);
}

/// Logs a warning message using the global logger.
pub fn warn(message: &str) {
    Logger::global().warn(message);
}

/// Logs an error message using the global logger.
pub fn error(message: &str) {
    Logger::global().error(message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("debug"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("warn"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("WARNING"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("invalid"), None);
    }

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Warn.as_str(), "WARN");
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
    }

    #[test]
    fn test_logger_config_default() {
        let config = LoggerConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert_eq!(config.destination, OutputDestination::Stderr);
        assert!(config.format.is_none());
    }
}
