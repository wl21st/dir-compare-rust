## Context

The project is a Rust-based directory comparison tool that needs to recursively scan two directory hierarchies and produce categorized listings of files/folders. This is a foundational feature that will support multiple comparison strategies and produce structured output. The implementation must handle cross-platform paths, efficiently traverse potentially large directory trees, and support pluggable comparison methods.

## Goals / Non-Goals

**Goals:**
- Implement a core engine that recursively compares two directory trees
- Support three comparison methods: filename-only, filename+size, and fast-hash
- Produce clear categorized results (A-only, B-only, both)
- Enable extensible comparison strategies for future additions
- Provide accurate and performant directory traversal

**Non-Goals:**
- Real-time file watching or continuous monitoring
- GUI or web interface (CLI only initially)
- Handling symlinks or special file types (treat as regular files/folders)
- Delta file transfers or synchronization operations

## Decisions

### 1. Strategy Pattern for Comparison Methods
**Decision:** Implement pluggable comparison strategies using a trait/interface pattern
- **Rationale:** Allows users to choose comparison method at runtime without code changes. Three methods (filename, size, hash) have different performance/accuracy trade-offs.
- **Alternatives Considered:**
  - Enum-based dispatch: Less extensible for future comparison types
  - Runtime plugin system: Overengineered for initial scope
- **Implementation:** `ComparisonStrategy` trait with three implementations

### 2. Rust Standard Library for Directory Traversal
**Decision:** Use `std::fs` and `walkdir` crate for recursive traversal
- **Rationale:** Reliable cross-platform support, handles permissions gracefully
- **Alternatives Considered:**
  - Manual recursive implementation: More error-prone and platform-specific
  - Custom allocation strategy: Unnecessary complexity at this stage
- **Implementation:** Use `walkdir::WalkDir` for efficient traversal with error handling

### 3. In-Memory Data Structure for Results
**Decision:** Collect results into three HashSets/HashMaps before output
- **Rationale:** Simple, fast membership testing, enables flexible output formatting
- **Alternatives Considered:**
  - Streaming output: Complicates comparison logic, worse for result categorization
  - Database storage: Overkill for typical use cases
- **Implementation:** `HashSet<PathBuf>` for A-only, B-only; HashMap/BTreeSet for "both" entries

### 4. Fast Hash Algorithm
**Decision:** Use xxHash or similar non-cryptographic hash for file comparison
- **Rationale:** Significantly faster than MD5/SHA1 for large files; sufficient for duplicate detection
- **Alternatives Considered:**
  - SHA256: Cryptographic security unnecessary, much slower
  - File content comparison: Byte-by-byte comparison without hashing for small files, hash for large
- **Implementation:** `fxhash` or `xxhash` crate; configurable chunk size for streaming hashes

### 5. CLI Structure
**Decision:** Single binary with subcommand for comparison mode
- **Rationale:** Simple, monolithic for initial version; can be refactored to separate commands later
- **Alternatives Considered:**
  - Separate binaries per strategy: More complex build and deployment
  - Library + binary: Premature abstraction
- **Implementation:** `clap` for CLI argument parsing

### 6. HTML and Markdown Output Formats
**Decision:** Implement formatter trait with HTML and Markdown implementations for result export
- **Rationale:** HTML enables interactive reports; Markdown enables version control, documentation integration, and easy sharing
- **Alternatives Considered:**
  - JSON only: Less human-friendly for reports and documentation
  - Template engine: Over-engineered; simple string builders sufficient
  - External tool dependency: Prefer native generation
- **Implementation:** `Formatter` trait with `HtmlFormatter` and `MarkdownFormatter` implementations; use simple HTML generation (no external template dependency); Markdown via direct string construction

### 7. HTML Report Styling
**Decision:** Inline CSS with embedded styling, no external stylesheets
- **Rationale:** Self-contained reports that work offline; no external dependencies; better for email sharing
- **Alternatives Considered:**
  - External CSS: Requires external resources, breaks in email
  - Tailwind CSS generation: Additional build complexity
- **Implementation:** Inline `<style>` tag with responsive grid layout; minimal but professional styling

## Risks / Trade-offs

- **Large Directory Trees**: In-memory storage of file lists could exceed memory for extremely large directories (100M+ files). *Mitigation*: Add streaming output option in future; document memory requirements.
- **Hash Collisions**: Non-cryptographic hashes have collision potential. *Mitigation*: Document limitation; offer fallback to size+mtime for users requiring higher certainty.
- **Permission Errors**: Some files may be unreadable. *Mitigation*: Skip unreadable files with warnings; add `--strict` flag to fail on errors.
- **Cross-Platform Path Handling**: Windows vs Unix path separators and case sensitivity. *Mitigation*: Normalize paths; offer case-sensitivity flag for comparison.
- **Performance on Network Filesystems**: Network delays during traversal. *Mitigation*: Not in scope for v1; document not recommended for network drives.

## Migration Plan

Since this is a new feature in an existing project:
1. Add new module `comparison` with strategy trait and implementations
2. Add CLI commands without affecting existing functionality
3. Tag release with new `compare` subcommand
4. No breaking changes to existing CLI

## Open Questions

- Should comparison ignore hidden files by default? (Unix dot-files, Windows hidden flag)
- What output format: table, JSON, CSV? Start with simple text, add JSON later?
- Should the tool support comparing individual files (not just directories)?
- Performance threshold: when should we recommend using hash vs size comparison?
- For HTML reports: should we include file metadata (dates, full paths)? Include counts? Navigation anchors?
- For Markdown: should we include a summary table of contents? Escape special characters or use code blocks?
