// RUSTLINGS: REBUILD THE CHECKER | Mission 021/112 | TODO
// Reveal Solution
// Prerequisite: restore_exercise. Target: src/embedded.rs::write_solution_to_disk (upstream line 122).
//
// Contract: Write the selected embedded solution under its chapter and name, and return that same path.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint reveal_solution`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn write_solution_to_disk(
        &self,
        exercise_ind: usize,
        exercise_name: &str,
    ) -> Result<String> {
        create_dir_if_not_exists("solutions")?;

        let exercise_files = &self.exercise_files[exercise_ind];
        let dir = &self.exercise_dirs[exercise_files.dir_ind];

        // 14 = 10 + 1 + 3
        // solutions/ + / + .rs
        let mut dir_path = String::with_capacity(14 + dir.name.len() + exercise_name.len());
        dir_path.push_str("solutions/");
        dir_path.push_str(dir.name);
        create_dir_if_not_exists(&dir_path)?;

        let mut solution_path = dir_path;
        solution_path.push('/');
        solution_path.push_str(exercise_name);
        solution_path.push_str(".rs");

        fs::write(&solution_path, exercise_files.solution)
            .with_context(|| format!("Failed to write the solution file {solution_path}"))?;

        Ok(solution_path)
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(21, include_str!("reveal_solution.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(21, include_str!("reveal_solution.rs"));
    }
}
