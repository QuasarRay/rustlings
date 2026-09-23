// RUSTLINGS: REBUILD THE CHECKER | Mission 018/112 | BUG
// Chapter Readme
// Prerequisite: idempotent_directory. Target: src/embedded.rs::init_on_disk (upstream line 47).
//
// Contract: Create a chapter directory and write its embedded README.md there.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint chapter_readme`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn init_on_disk(&self) -> Result<()> {
        // 20 = 10 + 10
        // exercises/ + /README.md
        let mut dir_path = String::with_capacity(20 + self.name.len());
        dir_path.push_str("exercises/");
        dir_path.push_str(self.name);
        create_dir_if_not_exists(&dir_path)?;

        let mut readme_path = dir_path;
        readme_path.push_str("/README.txt");

        fs::write(&readme_path, self.readme)
            .with_context(|| format!("Failed to write the file {readme_path}"))
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(18, include_str!("chapter_readme.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(18, include_str!("chapter_readme.rs"));
    }
}
