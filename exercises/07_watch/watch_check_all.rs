// RUSTLINGS: REBUILD THE CHECKER | Mission 080/112 | TODO
// Watch Check All
// Prerequisite: watch_hint. Target: src/watch/state.rs::check_all_exercises (upstream line 286).
//
// Contract: Run all checks while input is paused; select a newly pending exercise or display the final message.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_check_all`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn check_all_exercises(&mut self, stdout: &mut StdoutLock) -> Result<ExercisesProgress> {
        // TODO: Implement this operation using the contract above.
        todo!("watch_check_all")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(80, include_str!("watch_check_all.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(80, include_str!("watch_check_all.rs"));
    }
}
