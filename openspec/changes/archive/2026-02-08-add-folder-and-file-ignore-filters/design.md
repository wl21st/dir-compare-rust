## Context

The current implementation of `dir-compare` lacks the ability to ignore specific files and folders during comparison. This can lead to noisy output and slower performance when comparing directories with build artifacts, logs, or other non-essential files.

The proposal artifact outlines the need for a `.dir-compare-ignore` file to address this. This design document details the technical implementation.

## Goals / Non-Goals

**Goals:**

- Implement a mechanism to parse a `.dir-compare-ignore` file.
- The ignore file should support glob patterns.
- The ignore patterns should be applied to both files and directories.
- The `core` library should be responsible for the ignore logic.
- The `cli` and `gui` applications should be updated to use the ignore functionality.

**Non-Goals:**

- Support for ignore files other than `.dir-compare-ignore`.
- Complex ignore rule syntax beyond standard glob patterns.
- GUI for managing ignore patterns.

## Decisions

1.  **Ignore File Parsing Library**:
    -   **Decision**: Use the `ignore` crate.
    -   **Rationale**: The `ignore` crate is a mature and widely used library for parsing ignore files (`.gitignore` style). It provides a robust and efficient implementation of glob pattern matching and directory traversal. It also has good performance characteristics.
    -   **Alternatives**:
        -   `glob`: The `glob` crate could be used for basic pattern matching, but it doesn't provide the same level of integration for directory traversal and ignore file semantics.
        -   Custom implementation: Building a custom ignore file parser would be time-consuming and likely less robust than using a dedicated library.

2.  **Integration with Core Logic**:
    -   **Decision**: The `Comparison` struct in the `core` library will be modified to accept an optional `ignore` file path.
    -   **Rationale**: This will allow the ignore logic to be self-contained within the `core` library, making it easy to use from both the `cli` and `gui` applications.
    -   **Implementation**: A new `WalkBuilder` from the `ignore` crate will be created in the `Comparison::new` function. This builder will be configured with the provided ignore file path. The `WalkBuilder` will then be used to create a `Walk` iterator, which will be used to traverse the directories.

3.  **CLI Changes**:
    -   **Decision**: A new `--ignore` option will be added to the `cli` application.
    -   **Rationale**: This will allow users to specify the path to the `.dir-compare-ignore` file. If not provided, the `cli` will automatically search for a `.dir-compare-ignore` file in the current directory and its parent directories.
    -   **Implementation**: The `clap` crate will be used to add the new `--ignore` option.

4.  **GUI Changes**:
    -   **Decision**: A new file selection dialog will be added to the `gui` application to allow users to select a `.dir-compare-ignore` file.
    -   **Rationale**: This provides a user-friendly way to specify the ignore file.
    -   **Implementation**: A new button will be added to the directory selection view. When clicked, it will open a file dialog to select the ignore file. The selected file path will be stored in the application state and passed to the `Comparison` struct.

## Risks / Trade-offs

-   **Risk**: The `ignore` crate might have a slight performance overhead compared to a custom implementation.
    -   **Mitigation**: The `ignore` crate is highly optimized, and the performance impact is expected to be negligible for most use cases. The benefits of using a well-tested and robust library outweigh the potential performance cost.
