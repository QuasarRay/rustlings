// RUSTLINGS: REBUILD THE CHECKER | Mission 016/112 | BUG
// Solution Entry
// Prerequisite: exercise_entry. Target: src/exercise.rs::run_solution (upstream line 192).
//
// Contract: Check the _sol binary through the same pipeline, forcing strict Clippy for reference solutions.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint solution_entry`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn run_solution(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        let name = self.name();
        let mut bin_name = String::with_capacity(name.len() + 4);
        bin_name.push_str(name);
        bin_name.push_str("sol");

        self.run::<true>(&bin_name, output, cmd_runner)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(16, include_str!("solution_entry.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(16, include_str!("solution_entry.rs"));
    }
}
