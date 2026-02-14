# Proposal: Add sampled-hash comparison strategy

## Summary
Introduce a new comparison strategy that computes a sampled SHA-256 hash for large files to reduce IO. The strategy will read a fixed number of samples (default 7) of size 431 bytes: the first 431 bytes, the last 431 bytes, and five evenly distributed blocks between them. For files smaller than the total sampled span, the strategy reads the full file. The strategy can operate in sampling-only mode or optionally verify matches with a full hash.

## Goals
- Reduce IO for huge file comparisons while preserving deterministic results.
- Provide a new strategy distinct from the existing full-file fast hash.
- Allow callers/CLI to choose between sampling-only and verify-with-full-hash modes.

## Non-Goals
- Cryptographic integrity guarantees; collisions are acceptable in sampling-only mode.
- Changes to output formats or result structure.
- Modifying existing strategies.

## User Impact
- New comparison method option for sampled hashing.
- Option to enable/disable full-hash verification when sampled hashes match.

## Open Questions
- Naming of the new method for CLI (e.g., `sampled-hash`, `sampled`), and the flag name for full-hash verification.
- Exact defaults for the verification mode (assumed default: sampling-only, with optional full-hash verify).

## Acceptance Criteria
- A new sampled-hash strategy exists and is selectable via the comparison method.
- Sample selection is deterministic and based on file size: first/last 431 bytes plus five evenly distributed blocks.
- Files smaller than the sampling span are fully read.
- Sampling uses SHA-256 on the concatenated sampled buffer.
- Optional full-hash verification is available when sampled hashes match.
