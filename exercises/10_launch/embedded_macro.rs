// RUSTLINGS: REBUILD THE CHECKER | Mission 111/112 | TODO
// Embedded Macro
// Prerequisite: author_dispatch. Target: rustlings-macros/src/lib.rs::include_files (upstream line 20).
//
// Contract: Restore a stage of the real procedural macro: ordered exercises, solutions, directory indices, inputs, and READMEs must remain aligned.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint embedded_macro`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn include_files(_: TokenStream) -> TokenStream {
    let info_file = include_str!("../info.toml");
    let info = toml::de::from_str::<InfoFile>(info_file).expect("Failed to parse `info.toml`");
    let exercises = info.exercises;

    let exercise_files = exercises
        .iter()
        .map(|exercise| format!("../exercises/{}/{}.rs", exercise.dir, exercise.name));
    let solution_files = exercises
        .iter()
        .map(|exercise| format!("../solutions/{}/{}.rs", exercise.dir, exercise.name));

    let mut dirs = Vec::with_capacity(32);
    let mut dir_inds = vec![0; exercises.len()];


    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    let input_files = exercises.iter().map(|exercise| {
        let names = exercise.input_files.iter();
        let paths = exercise
            .input_files
            .iter()
            .map(|f| format!("../exercises/{}/{}", exercise.dir, f));
        quote! {
            &[#(InputFile {
                name: #names,
                content: include_str!(#paths),
            }),*]
        }
    });

    let readmes = dirs
        .iter()
        .map(|dir| format!("../exercises/{dir}/README.md"));

    quote! {
        EmbeddedFiles {
            info_file: #info_file,
            exercise_files: &[#(ExerciseFiles {
                exercise: include_bytes!(#exercise_files),
                solution: include_bytes!(#solution_files),
                dir_ind: #dir_inds,
                input_files: #input_files,
            }),*],
            exercise_dirs: &[#(ExerciseDir { name: #dirs, readme: include_bytes!(#readmes) }),*],
        }
    }
    .into()
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(111, include_str!("embedded_macro.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(111, include_str!("embedded_macro.rs"));
    }
}
