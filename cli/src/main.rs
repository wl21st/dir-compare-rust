use clap::Parser;
use dir_compare_core::comparison::{
    ComparisonStrategy, FastHashStrategy, FilenameOnlyStrategy, FilenameSizeStrategy,
    SampledHashStrategy, compare_directories,
};
use dir_compare_core::output::{Formatter, HtmlFormatter, MarkdownFormatter, TextFormatter};
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

    #[arg(short, long, default_value = "hash")]
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
}

fn main() {
    let args = Args::parse();

    if !args.dir_a.exists() {
        eprintln!(
            "Error: First directory does not exist: {}",
            args.dir_a.display()
        );
        process::exit(1);
    }

    if !args.dir_b.exists() {
        eprintln!(
            "Error: Second directory does not exist: {}",
            args.dir_b.display()
        );
        process::exit(1);
    }

    if !args.dir_a.is_dir() {
        eprintln!(
            "Error: First path is not a directory: {}",
            args.dir_a.display()
        );
        process::exit(1);
    }

    if !args.dir_b.is_dir() {
        eprintln!(
            "Error: Second path is not a directory: {}",
            args.dir_b.display()
        );
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
            eprintln!("Error: Invalid comparison method '{}'", args.method);
            eprintln!("Available methods: filename, size, hash, sampled");
            process::exit(1);
        }
    };

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
                    eprintln!("Error: Invalid format '{}'", args.format);
                    eprintln!("Available formats: text, html, markdown");
                    process::exit(1);
                }
            };

            let output = formatter.format(&result);

            match args.output {
                Some(path) => match std::fs::write(&path, &output) {
                    Ok(_) => println!("Report written to: {}", path.display()),
                    Err(e) => {
                        eprintln!("Error writing to file: {}", e);
                        process::exit(1);
                    }
                },
                None => {
                    println!("{}", output);
                }
            }
        }
        Err(e) => {
            eprintln!("Error during comparison: {}", e);
            process::exit(1);
        }
    }
}
