// RUSTLINGS: REBUILD THE CHECKER | Mission 033/112 | BUG
// Next Pending
// Prerequisite: reset_by_index. Target: src/app_state.rs::next_pending_exercise_ind (upstream line 363).
//
// Contract: Search after the current exercise first, then wrap to the beginning. Translate suffix-local positions back to catalog indices.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint next_pending`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn next_pending_exercise_ind(&self) -> Option<usize> {
        let next_ind = self.current_exercise_ind + 1;
        self.exercises
            // If the exercise done isn't the last, search for pending exercises after it.
            .get(next_ind..)
            .and_then(|later_exercises| {
                later_exercises
                    .iter()
                    .position(|exercise| !exercise.done)
                    .map(|ind| ind)
            })
            // Search from the start.
            .or_else(|| {
                self.exercises[..self.current_exercise_ind]
                    .iter()
                    .position(|exercise| !exercise.done)
            })
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(33, include_str!("next_pending.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(33, include_str!("next_pending.rs"));
    }
}
