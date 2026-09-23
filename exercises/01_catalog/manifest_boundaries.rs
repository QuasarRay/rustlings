// RUSTLINGS: REBUILD THE CHECKER | Mission 005/112 | BUG
// Manifest Boundaries
// Prerequisite: solution_paths. Target: src/cargo_toml.rs::bins_start_end_ind (upstream line 13).
//
// Contract: Locate only the contents of bin = [...]. Report missing delimiters; offsets are UTF-8 byte offsets.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint manifest_boundaries`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn bins_start_end_ind(cargo_toml: &str) -> Result<(usize, usize)> {
    let start_ind = cargo_toml
        .find("bin = [")
        .context("Failed to find the start of the `bin` list (`bin = [`)")?
        + 6;
    let end_ind = start_ind
        + cargo_toml
            .get(start_ind..)
            .and_then(|slice| slice.as_bytes().iter().position(|c| *c == b']'))
            .context("Failed to find the end of the `bin` list (`]`)")?;

    Ok((start_ind, end_ind))
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(5, include_str!("manifest_boundaries.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(5, include_str!("manifest_boundaries.rs"));
    }
}
