#!/usr/bin/env bash
# Convenience wrapper; the canonical driver is Rust and also runs on Windows.
set -euo pipefail
cd "$(dirname -- "${BASH_SOURCE[0]}")/../.."
cargo run --locked -- workshop audit
