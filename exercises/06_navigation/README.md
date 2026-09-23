# Chapter 6: The Navigator

Restore scrolling, filtering, search, selection mapping, and the list lifecycle.

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
| 049 [scroll_initial](scroll_initial.rs) | BUG | Initialize scroll state without subtracting below zero when the selected row is close to the start. | `src/list/scroll_state.rs::new` |
| 050 [scroll_window](scroll_window.rs) | TODO | Keep the selection visible with padding, then clamp the viewport against the total row count. | `src/list/scroll_state.rs::update_offset` |
| 051 [scroll_next](scroll_next.rs) | TODO | Move one row forward when a selection exists and clamp at the final row. | `src/list/scroll_state.rs::select_next` |
| 052 [scroll_previous](scroll_previous.rs) | BUG | Move one row backward without underflow; an absent selection remains absent. | `src/list/scroll_state.rs::select_previous` |
| 053 [scroll_resize](scroll_resize.rs) | TODO | Clear selection for an empty list and clamp a previous selection after rows are removed. | `src/list/scroll_state.rs::set_n_rows` |
| 054 [scroll_padding](scroll_padding.rs) | BUG | Limit padding to one quarter of the visible rows and the configured maximum. | `src/list/scroll_state.rs::update_scroll_padding` |
| 055 [scroll_viewport](scroll_viewport.rs) | TODO | Update the viewport size, recompute padding, and then make the selected row visible. | `src/list/scroll_state.rs::set_max_n_rows_to_display` |
| 056 [list_model](list_model.rs) | TODO | Construct list state from application progress and the terminal dimensions, preserving row-index mappings. | `src/list/state.rs::build` |
| 057 [list_resize](list_resize.rs) | TODO | Update the rendering dimensions and the scroll viewport for the available rows. | `src/list/state.rs::set_term_size` |
| 058 [list_rows](list_rows.rs) | TODO | Restore one stage of row rendering: selection, completion status, file names, and clipping must remain synchronized. | `src/list/state.rs::draw_rows` |
| 059 [list_render](list_render.rs) | TODO | Restore one stage of the existing list screen; retain search input, filters, help keys, and terminal-size handling. | `src/list/state.rs::draw` |
| 060 [list_filter](list_filter.rs) | TODO | Rebuild visible exercise indices from the active completion filter and optional search filter. | `src/list/state.rs::update_rows` |
| 061 [list_index_mapping](list_index_mapping.rs) | TODO | Translate a visible selection through filtered row indices, and report missing/out-of-range selections. | `src/list/state.rs::selected_to_exercise_ind` |
| 062 [list_reset](list_reset.rs) | TODO | Reset the selected exercise and update the corresponding filtered row state. | `src/list/state.rs::reset_selected` |
| 063 [list_search](list_search.rs) | TODO | Apply the original substring search to exercise names and synchronize the displayed rows and selection. | `src/list/state.rs::apply_search_query` |
| 064 [list_open](list_open.rs) | TODO | Resolve a selected visible row, make its exercise current, and indicate whether the list should exit. | `src/list/state.rs::selected_to_current_exercise` |
| 065 [list_keys](list_keys.rs) | TODO | Restore one input-handling branch while preserving filtering, navigation, search, reset, and quit behavior. | `src/list.rs::handle_list` |
| 066 [list_lifecycle](list_lifecycle.rs) | TODO | Enter the original list terminal mode and restore it when handling finishes, including error paths. | `src/list.rs::list` |

## Chapter checkpoint

Why is a visible row index different from a catalog index? What happens when filtering leaves zero rows?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Describe a course map for 200 exercises without confusing view order with dependency order.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
