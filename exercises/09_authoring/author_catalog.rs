// RUSTLINGS: REBUILD THE CHECKER | Mission 099/112 | TODO
// Author Catalog
// Prerequisite: author_manifest. Target: src/dev/check.rs::check_info_file_exercises (upstream line 60).
//
// Contract: Restore one validation stage for unique names, hints, paths, TODO guidance, and declared test presence.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint author_catalog`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn check_info_file_exercises(info_file: &InfoFile) -> Result<HashSet<PathBuf>> {
    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    let mut file_buf = String::with_capacity(1 << 14);
    for exercise_info in &info_file.exercises {
        let name = exercise_info.name;
        if name.is_empty() {
            bail!("Found an empty exercise name in `info.toml`");
        }
        if name.len() > MAX_EXERCISE_NAME_LEN {
            bail!(
                "The length of the exercise name `{name}` is bigger than the maximum {MAX_EXERCISE_NAME_LEN}"
            );
        }
        if let Some(c) = forbidden_char(name) {
            bail!("Char `{c}` in the exercise name `{name}` is not allowed");
        }

        if let Some(dir) = exercise_info.dir {
            if dir.is_empty() {
                bail!("The exercise `{name}` has an empty dir name in `info.toml`");
            }
            if let Some(c) = forbidden_char(dir) {
                bail!("Char `{c}` in the exercise dir `{dir}` is not allowed");
            }
        }

        if exercise_info.hint.trim_ascii().is_empty() {
            bail!(
                "The exercise `{name}` has an empty hint. Please provide a hint or at least tell the user why a hint isn't needed for this exercise"
            );
        }

        if !names.insert(name) {
            bail!("The exercise name `{name}` is duplicated. Exercise names must all be unique");
        }

        let path = exercise_info.path();

        OpenOptions::new()
            .read(true)
            .open(&path)
            .with_context(|| format!("Failed to open the file {path}"))?
            .read_to_string(&mut file_buf)
            .with_context(|| format!("Failed to read the file {path}"))?;

        if !file_buf.contains("fn main()") {
            bail!(
                "The `main` function is missing in the file `{path}`.\n\
                 Create at least an empty `main` function to avoid language server errors"
            );
        }

        if !file_buf.contains("// TODO") {
            bail!(
                "Didn't find any `// TODO` comment in the file `{path}`.\n\
                 You need to have at least one such comment to guide the user."
            );
        }

        let contains_tests = file_buf.contains("#[test]");
        if exercise_info.test {
            if !contains_tests {
                bail!(
                    "The file `{path}` doesn't contain any tests. If you don't want to add tests to this exercise, set `test = false` for this exercise in the `info.toml` file"
                );
            }
        } else if contains_tests {
            bail!(
                "The file `{path}` contains tests annotated with `#[test]` but the exercise `{name}` has `test = false` in the `info.toml` file"
            );
        }

        file_buf.clear();

        let path = PathBuf::from(path);
        let parent = path.parent().unwrap();

        for input_file in &exercise_info.input_files {
            paths.insert(parent.join(input_file));
        }

        paths.insert(path);
    }

    Ok(paths)
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(99, include_str!("author_catalog.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(99, include_str!("author_catalog.rs"));
    }
}
