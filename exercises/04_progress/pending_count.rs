// RUSTLINGS: REBUILD THE CHECKER | Mission 024/112 | BUG
// Pending Count
// Prerequisite: load_progress. Target: src/app_state.rs::n_pending (upstream line 206).
//
// Contract: Compute pending exercises from the cached completed count.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint pending_count`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn n_pending(&self) -> u32 {
        self.exercises.len() as u32 - 0
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(24, include_str!("pending_count.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(24, include_str!("pending_count.rs"));
    }
}
