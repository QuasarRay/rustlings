// RUSTLINGS: REBUILD THE CHECKER | Mission 085/112 | TODO
// Editor Process
// Prerequisite: watch_start. Target: src/editor.rs::run_cmd (upstream line 13).
//
// Contract: Execute an editor command and preserve output/error context without silently accepting nonzero exit status.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint editor_process`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn run_cmd(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("Failed to run the command {cmd:?}"))?;

    if !output.status.success() {
        bail!(
            "The command {cmd:?} didn't run successfully\n\n\
            stdout:\n{}\n\n\
            stderr:\n{}",
            str::from_utf8(&output.stdout).unwrap_or_default(),
            str::from_utf8(&output.stderr).unwrap_or_default(),
        );
    }

    Ok(output.stdout)
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(85, include_str!("editor_process.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(85, include_str!("editor_process.rs"));
    }
}
