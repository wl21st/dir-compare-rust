use clap::Parser;
use dir_compare_core::comparison::{
    compare_directories, compare_directories_flat, ComparisonStrategy, FastHashStrategy,
    FilenameOnlyStrategy, FilenameSizeStrategy, FlatComparisonOptions, SampledHashStrategy,
};
use dir_compare_core::logger::{self, LoggerConfig, LogLevel, OutputDestination};
use dir_compare_core::output::{
    FlatHtmlFormatter, FlatMarkdownFormatter, FlatTextFormatter, Formatter, HtmlFormatter,
    MarkdownFormatter, TextFormatter,
};
use std::path::PathBuf;
use std::process;

#[derive(clap::Parser)]
#[command(name = "dir-compare")]
#[command(author = "dir-compare contributors")]
#[command(version = "0.1.0")]
#[command(about = "Compare two directories and report differences", long_about = None)]
struct Args {
    /// First directory to compare
    #[arg(value_name = "DIR1")]
    dir_a: PathBuf,

    /// Second directory to compare
    #[arg(value_name = "DIR2")]
    dir_b: PathBuf,

    #[arg(short, long, default_value = "sampled")]
    method: String,

    #[arg(short, long)]
    case_insensitive: bool,

    #[arg(short, long, default_value = "text")]
    format: String,

    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Verify matches with full hash when using sampled-hash strategy
    #[arg(long)]
    verify: bool,

    #[arg(long)]
    ignore: Option<PathBuf>,

    /// Enable flat mode comparison (content-based matching across different structures)
    #[arg(long)]
    flat: bool,

    /// Use full-file hash instead of sampled hash (only applies to flat mode)
    #[arg(long)]
    full_hash: bool,

    /// Log level (debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Log output destination (stdout, stderr)
    #[arg(long, default_value = "stderr")]
    log_dest: String,
}

/// Initialize the logger from environment variables or CLI arguments.
fn init_logger(args: &Args) {
    // Check environment variables first, then fall back to CLI args
    let level = std::env::var("DIR_COMPARE_LOG_LEVEL")
        .ok()
        .and_then(|s| LogLevel::from_str(&s))
        .or_else(|| LogLevel::from_str(&args.log_level))
        .unwrap_or_default();

    let destination = std::env::var("DIR_COMPARE_LOG_DEST")
        .ok()
        .and_then(|s| match s.to_lowercase().as_str() {
            "stdout" => Some(OutputDestination::Stdout),
            "stderr" => Some(OutputDestination::Stderr),
            _ => None,
        })
        .or_else(|| match args.log_dest.to_lowercase().as_str() {
            "stdout" => Some(OutputDestination::Stdout),
            "stderr" => Some(OutputDestination::Stderr),
            _ => None,
        })
        .unwrap_or_default();

    logger::init(LoggerConfig {
        level,
        destination,
        format: None,
    });
}

fn main() {
    let args = Args::parse();
    init_logger(&args);

    if !args.dir_a.exists() {
        logger::error(&format!(
            "First directory does not exist: {}",
            args.dir_a.display()
        ));
        process::exit(1);
    }

    if !args.dir_b.exists() {
        logger::error(&format!(
            "Second directory does not exist: {}",
            args.dir_b.display()
        ));
        process::exit(1);
    }

    if !args.dir_a.is_dir() {
        logger::error(&format!(
            "First path is not a directory: {}",
            args.dir_a.display()
        ));
        process::exit(1);
    }

    if !args.dir_b.is_dir() {
        logger::error(&format!(
            "Second path is not a directory: {}",
            args.dir_b.display()
        ));
        process::exit(1);
    }

    let strategy: Box<dyn ComparisonStrategy> = match args.method.to_lowercase().as_str() {
        "filename" | "name" => Box::new(FilenameOnlyStrategy::new(args.case_insensitive)),
        "size" => Box::new(FilenameSizeStrategy::new(args.case_insensitive)),
        "hash" | "fxhash" | "fasthash" => Box::new(FastHashStrategy::new(args.case_insensitive)),
        "sampled" | "sampled-hash" => {
            Box::new(SampledHashStrategy::new(args.case_insensitive, args.verify))
        }
        _ => {
            logger::error(&format!("Invalid comparison method '{}'", args.method));
            logger::error("Available methods: filename, size, hash, sampled");
            process::exit(1);
        }
    };

    if args.flat {
        // Flat mode comparison
        let options = FlatComparisonOptions {
            use_full_hash: args.full_hash,
            ..Default::default()
        };

        match compare_directories_flat(&args.dir_a, &args.dir_b, &options, args.ignore.as_deref()) {
            Ok(result) => {
                let output = match args.format.to_lowercase().as_str() {
                    "text" | "txt" => FlatTextFormatter.format(&result),
                    "html" => FlatHtmlFormatter.format(&result),
                    "markdown" | "md" => FlatMarkdownFormatter.format(&result),
                    _ => {
                        logger::error(&format!("Invalid format '{}'", args.format));
                        logger::error("Available formats: text, html, markdown");
                        process::exit(1);
                    }
                };

                match args.output {
                    Some(path) => match std::fs::write(&path, &output) {
                        Ok(_) => logger::info(&format!("Report written to: {}", path.display())),
                        Err(e) => {
                            logger::error(&format!("Error writing to file: {}", e));
                            process::exit(1);
                        }
                    },
                    None => {
                        // Output the result directly to stdout (not through logger)
                        println!("{}", output);
                    }
                }
            }
            Err(e) => {
                logger::error(&format!("Error during comparison: {}", e));
                process::exit(1);
            }
        }
    } else {
        // Hierarchy mode comparison
        match compare_directories(
            &args.dir_a,
            &args.dir_b,
            strategy.as_ref(),
            args.ignore.as_deref(),
        ) {
            Ok(result) => {
                let formatter: Box<dyn Formatter> = match args.format.to_lowercase().as_str() {
                    "text" | "txt" => Box::new(TextFormatter),
                    "html" => Box::new(HtmlFormatter),
                    "markdown" | "md" => Box::new(MarkdownFormatter),
                    _ => {
                        logger::error(&format!("Invalid format '{}'", args.format));
                        logger::error("Available formats: text, html, markdown");
                        process::exit(1);
                    }
                };

                let output = formatter.format(&result);

                match args.output {
                    Some(path) => match std::fs::write(&path, &output) {
                        Ok(_) => logger::info(&format!("Report written to: {}", path.display())),
                        Err(e) => {
                            logger::error(&format!("Error writing to file: {}", e));
                            process::exit(1);
                        }
                    },
                    None => {
                        // Output the result directly to stdout (not through logger)
                        println!("{}", output);
                    }
                }
            }
            Err(e) => {
                logger::error(&format!("Error during comparison: {}", e));
                process::exit(1);
            }
        }
    }
}
