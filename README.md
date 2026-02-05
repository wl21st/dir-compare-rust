# dir-compare

A command-line tool for comparing two directories and reporting their differences. Recursively scans both directories and categorizes entries into three groups: files/folders only in the first directory, only in the second, and present in both.

## Features

- **Multiple comparison methods**: Compare by filename, filename+size, or content hash
- **Multiple output formats**: Text (default), HTML, and Markdown
- **Flexible filtering**: Case-sensitive or case-insensitive filename matching
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Clear categorization**: Results organized into A-only, B-only, and Both sections

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

Compare two directories using the default filename-only method:

```bash
dir-compare -a /path/to/dir_a -b /path/to/dir_b
```

### Comparison Methods

Choose how files are compared:

#### By Filename Only (Default)
```bash
dir-compare -a dir_a -b dir_b --method filename
```
Matches files with the same name, regardless of size or content.

#### By Filename and Size
```bash
dir-compare -a dir_a -b dir_b --method size
```
Matches files with the same name AND same size. Useful for finding modified files.

#### By Filename and Content Hash
```bash
dir-compare -a dir_a -b dir_b --method hash
```
Matches files with the same name AND identical content. Most accurate for finding duplicates.

#### By Filename and Sampled Hash
```bash
dir-compare -a dir_a -b dir_b --method sampled
```
Matches files with the same name AND identical sampled content hash.
Fast for large files as it reads small samples (default 7 samples of 431 bytes).
Use `--verify` to verify matches with full content hash.

### Case-Insensitive Comparison

Compare filenames without regard to case:

```bash
dir-compare -a dir_a -b dir_b --case-insensitive
```

### Output Formats

Choose how results are displayed:

#### Text (Default)
```bash
dir-compare -a dir_a -b dir_b --format text
```

#### HTML Report
```bash
dir-compare -a dir_a -b dir_b --format html --output report.html
```
Generates a styled HTML report with statistics.

#### Markdown Report
```bash
dir-compare -a dir_a -b dir_b --format markdown --output report.md
```
Generates a Markdown document suitable for documentation.

### Output to File

Write results to a file instead of stdout:

```bash
dir-compare -a dir_a -b dir_b --output results.txt
```

## Comparison Method Trade-offs

| method | Speed | Use Case |
|--------|-------|----------|
| filename | Fastest | Quick overview of missing files |
| size | Fast | Finding modified files with same name |
| sampled | Fast | Comparing large files with IO constraints |
| hash | Slower | Verifying file content identity |

### Performance Characteristics

- **filename**: O(n) - Only compares file names
- **size**: O(n) - Compares names and reads metadata
- **sampled**: O(n) - Constant IO per file (read ~3KB), much faster than hash for large files
- **hash**: O(n×f) - Must read file contents; time depends on total file size

For large directories with many files:
- Use `filename` for initial scans
- Use `size` to find obvious modifications
- Use `hash` only when content verification is critical

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
dir-compare -a /correct/path/a -b /correct/path/b
```

#### "Error: Path A is not a directory"

**Cause**: The path exists but is a file, not a directory.

**Solution**: Ensure you're comparing directories, not files.

#### "Error: Invalid comparison method"

**Cause**: The specified method is not recognized.

**Solution**: Use one of the valid methods:
```bash
--method filename    # or "name"
--method size
--method sampled     # or "sampled-hash"
--method hash        # or "fxhash" or "fasthash"
```

#### "Error: Invalid format"

**Cause**: The specified output format is not recognized.

**Solution**: Use one of the valid formats:
```bash
--format text        # or "txt"
--format html
--format markdown    # or "md"
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

## License

MIT License - see LICENSE file for details.
