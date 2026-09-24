// RUSTLINGS: REBUILD THE CHECKER | Mission 089/112 | TODO
// Editor Close
// Prerequisite: editor_launch. Target: src/editor.rs::close (upstream line 117).
//
// Contract: Close managed Zellij panes while retaining the original behavior for ordinary command-based editors.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint editor_close`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn close(&mut self) -> Result<()> {
        // TODO: Implement this operation using the contract above.
        // TODO: Complete the missing operation.
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(89, include_str!("editor_close.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(89, include_str!("editor_close.rs"));
    }
}
