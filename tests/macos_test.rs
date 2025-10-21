use std::fs;
use tempfile::tempdir;

// Import the MacosCleaner and Cleaner trait from the crate
use gnostr_clean::MacosCleaner;
use gnostr_clean::Cleaner;
// Import helper functions from the crate's cleaner module
use gnostr_clean::cleaner::{cmd, del};

#[test]
fn test_macos_cleaner_removes_ds_store_and_trash() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy .DS_Store file
    let ds_store_file = temp_path.join(".DS_Store");
    fs::File::create(&ds_store_file).expect("Failed to create .DS_Store file");

    // Create a dummy .Trash directory
    let trash_dir = temp_path.join(".Trash");
    fs::create_dir(&trash_dir).expect("Failed to create .Trash directory");
    // Create a dummy file inside .Trash to ensure recursive deletion works.
    let trash_file = trash_dir.join("some_file.txt");
    fs::File::create(&trash_file).expect("Failed to create .Trash/some_file.txt file");

    // Manually call the actions performed by MacosCleaner::clean
    // Note: We are skipping 'brew' commands as they require brew to be installed.

    // Test `rm .DS_Store` command
    cmd(&temp_path_str, "rm", &[ ".DS_Store" ]).expect("Failed to execute rm .DS_Store command");
    // Test `rm -rf .Trash` command
    cmd(&temp_path_str, "rm", &[ "-rf", ".Trash" ]).expect("Failed to execute rm -rf .Trash command");
    // Test `del .DS_Store` function
    del(&temp_path_str, ".DS_Store").expect("Failed to delete .DS_Store via del function");

    // Assert that .DS_Store file and .Trash directory are removed
    assert!(!ds_store_file.exists(), ".DS_Store file should have been removed");
    assert!(!trash_dir.exists(), ".Trash directory should have been removed");
}

#[test]
fn test_macos_cleaner_removes_ds_store_as_file_when_dir_exists() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy .DS_Store directory
    let ds_store_dir = temp_path.join(".DS_Store");
    fs::create_dir(&ds_store_dir).expect("Failed to create .DS_Store directory");

    // Manually call del for the .DS_Store directory
    // The del function first tries to remove as a directory, then as a file.
    // Here, we are testing the directory removal part.
    del(&temp_path_str, ".DS_Store").expect("Failed to delete .DS_Store directory via del function");

    // Assert that the .DS_Store directory is removed
    assert!(!ds_store_dir.exists(), ".DS_Store directory should have been removed");
}

// Note: Testing the `brew cleanup` and `brew autoremove` commands is complex without mocking.
// This test focuses on the `rm` and `del` parts of the cleaner.
// We also assume that `rm` command is available in the environment where tests are run.
