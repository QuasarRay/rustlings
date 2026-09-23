// RUSTLINGS: REBUILD THE CHECKER | Mission 011/112 | TODO
// Binary Location
// Prerequisite: process_status. Target: src/cmd.rs::run_debug_bin (upstream line 144).
//
// Contract: Run the compiled debug binary from Cargo target_directory, using the exercise directory as its working directory.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint binary_location`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn run_debug_bin(
        &self,
        bin_name: &str,
        cwd: &str,
        output: Option<&mut Vec<u8>>,
    ) -> Result<bool> {
        // 7 = "/debug/".len()
        let mut bin_path =
            PathBuf::with_capacity(self.target_dir.as_os_str().len() + 7 + bin_name.len());
        bin_path.push(&self.target_dir);
        bin_path.push("debug");
        bin_path.push(bin_name);

        run_cmd(Command::new(&bin_path), bin_name, Some(cwd), output)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(11, include_str!("binary_location.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(11, include_str!("binary_location.rs"));
    }
}
