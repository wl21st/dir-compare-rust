## ADDED Requirements

### Requirement: Sampled-hash comparison strategy SHALL compare by filename and sampled content hash
The strategy SHALL compute a sampled content hash and use it alongside filename equality for file comparisons.
#### Scenario: Match by filename and sampled hash
Given two file entries with the same relative filename
When the sampled-hash strategy is selected
Then the comparison SHALL compute a sampled hash for each file and treat them as matching when both sampled hashes are equal.

#### Scenario: Deterministic sampling offsets for equal-sized files
Given two files with the same size and sufficient length for sampling
When sampled hashes are computed
Then the strategy SHALL read the same deterministic offsets for both files based on file size.

#### Scenario: Sample layout uses 7 blocks of 431 bytes
Given a file large enough for sampling
When the sampled-hash strategy computes samples
Then it SHALL read 7 blocks of 431 bytes consisting of the first block, the last block, and five evenly distributed interior blocks.

#### Scenario: Small files are fully read
Given a file with size smaller than the total sampling span (7 * 431 bytes)
When the sampled-hash strategy computes the hash
Then it SHALL read the entire file content to produce the hash.

#### Scenario: Sample hashing uses SHA-256
Given a file sampled into a concatenated buffer
When the sampled-hash strategy computes the hash
Then it SHALL compute SHA-256 over the sampled buffer and represent the hash as a lowercase hex string.

### Requirement: Optional full-hash verification on sampled match SHALL be supported
The strategy SHALL support a mode that verifies sampled matches with full-file hashing.
#### Scenario: Sampling-only mode skips full hash
Given sampling-only mode is enabled
When sampled hashes match
Then the strategy SHALL treat the files as matching without computing a full-file hash.

#### Scenario: Verify-on-match computes full hash
Given verify-on-match mode is enabled
When sampled hashes match
Then the strategy SHALL compute a full-file hash for each file and treat the files as matching only if the full hashes are equal.
