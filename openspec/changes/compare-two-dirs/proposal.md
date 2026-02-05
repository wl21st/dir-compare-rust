## Why

Users need to efficiently identify differences between two directory trees to find missing, duplicate, or modified files. This is essential for backup verification, synchronization validation, and file auditing. Currently, no built-in tool provides flexible comparison options (by name, size, or hash) with clear categorization of results.

## What Changes

- Add recursive directory comparison functionality that scans two folder hierarchies
- Generate three categorized result lists:
  - Files/folders only in Directory A
  - Files/folders only in Directory B
  - Files/folders present in both directories
- Support multiple comparison methods: filename-based, filename+size, and fast-hash algorithms
- Provide clear output reporting for comparison results

## Capabilities

### New Capabilities
- `dir-comparison`: Core recursive directory comparison engine that processes two directory trees and produces categorized file/folder listings
- `comparison-methods`: Pluggable comparison strategies (filename, filename+size, fast-hash) for flexible matching logic
- `comparison-output`: Structured result reporting for files/folders in A-only, B-only, and both directories
- `formatted-output`: HTML and Markdown export formats for comparison results enabling sharing, documentation, and report generation

### Modified Capabilities
<!-- No existing capabilities need requirement changes -->

## Impact

- CLI interface for directory comparison operations
- File system I/O operations for recursive directory traversal
- Performance considerations for fast-hash algorithm implementation
- Cross-platform compatibility for directory path handling
