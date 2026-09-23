// RUSTLINGS: REBUILD THE CHECKER | Mission 096/112 | TODO
// App Editor Close
// Prerequisite: app_editor_join. Target: src/app_state.rs::close_editor (upstream line 567).
//
// Contract: Close the optional editor and preserve the no-editor path.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint app_editor_close`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn close_editor(&mut self) -> Result<()> {
        // TODO: Implement this operation using the contract above.
        todo!("app_editor_close")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(96, include_str!("app_editor_close.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(96, include_str!("app_editor_close.rs"));
    }
}
