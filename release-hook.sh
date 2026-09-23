#!/bin/bash

# Error out if any command fails
set -e

# This restoration course has a separate runtime MSRV and reconstruction gate.
# Preserve the original release path below for the upstream curriculum.
if test -f exercises/01_catalog/course.rs; then
    cargo run --locked -- workshop release
    exit "$?"
fi

typos

# Similar to CI
cargo clippy -- --deny warnings
cargo fmt --all --check
cargo test --workspace
cargo dev check --require-solutions

# MSRV
cargo +1.88 dev check --require-solutions
