// RUSTLINGS: REBUILD THE CHECKER | Mission 095/112 | TODO
// App Editor Join
// Prerequisite: app_editor_start. Target: src/app_state.rs::join_editor_handle (upstream line 561).
//
// Contract: Join the startup task and put the resulting editor back into AppState.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint app_editor_join`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn join_editor_handle(&mut self, handle: EditorJoinHandle) -> Result<()> {
        self.editor = handle.join()?;

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(95, include_str!("app_editor_join.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(95, include_str!("app_editor_join.rs"));
    }
}
