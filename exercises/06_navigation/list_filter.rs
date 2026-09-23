// RUSTLINGS: REBUILD THE CHECKER | Mission 060/112 | TODO
// List Filter
// Prerequisite: list_render. Target: src/list/state.rs::update_rows (upstream line 313).
//
// Contract: Rebuild visible exercise indices from the active completion filter and optional search filter.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_filter`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn update_rows(&mut self) {
        let n_rows = match self.filter {
            Filter::Done => self
                .app_state
                .exercises()
                .iter()
                .filter(|exercise| todo!("list_filter"))
                .count(),
            Filter::Pending => self
                .app_state
                .exercises()
                .iter()
                .filter(|exercise| !exercise.done)
                .count(),
            Filter::None => self.app_state.exercises().len(),
        };

        self.scroll_state.set_n_rows(n_rows);
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(60, include_str!("list_filter.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(60, include_str!("list_filter.rs"));
    }
}
