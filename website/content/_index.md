+++
+++

# Rebuild the Checker

An advanced second Rustlings course: restore the real Rustlings checker through 112 cumulative implementation and bug-fix missions. Finish ordinary Rustlings first.

## Quick start

Use Rust 1.89 or newer, Cargo, Clippy, rustfmt, Git, and a native linker. Build **this fork**; installing `rustlings` from crates.io selects a different curriculum.

```sh
git clone https://github.com/QuasarRay/rustlings.git
cd rustlings
rustup component add clippy rustfmt
cargo run --locked -- workshop prepare
cargo run --locked -- --no-editor
```

Read [setup](@/setup/index.md), then the [learner guide](https://github.com/QuasarRay/rustlings/blob/main/exercises/README.md).

The reference export reproduces the pinned original Rustlings project. Types, imports, helpers and the CLI argument model include supplied scaffold; the course does not ask you to type every line.
