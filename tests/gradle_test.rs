use std::fs;
use tempfile::tempdir;

// Import the GradleCleaner and Cleaner trait from the crate
use gnostr_clean::GradleCleaner;
use gnostr_clean::Cleaner;
// Import helper functions from the crate's cleaner module
use gnostr_clean::cleaner::{cmd, del};

#[test]
fn test_gradle_cleaner_removes_build_and_out_dirs() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy build.gradle file to simulate a Gradle project
    let build_gradle_content = "plugins {\n    id 'java'\n}\n\nrepositories {\n    mavenCentral()\n}\n";
    let build_gradle_path = temp_path.join("build.gradle");
    fs::write(&build_gradle_path, build_gradle_content).expect("Failed to create build.gradle");

    // Create build directory
    let build_dir = temp_path.join("build");
    fs::create_dir(&build_dir).expect("Failed to create build directory");
    // Create a dummy file inside build to ensure recursive deletion works.
    let build_file = build_dir.join("classes");
    fs::File::create(&build_file).expect("Failed to create build/classes file");

    // Create out directory
    let out_dir = temp_path.join("out");
    fs::create_dir(&out_dir).expect("Failed to create out directory");
    // Create a dummy file inside out to ensure recursive deletion works.
    let out_file = out_dir.join("output.log");
    fs::File::create(&out_file).expect("Failed to create out/output.log file");

    // Manually call del for build and out directories, as 'gradle clean' might not be available.
    // This focuses on testing the 'del' part of the cleaner.
    del(&temp_path_str, "build").expect("Failed to delete build dir");
    del(&temp_path_str, "out").expect("Failed to delete out dir");

    // Assert that build and out directories are removed
    assert!(!temp_path.join("build").exists(), "build directory should have been removed");
    assert!(!temp_path.join("out").exists(), "out directory should have been removed");
}

#[test]
fn test_gradle_cleaner_removes_build_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy build.gradle file
    let build_gradle_content = "plugins {\n    id 'java'\n}\n\nrepositories {\n    mavenCentral()\n}\n";
    let build_gradle_path = temp_path.join("build.gradle");
    fs::write(&build_gradle_path, build_gradle_content).expect("Failed to create build.gradle");

    // Create build as a file
    let build_file = temp_path.join("build");
    fs::File::create(&build_file).expect("Failed to create build file");

    // Manually call del for the build file
    del(&temp_path_str, "build").expect("Failed to delete build file");

    // Assert that the build file is removed
    assert!(!build_file.exists(), "build file should have been removed");
}

#[test]
fn test_gradle_cleaner_removes_out_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy build.gradle file
    let build_gradle_content = "plugins {\n    id 'java'\n}\n\nrepositories {\n    mavenCentral()\n}\n";
    let build_gradle_path = temp_path.join("build.gradle");
    fs::write(&build_gradle_path, build_gradle_content).expect("Failed to create build.gradle");

    // Create out as a file
    let out_file = temp_path.join("out");
    fs::File::create(&out_file).expect("Failed to create out file");

    // Manually call del for the out file
    del(&temp_path_str, "out").expect("Failed to delete out file");

    // Assert that the out file is removed
    assert!(!out_file.exists(), "out file should have been removed");
}

// Note: Testing the `gradle clean` command execution is complex without mocking.
// This test focuses on the `del` part of the cleaner.
// We also assume that `gradle` command is available in the environment where tests are run.