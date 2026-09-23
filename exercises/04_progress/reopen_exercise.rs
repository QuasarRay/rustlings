// RUSTLINGS: REBUILD THE CHECKER | Mission 029/112 | TODO
// Reopen Exercise
// Prerequisite: completion_count. Target: src/app_state.rs::set_pending (upstream line 306).
//
// Contract: Persist only when changing a completed exercise back to pending.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint reopen_exercise`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn set_pending(&mut self, exercise_ind: usize) -> Result<()> {
        // TODO: Implement this operation using the contract above.
        todo!("reopen_exercise")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(29, include_str!("reopen_exercise.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(29, include_str!("reopen_exercise.rs"));
    }
}
