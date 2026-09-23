// RUSTLINGS: REBUILD THE CHECKER | Mission 079/112 | TODO
// Watch Hint
// Prerequisite: watch_render. Target: src/watch/state.rs::show_hint (upstream line 277).
//
// Contract: Reveal a hint once and redraw only when its visibility changes.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_hint`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn show_hint(&mut self, stdout: &mut StdoutLock) -> io::Result<()> {
        // TODO: Implement this operation using the contract above.
        todo!("watch_hint")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(79, include_str!("watch_hint.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(79, include_str!("watch_hint.rs"));
    }
}
