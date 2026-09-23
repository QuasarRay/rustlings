// RUSTLINGS: REBUILD THE CHECKER | Mission 065/112 | TODO
// List Keys
// Prerequisite: list_open. Target: src/list.rs::handle_list (upstream line 22).
//
// Contract: Restore one input-handling branch while preserving filtering, navigation, search, reset, and quit behavior.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint list_keys`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
fn handle_list(app_state: &mut AppState, stdout: &mut StdoutLock) -> Result<()> {
    // TODO: Restore this step using the contract above.
    todo!("list_keys");

    loop {
        match event::read().context("Failed to read terminal event")? {
            Event::Key(key) => {
                match key.kind {
                    KeyEventKind::Release => continue,
                    KeyEventKind::Press | KeyEventKind::Repeat => (),
                }

                list_state.message.clear();

                if is_searching {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter => {
                            is_searching = false;
                            list_state.search_query.clear();
                        }
                        KeyCode::Char(c) => {
                            list_state.search_query.push(c);
                            list_state.apply_search_query();
                        }
                        KeyCode::Backspace => {
                            list_state.search_query.pop();
                            list_state.apply_search_query();
                        }
                        _ => continue,
                    }

                    list_state.draw(stdout)?;
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => list_state.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => list_state.select_previous(),
                    KeyCode::Home | KeyCode::Char('g') => list_state.select_first(),
                    KeyCode::End | KeyCode::Char('G') => list_state.select_last(),
                    KeyCode::Char('d') => {
                        if list_state.filter() == Filter::Done {
                            list_state.set_filter(Filter::None);
                            list_state.message.push_str("Disabled filter DONE");
                        } else {
                            list_state.set_filter(Filter::Done);
                            list_state.message.push_str(
                                "Enabled filter DONE │ Press d again to disable the filter",
                            );
                        }
                    }
                    KeyCode::Char('p') => {
                        if list_state.filter() == Filter::Pending {
                            list_state.set_filter(Filter::None);
                            list_state.message.push_str("Disabled filter PENDING");
                        } else {
                            list_state.set_filter(Filter::Pending);
                            list_state.message.push_str(
                                "Enabled filter PENDING │ Press p again to disable the filter",
                            );
                        }
                    }
                    KeyCode::Char('r') => list_state.reset_selected()?,
                    KeyCode::Char('c') | KeyCode::Enter => {
                        if list_state.selected_to_current_exercise()? {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('s' | '/') => {
                        is_searching = true;
                        list_state.apply_search_query();
                    }
                    // Redraw to remove the message.
                    KeyCode::Esc => (),
                    _ => continue,
                }
            }
            Event::Mouse(event) => match event.kind {
                MouseEventKind::ScrollDown => list_state.select_next(),
                MouseEventKind::ScrollUp => list_state.select_previous(),
                _ => continue,
            },
            Event::Resize(width, height) => list_state.set_term_size(width, height),
            // Ignore
            Event::FocusGained | Event::FocusLost => continue,
        }

        list_state.draw(stdout)?;
    }
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(65, include_str!("list_keys.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(65, include_str!("list_keys.rs"));
    }
}
