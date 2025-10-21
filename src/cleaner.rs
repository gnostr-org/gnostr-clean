//! Cleaning traits and implementations.
mod cargo;
mod git;
mod gnostr;
mod gradle;
mod macos;
mod makefile;
mod maven;
mod mix;
mod node;
mod rustup;

pub use cargo::CargoCleaner;
pub use git::GitCleaner;
pub use gnostr::GnostrCleaner;
pub use gradle::GradleCleaner;
pub use macos::MacosCleaner;
pub use makefile::MakeFileCleaner;
pub use maven::MavenCleaner;
pub use mix::MixCleaner;
pub use node::NodeCleaner;
pub use rustup::RustupCleaner;

use std::fs;
use std::io::{self};
use std::process::{Command, Stdio};
use std::str;

/// Trait to represent a cleaning structure.
pub trait Cleaner {
    /// Returns the name of the current cleaner.
    fn name(&self) -> &str;

    /// Cleans a directory assumed to be a relevant directory.
    fn clean(&self, dir: &str) -> io::Result<()>;

    /// Returns a set of file names which identify a relevant directory.
    fn triggers(&self) -> &[&str];
}


/// Executes a command in a directory using provided arguments.
pub fn cmd(dir: &str, cmd_name: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(cmd_name)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = output.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed: {} with status: {}\nStderr: {}", cmd_name, output.status, stderr),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Purges a location on disk, similar to `rm -rf`.
pub fn del(parent: &str, child: &str) -> io::Result<()> {
    let dir_path = format!("{}/{}", parent, child);
    println!("{}", dir_path);

    // Attempt to remove the directory. If it doesn't exist, it's not an error.
    if let Err(err) = fs::remove_dir_all(&dir_path) {
        if err.kind() != io::ErrorKind::NotFound {
            // If it's not 'NotFound', try removing as a file.
            // This handles cases where a directory might have been created with a file-like name or vice-versa.
            let file_path = format!("{}/{}", parent, child);
            if let Err(file_err) = fs::remove_file(&file_path) {
                if file_err.kind() != io::ErrorKind::NotFound {
                    // If neither removing as a directory nor as a file worked (and it wasn't NotFound),
                    // return the error.
                    return Err(file_err);
                }
            }
        }
    }
    Ok(())
}
