// RUSTLINGS: REBUILD THE CHECKER | Mission 059/112 | TODO
// List Render
// Prerequisite: list_rows. Target: src/list/state.rs::draw (upstream line 199).
//
// Contract: Restore one stage of the existing list screen; retain search input, filters, help keys, and terminal-size handling.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_render`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn draw(&mut self, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.term_height == 0 {
            return Ok(());
        }

        stdout.queue(BeginSynchronizedUpdate)?.queue(MoveTo(0, 0))?;

        // Header
        let mut writer = MaxLenWriter::new(stdout, self.term_width as usize);
        writer.write_ascii(b"  Current  State    Name")?;
        writer.write_ascii(&self.name_col_padding[4..])?;
        writer.write_ascii(b"Path")?;
        next_ln(stdout)?;

        // Rows
        let iter = self.app_state.exercises().iter().enumerate();
        let n_displayed_rows = match self.filter {
            Filter::Done => self.draw_rows(stdout, iter.filter(|(_, exercise)| exercise.done))?,
            Filter::Pending => {
                self.draw_rows(stdout, iter.filter(|(_, exercise)| !exercise.done))?
            }
            Filter::None => self.draw_rows(stdout, iter)?,
        };

        for _ in 0..self.scroll_state.max_n_rows_to_display() - n_displayed_rows {
            next_ln(stdout)?;
        }

        if self.show_footer {
            progress_bar(
                &mut MaxLenWriter::new(stdout, self.term_width as usize),
                self.app_state.n_done(),
                self.app_state.exercises().len() as u32,
                self.term_width,
            )?;
            next_ln(stdout)?;

            let mut writer = MaxLenWriter::new(stdout, self.term_width as usize);
            if self.message.is_empty() {
                // Help footer message
                if self.scroll_state.selected().is_some() {
                    writer.write_str("↓/")?;
                    hotkey(&mut writer, b"j")?;
                    writer.write_str(" ↑/")?;
                    hotkey(&mut writer, b"k")?;
                    writer.write_ascii(b" home/")?;
                    hotkey(&mut writer, b"g")?;
                    writer.write_ascii(b" end/")?;
                    hotkey(&mut writer, b"G")?;
                    writer.write_str(" | ↩️/")?;
                    hotkey(&mut writer, b"c")?;
                    writer.write_ascii(b"ontinue at | ")?;
                    hotkey(&mut writer, b"r")?;
                    writer.write_ascii(b"eset exercise")?;
                    next_ln(stdout)?;
                    writer = MaxLenWriter::new(stdout, self.term_width as usize);

                    hotkey(&mut writer, b"s")?;
                    writer.write_ascii(b"earch | filter ")?;
                } else {
                    // Nothing selected (and nothing shown), so only display filter and quit.
                    writer.write_ascii(b"filter ")?;
                }

                match self.filter {
                    Filter::Done => {
                        writer.stdout.queue(SetAttribute(Attribute::Underlined))?;
                        hotkey(&mut writer, b"d")?;
                        writer
                            .stdout
                            .queue(SetForegroundColor(Color::Magenta))?
                            .queue(SetAttribute(Attribute::Underlined))?;
                        writer.write_str("one")?;
                        writer.stdout.queue(ResetColor)?;
                        writer.write_ascii(b"/")?;
                        hotkey(&mut writer, b"p")?;
                        writer.write_ascii(b"ending")?;
                    }
                    Filter::Pending => {
                        hotkey(&mut writer, b"d")?;
                        writer.write_ascii(b"one/")?;
                        writer.stdout.queue(SetAttribute(Attribute::Underlined))?;
                        hotkey(&mut writer, b"p")?;
                        writer
                            .stdout
                            .queue(SetForegroundColor(Color::Magenta))?
                            .queue(SetAttribute(Attribute::Underlined))?;
                        writer.write_ascii(b"ending")?;
                        writer.stdout.queue(ResetColor)?;
                    }
                    Filter::None => {
                        hotkey(&mut writer, b"d")?;
                        writer.write_ascii(b"one/")?;
                        hotkey(&mut writer, b"p")?;
                        writer.write_ascii(b"ending")?;
                    }
                }

                writer.write_ascii(b" | ")?;
                hotkey(&mut writer, b"q")?;
                writer.write_ascii(b"uit list")?;
            } else {
                writer.stdout.queue(SetForegroundColor(Color::Magenta))?;
                writer.write_str(&self.message)?;
                stdout.queue(ResetColor)?;
                next_ln(stdout)?;
            }

            next_ln(stdout)?;
        }

        stdout.queue(EndSynchronizedUpdate)?.flush()
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(59, include_str!("list_render.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(59, include_str!("list_render.rs"));
    }
}
