# Chapter 7: The Watchkeeper

Restore debouncing, event ownership, input coordination, reruns, and watch/list transitions.

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
| 067 [input_pause](input_pause.rs) | BUG | Pause watcher-triggered work while an exercise is running; the existing Drop implementation resumes it. | `src/watch.rs::scoped_pause` |
| 068 [debounce_updates](debounce_updates.rs) | TODO | Restore the debouncer stage so bursts of file events produce one update per modified exercise per quiet interval. | `src/watch/notify_event.rs::build` |
| 069 [file_notifications](file_notifications.rs) | BUG | Ignore unrelated events and non-Rust files, then map changed Rust filenames to exercise indices. | `src/watch/notify_event.rs::handle_event` |
| 070 [terminal_events](terminal_events.rs) | TODO | Translate terminal events to watch events while respecting pause/resume coordination and manual-run mode. | `src/watch/terminal_event.rs::terminal_event_handler` |
| 071 [watch_model](watch_model.rs) | TODO | Create the watch state and terminal-event thread using the original channel types and initial pending status. | `src/watch/state.rs::build` |
| 072 [watch_run](watch_run.rs) | TODO | Run the selected exercise, record its output and done status, coordinate the editor, and render the result. | `src/watch/state.rs::run_current_exercise` |
| 073 [watch_reset](watch_reset.rs) | TODO | Restore a reset-confirmation stage and resume the event thread when the interaction completes. | `src/watch/state.rs::reset_exercise` |
| 074 [watch_change](watch_change.rs) | TODO | Only rerun for a change to the current exercise. Preserve the index comparison and output propagation. | `src/watch/state.rs::handle_file_change` |
| 075 [watch_completion](watch_completion.rs) | BUG | Both done variants permit advancing; Pending does not. | `src/watch/state.rs::done` |
| 076 [watch_advance](watch_advance.rs) | TODO | Advance through AppState only when the current exercise is complete; otherwise report CurrentPending. | `src/watch/state.rs::next_exercise` |
| 077 [watch_prompt](watch_prompt.rs) | TODO | Restore a prompt stage so only currently valid actions are advertised. | `src/watch/state.rs::show_prompt` |
| 078 [watch_render](watch_render.rs) | TODO | Restore one stage of the existing result screen: diagnostics, hints, solution link, progress, and current path. | `src/watch/state.rs::render` |
| 079 [watch_hint](watch_hint.rs) | TODO | Reveal a hint once and redraw only when its visibility changes. | `src/watch/state.rs::show_hint` |
| 080 [watch_check_all](watch_check_all.rs) | TODO | Run all checks while input is paused; select a newly pending exercise or display the final message. | `src/watch/state.rs::check_all_exercises` |
| 081 [watch_resize](watch_resize.rs) | TODO | Redraw after an actual terminal-width change and avoid redundant redraws. | `src/watch/state.rs::update_term_width` |
| 082 [watch_dispatch](watch_dispatch.rs) | TODO | Restore one branch of the existing watch-event loop without changing event ownership or exit semantics. | `src/watch.rs::run_watch` |
| 083 [watch_list_transition](watch_list_transition.rs) | TODO | Switch between watch and list modes using their existing exit states. | `src/watch.rs::watch_list_loop` |
| 084 [watch_start](watch_start.rs) | TODO | Construct the watcher and channels and enter the existing watch/list loop with its original terminal cleanup. | `src/watch.rs::watch` |

## Chapter checkpoint

Which events should trigger a check? Why are writes made by a running exercise different from learner edits? How does a reset interact with watch mode?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Trace a burst of three editor saves and predict how many checks should run in an event-driven course.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
