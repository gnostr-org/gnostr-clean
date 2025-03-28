#![allow(unused_imports)] // For potential future imports

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Node {
    folder: String,
    files: Vec<String>,
    children: Vec<Box<Node>>,
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

    Ok(Some(Box::new(Node {
        folder: folder_name,
        files,
        children,
    })))
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
