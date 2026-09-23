// RUSTLINGS: REBUILD THE CHECKER | Mission 056/112 | TODO
// List Model
// Prerequisite: scroll_viewport. Target: src/list/state.rs::build (upstream line 56).
//
// Contract: Construct list state from application progress and the terminal dimensions, preserving row-index mappings.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_model`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn build(app_state: &'a mut AppState, stdout: &mut StdoutLock) -> Result<Self> {
        stdout.queue(Clear(ClearType::All))?;

        let name_col_title_len = 4;
        let path_col_title_len = 4;
        let (name_col_width, path_col_width) = app_state.exercises().iter().fold(
            (name_col_title_len, path_col_title_len),
            |(name_col_width, path_col_width), exercise| {
                (
                    name_col_width.max(exercise.name.len()),
                    path_col_width.max(exercise.path.len()),
                )
            },
        );
        let name_col_padding = vec![b' '; name_col_width + COL_SPACING];
        let path_col_padding = vec![b' '; path_col_width];

        let filter = Filter::None;
        let n_rows_with_filter = app_state.exercises().len();
        let selected = app_state.current_exercise_ind();


        // TODO: Restore this step using the contract above.
        todo!("list_model");

        let mut slf = Self {
            message: String::with_capacity(128),
            search_query: String::new(),
            app_state,
            scroll_state,
            name_col_padding,
            path_col_padding,
            filter,
            // Set by `set_term_size`
            term_width: 0,
            term_height: 0,
            show_footer: true,
        };

        slf.set_term_size(width, height);
        slf.draw(stdout)?;

        Ok(slf)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(56, include_str!("list_model.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(56, include_str!("list_model.rs"));
    }
}
