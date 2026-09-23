// RUSTLINGS: REBUILD THE CHECKER | Mission 072/112 | TODO
// Watch Run
// Prerequisite: watch_model. Target: src/watch/state.rs::run_current_exercise (upstream line 76).
//
// Contract: Run the selected exercise, record its output and done status, coordinate the editor, and render the result.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_run`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn run_current_exercise(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        // Ignore any input until running the exercise is done.
        let _input_pause_guard = InputPauseGuard::scoped_pause();

        writeln!(
            stdout,
            "\nChecking the exercise `{}`. Please wait…",
            self.app_state.current_exercise().name,
        )?;

        let editor_handle = self.app_state.open_editor()?;

        self.show_hint = false;

        let success = self
            .app_state
            .current_exercise()
            .run_exercise(Some(&mut self.output), self.app_state.cmd_runner())?;
        self.output.push(b'\n');

        self.done_status = if success {
            if let Some(solution_path) = self.app_state.current_solution_path()? {
                DoneStatus::DoneWithSolution(solution_path)
            } else {
                DoneStatus::DoneWithoutSolution
            }
        } else {
            self.app_state
                .set_pending(self.app_state.current_exercise_ind())?;

            DoneStatus::Pending
        };

        self.app_state.join_editor_handle(editor_handle)?;
        self.render(stdout)?;

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(72, include_str!("watch_run.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(72, include_str!("watch_run.rs"));
    }
}
