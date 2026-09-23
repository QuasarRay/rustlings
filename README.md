# Rebuild the Checker — advanced Rustlings

This fork is a second course for people who have already finished Rustlings. Repair the actual Rustlings 6.5.0 checker through **112 cumulative missions**, mixing implementation and bug fixing. The reference repairs reproduce the pinned original project; some source remains supplied scaffold.

Use **this repository's binary**. The crates.io `rustlings` package installs the upstream beginner curriculum.

```sh
git clone https://github.com/QuasarRay/rustlings.git
cd rustlings
rustup component add clippy rustfmt
cargo run --locked -- workshop prepare
cargo run --locked -- --no-editor
```

The course requires Rust **1.89+**, Cargo, Clippy, rustfmt, Git, and a native linker. Preparation checks prerequisites and builds the reference; it grants no learner progress. Cold checks also work without preparation.

- [Learner guide and campaign map](exercises/README.md)
- [ISO/IEC/IEEE 42010 architecture description record](exercises/01_catalog/architecture.md)
- [Troubleshooting](exercises/01_catalog/troubleshooting.md)
- [Reference export](exercises/01_catalog/export.md)
- [Author validation](exercises/01_catalog/maintenance.md)

Run `cargo run --locked -- workshop info` to identify this fork and `cargo run --locked -- workshop audit` for author validation. The package version describes the preserved engine; course identity is reported separately. This course is distributed from source and is not published as the upstream crates.io curriculum.

## Original upstream course: [Rustlings](https://rustlings.rust-lang.org) 🦀

Small exercises to get you used to reading and writing [Rust](https://www.rust-lang.org) code - _Recommended in parallel to reading [the official Rust book](https://doc.rust-lang.org/book) 📚️_

Visit the **website** for a demo, info about setup and more:

## ➡️ [rustlings.rust-lang.org](https://rustlings.rust-lang.org) ⬅️
