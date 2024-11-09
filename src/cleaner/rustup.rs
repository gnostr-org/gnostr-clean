//! Basic cleaner module for Cargo projects.
use super::Cleaner;
use std::io;

/// Cleaner implementation for rustup
pub struct RustupCleaner;
impl Cleaner for RustupCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Rustup"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &[".rustup", "vendor"]
    }

    /// cleaner the provided directory based on a Cargo structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "cargo", &["clean"])?;
        super::del(dir, "target");
        super::del(dir, "vendor");
        super::del(dir, "node_modules")
    }
}
