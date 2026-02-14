pub mod comparison;
pub mod output;

pub use comparison::{
    ComparisonResult, ComparisonStrategy, ComparisonStrategyType, Entry, EntryKind,
    FastHashStrategy, FilenameOnlyStrategy, FilenameSizeStrategy, FlatComparisonOptions,
    FlatComparisonResult, FlatContentGroup, SampledHashStrategy, compare_directories,
    compare_directories_flat,
};
pub use output::Formatter;
