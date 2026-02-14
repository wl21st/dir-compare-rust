# dir-compare

A command-line tool for comparing two directories and reporting their differences. Recursively scans both directories and categorizes entries into three groups: files/folders only in the first directory, only in the second, and present in both.

## Features

- **Multiple comparison methods**: Compare by filename, filename+size, or content hash
- **Flat mode**: Content-based matching to find duplicates and moved files across different directory structures
- **Multiple output formats**: Text (default), HTML, and Markdown
- **Flexible filtering**: Case-sensitive or case-insensitive filename matching
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Clear categorization**: Results organized into A-only, B-only, and Both sections (hierarchy mode) or content hash groups (flat mode)

## Installation

To install the CLI tool:
```bash
cargo install --path cli
```

To install the GUI tool:
```bash
cargo install --path gui
```

Or build from source:

```bash
git clone <repository>
cd dir-compare
cargo build --release --workspace
```

## Usage

### GUI Usage

To launch the graphical interface:

```bash
cargo run --bin dir-compare-gui --release
# or if installed
dir-compare-gui
```

The GUI provides:
- Visual directory selection with validation
- Interactive tree view of differences
- Expandable folders for navigating deep structures
- Color-coded results (Red for A-only, Green for B-only, Blue for Both)
- Comparison method selection (Filename, Size, Hash)
- Light/Dark theme support

### CLI Basic Comparison

Compare two directories using the default content hash method:

```bash
dir-compare /path/to/dir_a /path/to/dir_b
```

### Comparison Methods

Choose how files are compared:

#### By Filename Only
```bash
dir-compare dir_a dir_b --method filename
```
Matches files with the same name, regardless of size or content.

#### By Filename and Size
```bash
dir-compare dir_a dir_b --method size
```
Matches files with the same name AND same size. Useful for finding modified files.

#### By Filename and Content Hash (Default)
```bash
dir-compare dir_a dir_b --method hash
```
Matches files with the same name AND identical content. Most accurate for finding duplicates.

#### By Filename and Sampled Hash
```bash
dir-compare dir_a dir_b --method sampled
```
Matches files with the same name AND identical sampled content hash.
Fast for large files as it reads small samples (default 7 samples of 431 bytes).
Use `--verify` to verify matches with full content hash.

**Note**: The sampled hash includes the file size as part of the hash computation to prevent
false positives. This ensures that files with different sizes will never match, even if their
sampled content overlaps. This is particularly important when comparing files where one might
be a prefix or subset of another.

### Flat Mode (Content-Based Matching)

Flat mode compares files by content hash rather than path, enabling detection of:
- **Duplicate files**: Files with identical content within or across directories
- **Moved files**: Files that exist in different locations but have the same content

This is useful when directory structures differ but contain equivalent files.

#### Basic Flat Mode
```bash
dir-compare dir_a dir_b --flat
```
Compares files by sampled content hash (fast, good for large files).

#### Flat Mode with Full Hash
```bash
dir-compare dir_a dir_b --flat --full-hash
```
Uses full-file SHA-256 hash for bit-perfect accuracy (slower but eliminates hash collisions).

#### Flat Mode Output Example
```
Flat Mode Comparison Summary
==================================================
Files in directory A: 5
Files in directory B: 3
Unique content hashes: 4
Duplicate content groups: 2

Hash: a1b2c3d4...e5f6 [DUPLICATE] (1024 bytes, 3 files)
--------------------------------------------------
  [A] documents/report.txt -> (moved/copied to B)
  [A] backup/report.txt -> (moved/copied to B)
  [B] archive/report.txt <- (moved/copied from A)

Hash: b2c3d4e5...f6a7 [MATCHED] (512 bytes, 2 files)
--------------------------------------------------
  [A] file1.txt -> (moved/copied to B)
  [B] documents/file1.txt <- (moved/copied from A)
```

**Status indicators:**
- `[DUPLICATE]`: Multiple files have identical content
- `[MATCHED]`: Same content exists in both directories (moved/copied)
- `[A-ONLY]`: Content only exists in directory A
- `[B-ONLY]`: Content only exists in directory B

### Case-Insensitive Comparison

Compare filenames without regard to case:

