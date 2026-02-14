use dir_compare_gui::{
    dialog::{FileDialogProvider, NativeFileDialog},
    theme::{Theme, load_theme, save_theme},
    tree_view, validate_path,
};

use dir_compare_core::{
    ComparisonResult, ComparisonStrategy, ComparisonStrategyType, Entry, FastHashStrategy,
    FilenameOnlyStrategy, FilenameSizeStrategy, SampledHashStrategy, compare_directories,
};
use eframe::egui;
use std::sync::mpsc::{Receiver, channel};
use tree_view::FileTreeNode;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("dir-compare"),
        ..Default::default()
    };

    // Load saved theme or use System default
    let saved_theme = load_theme().unwrap_or(Theme::System);

    eframe::run_native(
        "dir-compare",
        options,
        Box::new(move |cc| {
            // Apply saved theme on startup
            cc.egui_ctx.set_visuals(saved_theme.to_visuals());
            Box::new(DirCompareApp::new(saved_theme))
        }),
    )
}

/// Application state that can be inspected and modified by tests
pub struct AppState {
    pub dir_a_path: String,
    pub dir_b_path: String,
    pub ignore_file_path: Option<String>,
    pub comparison_method: ComparisonStrategyType,
    pub results: Option<ComparisonResult>,
    pub tree_cache: Option<TreeCache>,
    pub theme: Theme,
    pub is_comparing: bool,
    pub comparison_receiver: Option<Receiver<Result<ComparisonResult, String>>>,
    pub error_message: Option<String>,
}

/// Cached tree view data for displaying comparison results
pub struct TreeCache {
    pub a_only: Vec<FileTreeNode>,
    pub b_only: Vec<FileTreeNode>,
    pub both: Vec<FileTreeNode>,
}

/// Main application struct
pub struct DirCompareApp {
    pub state: AppState,
}

impl DirCompareApp {
    fn new(initial_theme: Theme) -> Self {
        Self {
            state: AppState {
                dir_a_path: String::new(),
                dir_b_path: String::new(),
                ignore_file_path: None,
                comparison_method: ComparisonStrategyType::FastHash,
                results: None,
                tree_cache: None,
                theme: initial_theme,
                is_comparing: false,
                comparison_receiver: None,
                error_message: None,
            },
        }
    }
}

