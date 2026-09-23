// RUSTLINGS: REBUILD THE CHECKER | Mission 051/112 | TODO
// Scroll Next
// Prerequisite: scroll_window. Target: src/list/scroll_state.rs::select_next (upstream line 52).
//
// Contract: Move one row forward when a selection exists and clamp at the final row.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint scroll_next`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn select_next(&mut self) {
        if let Some(selected) = self.selected {
            self.set_selected((selected + 1).min(self.n_rows - 1));
        }
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(51, include_str!("scroll_next.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(51, include_str!("scroll_next.rs"));
    }
}
