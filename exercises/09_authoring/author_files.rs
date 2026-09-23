// RUSTLINGS: REBUILD THE CHECKER | Mission 106/112 | TODO
// Author Files
// Prerequisite: author_directories. Target: src/dev/new.rs::write_rel_file (upstream line 20).
//
// Contract: Write generated project content to the named relative file and retain actionable error context.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_files`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn write_rel_file<C>(file_name: &str, current_dir: &str, content: C) -> Result<()>
where
    C: AsRef<[u8]>,
{
    // TODO: Implement this operation using the contract above.
    todo!("author_files")
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(106, include_str!("author_files.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(106, include_str!("author_files.rs"));
    }
}
