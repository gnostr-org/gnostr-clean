use std::fs;
use tempfile::tempdir;

// Import the MixCleaner and Cleaner trait from the crate
use gnostr_clean::MixCleaner;
use gnostr_clean::Cleaner;
// Import helper functions from the crate's cleaner module
use gnostr_clean::cleaner::{cmd, del};

#[test]
fn test_mix_cleaner_removes_dirs() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy mix.exs file to simulate a Mix project
    let mix_exs_content = "defproject :my_app, \"1.0.0\",
  app: :my_app,
  version: \"1.0.0\",
  elixir: \"~> 1.14\",
  start_permanent: Mix.env() == :prod,
  deps: deps()\n";
    let mix_exs_path = temp_path.join("mix.exs");
    fs::write(&mix_exs_path, mix_exs_content).expect("Failed to create mix.exs");

    // Create build directory
    let build_dir = temp_path.join("build");
    fs::create_dir(&build_dir).expect("Failed to create build directory");
    let build_file = build_dir.join("output.beam");
    fs::File::create(&build_file).expect("Failed to create build/output.beam file");

    // Create deps directory
    let deps_dir = temp_path.join("deps");
    fs::create_dir(&deps_dir).expect("Failed to create deps directory");
    let deps_file = deps_dir.join("some_dep.ez");
    fs::File::create(&deps_file).expect("Failed to create deps/some_dep.ez file");

    // Create doc directory
    let doc_dir = temp_path.join("doc");
    fs::create_dir(&doc_dir).expect("Failed to create doc directory");
    let doc_file = doc_dir.join("index.html");
    fs::File::create(&doc_file).expect("Failed to create doc/index.html file");

    // Manually call del for build, deps, and doc directories.
    // This focuses on testing the 'del' part of the cleaner.
    del(&temp_path_str, "build").expect("Failed to delete build directory");
    del(&temp_path_str, "deps").expect("Failed to delete deps directory");
    del(&temp_path_str, "doc").expect("Failed to delete doc directory");

    // Assert that the directories are removed
    assert!(!build_dir.exists(), "build directory should have been removed");
    assert!(!deps_dir.exists(), "deps directory should have been removed");
    assert!(!doc_dir.exists(), "doc directory should have been removed");
}

#[test]
fn test_mix_cleaner_removes_build_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create build as a file
    let build_file = temp_path.join("build");
    fs::File::create(&build_file).expect("Failed to create build file");

    // Manually call del for the build file
    del(&temp_path_str, "build").expect("Failed to delete build file");

    // Assert that the build file is removed
    assert!(!build_file.exists(), "build file should have been removed");
}

#[test]
fn test_mix_cleaner_removes_deps_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create deps as a file
    let deps_file = temp_path.join("deps");
    fs::File::create(&deps_file).expect("Failed to create deps file");

    // Manually call del for the deps file
    del(&temp_path_str, "deps").expect("Failed to delete deps file");

    // Assert that the deps file is removed
    assert!(!deps_file.exists(), "deps file should have been removed");
}

#[test]
fn test_mix_cleaner_removes_doc_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create doc as a file
    let doc_file = temp_path.join("doc");
    fs::File::create(&doc_file).expect("Failed to create doc file");

    // Manually call del for the doc file
    del(&temp_path_str, "doc").expect("Failed to delete doc file");

    // Assert that the doc file is removed
    assert!(!doc_file.exists(), "doc file should have been removed");
}

// Note: Testing the `mix clean` command execution is complex without mocking.
// This test focuses on the `del` part of the cleaner.
// We also assume that `mix` command is available in the environment where tests are run.

