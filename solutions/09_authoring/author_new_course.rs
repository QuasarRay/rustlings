// RUSTLINGS: REBUILD THE CHECKER | Mission 107/112 | TODO
// Author New Course
// Prerequisite: author_files. Target: src/dev/new.rs::new (upstream line 31).
//
// Contract: Restore one community-course initialization stage, including the original optional Git behavior.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_new_course`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn new(path: &Path, no_git: bool) -> Result<()> {
    let dir_path_str = path.to_string_lossy();

    create_dir(path).with_context(|| format!("Failed to create the directory {dir_path_str}"))?;
    println!("Created the directory {dir_path_str}");

    set_current_dir(path)
        .with_context(|| format!("Failed to set {dir_path_str} as the current directory"))?;

    if !no_git
        && !Command::new("git")
            .arg("init")
            .status()
            .context("Failed to run `git init`")?
            .success()
    {
        bail!("`git init` didn't run successfully. See the possible error message above");
    }

    write_rel_file(".gitignore", &dir_path_str, GITIGNORE)?;

    create_rel_dir("exercises", &dir_path_str)?;
    create_rel_dir("solutions", &dir_path_str)?;

    write_rel_file(
        "info.toml",
        &dir_path_str,
        format!(
            "{INFO_FILE_BEFORE_FORMAT_VERSION}{CURRENT_FORMAT_VERSION}{INFO_FILE_AFTER_FORMAT_VERSION}"
        ),
    )?;

    write_rel_file("Cargo.toml", &dir_path_str, CARGO_TOML)?;

    write_rel_file("README.md", &dir_path_str, README)?;

    write_rel_file("rust-analyzer.toml", &dir_path_str, RUST_ANALYZER_TOML)?;

    create_rel_dir(".vscode", &dir_path_str)?;
    write_rel_file(
        ".vscode/extensions.json",
        &dir_path_str,
        crate::init::VS_CODE_EXTENSIONS_JSON,
    )?;

    println!("\nInitialization done ✓");

    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(107, include_str!("author_new_course.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(107, include_str!("author_new_course.rs"));
    }
}
