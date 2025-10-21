use std::fs;
use tempfile::tempdir;

// Corrected use statement: import directly from the crate root
use gnostr_clean::{RustupCleaner, Cleaner};

#[test]
fn test_rustup_cleaner_removes_dot_rustup_dir() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();

    // Create a .rustup directory inside the temporary directory
    let dot_rustup_dir = temp_path.join(".rustup");
    fs::create_dir(&dot_rustup_dir).expect("Failed to create .rustup directory");

    let cleaner = RustupCleaner;
    cleaner.clean(&temp_path.to_string_lossy()).expect("RustupCleaner failed to clean");

    // Assert that the .rustup directory is removed
    assert!(!dot_rustup_dir.exists(), ".rustup directory should have been removed");
}

#[test]
fn test_rustup_cleaner_removes_dot_rustup_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();

    // Create a .rustup file inside the temporary directory
    let dot_rustup_file = temp_path.join(".rustup");
    fs::File::create(&dot_rustup_file).expect("Failed to create .rustup file");

    let cleaner = RustupCleaner;
    cleaner.clean(&temp_path.to_string_lossy()).expect("RustupCleaner failed to clean");

    // Assert that the .rustup file is removed
    assert!(!dot_rustup_file.exists(), ".rustup file should have been removed");
}

// Note: Testing the `cmd` execution (e.g., `rustup default stable`) is complex
// without mocking std::process::Command. This test focuses on the `del` part.
