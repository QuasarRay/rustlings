// RUSTLINGS: REBUILD THE CHECKER | Mission 082/112 | TODO
// Watch Dispatch
// Prerequisite: watch_resize. Target: src/watch.rs::run_watch (upstream line 59).
//
// Contract: Restore one branch of the existing watch-event loop without changing event ownership or exit semantics.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_dispatch`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn run_watch(
    app_state: &mut AppState,
    notify_exercise_names: Option<&'static [&'static [u8]]>,
) -> Result<WatchExit> {
    let (watch_event_sender, watch_event_receiver) = channel();

    let mut manual_run = false;
    // Prevent dropping the guard until the end of the function.
    // Otherwise, the file watcher exits.
    let _watcher_guard = if let Some(exercise_names) = notify_exercise_names {
        let notify_event_handler =
            NotifyEventHandler::build(watch_event_sender.clone(), exercise_names)?;

        let mut watcher = RecommendedWatcher::new(
            notify_event_handler,
            Config::default()
                .with_follow_symlinks(false)
                .with_poll_interval(Duration::from_secs(1)),
        )
        .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;

        watcher
            .watch(Path::new("exercises"), RecursiveMode::Recursive)
            .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;

        Some(watcher)
    } else {
        manual_run = true;
        None
    };


    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    watch_state.run_current_exercise(&mut stdout)?;

    while let Ok(event) = watch_event_receiver.recv() {
        match event {
            WatchEvent::Input(InputEvent::Next) => match watch_state.next_exercise(&mut stdout)? {
                ExercisesProgress::AllDone => break,
                ExercisesProgress::NewPending => {
                    watch_state.run_current_exercise(&mut stdout)?;
                    if watch_state.done() {
                        // An exercise is done although it was not marked as such.
                        // Trigger check all to fix the state file.
                        match watch_state.check_all_exercises(&mut stdout)? {
                            ExercisesProgress::AllDone => break,
                            ExercisesProgress::NewPending => {
                                watch_state.run_current_exercise(&mut stdout)?;
                            }
                            ExercisesProgress::CurrentPending => watch_state.render(&mut stdout)?,
                        }
                    }
                }
                ExercisesProgress::CurrentPending => (),
            },
            WatchEvent::Input(InputEvent::Run) => watch_state.run_current_exercise(&mut stdout)?,
            WatchEvent::Input(InputEvent::Hint) => watch_state.show_hint(&mut stdout)?,
            WatchEvent::Input(InputEvent::List) => return Ok(WatchExit::List),
            WatchEvent::Input(InputEvent::Reset) => watch_state.reset_exercise(&mut stdout)?,
            WatchEvent::Input(InputEvent::Quit) => {
                stdout.write_all(QUIT_MSG)?;
                break;
            }
            WatchEvent::FileChange { exercise_ind } => {
                watch_state.handle_file_change(exercise_ind, &mut stdout)?;
            }
            WatchEvent::TerminalResize { width } => {
                watch_state.update_term_width(width, &mut stdout)?;
            }
            WatchEvent::NotifyErr(e) => return Err(Error::from(e).context(NOTIFY_ERR)),
            WatchEvent::TerminalEventErr(e) => {
                return Err(Error::from(e).context("Terminal event listener failed"));
            }
        }
    }

    Ok(WatchExit::Shutdown)
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(82, include_str!("watch_dispatch.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(82, include_str!("watch_dispatch.rs"));
    }
}
