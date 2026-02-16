pub mod comparison;
pub mod logger;
pub mod output;

pub use comparison::{
    ComparisonResult, ComparisonStrategy, ComparisonStrategyType, Entry, EntryKind,
    FastHashStrategy, FilenameOnlyStrategy, FilenameSizeStrategy, FlatComparisonOptions,
    FlatComparisonResult, FlatContentGroup, SampledHashStrategy, compare_directories,
    compare_directories_flat,
};
pub use logger::{
    debug, error, info, init, set_destination, set_format, set_level, warn, Logger, LoggerConfig,
    LogLevel, OutputDestination,
};
pub use output::Formatter;
