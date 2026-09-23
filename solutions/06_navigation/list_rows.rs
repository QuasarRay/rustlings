// RUSTLINGS: REBUILD THE CHECKER | Mission 058/112 | TODO
// List Rows
// Prerequisite: list_resize. Target: src/list/state.rs::draw_rows (upstream line 140).
//
// Contract: Restore one stage of row rendering: selection, completion status, file names, and clipping must remain synchronized.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_rows`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn draw_rows(
        &self,
        stdout: &mut StdoutLock,
        filtered_exercises: impl Iterator<Item = (usize, &'a Exercise)>,
    ) -> io::Result<usize> {
        let current_exercise_ind = self.app_state.current_exercise_ind();
        let row_offset = self.scroll_state.offset();
        let mut n_displayed_rows = 0;

        for (exercise_ind, exercise) in filtered_exercises
            .skip(row_offset)
            .take(self.scroll_state.max_n_rows_to_display())
        {
            let mut writer = MaxLenWriter::new(stdout, self.term_width as usize);

            if self.scroll_state.selected() == Some(row_offset + n_displayed_rows) {
                // The crab emoji has the width of two ascii chars.
                writer.add_to_len(2);
                writer.stdout.write_all("🦀".as_bytes())?;
                writer
                    .stdout
                    .queue(SetAttributes(SELECTED_ROW_ATTRIBUTES))?;
            } else {
                writer.write_ascii(b"  ")?;
            }

            if exercise_ind == current_exercise_ind {
                writer.stdout.queue(SetForegroundColor(Color::Red))?;
                writer.write_ascii(b">>>>>>>  ")?;
            } else {
                writer.write_ascii(b"         ")?;
            }

            if exercise.done {
                writer.stdout.queue(SetForegroundColor(Color::Green))?;
                writer.write_ascii(b"DONE   ")?;
            } else {
                writer.stdout.queue(SetForegroundColor(Color::Yellow))?;
                writer.write_ascii(b"PENDING")?;
            }
            writer.stdout.queue(SetForegroundColor(Color::Reset))?;
            writer.write_ascii(b"  ")?;

            self.draw_exercise_name(&mut writer, exercise)?;

            writer.write_ascii(&self.name_col_padding[exercise.name.len()..])?;

            exercise.terminal_file_link(&mut writer, self.app_state.emit_file_links())?;

            writer.write_ascii(&self.path_col_padding[exercise.path.len()..])?;

            next_ln(stdout)?;
            stdout.queue(ResetColor)?;
            n_displayed_rows += 1;
        }

        Ok(n_displayed_rows)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(58, include_str!("list_rows.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(58, include_str!("list_rows.rs"));
    }
}
