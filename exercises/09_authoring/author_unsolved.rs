// RUSTLINGS: REBUILD THE CHECKER | Mission 101/112 | TODO
// Author Unsolved
// Prerequisite: author_file_inventory. Target: src/dev/check.rs::check_exercises_unsolved (upstream line 206).
//
// Contract: Run every non-exempt starter and reject an already solved exercise; propagate worker failures.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_unsolved`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn check_exercises_unsolved(
    info_file: &'static InfoFile,
    cmd_runner: &'static CmdRunner,
) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(b"Running all exercises to check that they aren't already solved...\n")?;


    // TODO: Restore this step using the contract above.
    todo!("author_unsolved");

    let mut progress_counter = ProgressCounter::new(&mut stdout, handles.len())?;

    for (exercise_name, handle) in handles {
        let Ok(result) = handle.join() else {
            bail!("Panic while trying to run the exercise {exercise_name}");
        };

        match result {
            Ok(true) => {
                bail!(
                    "The exercise {exercise_name} is already solved.\n\
                     {SKIP_CHECK_UNSOLVED_HINT}",
                )
            }
            Ok(false) => (),
            Err(e) => return Err(e),
        }

        progress_counter.increment()?;
    }

    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(101, include_str!("author_unsolved.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(101, include_str!("author_unsolved.rs"));
    }
}
