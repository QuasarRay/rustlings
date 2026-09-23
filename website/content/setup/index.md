+++
title = "Course setup"
+++

## Requirements

Install Rust 1.89 or newer with Cargo, Clippy and rustfmt, plus Git and a native C linker. The original engine's 1.88 minimum is separate from this course's runtime minimum.

## Start from this fork

```sh
git clone https://github.com/QuasarRay/rustlings.git
cd rustlings
rustup component add clippy rustfmt
cargo run --locked -- workshop doctor
cargo run --locked -- workshop prepare
cargo run --locked -- --no-editor
```

Preparation checks the actual reference project and does not award progress. Run `cargo run --locked -- workshop info` to confirm the course identity. Installing the crates.io `rustlings` package gives the upstream beginner course.

For a separate initialized course, build this fork with `cargo build --release --locked`, run that binary's absolute path with `init` from an empty directory, and enter the resulting `rustlings` directory. Continue using the same fork binary. It installs the course inputs and locked dependencies.

Use any Rust editor with rust-analyzer. For failures, see the [course troubleshooting guide](https://github.com/QuasarRay/rustlings/blob/main/exercises/01_catalog/troubleshooting.md). Report fork issues to [QuasarRay/rustlings](https://github.com/QuasarRay/rustlings/issues).
