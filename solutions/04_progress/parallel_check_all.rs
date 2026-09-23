// RUSTLINGS: REBUILD THE CHECKER | Mission 035/112 | TODO
// Parallel Check All
// Prerequisite: solution_visibility. Target: src/app_state.rs::check_all_exercises_impl (upstream line 406).
//
// Contract: Restore one stage of the parallel check-all coordinator. Keep the worker limit, result collection, status counts, and earliest pending result consistent.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint parallel_check_all`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn check_all_exercises_impl(&mut self, stdout: &mut StdoutLock) -> Result<Option<usize>> {
        let mut progress_visualizer = CheckProgressVisualizer::build(stdout, self.exercises.len())?;

        let next_exercise_ind = &AtomicUsize::new(0);
        let mut progresses = vec![None; self.exercises.len()];

        thread::scope(|s| {
            let (progress_sender, progress_receiver) = mpsc::channel();
            let n_threads = thread::available_parallelism()
                .map_or(DEFAULT_CHECK_PARALLELISM, |count| count.get());

            for _ in 0..n_threads {
                let progress_sender = progress_sender.clone();
                let slf = &self;
                thread::Builder::new()
                    .spawn_scoped(s, move || {
                        loop {
                            let exercise_ind = next_exercise_ind.fetch_add(1, Relaxed);
                            let Some(exercise) = slf.exercises.get(exercise_ind) else {
                                // No more exercises.
                                break;
                            };

                            if let Ok(success) = exercise.run_exercise(None, &slf.cmd_runner) {
                                let progress = if success {
                                    CheckProgress::Done
                                } else {
                                    CheckProgress::Pending
                                };

                                if progress_sender.send((exercise_ind, progress)).is_err() {
                                    break;
                                }
                            }
                        }
                    })
                    .context("Failed to spawn a thread to check all exercises")?;
            }

            // Drop this sender to detect when the last thread is done.
            drop(progress_sender);

            while let Ok((exercise_ind, progress)) = progress_receiver.recv() {
                let name = self.exercises[exercise_ind].name;
                match progress {
                    CheckProgress::Done => progress_visualizer.done(name)?,
                    CheckProgress::Pending => progress_visualizer.pending(name)?,
                }
                progresses[exercise_ind] = Some(progress);
            }

            Ok::<_, Error>(())
        })?;

        let mut first_pending_exercise_ind = None;
        for (exercise_ind, progress) in progresses.into_iter().enumerate() {
            match progress {
                Some(CheckProgress::Done) => {
                    self.set_status(exercise_ind, true)?;
                }
                Some(CheckProgress::Pending) => {
                    self.set_status(exercise_ind, false)?;
                    if first_pending_exercise_ind.is_none() {
                        first_pending_exercise_ind = Some(exercise_ind);
                    }
                }
                None => {
                    // If we got an error while checking all exercises in parallel,
                    // it could be because we exceeded the limit of open file descriptors.
                    // Therefore, try running exercises with errors sequentially.
                    let exercise = &self.exercises[exercise_ind];
                    let success = exercise.run_exercise(None, &self.cmd_runner)?;
                    if success {
                        progress_visualizer.done(exercise.name)?;
                    } else {
                        progress_visualizer.pending(exercise.name)?;
                        if first_pending_exercise_ind.is_none() {
                            first_pending_exercise_ind = Some(exercise_ind);
                        }
                    }
                    self.set_status(exercise_ind, success)?;
                }
            }
        }

        self.write()?;

        Ok(first_pending_exercise_ind)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(35, include_str!("parallel_check_all.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(35, include_str!("parallel_check_all.rs"));
    }
}