```bash
dir-compare dir_a dir_b --case-insensitive
```

### Output Formats

Choose how results are displayed:

#### Text (Default)
```bash
dir-compare dir_a dir_b --format text
```

#### HTML Report
```bash
dir-compare dir_a dir_b --format html --output report.html
```
Generates a styled HTML report with statistics.

#### Markdown Report
```bash
dir-compare dir_a dir_b --format markdown --output report.md
```
Generates a Markdown document suitable for documentation.

### Output to File

Write results to a file instead of stdout:

```bash
dir-compare dir_a dir_b --output results.txt
```

## Comparison Method Trade-offs

### Hierarchy Mode (Default)

| method | Speed | Use Case |
|--------|-------|----------|
| filename | Fastest | Quick overview of missing files |
| size | Fast | Finding modified files with same name |
| sampled | Fast | Comparing large files with IO constraints |
| hash | Slower | Verifying file content identity |

### Flat Mode

| option | Speed | Use Case |
|--------|-------|----------|
| `--flat` (default) | Fast | Finding duplicates and moved files across structures |
| `--flat --full-hash` | Slower | Bit-perfect duplicate detection |

### Performance Characteristics

- **filename**: O(n) - Only compares file names
- **size**: O(n) - Compares names and reads metadata
- **sampled**: O(n) - Constant IO per file (read ~3KB), much faster than hash for large files
- **hash**: O(n×f) - Must read file contents; time depends on total file size
- **flat mode**: O(n) - Hashes all files then groups by content; memory usage grows with file count

For large directories with many files:
- Use `filename` for initial scans
- Use `size` to find obvious modifications
- Use `hash` only when content verification is critical
- Use `--flat` when directory structures differ or you need to find duplicates

### Hierarchy Mode vs Flat Mode

| Feature | Hierarchy Mode | Flat Mode |
|---------|---------------|-----------|
| Matching criterion | Path + content | Content only |
| Use case | Syncing identical structures | Finding duplicates, moved files |
| Output format | A-only / B-only / Both | Content groups by hash |
| Handles renames | No | Yes (detects as moved) |
| Handles reorganization | No | Yes (content-based) |
| Performance | Faster (stops at first mismatch) | Slower (must hash all files) |

**When to use hierarchy mode:**
- Syncing backups with identical structures
- Finding missing files in mirrored directories
- Comparing version-controlled directories

**When to use flat mode:**
- Finding duplicate files consuming disk space
- Detecting files moved between folders
- Comparing reorganized directories
- Finding copied files with different names

## Output Format Examples

### Text Output

```
A-only (2 entries):
----------------------------------------
  dir_a-only/
  file_a.txt

B-only (2 entries):
----------------------------------------
  dir_b-only/
  file_b.txt

Both (1 entries):
----------------------------------------
  common.txt == common.txt
```

Directories are marked with a trailing `/`.

### HTML Output

The HTML format includes:
- Responsive layout with professional styling
- Summary statistics boxes
- Color-coded sections
- File/directory type indicators
- Completely self-contained (works offline)

### Markdown Output

```markdown
# Directory Comparison Report

## Summary

| Category | Count |
|---------|-------|
| A-only | 2 |
| B-only | 2 |
| Both | 1 |

## A-only

- `dir_a-only/`
- `file_a.txt`

## B-only

- `dir_b-only/`
- `file_b.txt`

## Both

- `common.txt` == `common.txt`
```

## Troubleshooting

### Common Errors

#### "Error: Directory A does not exist"

**Cause**: The specified directory path doesn't exist.

**Solution**:
```bash
# Check if the directory exists
ls -la /path/to/directory

# Use the correct path
dir-compare /correct/path/a /correct/path/b
```

#### "Error: Path A is not a directory"

**Cause**: The path exists but is a file, not a directory.

**Solution**: Ensure you're comparing directories, not files.

#### "Error: Invalid comparison method"

**Cause**: The specified method is not recognized.

**Solution**: Use one of the valid methods:
```bash
dir-compare dir_a dir_b --method filename    # or "name"
dir-compare dir_a dir_b --method size
dir-compare dir_a dir_b --method sampled     # or "sampled-hash"
dir-compare dir_a dir_b --method hash        # or "fxhash" or "fasthash"
```

