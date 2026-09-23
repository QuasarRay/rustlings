//! Additive execution policy for this heavyweight course. The original policy
//! remains in place for ordinary Rustlings courses and in the pinned export.
use anyhow::{Context, Result};
use std::{path::Path, process::Command, sync::Mutex, time::Duration};

#[path = "process.rs"]
mod process;
static COMMAND: Mutex<()> = Mutex::new(());

pub fn entry() -> Result<Option<std::process::ExitCode>> {
    if std::env::args().nth(1).as_deref() != Some("workshop") {
        return Ok(None);
    }
    anyhow::ensure!(enabled(), "Run workshop commands from the course root");
    std::fs::create_dir_all("target/workshop")?;
    let binary = format!("target/workshop/course{}", std::env::consts::EXE_SUFFIX);
    let mut compile = Command::new("rustc");
    compile.args([
        "--edition=2024",
        "exercises/01_catalog/course.rs",
        "-o",
        &binary,
    ]);
    let (status, output) = process::capture(&mut compile, Duration::from_secs(120))?;
    anyhow::ensure!(
        status.success(),
        "Course driver compilation failed: {}",
        String::from_utf8_lossy(&output)
    );
    let mut cmd = Command::new(binary);
    cmd.args(std::env::args().skip(2));
    let status = cmd.status().context("Could not launch course driver")?;
    Ok(Some(std::process::ExitCode::from(
        status.code().unwrap_or(2) as u8,
    )))
}

pub fn enabled() -> bool {
    Path::new("exercises/01_catalog/missions.tsv").is_file()
        && Path::new("exercises/01_catalog/upstream.txt").is_file()
}

pub fn install_lockfile() -> Result<()> {
    if enabled() {
        std::fs::copy("exercises/01_catalog/exercises.lock", "Cargo.lock")
            .context("Failed to install the course dependency lockfile")?;
    }
    Ok(())
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
