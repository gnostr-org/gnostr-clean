#![allow(unused_imports)] // For potential future imports

use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Node {
    folder: String,
    files: Vec<String>,
    children: Vec<Box<Node>>,
    hash: String,
}
impl Node {
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.folder.as_bytes());
        for file in &self.files {
            hasher.update(file.as_bytes());
        }
        for child in &self.children {
            hasher.update(child.hash.as_bytes());
        }
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn new(folder: String, files: Vec<String>, children: Vec<Box<Node>>) -> Node {
        let mut node = Node {
            folder,
            files,
            children,
            hash: String::new(), // Initialize hash as empty
        };
        node.hash = node.calculate_hash(); // Calculate and set the hash
        node
    }

    fn clone_with_new_hash(&self) -> Node {
        let mut cloned = self.clone();
        cloned.hash = cloned.calculate_hash();
        cloned
    }
}

fn build_git_tree(path: &Path) -> Result<Option<Box<Node>>, std::io::Error> {
    if !path.is_dir() {
        return Ok(None);
    }

    let folder_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| String::from(""));

    let mut files = Vec::new();
    let mut children = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            if let Some(file_name) = entry_path.file_name().and_then(|s| s.to_str()) {
                files.push(file_name.to_string());
            }
        } else if entry_path.is_dir() {
            if let Some(child_node) = build_git_tree(&entry_path)? {
                children.push(child_node);
            }
        }
    }

    let mut new_node = Box::new(Node {
        folder: folder_name,
        files,
        children,
        hash: String::new(),
    });

    new_node.hash = new_node.calculate_hash();

    Ok(Some(new_node))
}

fn main() -> Result<(), std::io::Error> {
    let git_path = Path::new(".git"); // Replace with your .git directory path
    if let Some(root_node) = build_git_tree(git_path)? {
        println!("{:#?}", root_node);
    } else {
        println!(".git not found or empty.");
    }
    Ok(())
}
