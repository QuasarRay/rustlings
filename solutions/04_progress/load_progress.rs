// RUSTLINGS: REBUILD THE CHECKER | Mission 023/112 | BUG
// Load Progress
// Prerequisite: initialize_workspace. Target: src/app_state.rs::new (upstream line 66).
//
// Contract: Reconstruct completed exercises by name from the saved state. Count each completed exercise once; tolerate an unreadable or incomplete state.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint load_progress`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    pub fn new(
        exercise_infos: Vec<ExerciseInfo<'static>>,
        final_message: String,
        editor: Option<Editor>,
        vs_code_term: bool,
    ) -> Result<(Self, StateFileStatus)> {
        let cmd_runner = CmdRunner::build()?;
        let mut state_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(STATE_FILE_NAME)
            .with_context(|| {
                format!("Failed to open or create the state file {STATE_FILE_NAME}")
            })?;

        let dir_canonical_path = term::canonicalize("exercises");
        let official_exercises = !Path::new("info.toml").exists();
        let mut exercises = exercise_infos
            .into_iter()
            .enumerate()
            .map(|(i, exercise_info)| {
                let canonical_path = dir_canonical_path.as_deref().map(|dir_canonical_path| {
                    let mut canonical_path;
                    if let Some(dir) = exercise_info.dir {
                        canonical_path = String::with_capacity(
                            2 + dir_canonical_path.len() + dir.len() + exercise_info.name.len(),
                        );
                        canonical_path.push_str(dir_canonical_path);
                        canonical_path.push_str(MAIN_SEPARATOR_STR);
                        canonical_path.push_str(dir);
                    } else {
                        canonical_path = String::with_capacity(
                            1 + dir_canonical_path.len() + exercise_info.name.len(),
                        );
                        canonical_path.push_str(dir_canonical_path);
                    }

                    canonical_path.push_str(MAIN_SEPARATOR_STR);
                    canonical_path.push_str(exercise_info.name);
                    canonical_path.push_str(".rs");
                    canonical_path
                });
                let embedded_input_files = if official_exercises {
                    EMBEDDED_FILES.exercise_files[i].input_files
                } else {
                    &[]
                };

                Exercise {
                    name: exercise_info.name,
                    dir: exercise_info.dir,
                    embedded_input_files,
                    // LEAKING: For `Editor::open`. The app state is used until the end of the program.
                    path: exercise_info.path().leak(),
                    canonical_path,
                    test: exercise_info.test,
                    strict_clippy: exercise_info.strict_clippy,
                    hint: exercise_info.hint,
                    // Updated below.
                    done: false,
                }
            })
            .collect::<Vec<_>>();

        let mut current_exercise_ind = 0;
        let mut n_done = 0;
        let mut file_buf = Vec::with_capacity(2048);
        let state_file_status = 'block: {
            if state_file.read_to_end(&mut file_buf).is_err() {
                break 'block StateFileStatus::NotRead;
            }

            // See `Self::write` for more information about the file format.
            let mut lines = file_buf.split(|c| *c == b'\n').skip(2);

            let Some(current_exercise_name) = lines.next() else {
                break 'block StateFileStatus::NotRead;
            };

            if current_exercise_name.is_empty() || lines.next().is_none() {
                break 'block StateFileStatus::NotRead;
            }

            let mut done_exercises = HashSet::with_capacity(exercises.len());

            for done_exercise_name in lines {
                if done_exercise_name.is_empty() {
                    break;
                }
                done_exercises.insert(done_exercise_name);
            }

            for (ind, exercise) in exercises.iter_mut().enumerate() {
                if done_exercises.contains(exercise.name.as_bytes()) {
                    exercise.done = true;
                    n_done += 1;
                }

                if exercise.name.as_bytes() == current_exercise_name {
                    current_exercise_ind = ind;
                }
            }

            StateFileStatus::Read
        };

        file_buf.clear();
        file_buf.extend_from_slice(STATE_FILE_HEADER);

        let slf = Self {
            current_exercise_ind,
            exercises,
            n_done,
            final_message,
            state_file,
            file_buf,
            official_exercises,
            cmd_runner,
            // VS Code has its own file link handling
            emit_file_links: !vs_code_term,
            editor,
        };

        Ok((slf, state_file_status))
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(23, include_str!("load_progress.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(23, include_str!("load_progress.rs"));
    }
}
