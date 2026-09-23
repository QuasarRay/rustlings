// RUSTLINGS: REBUILD THE CHECKER | Mission 062/112 | TODO
// List Reset
// Prerequisite: list_index_mapping. Target: src/list/state.rs::reset_selected (upstream line 382).
//
// Contract: Reset the selected exercise and update the corresponding filtered row state.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_reset`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn reset_selected(&mut self) -> Result<()> {
        let Some(selected) = self.scroll_state.selected() else {
            self.message.push_str("Nothing selected to reset!");
            return Ok(());
        };

        let exercise_ind = self.selected_to_exercise_ind(selected)?;
        let exercise_name = self.app_state.reset_exercise_by_ind(exercise_ind)?;
        write!(
            self.message,
            "The exercise `{exercise_name}` has been reset",
        )?;
        self.update_rows();

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(62, include_str!("list_reset.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(62, include_str!("list_reset.rs"));
    }
}
