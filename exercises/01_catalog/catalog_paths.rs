// RUSTLINGS: REBUILD THE CHECKER | Mission 001/112 | TODO
// Catalog Paths
// Prerequisite: none. Target: src/info_file.rs::path (upstream line 38).
//
// Contract: Build an exercise path for both a root-level file and a directory. Preserve the exercises/ prefix and .rs suffix.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint catalog_paths`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn path(&self) -> String {
        // TODO: Implement this operation using the contract above.
        todo!("catalog_paths")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(1, include_str!("catalog_paths.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(1, include_str!("catalog_paths.rs"));
    }
}
