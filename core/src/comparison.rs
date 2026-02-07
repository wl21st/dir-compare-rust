use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Represents the type of a directory entry.
///
/// # Examples
///
/// ```
/// use dir_compare_core::EntryKind;
///
/// let file_kind = EntryKind::File;
/// let dir_kind = EntryKind::Directory;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    /// A regular file entry
    File,
    /// A directory entry
    Directory,
}

/// Represents a single entry (file or directory) found during directory traversal.
///
/// Contains the relative path, entry type, and optional file size.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use dir_compare_core::{Entry, EntryKind};
///
/// let entry = Entry {
///     path: PathBuf::from("documents/report.txt"),
///     abs_path: PathBuf::from("/abs/documents/report.txt"),
///     kind: EntryKind::File,
///     size: Some(1024),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Entry {
    /// The relative path of the entry from the root directory
    pub path: PathBuf,
    /// The absolute path of the entry (for internal use)
    pub abs_path: PathBuf,
    /// The type of entry (file or directory)
    pub kind: EntryKind,
    /// The file size in bytes (None for directories)
    pub size: Option<u64>,
}

/// Trait for comparing entries between two directories.
///
/// Implement this trait to provide different comparison strategies
/// based on filename, size, or content hash.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use dir_compare_core::{Entry, EntryKind, ComparisonStrategy, FilenameOnlyStrategy};
///
/// let strategy = FilenameOnlyStrategy::new(false);
/// let entry1 = Entry {
///     path: PathBuf::from("file.txt"),
///     abs_path: PathBuf::from("/abs/file.txt"),
///     kind: EntryKind::File,
///     size: Some(100),
/// };
/// let entry2 = Entry {
///     path: PathBuf::from("file.txt"),
///     abs_path: PathBuf::from("/abs/file.txt"),
///     kind: EntryKind::File,
///     size: Some(200),
/// };
///
/// assert!(strategy.matches(&entry1, &entry2));
/// ```
pub trait ComparisonStrategy {
    /// Determines whether two entries match according to this strategy.
    ///
    /// # Arguments
    ///
    /// * `a` - An entry from directory A
    /// * `b` - An entry from directory B
    ///
    /// # Returns
    ///
    /// `true` if the entries match, `false` otherwise
    fn matches(&self, a: &Entry, b: &Entry) -> bool;
}

/// Comparison strategy that matches files based on filename only.
///
/// This strategy compares entries by their filename (relative path),
/// ignoring file size and content differences.
///
/// # Examples
///
/// ```
/// use dir_compare_core::{FilenameOnlyStrategy, ComparisonStrategy};
///
/// let case_sensitive = FilenameOnlyStrategy::new(false);
/// let case_insensitive = FilenameOnlyStrategy::new(true);
/// ```
pub struct FilenameOnlyStrategy {
    case_insensitive: bool,
}

impl FilenameOnlyStrategy {
    /// Creates a new FilenameOnlyStrategy.
    ///
    /// # Arguments
    ///
    /// * `case_insensitive` - If true, filenames are compared case-insensitively
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }
}

impl ComparisonStrategy for FilenameOnlyStrategy {
    fn matches(&self, a: &Entry, b: &Entry) -> bool {
        let name_a = if self.case_insensitive {
            a.path.to_string_lossy().to_lowercase()
        } else {
            a.path.to_string_lossy().to_string()
        };
        let name_b = if self.case_insensitive {
            b.path.to_string_lossy().to_lowercase()
        } else {
            b.path.to_string_lossy().to_string()
        };

        let path_a_normalized = PathBuf::from(name_a);
        let path_b_normalized = PathBuf::from(name_b);

        path_a_normalized == path_b_normalized
    }
}

/// Comparison strategy that matches files by filename and size.
///
/// Files match if they have the same filename and the same file size.
/// Directories are matched by filename only.
///
/// # Examples
///
/// ```
/// use dir_compare_core::{FilenameSizeStrategy, ComparisonStrategy};
///
/// let strategy = FilenameSizeStrategy::new(false);
/// ```
pub struct FilenameSizeStrategy {
    case_insensitive: bool,
}

