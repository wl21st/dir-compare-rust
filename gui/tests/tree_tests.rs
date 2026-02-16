use dir_compare_core::{Entry, EntryKind};
use dir_compare_gui::tree_view::FileTreeNode;
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
fn test_file_tree_node_from_entries_empty() {
    let entries: Vec<Entry> = vec![];
    let nodes = FileTreeNode::from_entries(&entries);
    assert!(nodes.is_empty());
}

#[test]
fn test_file_tree_node_from_entries_single_file() {
    let entries = vec![create_entry("file.txt", EntryKind::File)];
    let nodes = FileTreeNode::from_entries(&entries);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].name, "file.txt");
    assert_eq!(nodes[0].kind, EntryKind::File);
    assert!(nodes[0].children.is_empty());
}

#[test]
fn test_file_tree_node_from_entries_single_directory() {
    let entries = vec![create_entry("dir", EntryKind::Directory)];
    let nodes = FileTreeNode::from_entries(&entries);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].name, "dir");
    assert_eq!(nodes[0].kind, EntryKind::Directory);
}

#[test]
fn test_file_tree_node_from_entries_nested_structure() {
    let entries = vec![
        create_entry("dir/file1.txt", EntryKind::File),
        create_entry("dir/file2.txt", EntryKind::File),
        create_entry("dir/subdir/nested.txt", EntryKind::File),
    ];
    let nodes = FileTreeNode::from_entries(&entries);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].name, "dir");
    assert_eq!(nodes[0].children.len(), 3);

    let subdir = nodes[0].children.iter().find(|n| n.name == "subdir");
    assert!(subdir.is_some());
    assert_eq!(subdir.unwrap().children.len(), 1);
    assert_eq!(subdir.unwrap().children[0].name, "nested.txt");
}

#[test]
fn test_file_tree_node_from_entries_multiple_roots() {
    let entries = vec![
        create_entry("file1.txt", EntryKind::File),
        create_entry("file2.txt", EntryKind::File),
        create_entry("dir/file.txt", EntryKind::File),
    ];
    let nodes = FileTreeNode::from_entries(&entries);
    assert_eq!(nodes.len(), 3);
}

#[test]
fn test_file_tree_node_from_entries_deeply_nested() {
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

    assert_eq!(current.children.len(), 1);
    assert_eq!(current.children[0].name, "file.txt");
    assert_eq!(current.children[0].kind, EntryKind::File);
}

#[test]
fn test_file_tree_node_path_preserved() {
    let entries = vec![create_entry("dir/subdir/file.txt", EntryKind::File)];
    let nodes = FileTreeNode::from_entries(&entries);
    assert_eq!(nodes[0].path, PathBuf::from("dir"));
    assert_eq!(nodes[0].children[0].path, PathBuf::from("dir/subdir"));
    assert_eq!(
        nodes[0].children[0].children[0].path,
        PathBuf::from("dir/subdir/file.txt")
    );
}

#[test]
fn test_file_tree_node_mixed_kinds() {
    let entries = vec![
        create_entry("mixed/file.txt", EntryKind::File),
        create_entry("mixed/dir", EntryKind::Directory),
    ];
    let nodes = FileTreeNode::from_entries(&entries);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].name, "mixed");

    let mixed_dir = &nodes[0].children;
    let file_node = mixed_dir.iter().find(|n| n.name == "file.txt");
    let dir_node = mixed_dir.iter().find(|n| n.name == "dir");

    assert!(file_node.is_some());
    assert_eq!(file_node.unwrap().kind, EntryKind::File);
    assert!(dir_node.is_some());
    assert_eq!(dir_node.unwrap().kind, EntryKind::Directory);
}
