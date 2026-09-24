// RUSTLINGS: REBUILD THE CHECKER | Mission 088/112 | TODO
// Editor Launch
// Prerequisite: editor_selection. Target: src/editor.rs::open (upstream line 74).
//
// Contract: Restore one launch stage, preserving asynchronous startup and the editor-specific exercise path arguments.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint editor_launch`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn open(
        mut self,
        exercise_ind: usize,
        exercise_path: &'static str,
    ) -> Result<EditorJoinHandle> {
        let handle = thread::Builder::new()
            .spawn(move || {
                match &mut self {
                    Editor::Cmd(program, args) => {
                        // TODO: Complete the missing operation.
                    }
                    Editor::Zellij(open_pane) => {
                        if let Some((pane_id_str, pane_id, open_exercise_ind)) = open_pane {
                            if *open_exercise_ind == exercise_ind {
                                if zellij::pane_open(*pane_id)? {
                                    return Ok(self);
                                }
                            } else {
                                zellij::close_pane(pane_id_str)?;
                            }
                        }

                        let stdout = run_cmd(
                            Command::new("zellij")
                                .arg("action")
                                .arg("edit")
                                .arg(exercise_path),
                        )?;

                        let (pane_id_str, pane_id) = zellij::parse_pane_id(&stdout)
                            .context("Failed to parse the ID of the new Zellij pane")?;

                        *open_pane = Some((pane_id_str, pane_id, exercise_ind));
                    }
                }

                Ok(self)
            })
            .context("Failed to spawn a thread to open the editor")?;

        Ok(EditorJoinHandle(Some(handle)))
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(88, include_str!("editor_launch.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(88, include_str!("editor_launch.rs"));
    }
}
