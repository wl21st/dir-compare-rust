# Release Notes v1.0.0

## New Features

- **Cross-Platform GUI**: A new graphical user interface (`dir-compare-gui`) built with `egui`.
  - Interactive tree view of directory differences.
  - Visual color coding for added/removed/matching files.
  - Native file pickers for selecting directories.
  - Light/Dark theme support.
  - Comparison method selection (Filename, Size, Hash).

- **Core Library Extraction**: The core logic has been extracted to `dir-compare-core` crate, enabling reuse.

- **Workspace Structure**: The project is now a Cargo workspace with `cli`, `gui`, and `core` crates.

## Improvements

- **Performance**: Improved path handling and hashing performance (using absolute paths internally).
- **Robustness**: Better error handling for permission issues and invalid paths.

## Breaking Changes

- The CLI binary is now part of `dir-compare-cli` package. Install with `cargo install --path cli`.
