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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_entry(path: &str, kind: EntryKind) -> Entry {
        Entry {
            path: PathBuf::from(path),
            kind,
            abs_path: PathBuf::from(path),
            size: Some(0),
        }
    }

    #[test]
    fn test_from_entries_empty() {
        let entries: Vec<Entry> = vec![];
        let nodes = FileTreeNode::from_entries(&entries);
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_from_entries_single_file() {
        let entries = vec![create_entry("file.txt", EntryKind::File)];
        let nodes = FileTreeNode::from_entries(&entries);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "file.txt");
        assert_eq!(nodes[0].kind, EntryKind::File);
        assert!(nodes[0].children.is_empty());
    }

    #[test]
    fn test_from_entries_single_directory() {
        let entries = vec![create_entry("dir", EntryKind::Directory)];
        let nodes = FileTreeNode::from_entries(&entries);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "dir");
        assert_eq!(nodes[0].kind, EntryKind::Directory);
    }

    #[test]
    fn test_from_entries_nested_structure() {
        let entries = vec![
            create_entry("dir/file1.txt", EntryKind::File),
            create_entry("dir/file2.txt", EntryKind::File),
            create_entry("dir/subdir/nested.txt", EntryKind::File),
        ];
        let nodes = FileTreeNode::from_entries(&entries);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "dir");
        assert_eq!(nodes[0].children.len(), 3); // file1.txt, file2.txt, subdir

        // Find the subdir
        let subdir = nodes[0].children.iter().find(|n| n.name == "subdir");
        assert!(subdir.is_some());
        assert_eq!(subdir.unwrap().children.len(), 1);
        assert_eq!(subdir.unwrap().children[0].name, "nested.txt");
    }

    #[test]
    fn test_from_entries_multiple_roots() {
        let entries = vec![
            create_entry("file1.txt", EntryKind::File),
            create_entry("file2.txt", EntryKind::File),
            create_entry("dir/file.txt", EntryKind::File),
        ];
        let nodes = FileTreeNode::from_entries(&entries);
        assert_eq!(nodes.len(), 3); // file1.txt, file2.txt, dir
    }

    #[test]
    fn test_from_entries_deeply_nested() {
        let entries = vec![create_entry("a/b/c/d/e/file.txt", EntryKind::File)];
        let nodes = FileTreeNode::from_entries(&entries);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "a");

        let mut current = &nodes[0];
        let expected_names = ["b", "c", "d", "e"];
        for name in expected_names.iter() {
            assert_eq!(current.children.len(), 1);
            current = &current.children[0];
            assert_eq!(current.name, *name);
            assert_eq!(current.kind, EntryKind::Directory);
        }

        // Finally the file
        assert_eq!(current.children.len(), 1);
        assert_eq!(current.children[0].name, "file.txt");
        assert_eq!(current.children[0].kind, EntryKind::File);
    }
}
