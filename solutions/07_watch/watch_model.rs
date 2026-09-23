// RUSTLINGS: REBUILD THE CHECKER | Mission 071/112 | TODO
// Watch Model
// Prerequisite: terminal_events. Target: src/watch/state.rs::build (upstream line 44).
//
// Contract: Create the watch state and terminal-event thread using the original channel types and initial pending status.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_model`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn build(
        app_state: &'a mut AppState,
        watch_event_sender: Sender<WatchEvent>,
        manual_run: bool,
    ) -> Result<Self> {
        let term_width = terminal::size()
            .context("Failed to get the terminal size")?
            .0;

        let (terminal_event_unpause_sender, terminal_event_unpause_receiver) = sync_channel(0);

        thread::Builder::new()
            .spawn(move || {
                terminal_event_handler(
                    watch_event_sender,
                    terminal_event_unpause_receiver,
                    manual_run,
                );
            })
            .context("Failed to spawn a thread to handle terminal events")?;

        Ok(Self {
            app_state,
            output: Vec::with_capacity(OUTPUT_CAPACITY),
            show_hint: false,
            done_status: DoneStatus::Pending,
            manual_run,
            term_width,
            terminal_event_unpause_sender,
        })
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(71, include_str!("watch_model.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(71, include_str!("watch_model.rs"));
    }
}
