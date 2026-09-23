// RUSTLINGS: REBUILD THE CHECKER | Mission 093/112 | TODO
// Pane Close
// Prerequisite: pane_status. Target: src/editor/zellij.rs::close_pane (upstream line 45).
//
// Contract: Pass the pane ID as a separate argument to the original Zellij close-pane command.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint pane_close`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn close_pane(pane_id: &str) -> Result<()> {
    // TODO: Implement this operation using the contract above.
    todo!("pane_close")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(93, include_str!("pane_close.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(93, include_str!("pane_close.rs"));
    }
}
