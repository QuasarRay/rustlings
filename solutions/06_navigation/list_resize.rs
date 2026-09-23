// RUSTLINGS: REBUILD THE CHECKER | Mission 057/112 | TODO
// List Resize
// Prerequisite: list_model. Target: src/list/state.rs::set_term_size (upstream line 100).
//
// Contract: Update the rendering dimensions and the scroll viewport for the available rows.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_resize`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn set_term_size(&mut self, width: u16, height: u16) {
        self.term_width = width;
        self.term_height = height;

        if height == 0 {
            return;
        }

        let header_height = 1;
        // 1 progress bar, 2 footer message lines.
        let footer_height = 3;
        self.show_footer = height > header_height + footer_height;

        self.scroll_state.set_max_n_rows_to_display(
            height.saturating_sub(header_height + u16::from(self.show_footer) * footer_height)
                as usize,
        );
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(57, include_str!("list_resize.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(57, include_str!("list_resize.rs"));
    }
}
