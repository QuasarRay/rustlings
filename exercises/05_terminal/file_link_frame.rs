// RUSTLINGS: REBUILD THE CHECKER | Mission 046/112 | TODO
// File Link Frame
// Prerequisite: canonical_paths. Target: src/term.rs::file_path (upstream line 226).
//
// Contract: Style a file path using the caller-provided writer and restore terminal attributes afterward.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint file_link_frame`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn file_path<'a, W: CountedWrite<'a>>(
    writer: &mut W,
    color: Color,
    f: impl FnOnce(&mut W) -> io::Result<()>,
) -> io::Result<()> {
    // TODO: Implement this operation using the contract above.
    todo!("file_link_frame")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(46, include_str!("file_link_frame.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(46, include_str!("file_link_frame.rs"));
    }
}
