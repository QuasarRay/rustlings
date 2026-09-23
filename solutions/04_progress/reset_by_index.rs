// RUSTLINGS: REBUILD THE CHECKER | Mission 032/112 | TODO
// Reset By Index
// Prerequisite: reset_current. Target: src/app_state.rs::reset_exercise_by_ind (upstream line 350).
//
// Contract: Validate the index, persist pending status, restore the corresponding file, and return its name.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint reset_by_index`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn reset_exercise_by_ind(&mut self, exercise_ind: usize) -> Result<&'static str> {
        if exercise_ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        self.set_pending(exercise_ind)?;
        let exercise = &self.exercises[exercise_ind];
        self.reset(exercise_ind, exercise.path)?;

        Ok(exercise.name)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(32, include_str!("reset_by_index.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(32, include_str!("reset_by_index.rs"));
    }
}