#### "Error: Invalid format"

**Cause**: The specified output format is not recognized.

**Solution**: Use one of the valid formats:
```bash
dir-compare dir_a dir_b --format text        # or "txt"
dir-compare dir_a dir_b --format html
dir-compare dir_a dir_b --format markdown    # or "md"
```

#### "Error: Missing required positional argument"

**Cause**: The two directory paths are not provided.

**Solution**: Provide both directories:
```bash
dir-compare /path/to/dir_a /path/to/dir_b
```

#### Warning: "Could not access entry: [permission denied]"

**Cause**: Some files or directories couldn't be accessed due to permission restrictions.

**Solution**:
- Check directory permissions: `ls -la /path`
- Run with appropriate user permissions
- The comparison will continue and report what it can access

### Permission Issues

If you see permission-related warnings:

1. Check current permissions:
   ```bash
   ls -la /path/to/directory
   ```

2. Fix permissions (requires ownership or admin):
   ```bash
   chmod +r /path/to/directory
   ```

3. Or run as a user with appropriate access

### Empty Results

If all entries appear in "A-only" or "B-only":

1. Verify paths are correct
2. Check for case sensitivity issues (use `--case-insensitive`)
3. Ensure directories are not empty

### Large Directory Performance

For very large directories:

1. Use `--method filename` for fastest initial comparison
2. Redirect output to a file to avoid terminal scrollback
3. Consider using `--format markdown` for easier result parsing

### Unicode and Special Characters

The tool handles:
- Unicode characters in filenames (e.g., `файл_тест.txt`, `测试文件.txt`)
- Special characters (escaped properly in HTML/Markdown output)
- Emoji and other Unicode symbols

### Cross-Platform Path Handling

- Paths are normalized for cross-platform compatibility
- Relative paths are displayed from the compared directory root
- Forward slashes are used in output regardless of platform

## Programmatic Usage

You can also use the library in your Rust code:

```rust
use dir_compare::{compare_directories, FilenameOnlyStrategy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let strategy = FilenameOnlyStrategy::new(false);
    let result = compare_directories(
        &std::path::Path::new("dir_a"),
        &std::path::Path::new("dir_b"),
        &strategy
    )?;

    println!("A-only: {} entries", result.a_only.len());
    println!("B-only: {} entries", result.b_only.len());
    println!("Both: {} entries", result.both.len());

    Ok(())
}
```

## Testing

### Running Tests

To run all tests:

```bash
cargo test --workspace
```

### Running GUI Tests

The GUI has its own test suite:

```bash
# Run all GUI tests
cargo test --package dir-compare-gui

# Run specific test files
cargo test --package dir-compare-gui --test directory_selection_tests
cargo test --package dir-compare-gui --test comparison_method_tests
cargo test --package dir-compare-gui --test results_display_tests
cargo test --package dir-compare-gui --test corner_case_tests
```

### Running Unit Tests Only

To run only unit tests (without integration tests):

```bash
cargo test --package dir-compare-gui --lib
```

### Headless Testing

The GUI tests use mocked dependencies and temporary directories, so they can run headlessly:

```bash
# Run all non-ignored tests without opening any GUI windows
cargo test --package dir-compare-gui -- --nocapture
```

### Running Slow Tests

Some tests are marked with `#[ignore]` because they are slow or require special handling:

```bash
# Run only ignored tests (slow tests, theme tests)
cargo test --package dir-compare-gui -- --ignored

# Run theme tests serially to avoid config file conflicts
cargo test --package dir-compare-gui test_theme -- --ignored --test-threads=1

# Run ALL tests (including ignored ones)
cargo test --package dir-compare-gui -- --include-ignored
```

Ignored tests include:
- **Deep nesting test**: Creates 100-level directory structure
- **Performance test**: Creates 100 files and measures comparison time
- **Theme persistence tests**: Share config state and should run with `--test-threads=1`

### Test Coverage

To check test coverage (requires cargo-tarpaulin):

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --package dir-compare-gui --out Html --output-dir target/coverage
```

Then open `target/coverage/tarpaulin-report.html` to view the coverage report.

## License

MIT License - see LICENSE file for details.
