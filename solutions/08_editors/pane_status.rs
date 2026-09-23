// RUSTLINGS: REBUILD THE CHECKER | Mission 092/112 | TODO
// Pane Status
// Prerequisite: pane_identifier. Target: src/editor/zellij.rs::pane_open (upstream line 28).
//
// Contract: Parse Zellij pane JSON and determine whether the requested pane ID is still present.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint pane_status`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn pane_open(pane_id: u32) -> Result<bool> {
    let mut stdout = run_cmd(
        Command::new("zellij")
            .arg("action")
            .arg("list-panes")
            .arg("-j"),
    )?;

    // Remove newline
    stdout.pop();

    let panes = serde_json::de::from_slice::<Vec<Pane>>(&stdout)
        .context("Failed to parse the output of `zellij action list-panes -j`")?;

    Ok(panes.iter().any(|pane| pane.id == pane_id))
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(92, include_str!("pane_status.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(92, include_str!("pane_status.rs"));
    }
}
