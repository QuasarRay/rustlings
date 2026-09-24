// RUSTLINGS: REBUILD THE CHECKER | Mission 044/112 | TODO
// Progress Bar
// Prerequisite: progress_counter. Target: src/term.rs::progress_bar (upstream line 145).
//
// Contract: Restore a progress-bar stage while retaining the original width calculations and completed/pending formatting.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint progress_bar`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn progress_bar<'a>(
    writer: &mut impl CountedWrite<'a>,
    progress: u32,
    total: u32,
    term_width: u16,
) -> io::Result<()> {
    const PREFIX: &[u8] = b"Progress: [";
    const PREFIX_WIDTH: u16 = PREFIX.len() as u16;
    const POSTFIX_WIDTH: u16 = "] xxx/xxx".len() as u16;
    const WRAPPER_WIDTH: u16 = PREFIX_WIDTH + POSTFIX_WIDTH;
    const MIN_LINE_WIDTH: u16 = WRAPPER_WIDTH + 4;

    debug_assert!(total <= 999);
    debug_assert!(progress <= total);

    if term_width < MIN_LINE_WIDTH {
        writer.write_ascii(b"Progress: ")?;
        // Integers are in ASCII.
        return writer.write_ascii(format!("{progress}/{total}").as_bytes());
    }

    let stdout = writer.stdout();
    stdout.write_all(PREFIX)?;


    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    stdout.queue(SetForegroundColor(Color::Green))?;
    for _ in 0..filled {
        stdout.write_all(b"#")?;
    }

    if filled < width {
        stdout.write_all(b">")?;

        let width_minus_filled = width - filled;
        if width_minus_filled > 1 {
            stdout.queue(SetForegroundColor(Color::Red))?;
            for _ in 1..width_minus_filled {
                stdout.write_all(b"-")?;
            }
        }
    }

    stdout.queue(SetForegroundColor(Color::Reset))?;

    write!(stdout, "] {progress:>3}/{total}")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(44, include_str!("progress_bar.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(44, include_str!("progress_bar.rs"));
    }
}
