// RUSTLINGS: REBUILD THE CHECKER | Mission 102/112 | TODO
// Author Format
// Prerequisite: author_unsolved. Target: src/dev/check.rs::check_exercises (upstream line 254).
//
// Contract: Validate the format version and combine metadata, directory, and unsolved-starter checks.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_format`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn check_exercises(info_file: &'static InfoFile, cmd_runner: &'static CmdRunner) -> Result<()> {
    // TODO: Implement this operation using the contract above.
    todo!("author_format")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(102, include_str!("author_format.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(102, include_str!("author_format.rs"));
    }
}
