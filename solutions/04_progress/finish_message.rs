// RUSTLINGS: REBUILD THE CHECKER | Mission 038/112 | TODO
// Finish Message
// Prerequisite: advance_progress. Target: src/app_state.rs::render_final_message (upstream line 540).
//
// Contract: Render the configured completion message and progress through the original terminal output path.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint finish_message`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn render_final_message(&self, stdout: &mut StdoutLock) -> Result<()> {
        clear_terminal(stdout)?;
        stdout.write_all(FINISH_LINE.as_bytes())?;

        let final_message = self.final_message.as_bytes().trim_ascii();
        if !final_message.is_empty() {
            stdout.write_all(final_message)?;
            stdout.write_all(b"\n")?;
        }

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(38, include_str!("finish_message.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(38, include_str!("finish_message.rs"));
    }
}
