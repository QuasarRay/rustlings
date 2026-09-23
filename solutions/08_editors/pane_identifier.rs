// RUSTLINGS: REBUILD THE CHECKER | Mission 091/112 | BUG
// Pane Identifier
// Prerequisite: editor_join. Target: src/editor/zellij.rs::parse_pane_id (upstream line 13).
//
// Contract: Parse the existing terminal_<decimal-id> newline format and return both the original digits and their numeric value.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint pane_identifier`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn parse_pane_id(b: &[u8]) -> Option<(String, u32)> {
    // Remove newline
    let b = b.get("terminal_".len()..b.len().saturating_sub(1))?;
    let id_str = str::from_utf8(b).ok()?;

    let (first, rest) = b.split_first()?;
    let mut id = u32::from(first - b'0');

    for c in rest {
        id = 10 * id + u32::from(c - b'0');
    }

    Some((id_str.to_owned(), id))
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(91, include_str!("pane_identifier.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(91, include_str!("pane_identifier.rs"));
    }
}
