// RUSTLINGS: REBUILD THE CHECKER | Mission 070/112 | TODO
// Terminal Events
// Prerequisite: file_notifications. Target: src/watch/terminal_event.rs::terminal_event_handler (upstream line 18).
//
// Contract: Translate terminal events to watch events while respecting pause/resume coordination and manual-run mode.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint terminal_events`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn terminal_event_handler(
    sender: Sender<WatchEvent>,
    unpause_receiver: Receiver<()>,
    manual_run: bool,
) {
    // TODO: Implement this operation using the contract above.
    todo!("terminal_events")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(70, include_str!("terminal_events.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(70, include_str!("terminal_events.rs"));
    }
}
