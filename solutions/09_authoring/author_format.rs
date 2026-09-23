// RUSTLINGS: REBUILD THE CHECKER | Mission 102/112 | TODO
// Author Format
// Prerequisite: author_unsolved. Target: src/dev/check.rs::check_exercises (upstream line 254).
//
// Contract: Validate the format version and combine metadata, directory, and unsolved-starter checks.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_format`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn check_exercises(info_file: &'static InfoFile, cmd_runner: &'static CmdRunner) -> Result<()> {
    match info_file.format_version.cmp(&CURRENT_FORMAT_VERSION) {
        Ordering::Less => bail!(
            "`format_version` < {CURRENT_FORMAT_VERSION} (supported version)\n\
             Please migrate to the latest format version"
        ),
        Ordering::Greater => bail!(
            "`format_version` > {CURRENT_FORMAT_VERSION} (supported version)\n\
             Try updating the Rustlings program"
        ),
        Ordering::Equal => (),
    }

    let handle = thread::Builder::new()
        .spawn(move || check_exercises_unsolved(info_file, cmd_runner))
        .context("Failed to spawn a thread to check if any exercise is already solved")?;

    let info_file_paths = check_info_file_exercises(info_file)?;
    check_unexpected_files("exercises", &info_file_paths)?;

    handle.join().unwrap()
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(102, include_str!("author_format.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(102, include_str!("author_format.rs"));
    }
}
