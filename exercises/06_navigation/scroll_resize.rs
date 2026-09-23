// RUSTLINGS: REBUILD THE CHECKER | Mission 053/112 | TODO
// Scroll Resize
// Prerequisite: scroll_previous. Target: src/list/scroll_state.rs::set_n_rows (upstream line 76).
//
// Contract: Clear selection for an empty list and clamp a previous selection after rows are removed.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint scroll_resize`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn set_n_rows(&mut self, n_rows: usize) {
        // TODO: Implement this operation using the contract above.
        todo!("scroll_resize")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(53, include_str!("scroll_resize.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(53, include_str!("scroll_resize.rs"));
    }
}
