// RUSTLINGS: REBUILD THE CHECKER | Mission 098/112 | TODO
// Author Manifest
// Prerequisite: author_names. Target: src/dev/check.rs::check_cargo_toml (upstream line 30).
//
// Contract: Generate the expected binary registration and compare it against the manifest without modifying that manifest.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_manifest`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn check_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    cargo_toml_path: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let current_cargo_toml = fs::read_to_string(cargo_toml_path)
        .with_context(|| format!("Failed to read the file `{cargo_toml_path}`"))?;

    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(&current_cargo_toml)?;


    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    if old_bins != new_bins {
        if cfg!(debug_assertions) {
            bail!(
                "The file `dev/Cargo.toml` is outdated. Run `cargo dev update` to update it. Then run `cargo run -- dev check` again"
            );
        }

        bail!(
            "The file `Cargo.toml` is outdated. Run `rustlings dev update` to update it. Then run `rustlings dev check` again"
        );
    }

    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(98, include_str!("author_manifest.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(98, include_str!("author_manifest.rs"));
    }
}
