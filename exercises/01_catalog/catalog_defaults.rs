// RUSTLINGS: REBUILD THE CHECKER | Mission 002/112 | BUG
// Catalog Defaults
// Prerequisite: catalog_paths. Target: src/info_file.rs::default_true (upstream line 32).
//
// Contract: Missing test metadata must enable tests. Missing strict_clippy metadata must remain false.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint catalog_defaults`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
const fn default_false() -> bool {
    true
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(2, include_str!("catalog_defaults.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(2, include_str!("catalog_defaults.rs"));
    }
}