impl FilenameSizeStrategy {
    /// Creates a new FilenameSizeStrategy.
    ///
    /// # Arguments
    ///
    /// * `case_insensitive` - If true, filenames are compared case-insensitively
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }
}

impl ComparisonStrategy for FilenameSizeStrategy {
    fn matches(&self, a: &Entry, b: &Entry) -> bool {
        let name_match = {
            let name_a = if self.case_insensitive {
                a.path.to_string_lossy().to_lowercase()
            } else {
                a.path.to_string_lossy().to_string()
            };
            let name_b = if self.case_insensitive {
                b.path.to_string_lossy().to_lowercase()
            } else {
                b.path.to_string_lossy().to_string()
            };
            name_a == name_b
        };

        if !name_match {
            return false;
        }

        match (&a.kind, &b.kind) {
            (EntryKind::Directory, EntryKind::Directory) => true,
            (EntryKind::File, EntryKind::File) => a.size == b.size,
            _ => false,
        }
    }
}

/// Comparison strategy that matches files by filename and content hash.
///
/// Uses the fast FxHash algorithm to compute a hash of file contents.
/// Files match if they have the same filename and identical content.
/// Directories are matched by filename only.
///
/// This strategy is useful for detecting modified files that have the
/// same name but different content.
///
/// # Performance
///
/// The FxHash algorithm is very fast but not cryptographically secure.
/// Suitable for comparing file contents where collision resistance
/// is not critical.
///
/// # Examples
///
/// ```
/// use dir_compare_core::{FastHashStrategy, ComparisonStrategy};
///
/// let strategy = FastHashStrategy::new(false);
/// ```
pub struct FastHashStrategy {
    case_insensitive: bool,
}

impl FastHashStrategy {
    /// Creates a new FastHashStrategy.
    ///
    /// # Arguments
    ///
    /// * `case_insensitive` - If true, filenames are compared case-insensitively
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }
}

impl ComparisonStrategy for FastHashStrategy {
    fn matches(&self, a: &Entry, b: &Entry) -> bool {
        let name_match = {
            let name_a = if self.case_insensitive {
                a.path.to_string_lossy().to_lowercase()
            } else {
                a.path.to_string_lossy().to_string()
            };
            let name_b = if self.case_insensitive {
                b.path.to_string_lossy().to_lowercase()
            } else {
                b.path.to_string_lossy().to_string()
            };
            name_a == name_b
        };

        if !name_match {
            return false;
        }

        match (&a.kind, &b.kind) {
            (EntryKind::Directory, EntryKind::Directory) => true,
            (EntryKind::File, EntryKind::File) => {
                let hash_a = compute_file_hash(&a.abs_path);
                let hash_b = compute_file_hash(&b.abs_path);
                hash_a == hash_b
            }
            _ => false,
        }
    }
}

/// Comparison strategy that matches files by filename and sampled content hash.
///
/// Uses SHA-256 to hash samples of the file.
/// Files match if they have the same filename and identical sampled hash.
/// Directories are matched by filename only.
///
/// # Examples
///
/// ```
/// use dir_compare_core::{SampledHashStrategy, ComparisonStrategy};
///
/// let strategy = SampledHashStrategy::new(false, false);
/// ```
pub struct SampledHashStrategy {
    case_insensitive: bool,
    verify_on_match: bool,
}

impl SampledHashStrategy {
    /// Creates a new SampledHashStrategy.
    ///
    /// # Arguments
    ///
    /// * `case_insensitive` - If true, filenames are compared case-insensitively
    /// * `verify_on_match` - If true, performs a full hash check if sampled hashes match
    pub fn new(case_insensitive: bool, verify_on_match: bool) -> Self {
        Self {
            case_insensitive,
            verify_on_match,
        }
    }
}

