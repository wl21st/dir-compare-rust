## Context
The current implementation of `dir-compare-rust` traverses directories recursively without any filtering mechanism. All files and directories are included in the comparison. Users need a way to exclude noise (like `.git` directories) and focus on specific files.

## Goals / Non-Goals

**Goals:**
- Implement robust file filtering based on glob patterns.
- Support standard `.gitignore` files.
- Provide CLI flags for including and excluding files.
- Maintain high performance during directory traversal.

**Non-Goals:**
- Regex-based filtering (glob is sufficient and standard for file paths).
- Interactive filtering UI.

## Decisions

### Use `ignore` crate
We will use the `ignore` crate (used by ripgrep) for directory traversal and filtering.
- **Why**: It handles `.gitignore` parsing, global gitignore rules, and recursive traversal efficiently and correctly. It replaces manual `WalkDir` usage if we switch to it, or we can use its `Gitignore` builder to filter manually if we stick to `walkdir`.
- **Alternative**: Implement `.gitignore` parsing manually. Rejected due to complexity and edge cases.
- **Decision**: Replace `walkdir` with `ignore::WalkBuilder` if possible, or use `ignore::gitignore::Gitignore` to filter paths yielded by `walkdir`. Given `ignore` handles parallel walking too, it might be a good upgrade, but for now, to minimize disruption, we might just use the `gitignore` parsing part or switch the walker.
- *Refinement*: The `ignore` crate's `WalkBuilder` is the standard way to get gitignore compliance. We should switch to it for the traversal.

### CLI Configuration
Update `clap` struct in `src/main.rs`.
- `--exclude <pattern>`: repeatable flag.
- `--include <pattern>`: repeatable flag.
- `--no-ignore`: flag to disable `.gitignore` respect.

## Risks / Trade-offs

### Performance
Filtering adds overhead for every file.
- **Mitigation**: The `ignore` crate is highly optimized for this specific use case.

### Dependency size
Adding `ignore` adds dependencies.
- **Mitigation**: It's a standard utility crate, acceptable for a CLI tool.
