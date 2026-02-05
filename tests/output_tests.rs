#[cfg(test)]
mod tests {
    use dir_compare::comparison::{ComparisonResult, Entry, EntryKind};
    use dir_compare::output::{Formatter, HtmlFormatter, MarkdownFormatter, TextFormatter};
    use std::path::PathBuf;

    fn create_test_result() -> ComparisonResult {
        let mut a_only = Vec::new();
        let mut b_only = Vec::new();
        let mut both = Vec::new();

        a_only.push(Entry {
            path: PathBuf::from("dir_a-only/"),
            kind: EntryKind::Directory,
            size: None,
        });
        a_only.push(Entry {
            path: PathBuf::from("file_a.txt"),
            kind: EntryKind::File,
            size: Some(100),
        });

        b_only.push(Entry {
            path: PathBuf::from("dir_b-only/"),
            kind: EntryKind::Directory,
            size: None,
        });
        b_only.push(Entry {
            path: PathBuf::from("file_b.txt"),
            kind: EntryKind::File,
            size: Some(200),
        });

        both.push((
            Entry {
                path: PathBuf::from("common.txt"),
                kind: EntryKind::File,
                size: Some(150),
            },
            Entry {
                path: PathBuf::from("common.txt"),
                kind: EntryKind::File,
                size: Some(150),
            },
        ));

        ComparisonResult {
            a_only,
            b_only,
            both,
        }
    }

    #[test]
    fn test_text_formatter_output() {
        let result = create_test_result();
        let formatter = TextFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("A-only"));
        assert!(output.contains("B-only"));
        assert!(output.contains("Both"));
        assert!(output.contains("2 entries"));
        assert!(output.contains("dir_a-only/"));
        assert!(output.contains("file_a.txt"));
    }

    #[test]
    fn test_text_formatter_directory_indicators() {
        let result = create_test_result();
        let formatter = TextFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("dir_a-only/"));
        assert!(output.contains("dir_b-only/"));
        assert!(!output.ends_with("/") || output.contains("file_a.txt"));
    }

    #[test]
    fn test_text_formatter_empty_categories() {
        let empty_result = ComparisonResult {
            a_only: Vec::new(),
            b_only: Vec::new(),
            both: Vec::new(),
        };
        let formatter = TextFormatter;
        let output = formatter.format(&empty_result);

        assert!(output.contains("0 entries"));
    }

    #[test]
    fn test_html_formatter_structure() {
        let result = create_test_result();
        let formatter = HtmlFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("<html"));
        assert!(output.contains("<title>Directory Comparison Report</title>"));
        assert!(output.contains("<div class=\"container\">"));
    }

    #[test]
    fn test_html_formatter_summary() {
        let result = create_test_result();
        let formatter = HtmlFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("A-only"));
        assert!(output.contains("B-only"));
        assert!(output.contains("Both"));
        assert!(output.contains("class=\"summary\""));
    }

    #[test]
    fn test_html_formatter_sections() {
        let result = create_test_result();
        let formatter = HtmlFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("<h2>A-only</h2>"));
        assert!(output.contains("<h2>B-only</h2>"));
        assert!(output.contains("<h2>Both</h2>"));
    }

    #[test]
    fn test_markdown_formatter_structure() {
        let result = create_test_result();
        let formatter = MarkdownFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("# Directory Comparison Report"));
        assert!(output.contains("## Summary"));
        assert!(output.contains("## A-only"));
        assert!(output.contains("## B-only"));
        assert!(output.contains("## Both"));
    }

    #[test]
    fn test_markdown_formatter_table() {
        let result = create_test_result();
        let formatter = MarkdownFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("| A-only |"));
        assert!(output.contains("| B-only |"));
        assert!(output.contains("| Both |"));
    }

    #[test]
    fn test_markdown_formatter_code_formatting() {
        let result = create_test_result();
        let formatter = MarkdownFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("`"));
        assert!(output.contains(r"file\_a.txt"));
    }

    #[test]
    fn test_special_characters_html() {
        let mut result = ComparisonResult {
            a_only: Vec::new(),
            b_only: Vec::new(),
            both: Vec::new(),
        };
        result.a_only.push(Entry {
            path: PathBuf::from("file<with>&\"chars.txt"),
            kind: EntryKind::File,
            size: Some(100),
        });

        let formatter = HtmlFormatter;
        let output = formatter.format(&result);

        assert!(output.contains("&lt;"));
        assert!(output.contains("&gt;"));
        assert!(output.contains("&amp;"));
        assert!(output.contains("&quot;"));
    }

    #[test]
    fn test_special_characters_markdown() {
        let mut result = ComparisonResult {
            a_only: Vec::new(),
            b_only: Vec::new(),
            both: Vec::new(),
        };
        result.a_only.push(Entry {
            path: PathBuf::from("file_with_*.txt"),
            kind: EntryKind::File,
            size: Some(100),
        });

        let formatter = MarkdownFormatter;
        let output = formatter.format(&result);

        assert!(output.contains(r"\*"));
    }

    #[test]
    fn test_unicode_handling() {
        let mut result = ComparisonResult {
            a_only: Vec::new(),
            b_only: Vec::new(),
            both: Vec::new(),
        };
        result.a_only.push(Entry {
            path: PathBuf::from("файл-тест.txt"),
            kind: EntryKind::File,
            size: Some(100),
        });
        result.a_only.push(Entry {
            path: PathBuf::from("测试文件.txt"),
            kind: EntryKind::File,
            size: Some(200),
        });
        result.a_only.push(Entry {
            path: PathBuf::from("emoji🎉.txt"),
            kind: EntryKind::File,
            size: Some(50),
        });

        let text_formatter = TextFormatter;
        let text_output = text_formatter.format(&result);
        assert!(text_output.contains("файл-тест.txt"));
        assert!(text_output.contains("测试文件.txt"));
        assert!(text_output.contains("emoji🎉.txt"));

        let html_formatter = HtmlFormatter;
        let html_output = html_formatter.format(&result);
        assert!(html_output.contains("файл-тест.txt") || html_output.contains("файл"));

        let md_formatter = MarkdownFormatter;
        let md_output = md_formatter.format(&result);
        assert!(md_output.contains("файл-тест.txt"));
    }
}