/// Sample size in bytes for each sampled block.
///
/// 431 is chosen as a prime number to avoid alignment with common filesystem
/// block sizes (typically powers of 2 like 512, 4096, etc.), ensuring
/// more randomized sampling across different file layouts.
const SAMPLE_SIZE: u64 = 431;

/// Number of samples to read from each file.
///
/// 7 samples provides good coverage across file content while maintaining
/// constant-time performance (~3KB total read per file). This balances
/// detection accuracy with I/O efficiency.
const SAMPLE_COUNT: u64 = 7;

/// Compute sampled hash for a file
///
/// This function is exposed for testing purposes. It computes a SHA-256 hash
/// of the file with a size prefix (8 bytes, big-endian) followed by either
/// the entire file content (for small files) or sampled content (for large files).
#[doc(hidden)]
pub fn compute_sampled_hash(path: &Path) -> String {
    _compute_sampled_hash_internal(path)
}

fn _compute_sampled_hash_internal(path: &Path) -> String {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Warning: Could not open {}: {}", path.display(), e);
            // Return a unique error marker that won't match another error
            return format!("ERROR:{}", path.display());
        }
    };

    let size = match file.metadata() {
        Ok(m) => m.len(),
        Err(e) => {
            eprintln!(
                "Warning: Could not get metadata for {}: {}",
                path.display(),
                e
            );
            return format!("ERROR:{}", path.display());
        }
    };

    // File size is u64 which is exactly 8 bytes, so no overflow check needed
    // Add file size (8 bytes, big-endian) to the beginning of the hash
    let mut hasher = Sha256::new();
    hasher.update(size.to_be_bytes());

    if size < SAMPLE_COUNT * SAMPLE_SIZE {
        // File is smaller than total sample size, read entire file
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            hasher.update(&buffer);
        }
    } else {
        // File is large enough for sampling strategy
        // Allocate buffer once before loop for better performance
        let mut buffer = vec![0u8; SAMPLE_SIZE as usize];

        // Calculate step size for interior samples
        // Interior region: exclude first and last SAMPLE_SIZE bytes
        let interior_len = size - 2 * SAMPLE_SIZE;
        let step = interior_len / (SAMPLE_COUNT - 1);

        // Invariant: With SAMPLE_COUNT=7 and SAMPLE_SIZE=431, we need at least
        // 7*431 = 3017 bytes for full sampling. The condition above ensures this.

        // Sample 1: first 431 bytes
        if file.seek(SeekFrom::Start(0)).is_ok() && file.read_exact(&mut buffer).is_ok() {
            hasher.update(&buffer);
        }

        // Samples 2-6: interior samples evenly distributed
        for i in 1..=5 {
            // Calculate offset for interior sample
            // The min() clamps to prevent overlap with final sample
            let offset = std::cmp::min(SAMPLE_SIZE + i * step, size - 2 * SAMPLE_SIZE);
            if file.seek(SeekFrom::Start(offset)).is_ok() && file.read_exact(&mut buffer).is_ok() {
                hasher.update(&buffer);
            }
        }

        // Sample 7: last 431 bytes
        if file.seek(SeekFrom::Start(size - SAMPLE_SIZE)).is_ok()
            && file.read_exact(&mut buffer).is_ok()
        {
            hasher.update(&buffer);
        }
    }

    format!("{:x}", hasher.finalize())
}

impl ComparisonStrategy for SampledHashStrategy {
    fn matches(&self, a: &Entry, b: &Entry) -> bool {
        let name_match = {
            let name_a = if self.case_insensitive {
                a.path.to_string_lossy().to_lowercase()
            } else {
                a.path.to_string_lossy().to_string()
            };
            let name_b = if self.case_insensitive {
                b.path.to_string_lossy().to_lowercase()
            } else {
                b.path.to_string_lossy().to_string()
            };
            name_a == name_b
        };

        if !name_match {
            return false;
        }

        match (&a.kind, &b.kind) {
            (EntryKind::Directory, EntryKind::Directory) => true,
            (EntryKind::File, EntryKind::File) => {
                let hash_a = _compute_sampled_hash_internal(&a.abs_path);
                let hash_b = _compute_sampled_hash_internal(&b.abs_path);

                if hash_a != hash_b {
                    false
                } else if self.verify_on_match {
                    let full_a = compute_file_hash_sha256(&a.abs_path);
                    let full_b = compute_file_hash_sha256(&b.abs_path);
                    full_a == full_b
                } else {
                    true
                }
            }
            _ => false,
        }
    }
}

