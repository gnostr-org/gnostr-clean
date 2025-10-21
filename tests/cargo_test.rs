use std::fs;
use tempfile::tempdir;

// Import the CargoCleaner and Cleaner trait from the crate
use gnostr_clean::CargoCleaner;
use gnostr_clean::Cleaner;

#[test]
fn test_cargo_cleaner_removes_target_and_vendor_dirs() {
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

    // Create target directory
    let target_dir = temp_path.join("target");
    fs::create_dir(&target_dir).expect("Failed to create target directory");

    // Create vendor directory
    let vendor_dir = temp_path.join("vendor");
    fs::create_dir(&vendor_dir).expect("Failed to create vendor directory");

    let cleaner = CargoCleaner;
    cleaner.clean(&temp_path.to_string_lossy()).expect("CargoCleaner failed to clean");

    // Assert that target and vendor directories are removed
    assert!(!target_dir.exists(), "target directory should have been removed");
    assert!(!vendor_dir.exists(), "vendor directory should have been removed");
}

#[test]
fn test_cargo_cleaner_removes_target_as_file() {
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

    // Create target as a file
    let target_file = temp_path.join("target");
    fs::File::create(&target_file).expect("Failed to create target file");

    let cleaner = CargoCleaner;
    cleaner.clean(&temp_path.to_string_lossy()).expect("CargoCleaner failed to clean");

    // Assert that the target file is removed
    assert!(!target_file.exists(), "target file should have been removed");
}

#[test]
fn test_cargo_cleaner_removes_vendor_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();

    // Create a minimal valid Cargo.toml file with a binary target
    let cargo_toml_content = "[package]\nname = \"dummy-package\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[[bin]]\nname = \"main\"\npath = \"src/main.rs\"";
    let cargo_toml_path = temp_path.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content).expect("Failed to create Cargo.toml");

    // Create vendor as a file
    let vendor_file = temp_path.join("vendor");
    fs::File::create(&vendor_file).expect("Failed to create vendor file");

    let cleaner = CargoCleaner;
    cleaner.clean(&temp_path.to_string_lossy()).expect("CargoCleaner failed to clean");

    // Assert that the vendor file is removed
    assert!(!vendor_file.exists(), "vendor file should have been removed");
}

// Note: Testing the `cargo clean` command execution is complex without mocking.
// This test focuses on the `del` part of the cleaner.
