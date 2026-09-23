# Chapter 3: The Quartermaster

Restore initialization, packaged assets, reset, and solution materialization.

This chapter assumes you can already use Rust ownership, traits, iterators, error handling, and tests. The new work is understanding and repairing an application architecture.

## Play loop

1. Read the contract and the surrounding real implementation. Predict the failure before running it.
2. Run the named mission. Read the compiler error or failed regression test.
3. Repair only the Rust between the repair markers, then save.
4. Ask for a hint with `h` if needed. Compare the revealed solution after completion.
5. Explain the data flow and failure mode aloud, then press `n`.

## Missions

| Mission | Kind | Restore | Source |
| --- | --- | --- | --- |
| 017 [idempotent_directory](idempotent_directory.rs) | BUG | Accept an already existing directory; propagate other creation failures with the path in their context. | `src/embedded.rs::create_dir_if_not_exists` |
| 018 [chapter_readme](chapter_readme.rs) | BUG | Create a chapter directory and write its embedded README.md there. | `src/embedded.rs::init_on_disk` |
| 019 [materialize_exercises](materialize_exercises.rs) | TODO | Create the exercise tree, chapter READMEs, exercises, and input fixtures from the embedded catalog. | `src/embedded.rs::init_exercises_dir` |
| 020 [restore_exercise](restore_exercise.rs) | TODO | Restore exactly one embedded exercise at a caller-validated index. Propagate filesystem write failures with context. | `src/embedded.rs::write_exercise_to_disk` |
| 021 [reveal_solution](reveal_solution.rs) | TODO | Write the selected embedded solution under its chapter and name, and return that same path. | `src/embedded.rs::write_solution_to_disk` |
| 022 [initialize_workspace](initialize_workspace.rs) | TODO | Complete one missing initialization stage. Preserve existing-directory checks, workspace detection, catalog materialization, and the generated manifest. | `src/init.rs::init` |

## Chapter checkpoint

Which files belong to the course author, the learner, and the build cache? Why must resetting one exercise not reset a whole chapter?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Design a fixture lifecycle for a networking course: initial state, per-attempt reset, learner output, and cleanup.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
