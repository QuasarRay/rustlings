// RUSTLINGS: REBUILD THE CHECKER | Mission 037/112 | TODO
// Advance Progress
// Prerequisite: save_check_all. Target: src/app_state.rs::done_current_exercise (upstream line 508).
//
// Contract: Complete the current exercise, choose the next pending item, and run the final regression check before declaring the course finished.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint advance_progress`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn done_current_exercise<const CLEAR_BEFORE_FINAL_CHECK: bool>(
        &mut self,
        stdout: &mut StdoutLock,
    ) -> Result<ExercisesProgress> {
        let exercise = &mut self.exercises[self.current_exercise_ind];
        if !exercise.done {
            exercise.done = true;
            self.n_done += 1;
        }

        if let Some(ind) = self.next_pending_exercise_ind() {
            self.set_current_exercise_ind(ind)?;
            return Ok(ExercisesProgress::NewPending);
        }

        if CLEAR_BEFORE_FINAL_CHECK {
            clear_terminal(stdout)?;
        } else {
            stdout.write_all(b"\n")?;
        }

        if let Some(first_pending_exercise_ind) = self.check_all_exercises(stdout)? {
            self.set_current_exercise_ind(first_pending_exercise_ind)?;

            return Ok(ExercisesProgress::NewPending);
        }

        self.render_final_message(stdout)?;

        Ok(ExercisesProgress::AllDone)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(37, include_str!("advance_progress.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(37, include_str!("advance_progress.rs"));
    }
}
