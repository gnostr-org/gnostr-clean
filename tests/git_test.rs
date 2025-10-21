use std::fs;
use std::process::Command;
use tempfile::tempdir;

// Import the GitCleaner and Cleaner trait from the crate
use gnostr_clean::GitCleaner;
use gnostr_clean::Cleaner;

#[test]
fn test_git_cleaner_removes_git_dir() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();

    // Initialize a git repository in the temporary directory
    Command::new("git")
        .arg("init")
        .current_dir(temp_path)
        .output()
        .expect("Failed to initialize git repository");

    // Create a dummy file and commit it to ensure the git repository is fully initialized.
    let readme_path = temp_path.join("README.md");
    fs::File::create(&readme_path).expect("Failed to create README.md");
    Command::new("git")
        .arg("add")
        .arg("README.md")
        .current_dir(temp_path)
        .output()
        .expect("Failed to add README.md");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial commit")
        .current_dir(temp_path)
        .output()
        .expect("Failed to commit README.md");


    let cleaner = GitCleaner;
    // The cleaner should be called on the directory containing .git
    cleaner.clean(&temp_path.to_string_lossy()).expect("GitCleaner failed to clean");

    // Assert that the .git directory is removed
    assert!(!temp_path.join(".git").exists(), ".git directory should have been removed");
}

#[test]
fn test_git_cleaner_removes_libs_dir() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();

    // Initialize a git repository in the temporary directory
    Command::new("git")
        .arg("init")
        .current_dir(temp_path)
        .output()
        .expect("Failed to initialize git repository");

    // Create a dummy .libs directory
    let libs_dir = temp_path.join(".libs");
    fs::create_dir(&libs_dir).expect("Failed to create .libs directory");

    // Create a dummy file inside .libs
    let libs_file = libs_dir.join("libexample.so");
    fs::File::create(&libs_file).expect("Failed to create .libs/libexample.so file");

    let cleaner = GitCleaner;
    // The cleaner should be called on the directory containing both .git and .libs
    cleaner.clean(&temp_path.to_string_lossy()).expect("GitCleaner failed to clean");

    // Assert that the .libs directory is removed
    assert!(!libs_dir.exists(), ".libs directory should have been removed");
    // Note: The .git directory should also be cleaned by git gc, but we are not asserting that here.
}

// Note: Testing the `git` command execution (e.g., `git gc`) is complex without mocking.
// This test focuses on the `del` part of the cleaner.
// We also assume that `git` command is available in the environment where tests are run.
