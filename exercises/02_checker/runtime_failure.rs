// RUSTLINGS: REBUILD THE CHECKER | Mission 013/112 | BUG
// Runtime Failure
// Prerequisite: command_tail. Target: src/exercise.rs::run_bin (upstream line 38).
//
// Contract: Keep runtime failure visible in the collected output and return failure even when compilation succeeded.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint runtime_failure`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn run_bin(
    bin_name: &str,
    cwd: &str,
    mut output: Option<&mut Vec<u8>>,
    cmd_runner: &CmdRunner,
) -> Result<bool> {
    if let Some(output) = output.as_deref_mut() {
        write_ansi(output, SetAttribute(Attribute::Underlined));
        output.extend_from_slice(b"Output");
        write_ansi(output, ResetColor);
        output.push(b'\n');
    }

    let success = cmd_runner.run_debug_bin(bin_name, cwd, output.as_deref_mut())?;

    if let Some(output) = output
        && !success
    {
        // This output is important to show the user that something went wrong.
        // Otherwise, calling something like `exit(1)` in an exercise without further output
        // leaves the user confused about why the exercise isn't done yet.
        write_ansi(output, SetAttribute(Attribute::Bold));
        write_ansi(output, SetForegroundColor(Color::Red));
        output.extend_from_slice(b"The exercise didn't run successfully (nonzero exit code)");
        write_ansi(output, ResetColor);
        output.push(b'\n');
    }

    Ok(true)
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(13, include_str!("runtime_failure.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(13, include_str!("runtime_failure.rs"));
    }
}
