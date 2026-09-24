// RUSTLINGS: REBUILD THE CHECKER | Mission 078/112 | TODO
// Watch Render
// Prerequisite: watch_prompt. Target: src/watch/state.rs::render (upstream line 216).
//
// Contract: Restore one stage of the existing result screen: diagnostics, hints, solution link, progress, and current path.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_render`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn render(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        // Prevent having the first line shifted if clearing wasn't successful.
        stdout.write_all(b"\n")?;
        clear_terminal(stdout)?;

        stdout.write_all(&self.output)?;

        if self.show_hint {
            stdout
                .queue(SetAttributes(HEADING_ATTRIBUTES))?
                .queue(SetForegroundColor(Color::Cyan))?;
            stdout.write_all(b"Hint")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n")?;

            stdout.write_all(
                self.app_state
                    .current_exercise()
                    .hint
                    .as_bytes()
                    .trim_ascii(),
            )?;
            stdout.write_all(b"\n\n")?;
        }


        // TODO: Restore this step using the contract above.
        // TODO: Complete the missing operation.
        progress_bar(
            stdout,
            self.app_state.n_done(),
            self.app_state.exercises().len() as u32,
            self.term_width,
        )?;

        stdout.write_all(b"\nCurrent exercise: ")?;
        self.app_state
            .current_exercise()
            .terminal_file_link(stdout, self.app_state.emit_file_links())?;
        stdout.write_all(b"\n\n")?;

        self.show_prompt(stdout)?;

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(78, include_str!("watch_render.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(78, include_str!("watch_render.rs"));
    }
}
