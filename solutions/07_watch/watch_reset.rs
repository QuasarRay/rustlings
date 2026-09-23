// RUSTLINGS: REBUILD THE CHECKER | Mission 073/112 | TODO
// Watch Reset
// Prerequisite: watch_run. Target: src/watch/state.rs::reset_exercise (upstream line 115).
//
// Contract: Restore a reset-confirmation stage and resume the event thread when the interaction completes.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_reset`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn reset_exercise(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        clear_terminal(stdout)?;

        stdout.write_all(b"Resetting will undo all your changes to the file ")?;
        stdout.write_all(self.app_state.current_exercise().path.as_bytes())?;
        stdout.write_all(b"\nReset (y/n)? ")?;
        stdout.flush()?;

        {
            let mut stdin = io::stdin().lock();
            let mut answer = [0];
            loop {
                stdin
                    .read_exact(&mut answer)
                    .context("Failed to read the user's input")?;

                match answer[0] {
                    b'y' | b'Y' => {
                        self.app_state.close_editor()?;
                        self.app_state.reset_current_exercise()?;

                        // The file watcher reruns the exercise otherwise
                        if self.manual_run {
                            self.run_current_exercise(stdout)?;
                        }
                    }
                    b'n' | b'N' => self.render(stdout)?,
                    _ => continue,
                }

                break;
            }
        }

        self.terminal_event_unpause_sender.send(())?;

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(73, include_str!("watch_reset.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(73, include_str!("watch_reset.rs"));
    }
}
