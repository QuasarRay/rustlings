// RUSTLINGS: REBUILD THE CHECKER | Mission 017/112 | BUG
// Idempotent Directory
// Prerequisite: solution_entry. Target: src/embedded.rs::create_dir_if_not_exists (upstream line 30).
//
// Contract: Accept an already existing directory; propagate other creation failures with the path in their context.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint idempotent_directory`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn create_dir_if_not_exists(path: &str) -> Result<()> {
    if let Err(e) = create_dir(path)
        && e.kind() == io::ErrorKind::AlreadyExists
    {
        return Err(Error::from(e).context(format!("Failed to create the directory {path}")));
    }

    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(17, include_str!("idempotent_directory.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(17, include_str!("idempotent_directory.rs"));
    }
}
