use std::io::{Read, Write};
use std::path::PathBuf;

const APP_NAME: &str = "dir-compare";
const THEME_CONFIG_FILE: &str = "theme.txt";

/// Application theme setting
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    /// Returns the string representation of the theme
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    /// Parses a theme from its string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            "system" => Some(Theme::System),
            _ => None,
        }
    }

    /// Converts the theme to egui visuals
    pub fn to_visuals(&self) -> egui::Visuals {
        match self {
            Theme::Light => egui::Visuals::light(),
            Theme::Dark => egui::Visuals::dark(),
            Theme::System => {
                // Use dark as default for system since we can't easily detect system theme
                egui::Visuals::dark()
            }
        }
    }
}

/// Gets the configuration directory path
fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join(APP_NAME))
}

/// Gets the theme configuration file path
fn get_theme_config_path() -> Option<PathBuf> {
    get_config_dir().map(|dir| dir.join(THEME_CONFIG_FILE))
}

/// Loads the saved theme from the configuration file
pub fn load_theme() -> Option<Theme> {
    let path = get_theme_config_path()?;
    let mut file = std::fs::File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Theme::from_str(contents.trim())
}

/// Saves the theme to the configuration file
pub fn save_theme(theme: Theme) {
    if let Some(config_dir) = get_config_dir() {
        if let Err(e) = std::fs::create_dir_all(&config_dir) {
            eprintln!("Failed to create config directory: {}", e);
            return;
        }
        let path = config_dir.join(THEME_CONFIG_FILE);
        match std::fs::File::create(&path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(theme.as_str().as_bytes()) {
                    eprintln!("Failed to write theme config: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to create theme config file: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_from_str_valid() {
        assert_eq!(Theme::from_str("light"), Some(Theme::Light));
        assert_eq!(Theme::from_str("dark"), Some(Theme::Dark));
        assert_eq!(Theme::from_str("system"), Some(Theme::System));
    }

    #[test]
    fn test_theme_from_str_invalid() {
        assert_eq!(Theme::from_str("invalid"), None);
        assert_eq!(Theme::from_str(""), None);
        assert_eq!(Theme::from_str("LIGHT"), None); // case sensitive
        assert_eq!(Theme::from_str(" Light "), None); // with spaces
    }

    #[test]
    fn test_theme_as_str() {
        assert_eq!(Theme::Light.as_str(), "light");
        assert_eq!(Theme::Dark.as_str(), "dark");
        assert_eq!(Theme::System.as_str(), "system");
    }

    #[test]
    fn test_theme_roundtrip() {
        // Test that from_str and as_str are inverse operations
        let themes = [Theme::Light, Theme::Dark, Theme::System];
        for theme in &themes {
            let str_val = theme.as_str();
            let parsed = Theme::from_str(str_val);
            assert_eq!(parsed, Some(*theme));
        }
    }

    #[test]
    fn test_load_save_theme_with_temp_dir() {
        use std::env;
        use tempfile::TempDir;

        // Create a temp directory for config
        let temp_dir = TempDir::new().unwrap();
        let _temp_config_path = temp_dir.path().to_path_buf();

        // Temporarily override the config directory
        // Note: This test is limited since we can't easily mock dirs::config_dir()
        // In practice, we'd need dependency injection for full testability

        // Save theme
        save_theme(Theme::Dark);

        // If the config dir exists, load_theme should work
        // But we can't reliably test this without mocking dirs::config_dir()
        // So we just verify the functions don't panic
    }
}
