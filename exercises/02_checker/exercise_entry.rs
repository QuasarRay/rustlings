// RUSTLINGS: REBUILD THE CHECKER | Mission 015/112 | TODO
// Exercise Entry
// Prerequisite: checker_pipeline. Target: src/exercise.rs::run_exercise (upstream line 186).
//
// Contract: Delegate through the common pipeline with normal Clippy policy and the exercise name.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint exercise_entry`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn run_exercise(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        // TODO: Implement this operation using the contract above.
        todo!("exercise_entry")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(15, include_str!("exercise_entry.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(15, include_str!("exercise_entry.rs"));
    }
}
