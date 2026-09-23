// RUSTLINGS: REBUILD THE CHECKER | Mission 012/112 | BUG
// Command Tail
// Prerequisite: binary_location. Target: src/cmd.rs::args (upstream line 167).
//
// Contract: Append the extra arguments in order and return the builder for chaining.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint command_tail`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn args<'arg, I>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = &'arg str>,
    {
        self.cmd.args(args.into_iter().skip(1));
        self
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(12, include_str!("command_tail.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(12, include_str!("command_tail.rs"));
    }
}
