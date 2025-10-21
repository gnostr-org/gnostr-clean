use std::fs;
use tempfile::tempdir;

// Import the MavenCleaner and Cleaner trait from the crate
use gnostr_clean::MavenCleaner;
use gnostr_clean::Cleaner;
// Import helper functions from the crate's cleaner module
use gnostr_clean::cleaner::{cmd, del};

#[test]
fn test_maven_cleaner_removes_target_dir() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy pom.xml file to simulate a Maven project
    let pom_xml_content = "<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0-SNAPSHOT</version>
</project>";
    let pom_xml_path = temp_path.join("pom.xml");
    fs::write(&pom_xml_path, pom_xml_content).expect("Failed to create pom.xml");

    // Create target directory
    let target_dir = temp_path.join("target");
    fs::create_dir(&target_dir).expect("Failed to create target directory");
    // Create a dummy file inside target to ensure recursive deletion works.
    let target_file = target_dir.join("classes");
    fs::File::create(&target_file).expect("Failed to create target/classes file");

    // Manually call del for the target directory, as 'mvn clean' might not be available.
    // This focuses on testing the 'del' part of the cleaner.
    del(&temp_path_str, "target").expect("Failed to delete target directory");

    // Assert that the target directory is removed
    assert!(!target_dir.exists(), "target directory should have been removed");
}

#[test]
fn test_maven_cleaner_removes_target_as_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path();
    let temp_path_str = temp_path.to_string_lossy();

    // Create a dummy pom.xml file
    let pom_xml_content = "<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0-SNAPSHOT</version>
</project>";
    let pom_xml_path = temp_path.join("pom.xml");
    fs::write(&pom_xml_path, pom_xml_content).expect("Failed to create pom.xml");

    // Create target as a file
    let target_file = temp_path.join("target");
    fs::File::create(&target_file).expect("Failed to create target file");

    // Manually call del for the target file
    del(&temp_path_str, "target").expect("Failed to delete target file");

    // Assert that the target file is removed
    assert!(!target_file.exists(), "target file should have been removed");
}

// Note: Testing the `mvn clean` command execution is complex without mocking.
// This test focuses on the `del` part of the cleaner.
// We also assume that `mvn` command is available in the environment where tests are run.
