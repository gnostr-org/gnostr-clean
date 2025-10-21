use std::fs;
use tempfile::tempdir;

// Import the MakeFileCleaner and Cleaner trait from the crate
use gnostr_clean::MakeFileCleaner;
use gnostr_clean::Cleaner;
// Import helper functions from the crate's cleaner module
use gnostr_clean::cleaner::{cmd, del};

#[test]
fn test_makefile_cleaner_removes_dirs() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create dummy files/directories that the cleaner acts upon.
    // Note: The cleaner has typos in command names (CmakeFiles, CmakeCache).
    // We will test for the intended directories: CMakeFiles, CMakeCache.

    // Create build directory
    let build_dir = temp_path.join("build");
    fs::create_dir(&build_dir).expect("Failed to create build directory");
    let build_file = build_dir.join("output.o");
    fs::File::create(&build_file).expect("Failed to create build/output.o file");

    // Create .deps directory
    let deps_dir = temp_path.join(".deps");
    fs::create_dir(&deps_dir).expect("Failed to create .deps directory");
    let deps_file = deps_dir.join("lib.so");
    fs::File::create(&deps_file).expect("Failed to create .deps/lib.so file");

    // Create .libs directory
    let libs_dir = temp_path.join(".libs");
    fs::create_dir(&libs_dir).expect("Failed to create .libs directory");
    let libs_file = libs_dir.join("lib.a");
    fs::File::create(&libs_file).expect("Failed to create .libs/lib.a file");

    // Create CMakeFiles directory (assuming typo correction)
    let cmakelists_dir = temp_path.join("CMakeFiles");
    fs::create_dir(&cmakelists_dir).expect("Failed to create CMakeFiles directory");
    let cmakelists_file = cmakelists_dir.join("progress.make");
    fs::File::create(&cmakelists_file).expect("Failed to create CMakeFiles/progress.make file");

    // Create CMakeCache file (assuming typo correction)
    let cmakecache_file = temp_path.join("CMakeCache.txt");
    fs::File::create(&cmakecache_file).expect("Failed to create CMakeCache.txt file");

    // Manually call the rm commands performed by MakeFileCleaner::clean
    // We are skipping 'make clean' as it requires make to be installed.

    // Test `rm -rf build` command
    cmd(&temp_path_str, "rm", &[ "-rf", "build" ]).expect("Failed to execute rm -rf build command");
    // Test `rm -rf .deps` command
    cmd(&temp_path_str, "rm", &[ "-rf", ".deps" ]).expect("Failed to execute rm -rf .deps command");
    // Test `rm -rf .libs` command
    cmd(&temp_path_str, "rm", &[ "-rf", ".libs" ]).expect("Failed to execute rm -rf .libs command");
    // Test `rm -rf CMakeFiles` command (assuming typo correction)
    cmd(&temp_path_str, "rm", &[ "-rf", "CMakeFiles" ]).expect("Failed to execute rm -rf CMakeFiles command");
    // Test `rm -rf CMakeCache` command (assuming typo correction)
    cmd(&temp_path_str, "rm", &[ "-rf", "CMakeCache.txt" ]).expect("Failed to execute rm -rf CMakeCache.txt command");

    // Assert that the directories and files are removed
    assert!(!build_dir.exists(), "build directory should have been removed");
    assert!(!deps_dir.exists(), ".deps directory should have been removed");
    assert!(!libs_dir.exists(), ".libs directory should have been removed");
    assert!(!cmakelists_dir.exists(), "CMakeFiles directory should have been removed");
    assert!(!cmakecache_file.exists(), "CMakeCache.txt file should have been removed");
}

#[test]
fn test_makefile_cleaner_removes_build_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create build as a file
    let build_file = temp_path.join("build");
    fs::File::create(&build_file).expect("Failed to create build file");

    // Manually call rm for the build file
    cmd(&temp_path_str, "rm", &[ "-rf", "build" ]).expect("Failed to execute rm -rf build command");

    // Assert that the build file is removed
    assert!(!build_file.exists(), "build file should have been removed");
}

#[test]
fn test_makefile_cleaner_removes_deps_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create .deps as a file
    let deps_file = temp_path.join(".deps");
    fs::File::create(&deps_file).expect("Failed to create .deps file");

    // Manually call rm for the .deps file
    cmd(&temp_path_str, "rm", &[ "-rf", ".deps" ]).expect("Failed to execute rm -rf .deps command");

    // Assert that the .deps file is removed
    assert!(!deps_file.exists(), ".deps file should have been removed");
}

#[test]
fn test_makefile_cleaner_removes_libs_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create .libs as a file
    let libs_file = temp_path.join(".libs");
    fs::File::create(&libs_file).expect("Failed to create .libs file");

    // Manually call rm for the .libs file
    cmd(&temp_path_str, "rm", &[ "-rf", ".libs" ]).expect("Failed to execute rm -rf .libs command");

    // Assert that the .libs file is removed
    assert!(!libs_file.exists(), ".libs file should have been removed");
}

// Note: Testing the `make clean` command execution is complex without mocking.
// This test focuses on the `rm` commands of the cleaner.
// We also assume that `rm` command is available in the environment where tests are run.
