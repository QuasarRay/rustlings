// RUSTLINGS: REBUILD THE CHECKER | Mission 019/112 | TODO
// Materialize Exercises
// Prerequisite: chapter_readme. Target: src/embedded.rs::init_exercises_dir (upstream line 73).
//
// Contract: Create the exercise tree, chapter READMEs, exercises, and input fixtures from the embedded catalog.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint materialize_exercises`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn init_exercises_dir(&self, exercise_infos: &[ExerciseInfo]) -> Result<()> {
        create_dir("exercises").context("Failed to create the directory `exercises`")?;

        fs::write(
            "exercises/README.md",
            include_bytes!("../exercises/README.md"),
        )
        .context("Failed to write the file exercises/README.md")?;


        // TODO: Restore this step using the contract above.
        todo!("materialize_exercises");

        let mut exercise_path = String::with_capacity(64);
        let prefix = "exercises/";
        exercise_path.push_str(prefix);

        for (exercise_info, exercise_files) in exercise_infos.iter().zip(self.exercise_files) {
            let dir = &self.exercise_dirs[exercise_files.dir_ind];

            exercise_path.truncate(prefix.len());
            exercise_path.push_str(dir.name);
            exercise_path.push('/');
            exercise_path.push_str(exercise_info.name);
            exercise_path.push_str(".rs");

            fs::write(&exercise_path, exercise_files.exercise)
                .with_context(|| format!("Failed to write the exercise file {exercise_path}"))?;

            for InputFile { name, content } in exercise_files.input_files {
                let path = format!("{prefix}/{dir_name}/{name}", dir_name = dir.name);
                fs::write(&path, content)
                    .with_context(|| format!("Failed to write the input file {path}"))?;
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
    workshop::run(19, include_str!("materialize_exercises.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(19, include_str!("materialize_exercises.rs"));
    }
}
