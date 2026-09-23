// RUSTLINGS: REBUILD THE CHECKER | Mission 110/112 | TODO
// Author Dispatch
// Prerequisite: author_update. Target: src/dev.rs::run (upstream line 30).
//
// Contract: Dispatch New, Check, and Update through their original handlers and preserve the debug-build restriction.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_dispatch`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn run(self) -> Result<()> {
        // TODO: Implement this operation using the contract above.
        todo!("author_dispatch")
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(110, include_str!("author_dispatch.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(110, include_str!("author_dispatch.rs"));
    }
}
