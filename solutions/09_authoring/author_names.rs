// RUSTLINGS: REBUILD THE CHECKER | Mission 097/112 | BUG
// Author Names
// Prerequisite: app_editor_close. Target: src/dev/check.rs::forbidden_char (upstream line 25).
//
// Contract: Permit alphanumeric characters and underscores in exercise names. Report the first forbidden character.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_names`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn forbidden_char(input: &str) -> Option<char> {
    input.chars().find(|c| !c.is_alphanumeric() && *c != '_')
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(97, include_str!("author_names.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(97, include_str!("author_names.rs"));
    }
}
