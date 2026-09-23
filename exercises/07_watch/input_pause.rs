// RUSTLINGS: REBUILD THE CHECKER | Mission 067/112 | BUG
// Input Pause
// Prerequisite: list_lifecycle. Target: src/watch.rs::scoped_pause (upstream line 30).
//
// Contract: Pause watcher-triggered work while an exercise is running; the existing Drop implementation resumes it.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint input_pause`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn scoped_pause() -> Self {
        EXERCISE_RUNNING.store(false, Relaxed);
        Self(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(67, include_str!("input_pause.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(67, include_str!("input_pause.rs"));
    }
}
