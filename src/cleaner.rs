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

use std::env;
use std::fs;
use std::io::{self, ErrorKind};
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

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(&p_str).is_ok() {
                println!("Trying: {}", p_str);
                return true;
            }
        }
    }
    false
}

/// Executes a command in a directory using provided arguments.
pub fn cmd(dir: &str, cmd: &str, args: &[&str]) -> io::Result<()> {
    let is_command = is_program_in_path(cmd);
    if !is_command {
        let cmd = &"ls";
        let cmd_output = Command::new(cmd)
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .wait_with_output()?;
        if !cmd_output.status.success() {
            println!("Command failed with status: {}", cmd_output.status);
        }
        let cmd_stdout = str::from_utf8(&cmd_output.stdout).expect("");
        println!("ls Stdout: {}", cmd_stdout);
    } else {
        let cmd_output = Command::new(cmd)
            .args(args)
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .wait_with_output()?;
        if !cmd_output.status.success() {
            println!("Command failed with status: {}", cmd_output.status);
        }
        let cmd_stdout = str::from_utf8(&cmd_output.stdout).expect("");
        println!("Cmd Stdout: {}", cmd_stdout);
    }
    Ok(())
}

/// Purges a location on disk, similar to `rm -rf`.
pub fn del(parent: &str, child: &str) -> io::Result<()> {
    let dir_path = format!("{}/{}", parent, child);
    println!("{}", dir_path);

    // check for errors that we're ok with
    if let Err(err) = fs::remove_dir_all(dir_path) {
        // if already gone, happy days are upon us
        if err.kind() == ErrorKind::NotFound {
            return Ok(());
        }
        // if there's a permission error, we don't care
        if err.kind() == ErrorKind::PermissionDenied {
            return Ok(());
        }
        if err.kind() == ErrorKind::Other {
            let file_path = format!("{}/{}", parent, child);
            println!("{}", file_path);
            // check for errors that we're ok with
            if let Err(err) = fs::remove_file(file_path) {
                // if already gone, happy days are upon us
                if err.kind() == ErrorKind::NotFound {
                    return Ok(());
                }

                // if there's a permission error, we don't care
                if err.kind() == ErrorKind::PermissionDenied {
                    return Ok(());
                }
                if err.kind() == ErrorKind::Other {
                    return Ok(());
                }

                // others, bad!
                // return Err(err);
                println!("{:?}", Some(err));
            }

            return Ok(());
        }

        // others, bad!
        // return Err(err);
        println!("{:?}", Some(err));
    }

    Ok(())
}
