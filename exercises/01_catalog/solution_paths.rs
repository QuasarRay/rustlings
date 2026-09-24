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
        let name = self.name();

        let mut path = if let Some(dir) = self.dir() {
            // 14 = 10 + 1 + 3
            // solutions/ + / + .rs
            let mut path = String::with_capacity(14 + dir.len() + name.len());
            path.push_str("solutions/");
            path.push(dir); // TODO: Append the directory text.
            path.push('/');
            path
        } else {
            // 13 = 10 + 3
            // solutions/ + .rs
            let mut path = String::with_capacity(13 + name.len());
            path.push_str("solutions/");
            path
        };

        path.push_str(name);
        path.push_str(".rs");

        path
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
