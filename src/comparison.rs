use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Represents the type of a directory entry.
///
/// # Examples
///
/// ```
/// use dir_compare::EntryKind;
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
/// use dir_compare::{Entry, EntryKind};
///
/// let entry = Entry {
///     path: PathBuf::from("documents/report.txt"),
///     kind: EntryKind::File,
///     size: Some(1024),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Entry {
    /// The relative path of the entry from the root directory
    pub path: PathBuf,
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
/// use dir_compare::{Entry, EntryKind, ComparisonStrategy, FilenameOnlyStrategy};
///
/// let strategy = FilenameOnlyStrategy::new(false);
/// let entry1 = Entry {
///     path: PathBuf::from("file.txt"),
///     kind: EntryKind::File,
///     size: Some(100),
/// };
/// let entry2 = Entry {
///     path: PathBuf::from("file.txt"),
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
/// use dir_compare::{FilenameOnlyStrategy, ComparisonStrategy};
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
/// use dir_compare::{FilenameSizeStrategy, ComparisonStrategy};
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
/// use dir_compare::{FastHashStrategy, ComparisonStrategy};
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
                let hash_a = compute_file_hash(&a.path);
                let hash_b = compute_file_hash(&b.path);
                hash_a == hash_b
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
                    Err(_) => break,
                };
                hasher.write(&buffer[..bytes_read]);
            }
            format!("{:016x}", hasher.finish())
        }
        Err(_) => String::new(),
    }
}

pub fn traverse_directory(dir: &std::path::Path) -> std::io::Result<Vec<Entry>> {
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
                entries.push(Entry { path, kind, size });
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
pub enum ComparisonStrategyType {
    /// Compare by filename only
    Filename,
    /// Compare by filename and file size
    FilenameSize,
    /// Compare by filename and content hash
    FastHash,
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
/// use dir_compare::compare_directories;
/// use dir_compare::FilenameOnlyStrategy;
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
/// use dir_compare::{compare_directories, FilenameOnlyStrategy};
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
