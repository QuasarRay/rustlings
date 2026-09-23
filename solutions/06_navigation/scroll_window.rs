// RUSTLINGS: REBUILD THE CHECKER | Mission 050/112 | TODO
// Scroll Window
// Prerequisite: scroll_initial. Target: src/list/scroll_state.rs::update_offset (upstream line 26).
//
// Contract: Keep the selection visible with padding, then clamp the viewport against the total row count.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint scroll_window`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn update_offset(&mut self) {
        let Some(selected) = self.selected else {
            return;
        };

        let min_offset = (selected + self.scroll_padding)
            .saturating_sub(self.max_n_rows_to_display.saturating_sub(1));
        let max_offset = selected.saturating_sub(self.scroll_padding);
        let global_max_offset = self.n_rows.saturating_sub(self.max_n_rows_to_display);

        self.offset = self
            .offset
            .max(min_offset)
            .min(max_offset)
            .min(global_max_offset);
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(50, include_str!("scroll_window.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(50, include_str!("scroll_window.rs"));
    }
}
