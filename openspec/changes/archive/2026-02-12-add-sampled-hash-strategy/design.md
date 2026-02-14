# Design: Sampled-hash comparison strategy

## Overview
The sampled-hash strategy reduces IO by reading a fixed number of byte ranges from each file, concatenating those samples into a buffer, and hashing the buffer with SHA-256. This yields a deterministic, low-IO signature that is safe to treat as a probabilistic filter: if sampled hashes differ, files differ; if sampled hashes match, files may still differ.

The strategy supports two modes:
- Sampling-only: treat sampled hash equality as a match (fastest, probabilistic).
- Verify-on-match: if sampled hashes match, compute a full-file hash and require equality (slower, deterministic).

## Sampling layout
- Sample size: 431 bytes (prime-sized blocks).
- Sample count: 7 total.
- Samples are chosen deterministically by file size to ensure stable offsets for equal-sized files.
- For sufficiently large files:
  - Sample 1: first 431 bytes (offset 0).
  - Sample 7: last 431 bytes (offset file_size - 431).
  - Samples 2-6: five evenly spaced blocks across the interior range.
- For small files (file_size < sample_count * sample_size), read the entire file and hash it.

## Offset calculation
Let:
- `S` = sample size (431 bytes)
- `N` = sample count (7)
- `size` = file size in bytes

If `size < N * S`, read full file.

Otherwise:
- `first_offset = 0`
- `last_offset = size - S`
- Interior range is `[S, size - S]` with length `interior = size - 2 * S`.
- Place 5 interior samples at deterministic offsets so the blocks are evenly distributed across `interior`.
  - Use integer division to compute step = `interior / (N - 1)` (for 6 gaps between 7 sample anchors), and clamp each interior offset to stay within `[S, size - 2*S]`.

The exact arithmetic will be specified in the spec and implemented consistently for repeatable hashes.

## Hashing
- Concatenate sample bytes in order of offsets into a buffer.
- Compute SHA-256 of the buffer.
- Represent hash as lowercase hex string.

## Tradeoffs
- Sampling-only mode favors performance, with a non-zero collision risk.
- Verify-on-match removes collision risk at the expense of reading entire files for equal sampled hashes.

## Integration
- Add a new `SampledHashStrategy` implementing the existing `ComparisonStrategy` interface.
- Expose a new method token in CLI method selection.
- Add an option/flag to toggle verify-on-match behavior.
