# Chapter 2: The Gatekeeper

Restore the exact compile, test, lint, and execution contract used to award completion.

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
| 008 [cargo_metadata](cargo_metadata.rs) | TODO | Ask Cargo for its target_directory, check process success, and parse the JSON response with useful error context. | `src/cmd.rs::build` |
| 009 [cargo_arguments](cargo_arguments.rs) | BUG | Construct the Cargo subcommand with the requested binary and optional color. Preserve the debug development-manifest override. | `src/cmd.rs::cargo` |
| 010 [process_status](process_status.rs) | BUG | Return the actual child exit status in both captured and discarded-output modes. Preserve merged output and the 30-second deadline. | `src/cmd.rs::run_cmd` |
| 011 [binary_location](binary_location.rs) | TODO | Run the compiled debug binary from Cargo target_directory, using the exercise directory as its working directory. | `src/cmd.rs::run_debug_bin` |
| 012 [command_tail](command_tail.rs) | BUG | Append the extra arguments in order and return the builder for chaining. | `src/cmd.rs::args` |
| 013 [runtime_failure](runtime_failure.rs) | BUG | Keep runtime failure visible in the collected output and return failure even when compilation succeeded. | `src/exercise.rs::run_bin` |
| 014 [checker_pipeline](checker_pipeline.rs) | TODO | Restore the build, optional tests, Clippy, and runtime sequence. Each failed gate must prevent a completion result. | `src/exercise.rs::run` |
| 015 [exercise_entry](exercise_entry.rs) | TODO | Delegate through the common pipeline with normal Clippy policy and the exercise name. | `src/exercise.rs::run_exercise` |
| 016 [solution_entry](solution_entry.rs) | BUG | Check the _sol binary through the same pipeline, forcing strict Clippy for reference solutions. | `src/exercise.rs::run_solution` |

## Chapter checkpoint

Why is compiling insufficient? Distinguish infrastructure errors, rejected exercises, and successful exercises with warning output.

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Describe how a compiler course would check exit status, diagnostics, and generated code separately. Keep the same evidence-first discipline.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
