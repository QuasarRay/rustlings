# Chapter 4: The Archivist

Restore persistence, idempotent status accounting, selection, aggregate checking, and completion.

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
| 023 [load_progress](load_progress.rs) | BUG | Reconstruct completed exercises by name from the saved state. Count each completed exercise once; tolerate an unreadable or incomplete state. | `src/app_state.rs::new` |
| 024 [pending_count](pending_count.rs) | BUG | Compute pending exercises from the cached completed count. | `src/app_state.rs::n_pending` |
| 025 [persist_progress](persist_progress.rs) | TODO | Write the current exercise and completed names using the existing file format. Rewind and truncate so an older longer save cannot leave stale names. | `src/app_state.rs::write` |
| 026 [select_index](select_index.rs) | TODO | Reject indices at or beyond the catalog length, avoid redundant writes, and persist valid changes. | `src/app_state.rs::set_current_exercise_ind` |
| 027 [select_name](select_name.rs) | TODO | Resolve the first exact exercise name, report unknown names, and persist the selected index. | `src/app_state.rs::set_current_exercise_by_name` |
| 028 [completion_count](completion_count.rs) | BUG | Make repeated status updates idempotent. Completing increments the count; reopening decrements it. | `src/app_state.rs::set_status` |
| 029 [reopen_exercise](reopen_exercise.rs) | TODO | Persist only when changing a completed exercise back to pending. | `src/app_state.rs::set_pending` |
| 030 [reset_source](reset_source.rs) | TODO | Restore embedded official exercises; use the existing git-stash path for community exercises. Propagate command failures. | `src/app_state.rs::reset` |
| 031 [reset_current](reset_current.rs) | TODO | Mark the selected exercise pending before restoring its source. | `src/app_state.rs::reset_current_exercise` |
| 032 [reset_by_index](reset_by_index.rs) | TODO | Validate the index, persist pending status, restore the corresponding file, and return its name. | `src/app_state.rs::reset_exercise_by_ind` |
| 033 [next_pending](next_pending.rs) | BUG | Search after the current exercise first, then wrap to the beginning. Translate suffix-local positions back to catalog indices. | `src/app_state.rs::next_pending_exercise_ind` |
| 034 [solution_visibility](solution_visibility.rs) | TODO | Preserve debug-build behavior; materialize official solutions and only expose existing community solutions. | `src/app_state.rs::current_solution_path` |
| 035 [parallel_check_all](parallel_check_all.rs) | TODO | Restore one stage of the parallel check-all coordinator. Keep the worker limit, result collection, status counts, and earliest pending result consistent. | `src/app_state.rs::check_all_exercises_impl` |
| 036 [save_check_all](save_check_all.rs) | TODO | Run the aggregate checker and persist its updated progress before returning. | `src/app_state.rs::check_all_exercises` |
| 037 [advance_progress](advance_progress.rs) | TODO | Complete the current exercise, choose the next pending item, and run the final regression check before declaring the course finished. | `src/app_state.rs::done_current_exercise` |
| 038 [finish_message](finish_message.rs) | TODO | Render the configured completion message and progress through the original terminal output path. | `src/app_state.rs::render_final_message` |
| 039 [single_run_result](single_run_result.rs) | TODO | Restore the single-exercise command, including pending status on failure and advancing after successful validation. | `src/run.rs::run` |

## Chapter checkpoint

What is the difference between a saved done flag and evidence that the current source still passes? Why does finishing trigger another check?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Write a recovery scenario for a database course after the learner edits an earlier migration.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
