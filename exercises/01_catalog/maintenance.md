# Maintainer validation

From a repository checkout:

```sh
bash exercises/01_catalog/audit.sh
cargo dev check --require-solutions
cargo test --locked --workspace
cargo fmt --all --check
```

The canonical command is `cargo run --locked -- workshop audit`. It checks the real baseline, every isolated starter defect, exact reference reconstruction, workspace tests, and the native author gate. Compiler concurrency is bounded by the course host policy; warm artifacts are an optimization. Results are written under `target/workshop/audit.txt`. The release hook calls the same Rust driver.

Do not run the author audit against a learner's edited starter set: its purpose is to prove the published starters remain unsolved. Use normal Rustlings checks for learner work.

## Historical validation record — before audit remediation

Validated on Linux with Rust 1.98.1:

| Gate | Result |
| --- | --- |
| Isolated injected-defect audit | All 112 starters fail with every other repair correct. |
| Reference repair audit | All 112 canonical repairs pass; all 266 archived files reconstruct byte for byte. |
| Native author gate | `cargo dev check --require-solutions` passes for all 112 missions and solutions. |
| Original plus added engine regression tests | 22 unit tests and 8 integration tests pass. |
| Fresh release-mode initialization | All missions and declared inputs materialize without a source checkout. |
| Normal learner workflow | First starter fails; repaired first and second missions pass through Rustlings. |
| Prior-edit invalidation | Regressing the first repair relocks the second despite saved UI completion. |
| Cumulative learner export | All 112 learner prefixes pass; exported files match the pinned reference when using canonical repairs. |
| Exported standalone project | Its original workspace tests pass outside the workshop. |
| Completed initialized course | Native `rustlings check-all` passes after all repairs. |
| Scope | Existing engine source, production dependency manifests, lockfile, and build script are unchanged. |

Interactive full-screen behavior and external editor integrations retain the manual-validation limits described above. These results do not claim Windows/macOS execution coverage for the course verifier.
