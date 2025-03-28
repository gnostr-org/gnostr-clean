use nostr_sdk::Keys;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

#[derive(Debug, Clone)]
struct Node {
    folder: String,
    files: Vec<String>,
    children: Vec<Box<Node>>,
    hash: String,
    branch: Option<String>, // Added branch info
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
        if let Some(branch) = &self.branch { // Include branch in hash
            hasher.update(branch.as_bytes());
        }
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn new(folder: String, files: Vec<String>, children: Vec<Box<Node>>, branch: Option<String>) -> Node {
        let mut node = Node {
            folder,
            files,
            children,
            hash: String::new(),
            branch, // Added branch info
        };
        node.hash = node.calculate_hash();
        node
    }

    fn clone_with_new_hash(&self) -> Node {
        let mut cloned = self.clone();
        cloned.hash = cloned.calculate_hash();
        cloned
    }
}

fn get_git_branch(path: &Path) -> Result<Option<String>, std::io::Error> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(path)
        .output()?;

    if output.status.success() {
        let branch = str::from_utf8(&output.stdout)
            .map(|s| s.trim().to_string())
            .ok();
        Ok(branch)
    } else {
        Ok(None)
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

    let branch = get_git_branch(path).unwrap_or(None); // Get branch info

    let mut new_node = Box::new(Node::new(folder_name, files, children, branch));

    Ok(Some(new_node))
}

fn main() -> Result<(), std::io::Error> {
    let git_path = Path::new(".git");
    if let Some(root_node) = build_git_tree(git_path)? {
        println!("root_node:{:#?}", root_node);

        let keys = Keys::parse(&root_node.hash).expect("Keys::parse from &root_node.hash");
        println!("Public key: {}", keys.public_key());
        println!("Private key (hex): {:?}", *keys.secret_key());
    } else {
        println!(".git not found or empty.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn test_node_calculate_hash() {
        let node = Node::new(
            "test_folder".to_string(),
            vec!["file1.txt".to_string(), "file2.txt".to_string()],
            vec![],
            Some("main".to_string()),
        );
        let hash = node.calculate_hash();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_node_clone_with_new_hash() {
        let node1 = Node::new(
            "test_folder".to_string(),
            vec!["file1.txt".to_string(), "file2.txt".to_string()],
            vec![],
            Some("main".to_string()),
        );
        let node2 = node1.clone_with_new_hash();
        assert_eq!(node1.hash, node2.hash);
        assert_eq!(node1.folder, node2.folder);
        assert_eq!(node1.branch, node2.branch);
    }

    #[test]
    fn test_build_git_tree_empty_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = build_git_tree(temp_dir.path()).unwrap();
        assert!(result.is_some());
        let node = result.unwrap();
        assert_eq!(
            node.folder,
            temp_dir.path().file_name().unwrap().to_str().unwrap()
        );
        assert!(node.files.is_empty());
        assert!(node.children.is_empty());
        assert!(node.branch.is_none());
    }

    #[test]
    fn test_build_git_tree_with_files_and_dirs() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let sub_dir_path = temp_dir.path().join("sub_dir");
        fs::create_dir(&sub_dir_path).unwrap();
        let sub_file_path = sub_dir_path.join("sub_file.txt");
        let mut sub_file = fs::File::create(&sub_file_path).unwrap();
        sub_file.write_all(b"sub content").unwrap();

        let result = build_git_tree(temp_dir.path()).unwrap();
        assert!(result.is_some());
        let node = result.unwrap();
        assert_eq!(
            node.folder,
            temp_dir.path().file_name().unwrap().to_str().unwrap()
        );
        assert_eq!(node.files, vec!["test_file.txt".to_string()]);
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].folder, "sub_dir".to_string());
        assert_eq!(node.children[0].files, vec!["sub_file.txt".to_string()]);
        assert!(node.branch.is_none());
    }
    #[test]
    fn test_get_git_branch() {
        let temp_dir = tempfile::tempdir().unwrap();
        Command::new("git").arg("init").current_dir(temp_dir.path()).status().unwrap();
        Command::new("git").arg("checkout").arg("-b").arg("test_branch").current_dir(temp_dir.path()).status().unwrap();
        let branch = get_git_branch(temp_dir.path()).unwrap().unwrap();
        assert_eq!(branch, "test_branch");
    }
}
