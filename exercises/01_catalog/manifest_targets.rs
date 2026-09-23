// RUSTLINGS: REBUILD THE CHECKER | Mission 006/112 | TODO
// Manifest Targets
// Prerequisite: manifest_boundaries. Target: src/cargo_toml.rs::append_bins (upstream line 29).
//
// Contract: Register each exercise and only solutions that exist. Respect path prefixes and preserve catalog order.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint manifest_targets`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn append_bins(
    buf: &mut Vec<u8>,
    exercise_infos: &[ExerciseInfo],
    exercise_path_prefix: &[u8],
) {
    buf.push(b'\n');
    for exercise_info in exercise_infos {
        buf.extend_from_slice(b"  { name = \"");
        todo!("manifest_targets");
        buf.extend_from_slice(b"\", path = \"");
        buf.extend_from_slice(exercise_path_prefix);
        buf.extend_from_slice(b"exercises/");
        if let Some(dir) = exercise_info.dir {
            buf.extend_from_slice(dir.as_bytes());
            buf.push(b'/');
        }
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b".rs\" },\n");

        let sol_path = exercise_info.sol_path();
        if !Path::new(&sol_path).exists() {
            continue;
        }

        buf.extend_from_slice(b"  { name = \"");
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b"_sol");
        buf.extend_from_slice(b"\", path = \"");
        buf.extend_from_slice(exercise_path_prefix);
        buf.extend_from_slice(b"solutions/");
        if let Some(dir) = exercise_info.dir {
            buf.extend_from_slice(dir.as_bytes());
            buf.push(b'/');
        }
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b".rs\" },\n");
    }
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(6, include_str!("manifest_targets.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(6, include_str!("manifest_targets.rs"));
    }
}
