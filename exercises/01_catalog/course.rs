//! Portable course maintenance entry point. Compile with Rust 1.88+; doctor
//! checks the separate Rust 1.89 minimum before compiling the verifier.
#![forbid(unsafe_code)]
use std::{error::Error, fs, path::PathBuf, process::Command, time::Duration};
mod process;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn run(cmd: &mut Command) -> Result<()> {
    println!("Running {cmd:?}");
    if std::env::var_os("CARGO_BUILD_JOBS").is_none() {
        cmd.env("CARGO_BUILD_JOBS", "2");
    }
    // This supervisor also owns cleanup if the entire author gate stalls.
    let (status, output) = process::capture(cmd, Duration::from_secs(3600))?;
    use std::io::Write;
    std::io::stdout().write_all(&output)?;
    if !status.success() {
        return Err(format!("INFRA_ERROR: {cmd:?} exited with {status}").into());
    }
    Ok(())
}

fn doctor() -> Result<()> {
    let (status, output) = process::capture(
        Command::new("rustc").arg("--version"),
        Duration::from_secs(10),
    )?;
    let version = String::from_utf8(output)?;
    let numbers: Vec<u32> = version
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .split('.')
        .take(2)
        .filter_map(|n| n.parse().ok())
        .collect();
    if !status.success() || numbers.as_slice() < [1, 89].as_slice() {
        return Err("The restoration course requires Rust 1.89 or newer. The pinned original engine separately supports 1.88. Run rustup update stable.".into());
    }
    for (program, args) in [
        ("cargo", vec!["--version"]),
        ("cargo", vec!["clippy", "--version"]),
        ("rustfmt", vec!["--version"]),
        ("git", vec!["--version"]),
    ] {
        let (status, output) =
            process::capture(Command::new(program).args(args), Duration::from_secs(15))?;
        if !status.success() {
            return Err(format!("Missing course tool {program}: {}. Install Rust components with rustup component add clippy rustfmt.", String::from_utf8_lossy(&output)).into());
        }
    }
    if std::path::Path::new("dev/Cargo.toml").is_file() {
        if fs::read("dev/Cargo.lock")? != fs::read("exercises/01_catalog/exercises.lock")? {
            return Err("dev/Cargo.lock and the distributed exercises.lock differ; update both intentionally".into());
        }
    }
    println!(
        "Course preflight passed: Rust 1.89+, Cargo, Clippy, rustfmt, Git, and lockfile consistency."
    );
    Ok(())
}

fn grader() -> Result<PathBuf> {
    fs::create_dir_all("target/workshop")?;
    let binary = PathBuf::from(format!(
        "target/workshop/grader-cli{}",
        std::env::consts::EXE_SUFFIX
    ));
    run(Command::new("rustc")
        .args([
            "--edition=2024",
            "-Dwarnings",
            "exercises/01_catalog/grader.rs",
            "-o",
        ])
        .arg(&binary))?;
    Ok(binary)
}

fn audit(binary: &PathBuf) -> Result<()> {
    run(Command::new(binary).args([".", "audit"]))?;
    run(Command::new(binary).args([".", "mutations"]))?;
    run(Command::new("cargo").args(["test", "--locked", "--workspace"]))?;
    run(Command::new("cargo").args([
        "run",
        "--locked",
        "--",
        "dev",
        "check",
        "--require-solutions",
    ]))
}

fn main() -> Result<()> {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "doctor".into());
    if mode == "hint" {
        let name = std::env::args()
            .nth(2)
            .ok_or("usage: workshop hint MISSION [1|2|3]")?;
        let level: usize = std::env::args()
            .nth(3)
            .unwrap_or_else(|| "1".into())
            .parse()?;
        if !(1..=3).contains(&level) {
            return Err("Hint level must be 1, 2, or 3".into());
        }
        let hints = fs::read_to_string("exercises/01_catalog/hints.tsv")?;
        let fields: Vec<_> = hints
            .lines()
            .find(|line| line.split('\t').next() == Some(name.as_str()))
            .ok_or("Unknown mission name")?
            .split('\t')
            .collect();
        let mut chars = fields.get(level).ok_or("Missing hint level")?.chars();
        let mut text = String::new();
        while let Some(c) = chars.next() {
            text.push(if c == '\\' {
                match chars.next() {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('\\') => '\\',
                    _ => return Err("Invalid hint escape".into()),
                }
            } else {
                c
            });
        }
        println!("{name} — hint {level}/3\n{text}");
        return Ok(());
    }
    if mode == "info" {
        println!(
            "Rebuild the Checker course 1 (QuasarRay/rustlings)\n112 restoration missions; course Rust >=1.89; original engine Rust >=1.88\nReference: a650509c789da1656f813392b16aa1fa043b7f3e\nDistribution: source checkout; not the upstream crates.io curriculum"
        );
        return Ok(());
    }
    doctor()?;
    if mode == "doctor" {
        return Ok(());
    }
    let binary = grader()?;
    match mode.as_str() {
        "trace" => {
            run(Command::new(&binary)
                .args([".", "trace"])
                .args(std::env::args().skip(2)))?;
        }
        "prepare" => {
            run(Command::new(&binary).args([".", "prepare"]))?;
            println!(
                "Ready. Start the fork with --no-editor; in its source checkout use cargo run -- --no-editor."
            );
        }
        "audit" => audit(&binary)?,
        "mutations" => {
            run(Command::new(&binary)
                .args([".", "mutations"])
                .args(std::env::args().skip(2)))?;
        }
        "smoke" => {
            run(Command::new(&binary).args([".", "prepare"]))?;
            let (status, output) = process::capture(
                Command::new(&binary).args([
                    ".",
                    "check-file",
                    "1",
                    "solutions",
                    "exercises/01_catalog/catalog_paths.rs",
                ]),
                Duration::from_secs(600),
            )?;
            if status.code() != Some(1) {
                return Err(format!(
                    "Starter was not rejected as a learner failure: {}",
                    String::from_utf8_lossy(&output)
                )
                .into());
            }
            for (id, file) in [
                ("1", "solutions/01_catalog/catalog_paths.rs"),
                ("112", "solutions/10_launch/launch_dispatch.rs"),
            ] {
                run(Command::new(&binary).args([".", "check-file", id, "solutions", file]))?;
            }
            let destination = std::env::temp_dir().join(format!(
                "rustlings-reference-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_nanos()
            ));
            run(Command::new(&binary)
                .args([".", "export"])
                .arg(&destination)
                .arg("solutions"))?;
            let result = run(Command::new("cargo")
                .args(["test", "--locked", "--workspace"])
                .current_dir(&destination));
            fs::remove_dir_all(destination)?;
            result?;
        }
        "release" => {
            run(&mut Command::new("typos"))?;
            run(Command::new("cargo").args([
                "clippy",
                "--locked",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ]))?;
            run(Command::new("cargo").args(["fmt", "--all", "--check"]))?;
            audit(&binary)?;
            run(Command::new("rustup").args([
                "run", "1.89.0", "cargo", "run", "--locked", "--", "workshop", "audit",
            ]))?;
        }
        _ => {
            return Err(
                "usage: course doctor|info|hint MISSION [LEVEL]|trace MISSION [DEPTH]|prepare|audit|mutations [FIRST LAST]|smoke|release".into(),
            );
        }
    }
    Ok(())
}
