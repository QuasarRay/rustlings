// RUSTLINGS: REBUILD THE CHECKER | Mission 075/112 | BUG
// Watch Completion
// Prerequisite: watch_change. Target: src/watch/state.rs::done (upstream line 166).
//
// Contract: Both done variants permit advancing; Pending does not.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_completion`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn done(&self) -> bool {
        match self.done_status {
            DoneStatus::DoneWithSolution(_) | DoneStatus::DoneWithoutSolution => true,
            DoneStatus::Pending => false,
        }
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(75, include_str!("watch_completion.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(75, include_str!("watch_completion.rs"));
    }
}
