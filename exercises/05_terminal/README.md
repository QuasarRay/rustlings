# Chapter 5: The Signal Keeper

Restore bounded terminal output, progress, paths, and links.

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
| 040 [terminal_budget](terminal_budget.rs) | TODO | Initialize a bounded terminal writer with zero consumed width and the requested maximum. | `src/term.rs::new` |
| 041 [ascii_width](ascii_width.rs) | TODO | Write only ASCII bytes that fit the remaining terminal width and account for the bytes actually written. | `src/term.rs::write_ascii` |
| 042 [unicode_width](unicode_width.rs) | TODO | Limit by Unicode scalar values rather than slicing inside a UTF-8 character. Update the same output-width budget. | `src/term.rs::write_str` |
| 043 [progress_counter](progress_counter.rs) | TODO | Increment the completed check count and redraw the counter. | `src/term.rs::increment` |
| 044 [progress_bar](progress_bar.rs) | TODO | Restore a progress-bar stage while retaining the original width calculations and completed/pending formatting. | `src/term.rs::progress_bar` |
| 045 [canonical_paths](canonical_paths.rs) | TODO | Resolve a path to a Unicode canonical path; retain the existing Windows-prefix behavior and return None for failures. | `src/term.rs::canonicalize` |
| 046 [file_link_frame](file_link_frame.rs) | TODO | Style a file path using the caller-provided writer and restore terminal attributes afterward. | `src/term.rs::file_path` |
| 047 [file_hyperlink](file_hyperlink.rs) | TODO | Emit the original terminal hyperlink sequence around the displayed file path. | `src/term.rs::terminal_file_link` |
| 048 [ansi_buffer](ansi_buffer.rs) | TODO | Append an ANSI command to a byte buffer through the original formatting adapter. | `src/term.rs::write_ansi` |

## Chapter checkpoint

Which calculations use bytes, Unicode scalar values, and terminal columns? What assumptions does the pinned implementation actually make?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Sketch how an accessible command-line course can expose diagnostics without relying only on colors.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
