pub mod dialog;
pub mod theme;
pub mod tree_view;

// Re-export commonly used items for tests
pub use dialog::{FileDialogProvider, MockFileDialog, NativeFileDialog};
pub use theme::{Theme, load_theme, save_theme};
pub use tree_view::{FileTreeNode, render_tree};

/// Validates that a path is a non-empty string pointing to an existing directory
pub fn validate_path(path: &str) -> bool {
    if path.trim().is_empty() {
        return false;
    }
    std::path::Path::new(path).is_dir()
}
