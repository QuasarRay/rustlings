// RUSTLINGS: REBUILD THE CHECKER | Mission 054/112 | BUG
// Scroll Padding
// Prerequisite: scroll_resize. Target: src/list/scroll_state.rs::update_scroll_padding (upstream line 87).
//
// Contract: Limit padding to one quarter of the visible rows and the configured maximum.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint scroll_padding`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn update_scroll_padding(&mut self) {
        self.scroll_padding = (self.max_n_rows_to_display / 4).min(self.max_scroll_padding);
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(54, include_str!("scroll_padding.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(54, include_str!("scroll_padding.rs"));
    }
}
