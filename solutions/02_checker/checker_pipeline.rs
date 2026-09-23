// RUSTLINGS: REBUILD THE CHECKER | Mission 014/112 | TODO
// Checker Pipeline
// Prerequisite: runtime_failure. Target: src/exercise.rs::run (upstream line 108).
//
// Contract: Restore the build, optional tests, Clippy, and runtime sequence. Each failed gate must prevent a completion result.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint checker_pipeline`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn run<const FORCE_STRICT_CLIPPY: bool>(
        &self,
        bin_name: &str,
        mut output: Option<&mut Vec<u8>>,
        cmd_runner: &CmdRunner,
    ) -> Result<bool> {
        if let Some(output) = output.as_deref_mut() {
            output.clear();
        }

        // Input files are already written to disk during `rustlings init`.
        // We repeat it here to ensure an exercise doesn't fail because the
        // input files were deleted or modified in the meantime. Note that
        // `self.embedded_input_files()` is empty for community exercises.
        for input_file in self.embedded_input_files() {
            let name = input_file.name;
            let path = match self.dir() {
                Some(dir) => format!("exercises/{dir}/{name}"),
                None => format!("exercises/{name}"),
            };
            std::fs::write(path, input_file.content).context("failed to write input file")?;
        }

        let build_success = cmd_runner
            .cargo("build", bin_name, output.as_deref_mut())
            .run("cargo build …")?;
        if !build_success {
            return Ok(false);
        }

        // Discard the compiler output because it will be shown again by `cargo test` or Clippy.
        if let Some(output) = output.as_deref_mut() {
            output.clear();
        }

        let cwd_buf;
        let cwd = if let Some(dir) = self.dir() {
            cwd_buf = format!("exercises/{dir}");
            cwd_buf.as_str()
        } else {
            "exercises"
        };

        if self.test() {
            let output_is_some = output.is_some();
            let mut test_cmd = cmd_runner.cargo("test", bin_name, output.as_deref_mut());
            if output_is_some {
                test_cmd.args(["--", "--color", "always", "--format", "pretty"]);
            }
            let test_success = test_cmd.run("cargo test …")?;
            if !test_success {
                run_bin(bin_name, cwd, output, cmd_runner)?;
                return Ok(false);
            }

            // Discard the compiler output because it will be shown again by Clippy.
            if let Some(output) = output.as_deref_mut() {
                output.clear();
            }
        }

        let mut clippy_cmd = cmd_runner.cargo("clippy", bin_name, output.as_deref_mut());

        // `--profile test` is required to also check code with `#[cfg(test)]`.
        if FORCE_STRICT_CLIPPY || self.strict_clippy() {
            clippy_cmd.args(["--profile", "test", "--", "-D", "warnings"]);
        } else {
            clippy_cmd.args(["--profile", "test"]);
        }

        let clippy_success = clippy_cmd.run("cargo clippy …")?;
        let run_success = run_bin(bin_name, cwd, output, cmd_runner)?;

        Ok(clippy_success && run_success)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(14, include_str!("checker_pipeline.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(14, include_str!("checker_pipeline.rs"));
    }
}
