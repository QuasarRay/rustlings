// RUSTLINGS: REBUILD THE CHECKER | Mission 070/112 | TODO
// Terminal Events
// Prerequisite: file_notifications. Target: src/watch/terminal_event.rs::terminal_event_handler (upstream line 18).
//
// Contract: Translate terminal events to watch events while respecting pause/resume coordination and manual-run mode.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint terminal_events`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn terminal_event_handler(
    sender: Sender<WatchEvent>,
    unpause_receiver: Receiver<()>,
    manual_run: bool,
) {
    let last_watch_event = loop {
        match event::read() {
            Ok(Event::Key(key)) => {
                match key.kind {
                    KeyEventKind::Release | KeyEventKind::Repeat => continue,
                    KeyEventKind::Press => (),
                }

                if EXERCISE_RUNNING.load(Relaxed) {
                    continue;
                }

                let input_event = match key.code {
                    KeyCode::Char('n') => InputEvent::Next,
                    KeyCode::Char('r') if manual_run => InputEvent::Run,
                    KeyCode::Char('h') => InputEvent::Hint,
                    KeyCode::Char('l') => break WatchEvent::Input(InputEvent::List),
                    KeyCode::Char('x') => {
                        if sender.send(WatchEvent::Input(InputEvent::Reset)).is_err() {
                            return;
                        }

                        // Pause input until quitting the confirmation prompt.
                        if unpause_receiver.recv().is_err() {
                            return;
                        }

                        continue;
                    }
                    KeyCode::Char('q') => break WatchEvent::Input(InputEvent::Quit),
                    _ => continue,
                };

                if sender.send(WatchEvent::Input(input_event)).is_err() {
                    return;
                }
            }
            Ok(Event::Resize(width, _)) => {
                if sender.send(WatchEvent::TerminalResize { width }).is_err() {
                    return;
                }
            }
            Ok(Event::FocusGained | Event::FocusLost | Event::Mouse(_)) => (),
            Err(e) => break WatchEvent::TerminalEventErr(e),
        }
    };

    let _ = todo!("terminal_events");
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(70, include_str!("terminal_events.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(70, include_str!("terminal_events.rs"));
    }
}
