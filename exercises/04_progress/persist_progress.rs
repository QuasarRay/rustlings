// RUSTLINGS: REBUILD THE CHECKER | Mission 025/112 | TODO
// Persist Progress
// Prerequisite: pending_count. Target: src/app_state.rs::write (upstream line 230).
//
// Contract: Write the current exercise and completed names using the existing file format. Rewind and truncate so an older longer save cannot leave stale names.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint persist_progress`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn write(&mut self) -> Result<()> {
        self.file_buf.truncate(STATE_FILE_HEADER.len());

        self.file_buf
            .extend_from_slice(self.current_exercise().name.as_bytes());
        self.file_buf.push(b'\n');


        // TODO: Restore this step using the contract above.
        todo!("persist_progress");

        self.state_file
            .rewind()
            .with_context(|| format!("Failed to rewind the state file {STATE_FILE_NAME}"))?;
        self.state_file
            .set_len(0)
            .with_context(|| format!("Failed to truncate the state file {STATE_FILE_NAME}"))?;
        self.state_file
            .write_all(&self.file_buf)
            .with_context(|| format!("Failed to write the state file {STATE_FILE_NAME}"))?;

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(25, include_str!("persist_progress.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(25, include_str!("persist_progress.rs"));
    }
}
