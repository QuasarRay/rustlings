// RUSTLINGS: REBUILD THE CHECKER | Mission 048/112 | TODO
// Ansi Buffer
// Prerequisite: file_hyperlink. Target: src/term.rs::write_ansi (upstream line 259).
//
// Contract: Append an ANSI command to a byte buffer through the original formatting adapter.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint ansi_buffer`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn write_ansi(output: &mut Vec<u8>, command: impl Command) {
    // TODO: Implement this operation using the contract above.
    todo!("ansi_buffer")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(48, include_str!("ansi_buffer.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(48, include_str!("ansi_buffer.rs"));
    }
}
