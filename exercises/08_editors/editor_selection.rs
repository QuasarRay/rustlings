// RUSTLINGS: REBUILD THE CHECKER | Mission 087/112 | TODO
// Editor Selection
// Prerequisite: editor_discovery. Target: src/editor.rs::new (upstream line 48).
//
// Contract: Choose the configured editor, VS Code, or Zellij according to the original precedence and argument parsing.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint editor_selection`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn new(cmd: Option<String>, vs_code_term: bool) -> Result<Option<Self>> {
        // TODO: Implement this operation using the contract above.
        todo!("editor_selection")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(87, include_str!("editor_selection.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(87, include_str!("editor_selection.rs"));
    }
}
