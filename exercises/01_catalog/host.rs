//! Additive execution policy for this heavyweight course. The original policy
//! remains in place for ordinary Rustlings courses and in the pinned export.
use anyhow::{Context, Result};
use std::{path::Path, process::Command, sync::Mutex, time::Duration};

#[path = "process.rs"]
mod process;
static COMMAND: Mutex<()> = Mutex::new(());

pub fn enabled() -> bool {
    Path::new("exercises/01_catalog/missions.tsv").is_file()
        && Path::new("exercises/01_catalog/upstream.txt").is_file()
}

pub fn run_command(
    mut cmd: Command,
    description: &str,
    cwd: Option<&str>,
    output: Option<&mut Vec<u8>>,
) -> Result<bool> {
    // Rustlings may still schedule its usual author threads. Only one of them
    // may launch a compiler/checker process at a time. Queue time is not runtime.
    let _permit = COMMAND
        .lock()
        .map_err(|_| anyhow::anyhow!("course command queue was interrupted"))?;
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    if std::env::var_os("CARGO_BUILD_JOBS").is_none() {
        cmd.env("CARGO_BUILD_JOBS", "2");
    }
    let (status, bytes) = process::capture(&mut cmd, Duration::from_secs(600))
        .with_context(|| format!("Course infrastructure could not run `{description}`"))?;
    if let Some(output) = output {
        output.extend_from_slice(&bytes);
        output.push(b'\n');
    }
    Ok(status.success())
}
