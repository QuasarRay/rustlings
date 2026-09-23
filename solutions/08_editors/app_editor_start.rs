// RUSTLINGS: REBUILD THE CHECKER | Mission 094/112 | TODO
// App Editor Start
// Prerequisite: pane_close. Target: src/app_state.rs::open_editor (upstream line 553).
//
// Contract: Coordinate the application current-exercise path with the optional editor startup task.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint app_editor_start`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn open_editor(&mut self) -> Result<EditorJoinHandle> {
        if let Some(editor) = self.editor.take() {
            return editor.open(self.current_exercise_ind, self.current_exercise().path);
        }

        Ok(EditorJoinHandle::default())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(94, include_str!("app_editor_start.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(94, include_str!("app_editor_start.rs"));
    }
}
