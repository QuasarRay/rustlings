//! Course adapter. This is fixture infrastructure, not a learner repair.
use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn run(mission: usize, source: &str) {
    let cwd = std::env::current_dir().expect("read working directory");
    let root = cwd
        .ancestors()
        .find(|p| p.join("exercises/01_catalog/grader.rs").is_file())
        .expect("run from the initialized course or its repository");
    let support = root.join("exercises/01_catalog");
    let mut hash = DefaultHasher::new();
    for input in ["grader.rs", "process.rs"] {
        fs::read(support.join(input))
            .expect("read course verifier")
            .hash(&mut hash);
    }
    Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .expect("Rust is required")
        .stdout
        .hash(&mut hash);
    let cache = root.join("target/workshop");
    fs::create_dir_all(&cache).expect("create course build cache");
    let grader = cache.join(format!(
        "grader-{:x}{}",
        hash.finish(),
        std::env::consts::EXE_SUFFIX
    ));
    if !grader.is_file() {
        let temporary = PathBuf::from(format!(
            "{}.{}{}",
            grader.display(),
            std::process::id(),
            std::env::consts::EXE_SUFFIX
        ));
        let status = Command::new("rustc")
            .args(["--edition=2024", "-C", "debuginfo=0"])
            .arg(support.join("grader.rs"))
            .arg("-o")
            .arg(&temporary)
            .status()
            .expect("Rust is required to compile the course verifier");
        assert!(status.success(), "course verifier did not compile");
        if fs::rename(&temporary, &grader).is_err() {
            assert!(grader.is_file(), "could not install course verifier");
            let _ = fs::remove_file(temporary);
        }
    }
    let role = if env!("CARGO_BIN_NAME").ends_with("_sol") {
        "solutions"
    } else {
        "exercises"
    };
    let mut cmd = Command::new(grader);
    cmd.arg(root);
    let preparing = std::env::args().nth(1).as_deref() == Some("--prepare");
    if preparing {
        cmd.arg("prepare");
    } else {
        cmd.args(["check", &mission.to_string(), role]);
    }
    let mut child = cmd
        .stdin(if preparing {
            Stdio::null()
        } else {
            Stdio::piped()
        })
        .spawn()
        .expect("start verifier");
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(source.as_bytes()).expect("send repair");
    }
    let status = child.wait().expect("wait for verifier");
    if !status.success() {
        let reason = match status.code() {
            Some(1) => "REJECTED: repair the mission using the contract failure above",
            Some(3) => "LOCKED: recheck the named prerequisite before this mission",
            Some(4) => {
                "INFRA_TIMEOUT: the check did not judge your repair; retry when resources are available"
            }
            _ => "INFRA_ERROR: the verifier could not judge your repair; read the diagnostic above",
        };
        // The child already emitted the real compiler/test diagnostic. A panic
        // here would add a second, misleading backtrace into the course adapter.
        eprintln!("{reason}");
        std::process::exit(status.code().unwrap_or(2));
    }
}
