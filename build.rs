use git2::{Error, Repository};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Error> {
    let repo = Repository::open(".")?;

    let diff = repo.diff_index_to_workdir(None, None)?;

    for delta in diff.deltas() {
        if let Some(file_path) = delta.new_file().path() {
            if file_path.to_string_lossy().ends_with(".DS_Store") {
                eprintln!("Error: Committing .DS_Store file is not allowed.");
                std::process::exit(1);
            }
        }
    }

    // Re-run this build script if the script changes.
    println!("cargo:rerun-if-changed=install_script.sh");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("install_script.sh");

    // Copy the script to the OUT_DIR.
    fs::copy("install_script.sh", &dest_path).expect("Failed to copy install_script.sh");

    // Make the copied script executable.
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        Command::new("chmod")
            .arg("+x")
            .arg(&dest_path)
            .status()
            .expect("Failed to make install_script.sh executable");
    }

    // Tell cargo to include the script in the package.
    println!("cargo:rustc-cfg=feature=\"git2\"");
    println!("cargo:rustc-env=INSTALL_SCRIPT={}", dest_path.display());

    Ok(())
}
