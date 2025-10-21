use std::fs;
use tempfile::tempdir;

// Import the GnostrCleaner and Cleaner trait from the crate
use gnostr_clean::GnostrCleaner;
use gnostr_clean::Cleaner;

#[test]
fn test_gnostr_cleaner_removes_node_modules_dir() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();

    // Create a minimal valid Cargo.toml file with a binary target
    let cargo_toml_content = "[package]\nname = \"dummy-package\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[[bin]]\nname = \"main\"\npath = \"src/main.rs\"";
    let cargo_toml_path = temp_path.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content).expect("Failed to create Cargo.toml");

    // Create src directory and a dummy main.rs file
    let src_dir = temp_path.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");
    let main_rs_path = src_dir.join("main.rs");
    fs::write(&main_rs_path, "fn main() { println!(\"Hello, world!\"); }\n").expect("Failed to create src/main.rs");

    // Create node_modules directory
    let node_modules_dir = temp_path.join("node_modules");
    fs::create_dir(&node_modules_dir).expect("Failed to create node_modules directory");

    // Create a dummy file inside node_modules to ensure recursive deletion works.
    let node_modules_file = node_modules_dir.join("some_package.js");
    fs::File::create(&node_modules_file).expect("Failed to create node_modules/some_package.js file");

    let cleaner = GnostrCleaner;
    // The cleaner should be called on the directory containing Cargo.toml and node_modules
    cleaner.clean(&temp_path.to_string_lossy()).expect("GnostrCleaner failed to clean");

    // Assert that the node_modules directory is removed
    assert!(!node_modules_dir.exists(), "node_modules directory should have been removed");
}

#[test]
fn test_gnostr_cleaner_removes_node_modules_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();

    // Create a minimal valid Cargo.toml file with a binary target
    let cargo_toml_content = "[package]\nname = \"dummy-package\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[[bin]]\nname = \"main\"\npath = \"src/main.rs\"";
    let cargo_toml_path = temp_path.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content).expect("Failed to create Cargo.toml");

    // Create src directory and a dummy main.rs file
    let src_dir = temp_path.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");
    let main_rs_path = src_dir.join("main.rs");
    fs::write(&main_rs_path, "fn main() { println!(\"Hello, world!\"); }\n").expect("Failed to create src/main.rs");

    // Create node_modules as a file
    let node_modules_file = temp_path.join("node_modules");
    fs::File::create(&node_modules_file).expect("Failed to create node_modules file");

    let cleaner = GnostrCleaner;
    cleaner.clean(&temp_path.to_string_lossy()).expect("GnostrCleaner failed to clean");

    // Assert that the node_modules file is removed
    assert!(!node_modules_file.exists(), "node_modules file should have been removed");
}

// Note: Testing the `cargo clean` command execution is complex without mocking.
// This test focuses on the `del` part of the cleaner.
// We also assume that `cargo` command is available in the environment where tests are run.