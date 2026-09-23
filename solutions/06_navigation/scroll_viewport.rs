// RUSTLINGS: REBUILD THE CHECKER | Mission 055/112 | TODO
// Scroll Viewport
// Prerequisite: scroll_padding. Target: src/list/scroll_state.rs::set_max_n_rows_to_display (upstream line 95).
//
// Contract: Update the viewport size, recompute padding, and then make the selected row visible.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint scroll_viewport`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn set_max_n_rows_to_display(&mut self, max_n_rows_to_display: usize) {
        self.max_n_rows_to_display = max_n_rows_to_display;
        self.update_scroll_padding();
        self.update_offset();
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(55, include_str!("scroll_viewport.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(55, include_str!("scroll_viewport.rs"));
    }
}
