// RUSTLINGS: REBUILD THE CHECKER | Mission 042/112 | TODO
// Unicode Width
// Prerequisite: ascii_width. Target: src/term.rs::write_str (upstream line 49).
//
// Contract: Limit by Unicode scalar values rather than slicing inside a UTF-8 character. Update the same output-width budget.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint unicode_width`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
    fn write_str(&mut self, unicode: &str) -> io::Result<()> {
        if let Some((ind, c)) = unicode
            .char_indices()
            .take(self.max_len.saturating_sub(self.len))
            .last()
        {
            self.stdout
                .write_all(&unicode.as_bytes()[..ind + c.len_utf8()])?;
            self.len += ind + 1;
        }

        Ok(())
    }
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(42, include_str!("unicode_width.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(42, include_str!("unicode_width.rs"));
    }
}
