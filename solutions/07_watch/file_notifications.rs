// RUSTLINGS: REBUILD THE CHECKER | Mission 069/112 | BUG
// File Notifications
// Prerequisite: debounce_updates. Target: src/watch/notify_event.rs::handle_event (upstream line 72).
//
// Contract: Ignore unrelated events and non-Rust files, then map changed Rust filenames to exercise indices.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Diagnose and repair the injected regression.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint file_notifications`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn handle_event(&mut self, input_event: notify::Result<Event>) {
        if EXERCISE_RUNNING.load(Relaxed) {
            return;
        }

        let input_event = match input_event {
            Ok(v) => v,
            Err(e) => {
                // An error occurs when the receiver is dropped.
                // After dropping the receiver, the watcher guard should also be dropped.
                let _ = self.error_sender.send(WatchEvent::NotifyErr(e));
                return;
            }
        };

        match input_event.kind {
            EventKind::Any => (),
            EventKind::Modify(modify_kind) => match modify_kind {
                ModifyKind::Any | ModifyKind::Data(_) => (),
                ModifyKind::Name(rename_mode) => match rename_mode {
                    RenameMode::Any | RenameMode::To => (),
                    RenameMode::From | RenameMode::Both | RenameMode::Other => return,
                },
                ModifyKind::Metadata(metadata_kind) => match metadata_kind {
                    MetadataKind::Any | MetadataKind::WriteTime => (),
                    MetadataKind::AccessTime
                    | MetadataKind::Permissions
                    | MetadataKind::Ownership
                    | MetadataKind::Extended
                    | MetadataKind::Other => return,
                },
                ModifyKind::Other => return,
            },
            EventKind::Access(access_kind) => match access_kind {
                AccessKind::Any => (),
                AccessKind::Close(access_mode) => match access_mode {
                    AccessMode::Any | AccessMode::Write => (),
                    AccessMode::Execute | AccessMode::Read | AccessMode::Other => return,
                },
                AccessKind::Read | AccessKind::Open(_) | AccessKind::Other => return,
            },
            EventKind::Create(_) | EventKind::Remove(_) | EventKind::Other => return,
        }

        let _ = input_event
            .paths
            .into_iter()
            .filter_map(|path| {
                let file_name = path.file_name()?.to_str()?.as_bytes();

                let [file_name_without_ext @ .., b'.', b'r', b's'] = file_name else {
                    return None;
                };

                self.exercise_names
                    .iter()
                    .position(|exercise_name| *exercise_name == file_name_without_ext)
            })
            .try_for_each(|exercise_ind| self.update_sender.send(exercise_ind));
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(69, include_str!("file_notifications.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(69, include_str!("file_notifications.rs"));
    }
}
