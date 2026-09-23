// RUSTLINGS: REBUILD THE CHECKER | Mission 010/112 | BUG
// Process Status
// Prerequisite: cargo_arguments. Target: src/cmd.rs::run_cmd (upstream line 16).
//
// Contract: Return the actual child exit status in both captured and discarded-output modes. Preserve merged output and the 30-second deadline.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint process_status`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn run_cmd(
    mut cmd: Command,
    description: &str,
    cwd: Option<&str>,
    output: Option<&mut Vec<u8>>,
) -> Result<bool> {
    let spawn = |mut cmd: Command| {
        // The closure drops `cmd` which prevents a pipe deadlock.
        cmd.stdin(Stdio::null())
            .spawn()
            .with_context(|| format!("Failed to run `{description}`"))
    };
    let wait = |handle: &mut Child| {
        handle
            .wait_timeout(Duration::from_secs(TIMEOUT_SECS))
            .with_context(|| format!("Failed to wait on `{description}` to exit"))
    };
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }

    let mut handle = if let Some(output) = output {
        let (mut reader, writer) =
            pipe().with_context(|| format!("Failed to create a pipe to run `{description}``"))?;

        let writer_clone = writer
            .try_clone()
            .with_context(|| format!("Failed to clone the pipe writer for `{description}`"))?;

        cmd.stdout(writer_clone).stderr(writer);
        let mut handle = spawn(cmd)?;

        let thread_handle = thread::Builder::new()
            .spawn(move || {
                let mut out = Vec::with_capacity(128);
                reader.read_to_end(&mut out).map(|_| out)
            })
            .context("Failed to spawn a thread to collect a command's output")?;

        if let Some(status) = wait(&mut handle)? {
            let out = thread_handle
                .join()
                .unwrap()
                .with_context(|| format!("Failed to read the output of `{description}`"))?;
            output.extend_from_slice(&out);
            output.push(b'\n');
            return Ok(status.success());
        }

        handle
    } else {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        let mut handle = spawn(cmd)?;

        if let Some(status) = wait(&mut handle)? {
            return Ok(status.success());
        }

        handle
    };

    handle
        .kill()
        .with_context(|| format!("Failed to kill `{description}` after timeout"))?;
    bail!("`{description}` timed out after {TIMEOUT_SECS} seconds");
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(10, include_str!("process_status.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(10, include_str!("process_status.rs"));
    }
}
