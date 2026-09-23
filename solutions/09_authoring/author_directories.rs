// RUSTLINGS: REBUILD THE CHECKER | Mission 105/112 | TODO
// Author Directories
// Prerequisite: author_gate. Target: src/dev/new.rs::create_rel_dir (upstream line 12).
//
// Contract: Create a relative project directory and attach its path to any failure.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_directories`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn create_rel_dir(dir_name: &str, current_dir: &str) -> Result<()> {
    create_dir(dir_name)
        .with_context(|| format!("Failed to create the directory {current_dir}/{dir_name}"))?;
    println!("Created the directory {current_dir}/{dir_name}");
    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(105, include_str!("author_directories.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(105, include_str!("author_directories.rs"));
    }
}
