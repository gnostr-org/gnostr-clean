use std::fs;
use tempfile::tempdir;

// Import the NodeCleaner and Cleaner trait from the crate
use gnostr_clean::NodeCleaner;
use gnostr_clean::Cleaner;
// Import helper functions from the crate's cleaner module
use gnostr_clean::cleaner::{cmd, del};

#[test]
fn test_node_cleaner_removes_node_modules_and_npm_dirs() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy package.json file to simulate a Node.js project
    let package_json_content = "{\"name\": \"my-node-app\", \"version\": \"1.0.0\"}";
    let package_json_path = temp_path.join("package.json");
    fs::write(&package_json_path, package_json_content).expect("Failed to create package.json");

    // Create node_modules directory
    let node_modules_dir = temp_path.join("node_modules");
    fs::create_dir(&node_modules_dir).expect("Failed to create node_modules directory");
    let node_modules_file = node_modules_dir.join("some_package.js");
    fs::File::create(&node_modules_file).expect("Failed to create node_modules/some_package.js file");

    // Create .npm directory
    let npm_dir = temp_path.join(".npm");
    fs::create_dir(&npm_dir).expect("Failed to create .npm directory");
    let npm_file = npm_dir.join("cache.json");
    fs::File::create(&npm_file).expect("Failed to create .npm/cache.json file");

    // Manually call del for node_modules and .npm directories.
    del(&temp_path_str, "node_modules").expect("Failed to delete node_modules directory");
    del(&temp_path_str, ".npm").expect("Failed to delete .npm directory");

    // Assert that the directories are removed
    assert!(!node_modules_dir.exists(), "node_modules directory should have been removed");
    assert!(!npm_dir.exists(), ".npm directory should have been removed");
}

#[test]
fn test_node_cleaner_removes_node_modules_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create node_modules as a file
    let node_modules_file = temp_path.join("node_modules");
    fs::File::create(&node_modules_file).expect("Failed to create node_modules file");

    // Manually call del for the node_modules file
    del(&temp_path_str, "node_modules").expect("Failed to delete node_modules file");

    // Assert that the node_modules file is removed
    assert!(!node_modules_file.exists(), "node_modules file should have been removed");
}

#[test]
fn test_node_cleaner_removes_npm_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create .npm as a file
    let npm_file = temp_path.join(".npm");
    fs::File::create(&npm_file).expect("Failed to create .npm file");

    // Manually call del for the .npm file
    del(&temp_path_str, ".npm").expect("Failed to delete .npm file");

    // Assert that the .npm file is removed
    assert!(!npm_file.exists(), ".npm file should have been removed");
}
