use dir_compare_core::{Entry, EntryKind};
use eframe::egui;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileTreeNode {
    pub name: String,
    pub path: PathBuf,
    pub kind: EntryKind,
    pub children: Vec<FileTreeNode>,
}

impl FileTreeNode {
    pub fn from_entries(entries: &[Entry]) -> Vec<FileTreeNode> {
        let mut builder = NodeBuilder::default();
        for entry in entries {
            builder.insert(entry);
        }
        builder.into_vec()
    }
}

#[derive(Default)]
struct NodeBuilder {
    children: BTreeMap<String, NodeBuilder>,
    kind: Option<EntryKind>,
    path: PathBuf,
}

impl NodeBuilder {
    fn insert(&mut self, entry: &Entry) {
        let components: Vec<_> = entry.path.components().collect();
        let mut current = self;
        let mut path_accum = PathBuf::new();

        for (i, component) in components.iter().enumerate() {
            let name = component.as_os_str().to_string_lossy().to_string();
            path_accum.push(&name);

            let is_last = i == components.len() - 1;

            current = current.children.entry(name).or_insert_with(|| NodeBuilder {
                children: BTreeMap::new(),
                kind: None,
                path: path_accum.clone(),
            });

            if is_last {
                current.kind = Some(entry.kind.clone());
            } else if current.kind.is_none() {
                current.kind = Some(EntryKind::Directory);
            }
        }
    }

    fn into_vec(self) -> Vec<FileTreeNode> {
        self.children
            .into_iter()
            .map(|(name, mut node)| {
                let kind = node.kind.take().unwrap_or(EntryKind::Directory);
                let path = node.path.clone();
                let children = node.into_vec();
                FileTreeNode {
                    name,
                    path,
                    kind,
                    children,
                }
            })
            .collect()
    }
}

pub fn render_tree(ui: &mut egui::Ui, nodes: &[FileTreeNode], color: egui::Color32) {
    for node in nodes {
        render_node(ui, node, color);
    }
}

fn render_node(ui: &mut egui::Ui, node: &FileTreeNode, color: egui::Color32) {
    if node.children.is_empty() {
        ui.horizontal(|ui| {
            let icon = match node.kind {
                EntryKind::Directory => "📁",
                EntryKind::File => "📄",
            };
            ui.label(egui::RichText::new(format!("{} {}", icon, node.name)).color(color));
        });
    } else {
        egui::CollapsingHeader::new(egui::RichText::new(format!("📁 {}", node.name)).color(color))
            .id_source(&node.path)
            .show(ui, |ui| {
                render_tree(ui, &node.children, color);
            });
    }
}
