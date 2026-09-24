// RUSTLINGS: REBUILD THE CHECKER | Mission 045/112 | TODO
// Canonical Paths
// Prerequisite: progress_bar. Target: src/term.rs::canonicalize (upstream line 210).
//
// Contract: Resolve a path to a Unicode canonical path; retain the existing Windows-prefix behavior and return None for failures.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint canonical_paths`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn canonicalize(path: &str) -> Option<String> {
    // TODO: Implement this operation using the contract above.
    // TODO: Complete the missing operation.
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(45, include_str!("canonical_paths.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(45, include_str!("canonical_paths.rs"));
    }
}
