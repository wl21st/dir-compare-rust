use crate::comparison::{ComparisonResult, EntryKind, FlatComparisonResult};

/// Trait for formatting directory comparison results.
///
/// Implement this trait to provide different output formats for
/// comparison results (e.g., text, HTML, Markdown).
///
/// # Examples
///
/// ```
/// use dir_compare_core::output::{Formatter, TextFormatter};
///
/// let formatter = TextFormatter;
/// // Assuming `result` is a valid ComparisonResult
/// // let output = formatter.format(&result);
/// ```
pub trait Formatter {
    /// Formats the comparison result into a string representation.
    ///
    /// # Arguments
    ///
    /// * `result` - The comparison result to format
    ///
    /// # Returns
    ///
    /// A formatted string representation of the comparison result
    fn format(&self, result: &ComparisonResult) -> String;
}

/// Formatter that outputs human-readable text format.
///
/// Produces a simple text output with section headers for A-only,
/// B-only, and both entries.
///
/// # Output Format
///
/// ```text
/// A-only (2 entries):
/// ----------------------------------------
///   dir_a-only/
///   file_a.txt
///
/// B-only (2 entries):
/// ----------------------------------------
///   dir_b-only/
///   file_b.txt
///
/// Both (1 entries):
/// ----------------------------------------
///   common.txt == common.txt
/// ```
pub struct TextFormatter;

impl Formatter for TextFormatter {
    fn format(&self, result: &ComparisonResult) -> String {
        let mut output = String::new();

        output.push_str(&format!("A-only ({} entries):\n", result.a_only.len()));
        output.push_str(&"-".repeat(40));
        output.push('\n');
        for entry in &result.a_only {
            let indicator = match entry.kind {
                EntryKind::Directory => "/",
                EntryKind::File => "",
            };
            output.push_str(&format!("  {}{}\n", entry.path.display(), indicator));
        }
        output.push('\n');

        output.push_str(&format!("B-only ({} entries):\n", result.b_only.len()));
        output.push_str(&"-".repeat(40));
        output.push('\n');
        for entry in &result.b_only {
            let indicator = match entry.kind {
                EntryKind::Directory => "/",
                EntryKind::File => "",
            };
            output.push_str(&format!("  {}{}\n", entry.path.display(), indicator));
        }
        output.push('\n');

        output.push_str(&format!("Both ({} entries):\n", result.both.len()));
        output.push_str(&"-".repeat(40));
        output.push('\n');
        for (a, b) in &result.both {
            output.push_str(&format!("  {} == {}\n", a.path.display(), b.path.display()));
        }

        output
    }
}

/// Formatter that outputs HTML format with inline CSS styling.
///
/// Produces a self-contained HTML document with professional styling,
/// summary statistics, and organized sections for A-only, B-only,
/// and both entries.
///
/// # Features
///
/// - Responsive layout with mobile-friendly design
/// - Summary statistics with color-coded boxes
/// - Directory and file type distinction
/// - Special character escaping for safe HTML display
/// - Works completely offline (no external dependencies)
///
/// # Usage
///
/// ```
/// use dir_compare_core::output::{HtmlFormatter, Formatter};
///
/// let formatter = HtmlFormatter;
/// // let html = formatter.format(&result);
/// ```
pub struct HtmlFormatter;

