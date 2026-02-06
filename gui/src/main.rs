mod tree_view;

use dir_compare_core::{
    compare_directories, ComparisonResult, ComparisonStrategy, ComparisonStrategyType, Entry,
    FastHashStrategy, FilenameOnlyStrategy, FilenameSizeStrategy, SampledHashStrategy,
};
use eframe::egui;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use tree_view::FileTreeNode;

const APP_NAME: &str = "dir-compare";
const THEME_CONFIG_FILE: &str = "theme.txt";

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join(APP_NAME))
}

fn get_theme_config_path() -> Option<PathBuf> {
    get_config_dir().map(|dir| dir.join(THEME_CONFIG_FILE))
}

fn load_theme() -> Option<Theme> {
    let path = get_theme_config_path()?;
    let mut file = std::fs::File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Theme::from_str(contents.trim())
}

fn save_theme(theme: Theme) {
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

struct AppState {
    dir_a_path: String,
    dir_b_path: String,
    comparison_method: ComparisonStrategyType,
    results: Option<ComparisonResult>,
    tree_cache: Option<TreeCache>,
    theme: Theme,
    is_comparing: bool,
    comparison_receiver: Option<Receiver<Result<ComparisonResult, String>>>,
    error_message: Option<String>,
}

struct TreeCache {
    a_only: Vec<FileTreeNode>,
    b_only: Vec<FileTreeNode>,
    both: Vec<FileTreeNode>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            "system" => Some(Theme::System),
            _ => None,
        }
    }

    fn to_visuals(&self) -> egui::Visuals {
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

struct DirCompareApp {
    state: AppState,
}

impl DirCompareApp {
    fn new(initial_theme: Theme) -> Self {
        Self {
            state: AppState {
                dir_a_path: String::new(),
                dir_b_path: String::new(),
                comparison_method: ComparisonStrategyType::Filename,
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

impl DirCompareApp {
    fn validate_path(path: &str) -> bool {
        if path.trim().is_empty() {
            return false;
        }
        std::path::Path::new(path).is_dir()
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
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.state.dir_a_path = path.display().to_string();
                    }
                }

                if !self.state.dir_a_path.is_empty() {
                    if Self::validate_path(&self.state.dir_a_path) {
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
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.state.dir_b_path = path.display().to_string();
                    }
                }

                if !self.state.dir_b_path.is_empty() {
                    if Self::validate_path(&self.state.dir_b_path) {
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

            ui.add_space(20.0);

            // Compare Button
            let can_compare = Self::validate_path(&self.state.dir_a_path)
                && Self::validate_path(&self.state.dir_b_path)
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
