// RUSTLINGS: REBUILD THE CHECKER | Mission 090/112 | TODO
// Editor Join
// Prerequisite: editor_close. Target: src/editor.rs::join (upstream line 136).
//
// Contract: Join an optional editor startup task and propagate its returned editor or error.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint editor_join`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn join(self) -> Result<Option<Editor>> {
        if let Some(handle) = self.0 {
            let editor = handle.join().unwrap()?;
            return Ok(Some(editor));
        }

        Ok(None)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(90, include_str!("editor_join.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(90, include_str!("editor_join.rs"));
    }
}
