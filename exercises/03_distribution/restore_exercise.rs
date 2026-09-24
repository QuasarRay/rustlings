// RUSTLINGS: REBUILD THE CHECKER | Mission 020/112 | TODO
// Restore Exercise
// Prerequisite: materialize_exercises. Target: src/embedded.rs::write_exercise_to_disk (upstream line 112).
//
// Contract: Restore exactly one embedded exercise at a caller-validated index. Propagate filesystem write failures with context.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint restore_exercise`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn write_exercise_to_disk(&self, exercise_ind: usize, path: &str) -> Result<()> {
        // TODO: Implement this operation using the contract above.
        // TODO: Complete the missing operation.
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(20, include_str!("restore_exercise.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(20, include_str!("restore_exercise.rs"));
    }
}
