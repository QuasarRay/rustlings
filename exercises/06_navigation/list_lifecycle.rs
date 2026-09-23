// RUSTLINGS: REBUILD THE CHECKER | Mission 066/112 | TODO
// List Lifecycle
// Prerequisite: list_keys. Target: src/list.rs::list (upstream line 114).
//
// Contract: Enter the original list terminal mode and restore it when handling finishes, including error paths.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_lifecycle`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn list(app_state: &mut AppState) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout
        .queue(EnterAlternateScreen)?
        .queue(cursor::Hide)?
        .queue(DisableLineWrap)?
        .queue(EnableMouseCapture)?;
    enable_raw_mode()?;

    let res = todo!("list_lifecycle");

    // Restore the terminal even if we got an error.
    stdout
        .queue(LeaveAlternateScreen)?
        .queue(cursor::Show)?
        .queue(EnableLineWrap)?
        .queue(DisableMouseCapture)?
        .flush()?;
    disable_raw_mode()?;

    res
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(66, include_str!("list_lifecycle.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(66, include_str!("list_lifecycle.rs"));
    }
}
