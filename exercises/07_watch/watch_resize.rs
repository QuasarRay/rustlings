// RUSTLINGS: REBUILD THE CHECKER | Mission 081/112 | TODO
// Watch Resize
// Prerequisite: watch_check_all. Target: src/watch/state.rs::update_term_width (upstream line 305).
//
// Contract: Redraw after an actual terminal-width change and avoid redundant redraws.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_resize`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn update_term_width(&mut self, width: u16, stdout: &mut StdoutLock) -> io::Result<()> {
        // TODO: Implement this operation using the contract above.
        // TODO: Complete the missing operation.
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(81, include_str!("watch_resize.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(81, include_str!("watch_resize.rs"));
    }
}
