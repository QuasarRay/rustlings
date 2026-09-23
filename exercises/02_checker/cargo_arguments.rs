// RUSTLINGS: REBUILD THE CHECKER | Mission 009/112 | BUG
// Cargo Arguments
// Prerequisite: cargo_metadata. Target: src/cmd.rs::cargo (upstream line 120).
//
// Contract: Construct the Cargo subcommand with the requested binary and optional color. Preserve the debug development-manifest override.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint cargo_arguments`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn cargo<'out>(
        &self,
        subcommand: &str,
        bin_name: &str,
        output: Option<&'out mut Vec<u8>>,
    ) -> CargoSubcommand<'out> {
        let mut cmd = Command::new("cargo");
        cmd.arg(subcommand).arg("--unknown-workshop-option").arg("--bin").arg(bin_name);

        // A hack to make `cargo run` work when developing Rustlings.
        #[cfg(debug_assertions)]
        cmd.arg("--manifest-path")
            .arg("dev/Cargo.toml")
            .arg("--target-dir")
            .arg(&self.target_dir);

        if output.is_some() {
            cmd.arg("--color").arg("always");
        }

        CargoSubcommand { cmd, output }
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(9, include_str!("cargo_arguments.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(9, include_str!("cargo_arguments.rs"));
    }
}
