// RUSTLINGS: REBUILD THE CHECKER | Mission 039/112 | TODO
// Single Run Result
// Prerequisite: finish_message. Target: src/run.rs::run (upstream line 16).
//
// Contract: Restore the single-exercise command, including pending status on failure and advancing after successful validation.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint single_run_result`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn run(app_state: &mut AppState) -> Result<ExitCode> {
    let exercise = app_state.current_exercise();
    let mut output = Vec::with_capacity(OUTPUT_CAPACITY);
    let success = exercise.run_exercise(Some(&mut output), app_state.cmd_runner())?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&output)?;

    if !success {
        app_state.set_pending(app_state.current_exercise_ind())?;

        stdout.write_all(b"Ran ")?;
        app_state
            .current_exercise()
            .terminal_file_link(&mut stdout, app_state.emit_file_links())?;
        stdout.write_all(b" with errors\n")?;

        return Ok(ExitCode::FAILURE);
    }


    // TODO: Restore this step using the contract above.
    todo!("single_run_result");

    if let Some(solution_path) = app_state.current_solution_path()? {
        stdout.write_all(b"\n")?;
        solution_link_line(&mut stdout, &solution_path, app_state.emit_file_links())?;
        stdout.write_all(b"\n")?;
    }

    match app_state.done_current_exercise::<false>(&mut stdout)? {
        ExercisesProgress::NewPending | ExercisesProgress::CurrentPending => {
            stdout.write_all(b"Next exercise: ")?;
            app_state
                .current_exercise()
                .terminal_file_link(&mut stdout, app_state.emit_file_links())?;
            stdout.write_all(b"\n")?;
        }
        ExercisesProgress::AllDone => (),
    }

    Ok(ExitCode::SUCCESS)
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(39, include_str!("single_run_result.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(39, include_str!("single_run_result.rs"));
    }
}
