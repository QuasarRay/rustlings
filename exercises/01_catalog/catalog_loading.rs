// RUSTLINGS: REBUILD THE CHECKER | Mission 003/112 | TODO
// Catalog Loading
// Prerequisite: catalog_defaults. Target: src/info_file.rs::parse (upstream line 104).
//
// Contract: Prefer a local community catalog. Fall back to the embedded catalog only for NotFound; reject malformed or empty local catalogs.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint catalog_loading`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn parse() -> Result<Self> {
        // Read a local `info.toml` if it exists.
        let slf = match fs::read_to_string("info.toml") {
            Ok(file_content) => {
                // LEAKING: The info file is used until the end of the program.
                toml::de::from_str::<Self>(file_content.leak())
                    .context("Failed to parse the `info.toml` file")?
            }
            Err(e) => {
                if todo!("catalog_loading") {
                    return toml::de::from_str(EMBEDDED_FILES.info_file)
                        .context("Failed to parse the embedded `info.toml` file");
                }

                return Err(Error::from(e).context("Failed to read the `info.toml` file"));
            }
        };

        if slf.exercises.is_empty() {
            bail!("{NO_EXERCISES_ERR}");
        }

        Ok(slf)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(3, include_str!("catalog_loading.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(3, include_str!("catalog_loading.rs"));
    }
}
