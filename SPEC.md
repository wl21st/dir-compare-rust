# Directory Compare & Sync Tool

## Overview

A high-performance CLI and GUI tool written in Rust for recursive file and directory synchronization.

## Key Features

### 1. Smart & Fast Comparison

- **Fast Sampling Hash**: Utilizes fast sampling hashes by default to rapidly compare large files content without reading the entire file.
- **Full Comparison**: Optional full-file comparison mode for bit-for-bit accuracy when needed.

### 2. Structure-Agnostic Comparison (Flat View)

- **Bridge Folder Structures**: Includes a "Flat View" mode that uses content hashing to match files across different folder structures.
- **Duplicate Detection**: Easily identify moved or duplicate files regardless of their location within the directory tree.

### 3. Versatile Sync Strategies

Supports a comprehensive range of synchronization modes to cover all use cases:

- **Source Only**: Mirror source to target.
- **Target Only**: Mirror target to source.
- **Merge**: Bidirectional synchronization.
- **Newer Only**: Update files only if the source is newer.
- **Orphan Removal**: Cleanup files that no longer exist in the source.
- **Inclusive/Exclusive**: Full support for inclusion and exclusion patterns.

## Technical Highlights

- **Language**: Rust (for memory safety and performance).
- **Interface**: Dual CLI and GUI support.
- **Efficiency**: Optimized for large file sets and complex directory structures.
