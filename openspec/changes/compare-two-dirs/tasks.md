## 1. Project Setup & Infrastructure

- [x] 1.1 Add project dependencies to Cargo.toml (`walkdir`, `clap`, `fxhash` or `xxhash`)
- [x] 1.2 Create module structure: `src/lib.rs`, `src/main.rs`, `src/cli.rs`, `src/comparison.rs`, `src/output.rs`
- [x] 1.3 Set up error handling with custom error types for permissions, invalid paths, I/O errors

## 2. Core Directory Traversal

- [x] 2.1 Implement directory traversal module that recursively scans both directories
- [x] 2.2 Handle permission errors gracefully (log warnings, continue traversal)
- [x] 2.3 Validate input paths and report errors for non-existent directories
- [x] 2.4 Normalize paths for cross-platform compatibility (Windows/Unix)
- [x] 2.5 Distinguish files from directories in results
- [x] 2.6 Handle symlinks by treating them as regular files (no following)

## 3. Comparison Strategy Pattern Implementation

- [x] 3.1 Define `ComparisonStrategy` trait with comparison logic
- [x] 3.2 Implement `FilenameOnlyStrategy` (case-sensitive and case-insensitive options)
- [x] 3.3 Implement `FilenameSizeStrategy` (filename + size matching for files, name-only for directories)
- [x] 3.4 Implement `FastHashStrategy` (filename + content hash using fxhash/xxhash)
- [x] 3.5 Add streaming hash computation for large files
- [x] 3.6 Create strategy factory/selector based on CLI argument

## 4. Comparison Results Organization

- [x] 4.1 Implement result categorization logic (A-only, B-only, both)
- [x] 4.2 Create data structures to store categorized results
- [x] 4.3 Implement sorting of entries within each category (alphabetical by path)
- [x] 4.4 Calculate and store count summaries for each category
- [x] 4.5 Handle empty directory cases correctly

## 5. Text Output Formatter

- [x] 5.1 Implement `Formatter` trait for pluggable output formats
- [x] 5.2 Implement `TextFormatter` (human-readable console output)
- [x] 5.3 Format text output with clear section headers (A-only, B-only, both)
- [x] 5.4 Display relative paths with file/directory indicators (e.g., "/" suffix for directories)
- [x] 5.5 Include summary counts in text output
- [x] 5.6 Handle special characters and Unicode in text output
- [x] 5.7 Ensure sorted, consistent output

## 6. HTML Output Formatter

- [x] 6.1 Implement `HtmlFormatter` with inline CSS styling
- [x] 6.2 Generate well-formed HTML document structure with DOCTYPE and meta tags
- [x] 6.3 Create inline CSS with professional styling (responsive layout, colors, borders)
- [x] 6.4 Structure HTML with three sections for A-only, B-only, both entries
- [x] 6.5 Include summary statistics table in HTML output
- [x] 6.6 Display relative paths with file/directory distinction in HTML
- [x] 6.7 Use HTML tables or lists for organized entry display
- [x] 6.8 Ensure HTML is self-contained and works offline

## 7. Markdown Output Formatter

- [x] 7.1 Implement `MarkdownFormatter` for Markdown document generation
- [x] 7.2 Create Markdown structure with proper headers (##) for sections
- [x] 7.3 Use Markdown lists for entry display
- [x] 7.4 Add summary statistics section with count summaries
- [x] 7.5 Properly escape and handle special characters in Markdown
- [x] 7.6 Use Markdown code formatting (backticks) for file paths
- [x] 7.7 Ensure output is plain-text readable and properly formatted

## 8. Output File Writing

- [x] 8.1 Implement file writing functionality with error handling
- [x] 8.2 Create parent directories if they don't exist
- [x] 8.3 Handle file overwrite behavior (document choice: prompt or silently overwrite)
- [x] 8.4 Report clear error messages for permission/path errors
- [x] 8.5 Support writing to stdout (when no output file specified)

## 9. CLI Interface

- [x] 9.1 Set up argument parsing with `clap` for directory paths
- [x] 9.2 Add `--method` flag to select comparison strategy (filename/size/hash)
- [x] 9.3 Add `--case-insensitive` flag for case-insensitive filename comparison
- [x] 9.4 Add `--format` flag to select output format (text/html/markdown)
- [x] 9.5 Add `--output` flag to specify output file path
- [x] 9.6 Set sensible defaults (filename comparison, text output to stdout)
- [x] 9.7 Add help messages and usage documentation

## 10. Integration & Testing

- [x] 10.1 Write unit tests for directory traversal with various directory structures
- [x] 10.2 Test permission error handling
- [x] 10.3 Write tests for each comparison strategy with sample files
- [x] 10.4 Test result categorization logic with overlapping/non-overlapping directories
- [x] 10.5 Test text, HTML, and Markdown formatters with various file/directory names
- [x] 10.6 Test special character and Unicode handling in all output formats
- [x] 10.7 Test CLI argument parsing and flag combinations
- [x] 10.8 Create integration test with real directories
- [x] 10.9 Test cross-platform path handling (if applicable to test environment)

## 11. Documentation & Examples

- [x] 11.1 Add inline code documentation for public API
- [x] 11.2 Create example usage in README
- [x] 11.3 Document comparison method trade-offs and performance characteristics
- [x] 11.4 Document HTML and Markdown output format examples
- [x] 11.5 Add troubleshooting guide for common errors
