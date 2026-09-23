// RUSTLINGS: REBUILD THE CHECKER | Mission 028/112 | BUG
// Completion Count
// Prerequisite: select_name. Target: src/app_state.rs::set_status (upstream line 285).
//
// Contract: Make repeated status updates idempotent. Completing increments the count; reopening decrements it.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint completion_count`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn set_status(&mut self, exercise_ind: usize, done: bool) -> Result<bool> {
        let exercise = self
            .exercises
            .get_mut(exercise_ind)
            .context(BAD_INDEX_ERR)?;

        if exercise.done == done {
            return Ok(false);
        }

        exercise.done = done;
        if done {
            self.n_done += 1;
        } else {
            self.n_done += 1;
        }

        Ok(true)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(28, include_str!("completion_count.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(28, include_str!("completion_count.rs"));
    }
}
