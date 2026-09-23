// RUSTLINGS: REBUILD THE CHECKER | Mission 008/112 | TODO
// Cargo Metadata
// Prerequisite: manifest_update. Target: src/cmd.rs::build (upstream line 94).
//
// Contract: Ask Cargo for its target_directory, check process success, and parse the JSON response with useful error context.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint cargo_metadata`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn build() -> Result<Self> {
        // Get the target directory from Cargo.
        let metadata_output = Command::new("cargo")
            .arg("metadata")
            .arg("-q")
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps")
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .output()
            .context(CARGO_METADATA_ERR)?;

        if !metadata_output.status.success() {
            bail!("`cargo metadata …` failed. Are you in the `rustlings/` directory?");
        }

        let metadata: CargoMetadata = serde_json::de::from_slice(&metadata_output.stdout).context(
            "Failed to read the field `target_directory` from the output of `cargo metadata …`",
        )?;

        Ok(Self {
            target_dir: metadata.target_directory,
        })
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(8, include_str!("cargo_metadata.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(8, include_str!("cargo_metadata.rs"));
    }
}
