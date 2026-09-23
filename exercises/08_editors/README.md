# Chapter 8: The Toolsmith

Restore process boundaries, editor selection, asynchronous startup, and Zellij pane management.

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
| 085 [editor_process](editor_process.rs) | TODO | Execute an editor command and preserve output/error context without silently accepting nonzero exit status. | `src/editor.rs::run_cmd` |
| 086 [editor_discovery](editor_discovery.rs) | TODO | Use the existing version probe to detect a program; process spawning failures mean unavailable. | `src/editor.rs::program_exists` |
| 087 [editor_selection](editor_selection.rs) | TODO | Choose the configured editor, VS Code, or Zellij according to the original precedence and argument parsing. | `src/editor.rs::new` |
| 088 [editor_launch](editor_launch.rs) | TODO | Restore one launch stage, preserving asynchronous startup and the editor-specific exercise path arguments. | `src/editor.rs::open` |
| 089 [editor_close](editor_close.rs) | TODO | Close managed Zellij panes while retaining the original behavior for ordinary command-based editors. | `src/editor.rs::close` |
| 090 [editor_join](editor_join.rs) | TODO | Join an optional editor startup task and propagate its returned editor or error. | `src/editor.rs::join` |
| 091 [pane_identifier](pane_identifier.rs) | BUG | Parse the existing terminal_<decimal-id> newline format and return both the original digits and their numeric value. | `src/editor/zellij.rs::parse_pane_id` |
| 092 [pane_status](pane_status.rs) | TODO | Parse Zellij pane JSON and determine whether the requested pane ID is still present. | `src/editor/zellij.rs::pane_open` |
| 093 [pane_close](pane_close.rs) | TODO | Pass the pane ID as a separate argument to the original Zellij close-pane command. | `src/editor/zellij.rs::close_pane` |
| 094 [app_editor_start](app_editor_start.rs) | TODO | Coordinate the application current-exercise path with the optional editor startup task. | `src/app_state.rs::open_editor` |
| 095 [app_editor_join](app_editor_join.rs) | TODO | Join the startup task and put the resulting editor back into AppState. | `src/app_state.rs::join_editor_handle` |
| 096 [app_editor_close](app_editor_close.rs) | TODO | Close the optional editor and preserve the no-editor path. | `src/app_state.rs::close_editor` |

## Chapter checkpoint

Why are arguments passed as separate values? Which resources belong to the course and which belong to the learner?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Specify a nonblocking editor adapter contract for another subject without embedding shell command concatenation.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
