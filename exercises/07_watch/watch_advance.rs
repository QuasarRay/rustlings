// RUSTLINGS: REBUILD THE CHECKER | Mission 076/112 | TODO
// Watch Advance
// Prerequisite: watch_completion. Target: src/watch/state.rs::next_exercise (upstream line 174).
//
// Contract: Advance through AppState only when the current exercise is complete; otherwise report CurrentPending.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_advance`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn next_exercise(&mut self, stdout: &mut StdoutLock) -> Result<ExercisesProgress> {
        // TODO: Implement this operation using the contract above.
        todo!("watch_advance")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(76, include_str!("watch_advance.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(76, include_str!("watch_advance.rs"));
    }
}
