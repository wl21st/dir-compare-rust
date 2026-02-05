pub mod comparison;
pub mod output;

pub use comparison::{
    compare_directories, ComparisonResult, ComparisonStrategy, ComparisonStrategyType, Entry,
    EntryKind, FastHashStrategy, FilenameOnlyStrategy, FilenameSizeStrategy, SampledHashStrategy,
};
pub use output::Formatter;
