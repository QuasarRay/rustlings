// RUSTLINGS: REBUILD THE CHECKER | Mission 007/112 | TODO
// Manifest Update
// Prerequisite: manifest_targets. Target: src/cargo_toml.rs::updated_cargo_toml (upstream line 69).
//
// Contract: Replace the bin-list contents while preserving all manifest text before and after the list.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint manifest_update`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn updated_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    current_cargo_toml: &str,
    exercise_path_prefix: &[u8],
) -> Result<Vec<u8>> {
    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(current_cargo_toml)?;

    let mut updated_cargo_toml = Vec::with_capacity(BINS_BUFFER_CAPACITY);
    updated_cargo_toml.extend_from_slice(&current_cargo_toml.as_bytes()[..bins_start_ind]);
    append_bins(
        &mut updated_cargo_toml,
        exercise_infos,
        exercise_path_prefix,
    );
    updated_cargo_toml.extend_from_slice(&current_cargo_toml.as_bytes()[bins_end_ind..]);

    Ok(updated_cargo_toml)
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(7, include_str!("manifest_update.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(7, include_str!("manifest_update.rs"));
    }
}
