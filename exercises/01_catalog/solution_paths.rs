// RUSTLINGS: REBUILD THE CHECKER | Mission 004/112 | TODO
// Solution Paths
// Prerequisite: catalog_loading. Target: src/exercise.rs::sol_path (upstream line 201).
//
// Contract: Produce the matching solutions/ path without losing the optional directory or file extension.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint solution_paths`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn sol_path(&self) -> String {
        // TODO: Implement this operation using the contract above.
        todo!("solution_paths")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(4, include_str!("solution_paths.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(4, include_str!("solution_paths.rs"));
    }
}
