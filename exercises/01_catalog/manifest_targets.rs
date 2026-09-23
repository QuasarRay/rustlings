// RUSTLINGS: REBUILD THE CHECKER | Mission 006/112 | TODO
// Manifest Targets
// Prerequisite: manifest_boundaries. Target: src/cargo_toml.rs::append_bins (upstream line 29).
//
// Contract: Register each exercise and only solutions that exist. Respect path prefixes and preserve catalog order.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint manifest_targets`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn append_bins(
    buf: &mut Vec<u8>,
    exercise_infos: &[ExerciseInfo],
    exercise_path_prefix: &[u8],
) {
    // TODO: Implement this operation using the contract above.
    todo!("manifest_targets")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(6, include_str!("manifest_targets.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(6, include_str!("manifest_targets.rs"));
    }
}
