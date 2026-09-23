// RUSTLINGS: REBUILD THE CHECKER | Mission 100/112 | TODO
// Author File Inventory
// Prerequisite: author_catalog. Target: src/dev/check.rs::check_unexpected_files (upstream line 152).
//
// Contract: Reject undeclared files and excessive directory nesting while allowing chapter READMEs and declared inputs.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_file_inventory`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn check_unexpected_files(dir: &str, allowed_files: &HashSet<PathBuf>) -> Result<()> {
    // TODO: Restore this step using the contract above.
    todo!("author_file_inventory");

    for entry in read_dir(dir).with_context(|| format!("Failed to open the `{dir}` directory"))? {
        let entry = entry.with_context(|| format!("Failed to read the `{dir}` directory"))?;

        if entry.file_type().unwrap().is_file() {
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !allowed_files.contains(&path) {
                return Err(unexpected_file(&path));
            }

            continue;
        }

        let dir_path = entry.path();
        for entry in read_dir(&dir_path)
            .with_context(|| format!("Failed to open the directory {}", dir_path.display()))?
        {
            let entry = entry
                .with_context(|| format!("Failed to read the directory {}", dir_path.display()))?;
            let path = entry.path();

            if !entry.file_type().unwrap().is_file() {
                bail!(
                    "Found `{}` but expected only files. Only one level of exercise nesting is allowed",
                    path.display()
                );
            }

            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !allowed_files.contains(&path) {
                return Err(unexpected_file(&path));
            }
        }
    }

    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(100, include_str!("author_file_inventory.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(100, include_str!("author_file_inventory.rs"));
    }
}
