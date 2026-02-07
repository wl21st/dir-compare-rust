use std::path::PathBuf;

/// Trait for abstracting file dialog operations to enable mocking in tests
pub trait FileDialogProvider {
    /// Opens a folder picker dialog and returns the selected path
    fn pick_folder(&self) -> Option<PathBuf>;
}

/// Native file dialog implementation using rfd
pub struct NativeFileDialog;

impl FileDialogProvider for NativeFileDialog {
    fn pick_folder(&self) -> Option<PathBuf> {
        rfd::FileDialog::new().pick_folder()
    }
}

/// Mock file dialog for testing
pub struct MockFileDialog {
    return_path: Option<PathBuf>,
}

impl MockFileDialog {
    /// Creates a new mock file dialog that returns the specified path
    pub fn new(return_path: Option<PathBuf>) -> Self {
        Self { return_path }
    }

    /// Sets the path that will be returned by pick_folder
    pub fn set_return_path(&mut self, path: Option<PathBuf>) {
        self.return_path = path;
    }
}

impl FileDialogProvider for MockFileDialog {
    fn pick_folder(&self) -> Option<PathBuf> {
        self.return_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_file_dialog_returns_path() {
        let path = PathBuf::from("/test/path");
        let dialog = MockFileDialog::new(Some(path.clone()));
        assert_eq!(dialog.pick_folder(), Some(path));
    }

    #[test]
    fn test_mock_file_dialog_returns_none() {
        let dialog = MockFileDialog::new(None);
        assert_eq!(dialog.pick_folder(), None);
    }

    #[test]
    fn test_mock_file_dialog_set_return_path() {
        let mut dialog = MockFileDialog::new(None);
        assert_eq!(dialog.pick_folder(), None);

        let path = PathBuf::from("/new/path");
        dialog.set_return_path(Some(path.clone()));
        assert_eq!(dialog.pick_folder(), Some(path));
    }

    #[test]
    fn test_mock_file_dialog_change_path() {
        let path1 = PathBuf::from("/first/path");
        let mut dialog = MockFileDialog::new(Some(path1));
        assert_eq!(dialog.pick_folder(), Some(PathBuf::from("/first/path")));

        let path2 = PathBuf::from("/second/path");
        dialog.set_return_path(Some(path2.clone()));
        assert_eq!(dialog.pick_folder(), Some(path2));
    }
}
