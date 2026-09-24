// RUSTLINGS: REBUILD THE CHECKER | Mission 063/112 | TODO
// List Search
// Prerequisite: list_reset. Target: src/list/state.rs::apply_search_query (upstream line 399).
//
// Contract: Apply the original substring search to exercise names and synchronize the displayed rows and selection.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_search`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn apply_search_query(&mut self) {
        self.message.push_str("search:");
        self.message.push_str(&self.search_query);
        self.message.push('|');

        if self.search_query.is_empty() {
            return;
        }


        // TODO: Restore this step using the contract above.
        // TODO: Complete the missing operation.
        match ind {
            Some(exercise_ind) => self.scroll_state.set_selected(exercise_ind),
            None => self.message.push_str(" (not found)"),
        }
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(63, include_str!("list_search.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(63, include_str!("list_search.rs"));
    }
}
