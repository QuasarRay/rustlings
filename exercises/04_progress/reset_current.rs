// RUSTLINGS: REBUILD THE CHECKER | Mission 031/112 | TODO
// Reset Current
// Prerequisite: reset_source. Target: src/app_state.rs::reset_current_exercise (upstream line 343).
//
// Contract: Mark the selected exercise pending before restoring its source.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint reset_current`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn reset_current_exercise(&mut self) -> Result<()> {
        // TODO: Implement this operation using the contract above.
        // TODO: Complete the missing operation.
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(31, include_str!("reset_current.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(31, include_str!("reset_current.rs"));
    }
}
