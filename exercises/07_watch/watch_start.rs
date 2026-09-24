// RUSTLINGS: REBUILD THE CHECKER | Mission 084/112 | TODO
// Watch Start
// Prerequisite: watch_list_transition. Target: src/watch.rs::watch (upstream line 155).
//
// Contract: Construct the watcher and channels and enter the existing watch/list loop with its original terminal cleanup.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_start`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn watch(
    app_state: &mut AppState,
    notify_exercise_names: Option<&'static [&'static [u8]]>,
) -> Result<()> {
    // TODO: Use cfg_select! after MSRV 1.95

    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    #[cfg(windows)]
    watch_list_loop(app_state, notify_exercise_names)
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(84, include_str!("watch_start.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(84, include_str!("watch_start.rs"));
    }
}
