## ADDED Requirements

### Requirement: Pluggable output formatter interface
The system SHALL support multiple output formats through a pluggable formatter interface, enabling users to export results in their preferred format without code modifications.

#### Scenario: Initialize formatter with selected output type
- **WHEN** user specifies an output format (text, HTML, or Markdown)
- **THEN** system initializes the appropriate formatter for that operation

#### Scenario: Extend formatters without core changes
- **WHEN** system is designed with pluggable formatters
- **THEN** new output formats can be added by implementing the Formatter trait without modifying existing code

### Requirement: HTML output format
The system SHALL export comparison results as a self-contained HTML report with embedded styling, suitable for sharing and viewing in web browsers.

#### Scenario: Generate HTML report
- **WHEN** user selects HTML output format
- **THEN** system generates a well-formed HTML document with comparison results

#### Scenario: HTML includes all three categories
- **WHEN** HTML report is generated
- **THEN** report contains clearly labeled sections for: A-only entries, B-only entries, and entries in both

#### Scenario: HTML has inline CSS styling
- **WHEN** HTML report is generated
- **THEN** report includes inline `<style>` tag with professional styling (no external stylesheets required)

#### Scenario: HTML is self-contained
- **WHEN** HTML report is saved and shared
- **THEN** report works offline and displays correctly without external resources or dependencies

#### Scenario: HTML includes summary statistics
- **WHEN** HTML report is generated
- **THEN** report displays count summaries for each category in an easy-to-scan format

#### Scenario: HTML displays relative paths
- **WHEN** HTML report shows entries
- **THEN** each entry displays relative path with clear file/directory distinction (e.g., trailing slash for directories)

#### Scenario: HTML table is sortable in concept
- **WHEN** HTML report is viewed in browser
- **THEN** user can easily scan and understand the comparison results with organized table layout

### Requirement: Markdown output format
The system SHALL export comparison results as a Markdown document, suitable for version control, documentation, and integration with wikis and READMEs.

#### Scenario: Generate Markdown report
- **WHEN** user selects Markdown output format
- **THEN** system generates a Markdown document with comparison results

#### Scenario: Markdown includes all three categories
- **WHEN** Markdown report is generated
- **THEN** report contains clearly labeled sections (using Markdown headers) for: A-only, B-only, and both

#### Scenario: Markdown uses proper formatting
- **WHEN** Markdown report is generated
- **THEN** report uses appropriate Markdown syntax: headers (##), lists, code blocks, emphasis as needed

#### Scenario: Markdown handles special characters
- **WHEN** report contains paths with special characters
- **THEN** Markdown properly escapes or formats them (e.g., backticks for code-like entries or HTML entities for special chars)

#### Scenario: Markdown includes statistics summary
- **WHEN** Markdown report is generated
- **THEN** report includes count summaries in a summary section

#### Scenario: Markdown is plain text compatible
- **WHEN** Markdown document is created
- **THEN** document is human-readable as plain text and renders correctly in Markdown viewers

### Requirement: Output file writing
The system SHALL write formatted output to specified file paths with proper error handling and user feedback.

#### Scenario: Write to file
- **WHEN** user specifies output file path
- **THEN** system writes the formatted report to that file

#### Scenario: Handle file overwrite
- **WHEN** output file already exists
- **THEN** system either prompts user for confirmation or silently overwrites (behavior specified in CLI design)

#### Scenario: Report write errors
- **WHEN** system cannot write to specified path (permissions, invalid path)
- **THEN** system reports clear error message indicating the issue

#### Scenario: Create parent directories if needed
- **WHEN** output path contains non-existent parent directories
- **THEN** system creates parent directories as needed (or reports error if unable)

### Requirement: CLI integration for output formats
The system SHALL provide CLI flags to select output format and specify output file path.

#### Scenario: Specify output format via CLI flag
- **WHEN** user runs comparison with `--format html` or `--format markdown`
- **THEN** system uses that formatter for output

#### Scenario: Specify output file via CLI flag
- **WHEN** user runs comparison with `--output report.html`
- **THEN** system writes formatted output to that file

#### Scenario: Combined format and output flags
- **WHEN** user specifies both `--format markdown --output results.md`
- **THEN** system generates Markdown-formatted report and saves to `results.md`

#### Scenario: Default output format
- **WHEN** user does not specify output format
- **THEN** system defaults to text output (human-readable format)
