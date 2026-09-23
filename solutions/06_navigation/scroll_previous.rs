// RUSTLINGS: REBUILD THE CHECKER | Mission 052/112 | BUG
// Scroll Previous
// Prerequisite: scroll_next. Target: src/list/scroll_state.rs::select_previous (upstream line 58).
//
// Contract: Move one row backward without underflow; an absent selection remains absent.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint scroll_previous`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn select_previous(&mut self) {
        if let Some(selected) = self.selected {
            self.set_selected(selected.saturating_sub(1));
        }
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(52, include_str!("scroll_previous.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(52, include_str!("scroll_previous.rs"));
    }
}
