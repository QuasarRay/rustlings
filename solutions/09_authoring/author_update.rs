// RUSTLINGS: REBUILD THE CHECKER | Mission 109/112 | TODO
// Author Update
// Prerequisite: author_update_manifest. Target: src/dev/update.rs::update (upstream line 27).
//
// Contract: Update the proper manifest for official development or a community course without redesigning either workflow.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_update`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn update() -> Result<()> {
    let info_file = InfoFile::parse()?;

    if cfg!(debug_assertions) {
        // A hack to make `cargo dev update` work when developing Rustlings.
        update_cargo_toml(&info_file.exercises, "dev/Cargo.toml", b"../")
            .context("Failed to update the file `dev/Cargo.toml`")?;

        println!("Updated `dev/Cargo.toml`");
    } else {
        update_cargo_toml(&info_file.exercises, "Cargo.toml", &[])
            .context("Failed to update the file `Cargo.toml`")?;

        println!("Updated `Cargo.toml`");
    }

    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(109, include_str!("author_update.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(109, include_str!("author_update.rs"));
    }
}
