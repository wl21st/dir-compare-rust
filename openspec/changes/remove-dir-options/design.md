## Context

The `dir-compare` CLI currently uses clap-derived named arguments `--dir-a` and `--dir-b` for specifying comparison directories. This is verbose compared to standard Unix tools (e.g., `diff dir1 dir2`). The change involves refactoring the CLI argument parser to accept positional arguments instead of named flags.

**Current state**: `cli/src/main.rs` uses clap `#[arg(short = 'a', long = "dir-a")]` and `#[arg(short = 'b', long = "dir-b")]` attributes.

**Constraints**:
- Must maintain backward compatibility at the library level (`dir_compare_core`)
- Only the CLI interface changes; comparison logic remains unchanged
- clap version and API compatibility should be considered

## Goals / Non-Goals

**Goals:**
- Replace `--dir-a` and `--dir-b` named arguments with positional arguments
- Improve user experience and align with standard Unix directory comparison tools
- Maintain all existing comparison strategies (`hash`, `sampled`, `filename`, `size`)
- Maintain all existing output formats (`text`, `html`, `markdown`)
- Update CLI help text and documentation

**Non-Goals:**
- Change the library API (`compare_directories` signature)
- Add new comparison or output capabilities
- Support legacy mixed syntax (e.g., first dir positional, second named)

## Decisions

**Decision 1: Use clap positional arguments**
- **Rationale**: clap provides robust positional argument handling with validation. This avoids custom argument parsing.
- **Alternative considered**: Manual argument parsing via `std::env::args()` - rejected because clap provides better error messages and automatic help generation.

**Decision 2: Require exactly 2 positional arguments**
- **Rationale**: Directory comparison inherently requires exactly two inputs. Enforcing this at parse time provides immediate feedback.
- **How**: Set `num_args = 2` on the positional field or use explicit validation in Args struct.

**Decision 3: Keep all optional flags (method, format, output, case_insensitive, verify)**
- **Rationale**: These flags are useful and don't conflict with positional arguments. Remove only the directory-specific flags.

**Decision 4: Update tests to use new syntax**
- **Rationale**: Integration tests in `tests/cli_tests.rs` validate the CLI interface. They must be updated to reflect the new argument order.
- **Implementation**: Replace `--dir-a dir1 --dir-b dir2` with `dir1 dir2` in all test invocations.

## Risks / Trade-offs

**Risk: Breaking change for existing users**
- **Mitigation**: Document clearly in CHANGELOG and README. This is a major version bump candidate. Consider a deprecation warning in an intermediate release if needed.

**Risk: Positional argument order confusion**
- **Mitigation**: Help text clearly documents order: `dir-compare <DIR1> <DIR2>`. Examples in README reinforce this.

**Risk: Integration tests may be fragmented**
- **Mitigation**: Update all tests at once rather than gradual rollout. Run full test suite before merge.

## Migration Plan

1. **Phase 1 (This change)**: Update CLI argument parser; update tests; update documentation
2. **Phase 2 (Release)**: Release as breaking change (e.g., v0.2.0)
3. **Rollback**: If needed, revert commits and re-release prior version

## Open Questions

- Should we add a transitional deprecation warning if the old flags are used? (Deferred to future release if needed)
- Should positional arguments be strictly validated for existing directories before comparison? (Existing validation in main() already handles this)
