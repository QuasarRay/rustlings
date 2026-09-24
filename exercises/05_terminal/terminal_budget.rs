// RUSTLINGS: REBUILD THE CHECKER | Mission 040/112 | TODO
// Terminal Budget
// Prerequisite: single_run_result. Target: src/term.rs::new (upstream line 19).
//
// Contract: Initialize a bounded terminal writer with zero consumed width and the requested maximum.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint terminal_budget`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn new(stdout: &'a mut StdoutLock<'lock>, max_len: usize) -> Self {
        // TODO: Implement this operation using the contract above.
        // TODO: Complete the missing operation.
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(40, include_str!("terminal_budget.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(40, include_str!("terminal_budget.rs"));
    }
}
