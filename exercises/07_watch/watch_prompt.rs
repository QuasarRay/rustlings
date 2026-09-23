// RUSTLINGS: REBUILD THE CHECKER | Mission 077/112 | TODO
// Watch Prompt
// Prerequisite: watch_advance. Target: src/watch/state.rs::show_prompt (upstream line 182).
//
// Contract: Restore a prompt stage so only currently valid actions are advertised.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint watch_prompt`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn show_prompt(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.done() {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.write_all(b"n")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b":")?;
            stdout.queue(SetAttribute(Attribute::Underlined))?;
            stdout.write_all(b"next")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b" / ")?;
        }

        let mut show_key = |key, postfix| {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.write_all(&[key])?;
            stdout.queue(ResetColor)?;
            stdout.write_all(postfix)
        };


        // TODO: Restore this step using the contract above.
        todo!("watch_prompt");

        if !self.show_hint {
            show_key(b'h', b":hint / ")?;
        }

        show_key(b'l', b":list / ")?;
        show_key(b'x', b":reset / ")?;
        show_key(b'q', b":quit ? ")?;

        stdout.flush()
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(77, include_str!("watch_prompt.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(77, include_str!("watch_prompt.rs"));
    }
}
