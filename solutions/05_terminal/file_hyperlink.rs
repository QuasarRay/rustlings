// RUSTLINGS: REBUILD THE CHECKER | Mission 047/112 | TODO
// File Hyperlink
// Prerequisite: file_link_frame. Target: src/term.rs::terminal_file_link (upstream line 246).
//
// Contract: Emit the original terminal hyperlink sequence around the displayed file path.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint file_hyperlink`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn terminal_file_link<'a>(
    writer: &mut impl CountedWrite<'a>,
    path: &str,
    canonical_path: &str,
) -> io::Result<()> {
    writer.stdout().write_all(b"\x1b]8;;file://")?;
    writer.stdout().write_all(canonical_path.as_bytes())?;
    writer.stdout().write_all(b"\x1b\\")?;
    // Only this part is visible.
    writer.write_str(path)?;
    writer.stdout().write_all(b"\x1b]8;;\x1b\\")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(47, include_str!("file_hyperlink.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(47, include_str!("file_hyperlink.rs"));
    }
}
