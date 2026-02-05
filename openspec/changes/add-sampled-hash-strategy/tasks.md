1. Review existing comparison strategies and CLI method selection to confirm integration points.
2. Define the sampled-hash strategy parameters (sample size 431, count 7, offset calculation) and hashing mode toggle in code.
3. Implement sampled block reads and SHA-256 hash computation for sampled buffer.
4. Add verification mode to compute full hash when sampled hashes match.
5. Wire the new method into CLI selection and update user-facing method list/help text.
6. Add unit tests for sampling offsets, small-file full read, and verify-on-match behavior.
7. Add documentation updates for comparison methods and performance guidance.
