// RUSTLINGS: REBUILD THE CHECKER | Mission 103/112 | TODO
// Author Solutions
// Prerequisite: author_format. Target: src/dev/check.rs::check_solutions (upstream line 284).
//
// Contract: Restore one solution-validation stage, preserving required-solution handling, strict Clippy, formatting, and unexpected-file checks.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_solutions`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn check_solutions(
    require_solutions: bool,
    info_file: &'static InfoFile,
    cmd_runner: &'static CmdRunner,
) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(b"Running all solutions...\n")?;

    let handles = info_file
        .exercises
        .iter()
        .map(|exercise_info| {
            thread::Builder::new().spawn(move || {
                let sol_path = exercise_info.sol_path();
                if !Path::new(&sol_path).exists() {
                    if require_solutions {
                        return SolutionCheck::Err(anyhow!(
                            "The solution of the exercise {} is missing",
                            exercise_info.name,
                        ));
                    }

                    return SolutionCheck::MissingOptional;
                }

                let mut output = Vec::with_capacity(OUTPUT_CAPACITY);
                match exercise_info.run_solution(Some(&mut output), cmd_runner) {
                    Ok(true) => SolutionCheck::Success { sol_path },
                    Ok(false) => SolutionCheck::RunFailure { output },
                    Err(e) => SolutionCheck::Err(e),
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to spawn a thread to check a solution")?;

    let mut sol_paths = HashSet::with_capacity(info_file.exercises.len());
    let mut fmt_cmd = Command::new("rustfmt");
    fmt_cmd
        .arg("--check")
        .arg("--edition")
        .arg("2024")
        .arg("--color")
        .arg("always")
        .stdin(Stdio::null());


    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    for (exercise_info, handle) in info_file.exercises.iter().zip(handles) {
        let Ok(check_result) = handle.join() else {
            bail!(
                "Panic while trying to run the solution of the exercise {}",
                exercise_info.name,
            );
        };

        match check_result {
            SolutionCheck::Success { sol_path } => {
                fmt_cmd.arg(&sol_path);
                sol_paths.insert(PathBuf::from(sol_path));
            }
            SolutionCheck::MissingOptional => (),
            SolutionCheck::RunFailure { output } => {
                drop(progress_counter);
                stdout.write_all(&output)?;
                bail!(
                    "Running the solution of the exercise {} failed with the error above",
                    exercise_info.name,
                );
            }
            SolutionCheck::Err(e) => return Err(e),
        }

        progress_counter.increment()?;
    }

    let n_solutions = sol_paths.len();
    let handle = thread::Builder::new()
        .spawn(move || check_unexpected_files("solutions", &sol_paths))
        .context(
            "Failed to spawn a thread to check for unexpected files in the solutions directory",
        )?;

    if n_solutions > 0
        && !fmt_cmd
            .status()
            .context("Failed to run `rustfmt` on all solution files")?
            .success()
    {
        bail!("Some solutions aren't formatted. Run `rustfmt` on them");
    }

    handle.join().unwrap()
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(103, include_str!("author_solutions.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(103, include_str!("author_solutions.rs"));
    }
}