impl Formatter for HtmlFormatter {
    fn format(&self, result: &ComparisonResult) -> String {
        let mut html = String::new();

        html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Directory Comparison Report</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        h1 { color: #333; border-bottom: 2px solid #4a90d9; padding-bottom: 10px; }
        .summary { display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; margin-bottom: 30px; }
        .summary-box { background: #f8f9fa; padding: 15px; border-radius: 6px; text-align: center; border-left: 4px solid #4a90d9; }
        .summary-box.a-only { border-left-color: #dc3545; }
        .summary-box.b-only { border-left-color: #28a745; }
        .summary-box.both { border-left-color: #6c757d; }
        .count { font-size: 2em; font-weight: bold; color: #333; }
        .label { color: #666; font-size: 0.9em; }
        .section { margin-bottom: 30px; }
        .section h2 { color: #444; border-bottom: 1px solid #eee; padding-bottom: 8px; }
        .entry-list { list-style: none; padding: 0; margin: 0; }
        .entry-list li { padding: 8px 12px; border-bottom: 1px solid #eee; }
        .entry-list li:last-child { border-bottom: none; }
        .dir { color: #0066cc; }
        .file { color: #333; }
        .empty { color: #999; font-style: italic; }
        .comparison { color: #666; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Directory Comparison Report</h1>
        <div class="summary">
            <div class="summary-box a-only">
                <div class="count">"#);
        html.push_str(&result.a_only.len().to_string());
        html.push_str(
            r#"</div>
                <div class="label">A-only</div>
            </div>
            <div class="summary-box b-only">
                <div class="count">"#,
        );
        html.push_str(&result.b_only.len().to_string());
        html.push_str(
            r#"</div>
                <div class="label">B-only</div>
            </div>
            <div class="summary-box both">
                <div class="count">"#,
        );
        html.push_str(&result.both.len().to_string());
        html.push_str(
            r#"</div>
                <div class="label">Both</div>
            </div>
        </div>
"#,
        );

        html.push_str(
            r#"
        <div class="section">
            <h2>A-only</h2>
"#,
        );
        if result.a_only.is_empty() {
            html.push_str(
                r#"            <p class="empty">No entries</p>
"#,
            );
        } else {
            html.push_str(
                r#"            <ul class="entry-list">
"#,
            );
            for entry in &result.a_only {
                let class = match entry.kind {
                    EntryKind::Directory => "dir",
                    EntryKind::File => "file",
                };
                let indicator = match entry.kind {
                    EntryKind::Directory => "/",
                    EntryKind::File => "",
                };
                html.push_str(&format!(
                    r#"                <li class="{}">{}{}</li>
"#,
                    class,
                    escape_html(&entry.path.display().to_string()),
                    indicator
                ));
            }
            html.push_str(
                r#"            </ul>
"#,
            );
        }
        html.push_str(
            r#"        </div>
"#,
        );

        html.push_str(
            r#"
        <div class="section">
            <h2>B-only</h2>
"#,
        );
        if result.b_only.is_empty() {
            html.push_str(
                r#"            <p class="empty">No entries</p>
"#,
            );
        } else {
            html.push_str(
                r#"            <ul class="entry-list">
"#,
            );
            for entry in &result.b_only {
                let class = match entry.kind {
                    EntryKind::Directory => "dir",
                    EntryKind::File => "file",
                };
                let indicator = match entry.kind {
                    EntryKind::Directory => "/",
                    EntryKind::File => "",
                };
                html.push_str(&format!(
                    r#"                <li class="{}">{}{}</li>
"#,
                    class,
                    escape_html(&entry.path.display().to_string()),
                    indicator
                ));
            }
            html.push_str(
                r#"            </ul>
"#,
            );
        }
        html.push_str(
            r#"        </div>
"#,
        );

        html.push_str(
            r#"
        <div class="section">
            <h2>Both</h2>
"#,
        );
        if result.both.is_empty() {
            html.push_str(
                r#"            <p class="empty">No matching entries</p>
"#,
            );
        } else {
            html.push_str(
                r#"            <ul class="entry-list">
"#,
            );
            for (a, b) in &result.both {
                html.push_str(&format!(
                    r#"                <li class="comparison">{} == {}</li>
"#,
                    escape_html(&a.path.display().to_string()),
                    escape_html(&b.path.display().to_string())
                ));
            }
            html.push_str(
                r#"            </ul>
"#,
            );
        }
        html.push_str(
            r#"        </div>
    </div>
</body>
</html>
"#,
        );

        html
    }
}

fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}

/// Formatter that outputs Markdown format.
///
/// Produces a clean Markdown document suitable for documentation,
/// version control, or further processing.
///
/// # Features
///
/// - Proper Markdown headers and structure
/// - Summary statistics table
/// - Code formatting for file paths
/// - Special character escaping
/// - Plain-text readable output
///
/// # Output Example
///
/// ```markdown
/// # Directory Comparison Report
///
/// ## Summary
///
/// | Category | Count |
/// |---------|-------|
/// | A-only | 2 |
/// | B-only | 2 |
/// | Both | 1 |
///
/// ## A-only
///
/// - `dir_a-only/`
/// - `file_a.txt`
///
/// ## B-only
///
/// - `dir_b-only/`
/// - `file_b.txt`
///
/// ## Both
///
/// - `common.txt` == `common.txt`
/// ```
pub struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, result: &ComparisonResult) -> String {
        let mut md = String::new();

        md.push_str("# Directory Comparison Report\n\n");

        md.push_str("## Summary\n\n");
        md.push_str(&format!("| Category | Count |\n|---------|-------|\n"));
        md.push_str(&format!("| A-only | {} |\n", result.a_only.len()));
        md.push_str(&format!("| B-only | {} |\n", result.b_only.len()));
        md.push_str(&format!("| Both | {} |\n\n", result.both.len()));

        md.push_str("## A-only\n\n");
        if result.a_only.is_empty() {
            md.push_str("*No entries*\n\n");
        } else {
            for entry in &result.a_only {
                let indicator = match entry.kind {
                    EntryKind::Directory => "/",
                    EntryKind::File => "",
                };
                md.push_str(&format!(
                    "- `{}{}`\n",
                    escape_markdown(&entry.path.display().to_string()),
                    indicator
                ));
            }
            md.push('\n');
        }

        md.push_str("## B-only\n\n");
        if result.b_only.is_empty() {
            md.push_str("*No entries*\n\n");
        } else {
            for entry in &result.b_only {
                let indicator = match entry.kind {
                    EntryKind::Directory => "/",
                    EntryKind::File => "",
                };
                md.push_str(&format!(
                    "- `{}{}`\n",
                    escape_markdown(&entry.path.display().to_string()),
                    indicator
                ));
            }
            md.push('\n');
        }

        md.push_str("## Both\n\n");
        if result.both.is_empty() {
            md.push_str("*No matching entries*\n\n");
        } else {
            for (a, b) in &result.both {
                md.push_str(&format!(
                    "- `{}` == `{}`\n",
                    escape_markdown(&a.path.display().to_string()),
                    escape_markdown(&b.path.display().to_string())
                ));
            }
            md.push('\n');
        }

        md
    }
}

fn escape_markdown(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('`', r"\`")
        .replace('*', r"\*")
        .replace('_', r"\_")
}

/// Formatter for flat mode comparison results (text format).
///
/// Groups files by content hash and shows duplicate/moved files.
pub struct FlatTextFormatter;

impl FlatTextFormatter {
    /// Formats the flat comparison result into a string representation.
    pub fn format(&self, result: &FlatComparisonResult) -> String {
        let mut output = String::new();

        // Summary section
        output.push_str("Flat Mode Comparison Summary\n");
        output.push_str(&"=".repeat(50));
        output.push('\n');
        output.push_str(&format!("Files in directory A: {}\n", result.total_files_a));
        output.push_str(&format!("Files in directory B: {}\n", result.total_files_b));
        output.push_str(&format!(
            "Unique content hashes: {}\n",
            result.unique_hashes
        ));
        output.push_str(&format!(
            "Duplicate content groups: {}\n",
            result.duplicate_count
        ));
        output.push('\n');

        // Group details
        for group in &result.groups {
            let in_a = !group.files_in_a.is_empty();
            let in_b = !group.files_in_b.is_empty();
            let is_duplicate = group.file_count > 1;

            // Header line with hash and metadata
            let status = if is_duplicate {
                "[DUPLICATE]"
            } else if in_a && in_b {
                "[MATCHED]"
            } else if in_a {
                "[A-ONLY]"
            } else {
                "[B-ONLY]"
            };

            output.push_str(&format!(
                "Hash: {} {} ({} bytes, {} files)\n",
                &group.hash[..16.min(group.hash.len())],
                status,
                group.size,
                group.file_count
            ));
            output.push_str(&"-".repeat(50));
            output.push('\n');

            // Files in A
            for path in &group.files_in_a {
                if in_b {
                    output.push_str(&format!(
                        "  [A] {} -> (moved/copied to B)\n",
                        path.display()
                    ));
                } else {
                    output.push_str(&format!("  [A] {}\n", path.display()));
                }
            }

            // Files in B
            for path in &group.files_in_b {
                if in_a {
                    output.push_str(&format!(
                        "  [B] {} <- (moved/copied from A)\n",
                        path.display()
                    ));
                } else {
                    output.push_str(&format!("  [B] {}\n", path.display()));
                }
            }

            output.push('\n');
        }

        output
    }
}

/// Formatter for flat mode comparison results (HTML format).
pub struct FlatHtmlFormatter;

impl FlatHtmlFormatter {
    /// Formats the flat comparison result into an HTML representation.
    pub fn format(&self, result: &FlatComparisonResult) -> String {
        let mut html = String::new();

        html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Flat Mode Directory Comparison Report</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        h1 { color: #333; border-bottom: 2px solid #4a90d9; padding-bottom: 10px; }
        .summary { display: grid; grid-template-columns: repeat(4, 1fr); gap: 20px; margin-bottom: 30px; }
        .summary-box { background: #f8f9fa; padding: 15px; border-radius: 6px; text-align: center; border-left: 4px solid #4a90d9; }
        .count { font-size: 2em; font-weight: bold; color: #333; }
        .label { color: #666; font-size: 0.9em; }
        .hash-group { margin-bottom: 20px; border: 1px solid #ddd; border-radius: 6px; overflow: hidden; }
        .hash-header { background: #f8f9fa; padding: 12px 15px; border-bottom: 1px solid #ddd; }
        .hash-header.duplicate { background: #fff3cd; border-left: 4px solid #ffc107; }
        .hash-header.matched { background: #d1ecf1; border-left: 4px solid #17a2b8; }
        .hash-header.a-only { background: #f8d7da; border-left: 4px solid #dc3545; }
        .hash-header.b-only { background: #d4edda; border-left: 4px solid #28a745; }
        .hash-value { font-family: monospace; font-weight: bold; color: #333; }
        .hash-meta { color: #666; font-size: 0.9em; margin-top: 4px; }
        .file-list { list-style: none; padding: 0; margin: 0; }
        .file-list li { padding: 8px 15px; border-bottom: 1px solid #eee; }
        .file-list li:last-child { border-bottom: none; }
        .file-a { color: #dc3545; }
        .file-b { color: #28a745; }
        .badge { display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 0.75em; font-weight: bold; margin-right: 8px; }
        .badge-a { background: #dc3545; color: white; }
        .badge-b { background: #28a745; color: white; }
        .badge-moved { background: #17a2b8; color: white; }
        .badge-dup { background: #ffc107; color: #333; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Flat Mode Comparison Report</h1>
        <div class="summary">
            <div class="summary-box">
                <div class="count">"#);
        html.push_str(&result.total_files_a.to_string());
        html.push_str(
            r#"</div>
                <div class="label">Files in A</div>
            </div>
            <div class="summary-box">
                <div class="count">"#,
        );
        html.push_str(&result.total_files_b.to_string());
        html.push_str(
            r#"</div>
                <div class="label">Files in B</div>
            </div>
            <div class="summary-box">
                <div class="count">"#,
        );
        html.push_str(&result.unique_hashes.to_string());
        html.push_str(
            r#"</div>
                <div class="label">Unique Hashes</div>
            </div>
            <div class="summary-box">
                <div class="count">"#,
        );
        html.push_str(&result.duplicate_count.to_string());
        html.push_str(
            r#"</div>
                <div class="label">Duplicates</div>
            </div>
        </div>
"#,
        );

        for group in &result.groups {
            let in_a = !group.files_in_a.is_empty();
            let in_b = !group.files_in_b.is_empty();
            let is_duplicate = group.file_count > 1;

            let header_class = if is_duplicate {
                "duplicate"
            } else if in_a && in_b {
                "matched"
            } else if in_a {
                "a-only"
            } else {
                "b-only"
            };

            let status_badge = if is_duplicate {
                r#"<span class="badge badge-dup">DUPLICATE</span>"#
            } else if in_a && in_b {
                r#"<span class="badge badge-moved">MOVED</span>"#
            } else if in_a {
                r#"<span class="badge badge-a">A-ONLY</span>"#
            } else {
                r#"<span class="badge badge-b">B-ONLY</span>"#
            };

            html.push_str(&format!(
                r#"        <div class="hash-group">
            <div class="hash-header {}">
                <div class="hash-value">{}{}</div>
                <div class="hash-meta">Size: {} bytes | {} file(s)</div>
            </div>
            <ul class="file-list">
"#,
                header_class,
                status_badge,
                escape_html(&group.hash),
                group.size,
                group.file_count
            ));

            for path in &group.files_in_a {
                let moved_indicator = if in_b {
                    r#" <span class="badge badge-moved">→</span>"#
                } else {
                    ""
                };
                html.push_str(&format!(
                    r#"                <li class="file-a"><span class="badge badge-a">A</span>{}{}</li>
"#,
                    escape_html(&path.display().to_string()),
                    moved_indicator
                ));
            }

            for path in &group.files_in_b {
                let moved_indicator = if in_a {
                    r#" <span class="badge badge-moved">←</span>"#
                } else {
                    ""
                };
                html.push_str(&format!(
                    r#"                <li class="file-b"><span class="badge badge-b">B</span>{}{}</li>
"#,
                    escape_html(&path.display().to_string()),
                    moved_indicator
                ));
            }

            html.push_str(
                r#"            </ul>
        </div>
"#,
            );
        }

        html.push_str(
            r#"    </div>
</body>
</html>
"#,
        );

        html
    }
}

/// Formatter for flat mode comparison results (Markdown format).
pub struct FlatMarkdownFormatter;

impl FlatMarkdownFormatter {
    /// Formats the flat comparison result into a Markdown representation.
    pub fn format(&self, result: &FlatComparisonResult) -> String {
        let mut md = String::new();

        md.push_str("# Flat Mode Comparison Report\n\n");

        md.push_str("## Summary\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!(
            "| Files in directory A | {} |\n",
            result.total_files_a
        ));
        md.push_str(&format!(
            "| Files in directory B | {} |\n",
            result.total_files_b
        ));
        md.push_str(&format!(
            "| Unique content hashes | {} |\n",
            result.unique_hashes
        ));
        md.push_str(&format!(
            "| Duplicate groups | {} |\n\n",
            result.duplicate_count
        ));

        md.push_str("## Content Groups\n\n");

        for group in &result.groups {
            let in_a = !group.files_in_a.is_empty();
            let in_b = !group.files_in_b.is_empty();
            let is_duplicate = group.file_count > 1;

            let status = if is_duplicate {
                "DUPLICATE"
            } else if in_a && in_b {
                "MOVED"
            } else if in_a {
                "A-ONLY"
            } else {
                "B-ONLY"
            };

            md.push_str(&format!(
                "### Hash: `{}...` ({} - {} bytes, {} files)\n\n",
                &group.hash[..16.min(group.hash.len())],
                status,
                group.size,
                group.file_count
            ));

            if !group.files_in_a.is_empty() {
                md.push_str("**Directory A:**\n\n");
                for path in &group.files_in_a {
                    if in_b {
                        md.push_str(&format!(
                            "- `{}` *(moved/copied to B)*\n",
                            escape_markdown(&path.display().to_string())
                        ));
                    } else {
                        md.push_str(&format!(
                            "- `{}`\n",
                            escape_markdown(&path.display().to_string())
                        ));
                    }
                }
                md.push('\n');
            }

            if !group.files_in_b.is_empty() {
                md.push_str("**Directory B:**\n\n");
                for path in &group.files_in_b {
                    if in_a {
                        md.push_str(&format!(
                            "- `{}` *(moved/copied from A)*\n",
                            escape_markdown(&path.display().to_string())
                        ));
                    } else {
                        md.push_str(&format!(
                            "- `{}`\n",
                            escape_markdown(&path.display().to_string())
                        ));
                    }
                }
                md.push('\n');
            }
        }

        md
    }
}