fn compute_file_hash(path: &std::path::Path) -> String {
    use std::fs::File;
    use std::hash::Hasher;
    use std::io::{BufReader, Read};

    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut hasher = fxhash::FxHasher::default();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Warning: Error reading {}: {}", path.display(), e);
                        break;
                    }
                };
                hasher.write(&buffer[..bytes_read]);
            }
            format!("{:016x}", hasher.finish())
        }
        Err(e) => {
            eprintln!("Warning: Could not open {}: {}", path.display(), e);
            // Return a unique error marker that won't match another error
            format!("ERROR:{}", path.display())
        }
    }
}

fn compute_file_hash_sha256(path: &std::path::Path) -> String {
    use std::fs::File;
    use std::io::{BufReader, Read};

    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Warning: Error reading {}: {}", path.display(), e);
                        break;
                    }
                };
                hasher.update(&buffer[..bytes_read]);
            }
            format!("{:x}", hasher.finalize())
        }
        Err(e) => {
            eprintln!("Warning: Could not open {}: {}", path.display(), e);
            // Return a unique error marker that won't match another error
            format!("ERROR:{}", path.display())
        }
    }
}

pub fn traverse_directory(dir: &std::path::Path) -> std::io::Result<Vec<Entry>> {
    // Canonicalize the directory path to ensure we work with absolute paths
    let dir = std::fs::canonicalize(dir)?;
    let mut entries = Vec::new();

    for entry in walkdir::WalkDir::new(dir)
        .follow_links(false)
        .sort_by_file_name()
        .min_depth(1)
        .into_iter()
    {
        match entry {
            Ok(entry) => {
                let path = entry.path().to_path_buf();
                let abs_path = path.clone();
                let kind = if entry.file_type().is_dir() {
                    EntryKind::Directory
                } else {
                    EntryKind::File
                };
                let size = if entry.file_type().is_file() {
                    Some(entry.metadata()?.len())
                } else {
                    None
                };
                entries.push(Entry {
                    path,
                    abs_path,
                    kind,
                    size,
                });
            }
            Err(ref e) => {
                if let Some(path) = e.path() {
                    eprintln!("Warning: Could not access {}: {}", path.display(), e);
                } else {
                    eprintln!("Warning: Could not access entry: {}", e);
                }
            }
        }
    }

    Ok(entries)
}

/// Enumeration of available comparison strategy types.
///
/// Used by CLI and programmatic interfaces to select the comparison method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonStrategyType {
    /// Compare by filename only
    Filename,
    /// Compare by filename and file size
    FilenameSize,
    /// Compare by filename and content hash
    FastHash,
    /// Compare by filename and sampled content hash
    SampledHash,
}

/// The result of comparing two directories.
///
/// Contains three categories of entries:
/// - Entries only in directory A
/// - Entries only in directory B
/// - Entries present in both directories (matching according to the strategy)
///
/// # Examples
///
/// ```
/// use dir_compare_core::compare_directories;
/// use dir_compare_core::FilenameOnlyStrategy;
///
/// let strategy = FilenameOnlyStrategy::new(false);
/// let result = compare_directories(
///     &std::path::Path::new("dir_a"),
///     &std::path::Path::new("dir_b"),
///     &strategy
/// );
///
/// match result {
///     Ok(res) => {
///         println!("A-only: {} entries", res.a_only.len());
///         println!("B-only: {} entries", res.b_only.len());
///         println!("Both: {} entries", res.both.len());
///     }
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub struct ComparisonResult {
    /// Entries found only in directory A
    pub a_only: Vec<Entry>,
    /// Entries found only in directory B
    pub b_only: Vec<Entry>,
    /// Entries found in both directories (tuple of A-entry, B-entry)
    pub both: Vec<(Entry, Entry)>,
}

