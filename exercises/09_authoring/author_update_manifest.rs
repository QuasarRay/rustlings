// RUSTLINGS: REBUILD THE CHECKER | Mission 108/112 | TODO
// Author Update Manifest
// Prerequisite: author_new_course. Target: src/dev/update.rs::update_cargo_toml (upstream line 10).
//
// Contract: Regenerate only the bin list and preserve the rest of the existing manifest.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_update_manifest`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn update_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    cargo_toml_path: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    // TODO: Implement this operation using the contract above.
    // TODO: Complete the missing operation.
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(108, include_str!("author_update_manifest.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(108, include_str!("author_update_manifest.rs"));
    }
}
