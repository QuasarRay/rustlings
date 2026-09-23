// RUSTLINGS: REBUILD THE CHECKER | Mission 064/112 | TODO
// List Open
// Prerequisite: list_search. Target: src/list/state.rs::selected_to_current_exercise (upstream line 427).
//
// Contract: Resolve a selected visible row, make its exercise current, and indicate whether the list should exit.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_open`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn selected_to_current_exercise(&mut self) -> Result<bool> {
        let Some(selected) = self.scroll_state.selected() else {
            self.message.push_str("Nothing selected to continue at!");
            return Ok(false);
        };

        let exercise_ind = self.selected_to_exercise_ind(selected)?;
        self.app_state.set_current_exercise_ind(exercise_ind)?;

        Ok(true)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(64, include_str!("list_open.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(64, include_str!("list_open.rs"));
    }
}
