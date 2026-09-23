// RUSTLINGS: REBUILD THE CHECKER | Mission 104/112 | TODO
// Author Gate
// Prerequisite: author_solutions. Target: src/dev/check.rs::check (upstream line 379).
//
// Contract: Enforce catalog-size and manifest constraints before validating starters and solutions.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_gate`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn check(require_solutions: bool) -> Result<()> {
    // TODO: Implement this operation using the contract above.
    todo!("author_gate")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(104, include_str!("author_gate.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(104, include_str!("author_gate.rs"));
    }
}
