# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Library and CLI implementation.
- `src/main.rs`: CLI entrypoint and argument parsing.
- `src/comparison.rs`: Directory scanning and comparison strategies.
- `src/output.rs`: Text/HTML/Markdown formatters.
- `src/lib.rs`: Public library surface.
- `tests/`: Integration and unit tests (`cli_tests.rs`, `unit_tests.rs`, `output_tests.rs`).
- `Cargo.toml`: Crate metadata and dependencies.
- `README.md`: User-facing usage and examples.

## Build, Test, and Development Commands
- `cargo build`: Build debug binaries.
- `cargo build --release`: Build optimized binaries for distribution.
- `cargo run -- -a dir_a -b dir_b`: Run the CLI locally with arguments.
- `cargo test`: Run the full test suite (unit + integration).
- `cargo clippy`: Run the Rust linter (if installed) to catch common issues.

## Test Coverage Commands
- **Install tarpaulin**: `cargo install cargo-tarpaulin`
- **Run coverage**: `cargo tarpaulin --workspace --out Html --output-dir target/coverage`
- **View coverage report**: Open `target/coverage/tarpaulin-report.html` in a browser
- **Coverage configuration**: `.tarpaulin.toml` contains settings for coverage generation
- **Coverage outputs**: HTML, XML (Cobertura), and LCOV formats in `target/coverage/`
- **Example**: `cargo tarpaulin --package dir-compare-core --out Html` for single package coverage

## Coding Style & Naming Conventions
- Rust 2024 edition; follow standard Rust formatting (4-space indentation).
- Keep modules focused: comparison logic in `comparison.rs`, output concerns in `output.rs`.
- Test functions are named `test_<behavior>` (see `tests/*.rs`).
- Use idiomatic Rust naming: `snake_case` for functions/variables, `CamelCase` for types.
- Formatting/linting: no repo-specific config is present.
- Check style/formatting with `cargo fmt` (and `cargo fmt -- --check` in CI-style workflows).
- Optional linting with `cargo clippy` if installed.

## Testing Guidelines
- Frameworks: Rust test harness plus `assert_cmd`, `predicates`, and `tempfile`.
- Integration tests live in `tests/` and exercise the CLI and formatters.
- Prefer adding tests alongside the area touched (e.g., formatter changes in `output_tests.rs`).
- Run all tests with `cargo test`.
- Run a single test with `cargo test test_name` (e.g., `cargo test test_cli_invalid_format`).

## Commit & Pull Request Guidelines
- Commit messages are short, descriptive sentences (see `git log --oneline`).
  Example: `Add change notes for GUI and logger deprecation`.
- PRs should include a concise summary, test results, and any user-facing updates (README/examples) when behavior changes.
- Add sample output (text/HTML/Markdown) for formatter changes when practical.

## Security & Configuration Tips
- The tool reads filesystem paths provided by the user; avoid introducing new unsafe path handling.
- Keep CLI flags and error messages consistent with existing patterns in `src/main.rs`.
