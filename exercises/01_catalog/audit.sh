#!/usr/bin/env bash
# Maintainer gate: prove each planted defect independently before Rustlings dev check.
set -euo pipefail
cd "$(dirname -- "${BASH_SOURCE[0]}")/../.."
mkdir -p target/workshop
rustc --edition=2024 exercises/01_catalog/grader.rs -o target/workshop/grader-cli
target/workshop/grader-cli "$PWD" audit
# Warm Rustlings' own build/test/lint artifacts before its parallel 30-second checks.
cargo build --locked --manifest-path dev/Cargo.toml --target-dir target --bins --quiet
cargo test --locked --manifest-path dev/Cargo.toml --target-dir target --no-run --quiet
cargo clippy --locked --manifest-path dev/Cargo.toml --target-dir target --all-targets --quiet -- --deny warnings
