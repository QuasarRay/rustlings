// RUSTLINGS: REBUILD THE CHECKER | Mission 068/112 | TODO
// Debounce Updates
// Prerequisite: input_pause. Target: src/watch/notify_event.rs::build (upstream line 28).
//
// Contract: Restore the debouncer stage so bursts of file events produce one update per modified exercise per quiet interval.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint debounce_updates`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn build(
        watch_event_sender: Sender<WatchEvent>,
        exercise_names: &'static [&'static [u8]],
    ) -> Result<Self> {
        let (update_sender, update_receiver) = sync_channel(0);
        let error_sender = watch_event_sender.clone();

        // Debouncer
        thread::Builder::new()
            .spawn(move || {
                let mut exercise_updated = vec![false; exercise_names.len()];

                loop {
                    match update_receiver.recv_timeout(DEBOUNCE_DURATION) {
                        Ok(exercise_ind) => exercise_updated[exercise_ind] = true,
                        Err(RecvTimeoutError::Timeout) => {
                            for (exercise_ind, updated) in exercise_updated.iter_mut().enumerate() {
                                if *updated {
                                    if watch_event_sender
                                        .send(WatchEvent::FileChange { exercise_ind })
                                        .is_err()
                                    {
                                        break;
                                    }

                                    *updated = false;
                                }
                            }
                        }
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                }
            })
            .context("Failed to spawn a thread to debounce file changes")?;

        Ok(Self {
            error_sender,
            update_sender,
            exercise_names,
        })
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(68, include_str!("debounce_updates.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(68, include_str!("debounce_updates.rs"));
    }
}
