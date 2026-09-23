// RUSTLINGS: REBUILD THE CHECKER | Mission 049/112 | BUG
// Scroll Initial
// Prerequisite: ansi_buffer. Target: src/list/scroll_state.rs::new (upstream line 11).
//
// Contract: Initialize scroll state without subtracting below zero when the selected row is close to the start.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint scroll_initial`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn new(n_rows: usize, selected: Option<usize>, max_scroll_padding: usize) -> Self {
        Self {
            n_rows,
            max_n_rows_to_display: 0,
            selected,
            offset: selected.map_or(0, |selected| selected.saturating_add(max_scroll_padding)),
            scroll_padding: 0,
            max_scroll_padding,
        }
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(49, include_str!("scroll_initial.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(49, include_str!("scroll_initial.rs"));
    }
}
