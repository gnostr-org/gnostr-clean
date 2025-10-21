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
        &[
            ".DS_Store",
            ".Trash",
            "MailData/Downloads",
            "Library/Logs",
            "Library/Developer/Xcode/Archives",
            "Library/Developer/Xcode/DerivedData",
            "Library/Developer/CoreSimulator/Devices",
            "Library/Caches/org.swift.swiftpm",
            "Library/Caches/Homebrew",
            "Developer/CoreSimulator/Caches",
        ]
    }

    /// Cleans the provided directory based on a certain criteria.
    fn clean(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "brew", &["cleanup"])?;
        super::cmd(dir, "brew", &["autoremove"])?;
        super::cmd(dir, "rm", &[".DS_Store"])?;
        super::cmd(dir, "rm", &["-rf", ".Trash"])?;

        super::del(dir, ".DS_Store")?;
        Ok(())
    }
}
