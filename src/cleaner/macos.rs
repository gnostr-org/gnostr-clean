//! Basic cleaner module for Node.js projects.
use super::Cleaner;
use std::io;

/// Cleaner implementation for Node.js projects.
pub struct MacosCleaner;
impl Cleaner for MacosCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Macos"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &[".DS_Store"]
    }

    /// Cleans the provided directory based on a NodeJS structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
        super::del(dir, ".DS_Store")
    }
}
