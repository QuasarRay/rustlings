//! Course subprocess ownership, shared by the host extension and verifier.
use std::{
    fs::{self, File},
    io,
    process::{Child, Command, ExitStatus, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

// Kill descendants as well as the process group: a nested tool can create its
// own group. Collect before terminating the parent so PPIDs are still usable.
fn terminate(child: &mut Child) {
    #[cfg(unix)]
    {
        let mut descendants = vec![child.id()];
        if let Ok(out) = Command::new("ps").args(["-axo", "pid=,ppid="]).output() {
            let table: Vec<(u32, u32)> = String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|line| {
                    let mut fields = line.split_whitespace();
                    Some((fields.next()?.parse().ok()?, fields.next()?.parse().ok()?))
                })
                .collect();
            let mut index = 0;
            while index < descendants.len() {
                let parent = descendants[index];
                for &(pid, ppid) in &table {
                    if ppid == parent && !descendants.contains(&pid) {
                        descendants.push(pid);
                    }
                }
                index += 1;
            }
        }
        for pid in descendants.into_iter().rev() {
            let _ = Command::new("kill")
                .args(["-KILL", "--", &format!("-{pid}")])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            let _ = Command::new("kill")
                .args(["-KILL", "--", &pid.to_string()])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/F", "/T", "/PID", &child.id().to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

pub fn capture(cmd: &mut Command, budget: Duration) -> io::Result<(ExitStatus, Vec<u8>)> {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let nonce = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "rustlings-command-{}-{nonce}-{}.log",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    struct Log(std::path::PathBuf);
    impl Drop for Log {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }
    let log = Log(path);
    let file = File::create_new(&log.0)?;
    cmd.stdin(Stdio::null())
        .stdout(file.try_clone()?)
        .stderr(file);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let mut child = cmd.spawn()?;
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok((status, fs::read(&log.0)?)),
            Ok(None) => {}
            Err(error) => {
                terminate(&mut child);
                return Err(error);
            }
        }
        if start.elapsed() >= budget {
            terminate(&mut child);
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                format!(
                    "INFRA_TIMEOUT: command exceeded {} seconds; its process tree was terminated.\n{}",
                    budget.as_secs(),
                    String::from_utf8_lossy(&fs::read(&log.0)?)
                ),
            ));
        }
        thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_failure_output() {
        let (status, output) = capture(
            Command::new("rustc").arg("--definitely-not-a-rustc-option"),
            Duration::from_secs(10),
        )
        .unwrap();
        assert!(!status.success());
        assert!(!output.is_empty());
    }
    #[cfg(unix)]
    #[test]
    fn deadline_kills_a_grandchild_without_waiting_for_inherited_pipes() {
        let pid_path =
            std::env::temp_dir().join(format!("rustlings-grandchild-{}", std::process::id()));
        let mut cmd = Command::new("sh");
        cmd.args(["-c", "sleep 60 & echo $! > \"$1\"; wait", "sh"])
            .arg(&pid_path);
        let start = Instant::now();
        let error = capture(&mut cmd, Duration::from_millis(300)).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        assert!(start.elapsed() < Duration::from_secs(5));
        let pid = fs::read_to_string(&pid_path).unwrap();
        let out = Command::new("ps")
            .args(["-o", "stat=", "-p", pid.trim()])
            .output()
            .unwrap();
        let state = String::from_utf8_lossy(&out.stdout);
        assert!(
            state.trim().is_empty() || state.trim().starts_with('Z'),
            "descendant is still running: {state}"
        );
        fs::remove_file(pid_path).unwrap();
    }
}
