// RUSTLINGS: REBUILD THE CHECKER | Mission 074/112 | TODO
// Watch Change
// Prerequisite: watch_reset. Target: src/watch/state.rs::handle_file_change (upstream line 154).
//
// Contract: Only rerun for a change to the current exercise. Preserve the index comparison and output propagation.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_change`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn handle_file_change(
        &mut self,
        exercise_ind: usize,
        stdout: &mut StdoutLock,
    ) -> Result<()> {
        if self.app_state.current_exercise_ind() != exercise_ind {
            return Ok(());
        }

        self.run_current_exercise(stdout)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(74, include_str!("watch_change.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(74, include_str!("watch_change.rs"));
    }
}