impl eframe::App for DirCompareApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for results
        if let Some(rx) = &self.state.comparison_receiver {
            if let Ok(result) = rx.try_recv() {
                self.state.is_comparing = false;
                self.state.comparison_receiver = None;
                match result {
                    Ok(res) => {
                        let a_only = FileTreeNode::from_entries(&res.a_only);
                        let b_only = FileTreeNode::from_entries(&res.b_only);
                        let both_entries: Vec<Entry> =
                            res.both.iter().map(|(a, _)| a.clone()).collect();
                        let both = FileTreeNode::from_entries(&both_entries);

                        self.state.results = Some(res);
                        self.state.tree_cache = Some(TreeCache {
                            a_only,
                            b_only,
                            both,
                        });
                    }
                    Err(e) => self.state.error_message = Some(e),
                }
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.menu_button("Theme", |ui| {
                        let old_theme = self.state.theme;
                        if ui
                            .radio_value(&mut self.state.theme, Theme::Light, "Light")
                            .clicked()
                        {
                            ctx.set_visuals(egui::Visuals::light());
                            save_theme(Theme::Light);
                        }
                        if ui
                            .radio_value(&mut self.state.theme, Theme::Dark, "Dark")
                            .clicked()
                        {
                            ctx.set_visuals(egui::Visuals::dark());
                            save_theme(Theme::Dark);
                        }
                        if ui
                            .radio_value(&mut self.state.theme, Theme::System, "System")
                            .clicked()
                        {
                            // System default - use dark as fallback since we can't easily detect
                            ctx.set_visuals(egui::Visuals::dark());
                            save_theme(Theme::System);
                        }
                        if old_theme != self.state.theme {
                            ui.close_menu();
                        }
                    });
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            if let Some(results) = &self.state.results {
                ui.horizontal(|ui| {
                    ui.label(format!("A Only: {}", results.a_only.len()));
                    ui.separator();
                    ui.label(format!("B Only: {}", results.b_only.len()));
                    ui.separator();
                    ui.label(format!("Both: {}", results.both.len()));
                    ui.separator();
                    ui.label(format!(
                        "Total: {}",
                        results.a_only.len() + results.b_only.len() + results.both.len()
                    ));
                });
            } else {
                ui.label("Ready");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("dir-compare");

            ui.add_space(20.0);

            // Directory A
            ui.horizontal(|ui| {
                ui.label("Directory A:");
                ui.text_edit_singleline(&mut self.state.dir_a_path);

                if ui.button("Browse...").clicked() {
                    let dialog = NativeFileDialog;
                    if let Some(path) = dialog.pick_folder() {
                        self.state.dir_a_path = path.display().to_string();
                    }
                }

                if !self.state.dir_a_path.is_empty() {
                    if validate_path(&self.state.dir_a_path) {
                        ui.label("✅").on_hover_text("Valid directory");
                    } else {
                        ui.label("❌").on_hover_text("Invalid directory");
                    }
                }
            });

            ui.add_space(10.0);

            // Directory B
            ui.horizontal(|ui| {
                ui.label("Directory B:");
                ui.text_edit_singleline(&mut self.state.dir_b_path);

                if ui.button("Browse...").clicked() {
                    let dialog = NativeFileDialog;
                    if let Some(path) = dialog.pick_folder() {
                        self.state.dir_b_path = path.display().to_string();
                    }
                }

                if !self.state.dir_b_path.is_empty() {
                    if validate_path(&self.state.dir_b_path) {
                        ui.label("✅").on_hover_text("Valid directory");
                    } else {
                        ui.label("❌").on_hover_text("Invalid directory");
                    }
                }
            });

            ui.add_space(10.0);

            // Comparison Method
            ui.horizontal(|ui| {
                ui.label("Comparison Method:");
                egui::ComboBox::from_label("")
                    .selected_text(match self.state.comparison_method {
                        ComparisonStrategyType::Filename => "Filename",
                        ComparisonStrategyType::FilenameSize => "Filename & Size",
                        ComparisonStrategyType::FastHash => "Content Hash",
                        ComparisonStrategyType::SampledHash => "Sampled Hash",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.state.comparison_method,
                            ComparisonStrategyType::Filename,
                            "Filename",
                        );
                        ui.selectable_value(
                            &mut self.state.comparison_method,
                            ComparisonStrategyType::FilenameSize,
                            "Filename & Size",
                        );
                        ui.selectable_value(
                            &mut self.state.comparison_method,
                            ComparisonStrategyType::FastHash,
                            "Content Hash",
                        );
                        ui.selectable_value(
                            &mut self.state.comparison_method,
                            ComparisonStrategyType::SampledHash,
                            "Sampled Hash",
                        );
                    });
            });

            // Ignore File
            ui.horizontal(|ui| {
                ui.label("Ignore File:");
                let mut ignore_path_display = self
                    .state
                    .ignore_file_path
                    .as_deref()
                    .unwrap_or("")
                    .to_string();
                ui.text_edit_singleline(&mut ignore_path_display);
                self.state.ignore_file_path = if ignore_path_display.is_empty() {
                    None
                } else {
                    Some(ignore_path_display)
                };

                if ui.button("Browse...").clicked() {
                    let dialog = NativeFileDialog;
                    if let Some(path) = dialog.pick_file() {
                        self.state.ignore_file_path = Some(path.display().to_string());
                    }
                }
            });

            ui.add_space(20.0);

            // Compare Button
            let can_compare = validate_path(&self.state.dir_a_path)
                && validate_path(&self.state.dir_b_path)
                && !self.state.is_comparing;

            if self.state.is_comparing {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Comparing...");
                });
            } else {
                if ui
                    .add_enabled(can_compare, egui::Button::new("Compare"))
                    .clicked()
                {
                    self.state.is_comparing = true;
                    self.state.results = None;
                    self.state.tree_cache = None;
                    self.state.error_message = None;
                    let (tx, rx) = channel();
                    self.state.comparison_receiver = Some(rx);

                    let dir_a = self.state.dir_a_path.clone();
                    let dir_b = self.state.dir_b_path.clone();
                    let method = self.state.comparison_method;
                    let ignore_file_path = self.state.ignore_file_path.clone();

                    std::thread::spawn(move || {
                        let strategy: Box<dyn ComparisonStrategy> = match method {
                            ComparisonStrategyType::Filename => {
                                Box::new(FilenameOnlyStrategy::new(false))
                            }
                            ComparisonStrategyType::FilenameSize => {
                                Box::new(FilenameSizeStrategy::new(false))
                            }
                            ComparisonStrategyType::FastHash => {
                                Box::new(FastHashStrategy::new(false))
                            }
                            ComparisonStrategyType::SampledHash => {
                                Box::new(SampledHashStrategy::new(false, true))
                            }
                        };

                        let result = compare_directories(
                            std::path::Path::new(&dir_a),
                            std::path::Path::new(&dir_b),
                            strategy.as_ref(),
                            ignore_file_path.as_deref().map(std::path::Path::new),
                        );

                        match result {
                            Ok(res) => tx.send(Ok(res)).ok(),
                            Err(e) => tx.send(Err(e.to_string())).ok(),
                        };
                    });
                }
            }

            // Error Message
            if let Some(err) = &self.state.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
            }

            // Results Tree
            if let Some(cache) = &self.state.tree_cache {
                ui.separator();
                ui.label("Results:");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.collapsing("Only in A", |ui| {
                        tree_view::render_tree(
                            ui,
                            &cache.a_only,
                            egui::Color32::from_rgb(255, 100, 100),
                        );
                    });
                    ui.collapsing("Only in B", |ui| {
                        tree_view::render_tree(
                            ui,
                            &cache.b_only,
                            egui::Color32::from_rgb(100, 255, 100),
                        );
                    });
                    ui.collapsing("In Both", |ui| {
                        tree_view::render_tree(
                            ui,
                            &cache.both,
                            egui::Color32::from_rgb(100, 200, 255),
                        );
                    });
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_path_with_valid_directory() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_str().unwrap();
        assert!(validate_path(path));
    }

    #[test]
    fn test_validate_path_with_nonexistent_directory() {
        assert!(!validate_path("/nonexistent/path/that/does/not/exist"));
    }

    #[test]
    fn test_validate_path_with_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();
        assert!(!validate_path(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_validate_path_with_empty_string() {
        assert!(!validate_path(""));
    }

    #[test]
    fn test_validate_path_with_whitespace_only() {
        assert!(!validate_path("   "));
        assert!(!validate_path("\t"));
        assert!(!validate_path("\n"));
        assert!(!validate_path("  \t\n  "));
    }

    #[test]
    fn test_validate_path_with_relative_path() {
        // Current directory should exist
        assert!(validate_path("."));
        // Parent directory should exist
        assert!(validate_path(".."));
    }
}
