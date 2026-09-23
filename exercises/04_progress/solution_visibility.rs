// RUSTLINGS: REBUILD THE CHECKER | Mission 034/112 | TODO
// Solution Visibility
// Prerequisite: next_pending. Target: src/app_state.rs::current_solution_path (upstream line 384).
//
// Contract: Preserve debug-build behavior; materialize official solutions and only expose existing community solutions.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint solution_visibility`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn current_solution_path(&self) -> Result<Option<String>> {
        if cfg!(debug_assertions) {
            return Ok(None);
        }

        let current_exercise = self.current_exercise();

        if self.official_exercises {
            EMBEDDED_FILES
                .write_solution_to_disk(self.current_exercise_ind, current_exercise.name)
                .map(Some)
        } else {
            let sol_path = todo!("solution_visibility");

            if Path::new(&sol_path).exists() {
                return Ok(Some(sol_path));
            }

            Ok(None)
        }
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(34, include_str!("solution_visibility.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(34, include_str!("solution_visibility.rs"));
    }
}
