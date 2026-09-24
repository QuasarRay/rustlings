# Maintainer validation

From a repository checkout:

```sh
cargo run --locked -- workshop audit
```

The canonical command checks the real baseline, every isolated starter defect, exact reference reconstruction, compiling mutations, workspace tests, and the native author gate. Compiler concurrency is bounded by the course host policy; warm artifacts are an optimization. The release hook calls the same Rust driver. The Bash script is an optional shortcut.

For a quick environment/reference check use `workshop prepare`; for a platform and export check use `workshop smoke`. During a focused test change use `workshop mutations FIRST LAST`, with inclusive mission IDs. The full gate remains required before release. It costs more than a single learner check because it intentionally rebuilds independent wrong implementations; run it once after a coherent change rather than after every editor save.

Reports under `target/workshop` distinguish three claims:

| Report | Evidence and limits |
| --- | --- |
| `starter-audit.tsv` and `starter-NNN.log` | Per-mission compiler or behavioral rejection and unedited tool output. Escaping starters and starters rejected only by lint denial fail the audit. |
| `audit.txt` | All 112 isolated starters fail, all reference repairs pass, and all 266 archived files reconstruct exactly. A TODO lint failure alone establishes no behavioral coverage. |
| `mutation-audit.tsv` | Mission, source file, fault kind, outcome, and executed test names. Every mission has a compiling runtime-fault check (or a changed constant); 19 also have wrong-value, boundary, predicate, or side-effect mutations. The procedural macro is observed at compile time. |
| `mutation-NNN-KIND.log` | Commands and raw diagnostics for that individual mutation. `CAUGHT` requires a running behavioral test to fail after compilation, or the expected procedural-macro panic. Clippy runs after behavioral checks; a lint failure alone is not mutation evidence. Runtime-fault evidence must contain that mission's marker. |

`ESCAPED`, `INVALID_MUTANT`, and `UNOBSERVED_FAILURE` fail the author gate. A module filter only saves work: a mutation that survives it is checked against the complete integration suite before being classified as escaped. Filtered results cannot certify a learner repair. These 131 mutations establish a minimum set of negative examples, not exhaustive semantic equivalence. Assertions in `probes.txt` cover values and transitions; live terminal/editor versions still need appropriate manual checks.

Normal grading also runs the focused release-profile solution-visibility test, because the original function deliberately returns early in debug builds.

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

## Compiled artifact identity

The disposable main and macro crates include a Cargo-tracked environment dependency on the reconstructed input key and a fresh attempt token. A dedicated test prints each crate's compiled identity before behavioral tests run. The verifier requires both identities to match the requested input; stale or missing identities are infrastructure errors and are never cached as learner rejections. This protects checks when files change faster than a filesystem's timestamp resolution. A regression test changes same-size source while preserving its modification time and verifies Cargo rebuilds the changed behavior. The identity probes never enter an export.

Only passing certification reports are cached. Failing Cargo/test reports are rerun on retry, including old saved failures, so transient filesystem or process errors cannot freeze a learner's unchanged source in a rejected state. Known permission, busy-executable, and disk-space diagnostics are reported as infrastructure errors.

## Cargo and direct execution

Rustlings tests an adapter through Cargo and also executes its binary directly. Cargo adds default `CARGO_HOME` and rustup proxy variables that can be absent in direct execution. These launch details must not relock an unchanged prerequisite. Certification therefore records the resolved Cargo home, Rust sysroot and actual tool versions; proxy bookkeeping is excluded. Cargo configuration, Rust flags and other build inputs still invalidate evidence. A regression compares the same identity probe through a real Cargo-launched process and a direct child process.

On Windows, Cargo also extends `PATH` for [dynamic library discovery](https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths). The course host and maintenance driver preserve the caller's tool path in `WORKSHOP_TOOL_PATH`; the verifier fingerprints it and uses it when launching its own tools. Cargo can add the reconstructed project's own loader paths to those children. Deliberate caller-path changes still invalidate evidence. Environment-comparison failures name differing inputs while hashing their values.