/// Compares two directories using the specified comparison strategy.
///
/// This function recursively traverses both directories and categorizes
/// entries into three groups: A-only, B-only, and both.
///
/// # Arguments
///
/// * `dir_a` - Path to the first directory
/// * `dir_b` - Path to the second directory
/// * `strategy` - The comparison strategy to use
///
/// # Returns
///
/// A `ComparisonResult` containing categorized entries, or an I/O error
/// if directory traversal fails.
///
/// # Examples
///
/// ```
/// use dir_compare_core::{compare_directories, FilenameOnlyStrategy};
///
/// let strategy = FilenameOnlyStrategy::new(false);
/// let result = compare_directories(
///     &std::path::Path::new("test_data/original"),
///     &std::path::Path::new("test_data/modified"),
///     &strategy
/// );
/// ```
pub fn compare_directories(
    dir_a: &Path,
    dir_b: &Path,
    strategy: &dyn ComparisonStrategy,
) -> std::io::Result<ComparisonResult> {
    let entries_a = traverse_directory(dir_a)?;
    let entries_b = traverse_directory(dir_b)?;

    let mut a_only: Vec<Entry> = Vec::new();
    let mut b_only: Vec<Entry> = Vec::new();
    let mut both: Vec<(Entry, Entry)> = Vec::new();

    let dir_a_canonical = std::fs::canonicalize(dir_a)?;
    let dir_b_canonical = std::fs::canonicalize(dir_b)?;

    let map_a: HashMap<PathBuf, Entry> = entries_a
        .into_iter()
        .map(|e| {
            let rel_path = if let Ok(stripped) = e.path.strip_prefix(&dir_a_canonical) {
                stripped.to_path_buf()
            } else {
                e.path
                    .file_name()
                    .map(|n| PathBuf::from(n))
                    .unwrap_or_else(|| e.path.clone())
            };
            let mut entry = e;
            entry.path = rel_path.clone();
            (rel_path, entry)
        })
        .collect();

    let map_b: HashMap<PathBuf, Entry> = entries_b
        .into_iter()
        .map(|e| {
            let rel_path = if let Ok(stripped) = e.path.strip_prefix(&dir_b_canonical) {
                stripped.to_path_buf()
            } else {
                e.path
                    .file_name()
                    .map(|n| PathBuf::from(n))
                    .unwrap_or_else(|| e.path.clone())
            };
            let mut entry = e;
            entry.path = rel_path.clone();
            (rel_path, entry)
        })
        .collect();

    let keys_a: HashSet<PathBuf> = map_a.keys().cloned().collect();
    let keys_b: HashSet<PathBuf> = map_b.keys().cloned().collect();

    for key in keys_a.difference(&keys_b) {
        if let Some(entry) = map_a.get(key) {
            a_only.push(entry.clone());
        }
    }

    for key in keys_b.difference(&keys_a) {
        if let Some(entry) = map_b.get(key) {
            b_only.push(entry.clone());
        }
    }

    for key in keys_a.intersection(&keys_b) {
        let entry_a = map_a.get(key).cloned();
        let entry_b = map_b.get(key).cloned();
        if let (Some(a), Some(b)) = (entry_a, entry_b) {
            if strategy.matches(&a, &b) {
                both.push((a, b));
            } else {
                a_only.push(a);
                b_only.push(b);
            }
        }
    }

    a_only.sort_by(|a, b| a.path.cmp(&b.path));
    b_only.sort_by(|a, b| a.path.cmp(&b.path));
    both.sort_by(|(a, _), (b, _)| a.path.cmp(&b.path));

    Ok(ComparisonResult {
        a_only,
        b_only,
        both,
    })
}
