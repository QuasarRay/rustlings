// RUSTLINGS: REBUILD THE CHECKER | Mission 083/112 | TODO
// Watch List Transition
// Prerequisite: watch_dispatch. Target: src/watch.rs::watch_list_loop (upstream line 139).
//
// Contract: Switch between watch and list modes using their existing exit states.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_list_transition`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn watch_list_loop(
    app_state: &mut AppState,
    notify_exercise_names: Option<&'static [&'static [u8]]>,
) -> Result<()> {
    // TODO: Implement this operation using the contract above.
    // TODO: Complete the missing operation.
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(83, include_str!("watch_list_transition.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(83, include_str!("watch_list_transition.rs"));
    }
}
