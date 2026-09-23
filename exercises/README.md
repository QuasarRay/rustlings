# Rebuild the Checker

Restore Rustlings by repairing Rustlings. This is a second course for learners who have already finished the normal Rustlings curriculum.

There are **112 cumulative missions** across ten chapters: **91 TODO implementations** and **21 injected regressions**. Every mission targets a real function in the pinned Rustlings implementation. The assessment compiles and tests that implementation with your repairs.

The reference is [`QuasarRay/rustlings@a650509c789d`](https://github.com/QuasarRay/rustlings/tree/a650509c789da1656f813392b16aa1fa043b7f3e), Rustlings 6.5.0. Finishing restores that version, including its existing behavior and limitations. This course does not silently upgrade its design.

## Start playing

Requirements: Rust 1.89 or newer, Cargo, Clippy, and rustfmt. The original engine needs Rust 1.88; the course verifier uses the standard library file-lock API introduced in Rust 1.89. The first preparation downloads the original Cargo.lock dependencies. After that, the verifier needs no network when those dependencies remain cached.

From a checkout of **this fork**:

```sh
cargo run --locked -- workshop prepare
cargo run -- --no-editor
```

The preparation compiles the original checker and runs its regression suite. It grants no mission completion. It is optional for correctness: this fork adds a course-specific command policy that allows cold builds. Native author threads queue behind one compiler permit, each outer command has a 600-second budget, and the verifier shares a 540-second budget across lock waiting, Clippy, and tests. Timeout cleanup terminates descendant processes. Ordinary courses and the pinned reference export retain the original engine policy.

For the normal installed-course experience, build this fork with `cargo build --release --locked`, then use that binary's absolute path in an empty directory:

```sh
/absolute/path/to/fork/target/release/rustlings init
cd rustlings
/absolute/path/to/fork/target/release/rustlings workshop prepare
/absolute/path/to/fork/target/release/rustlings --no-editor
```

Use the binary built from this fork. Installing the upstream crate from crates.io gives the original beginner exercises.

The portable maintenance entry point is `rustlings workshop`: `doctor` checks prerequisites, `info` identifies the fork, `prepare` checks the reference, `audit` runs the complete author gate, and `smoke` checks distribution and export. In this checkout, prefix these commands with `cargo run --locked --` instead of `rustlings`. The Rust driver is also directly compilable without Bash.

The package's existing `rust-version = "1.88"` describes the host engine. `[package.metadata.workshop].rust-version = "1.89"` describes this course's runtime verifier. The course preflight, CI, and release path enforce the latter; the archived original engine retains its own declared minimum. `rustlings --version` is the engine version; use `rustlings workshop info` to identify this course.

## Mission rules

1. Open the current mission. Its header gives its prerequisite, target function, contract, and failure type.
2. Read the provided context and predict the failure. You are repairing application behavior, not relearning Rust syntax.
3. Edit the Rust between `BEGIN RUSTLINGS REPAIR` and `END RUSTLINGS REPAIR`. Keep the markers, adapter, and test intact.
4. Save. Rustlings builds the exercise, runs its test, runs Clippy, and executes it. The additive course command policy bounds compiler concurrency and accommodates cold builds. The exercise asks the course verifier to check the actual reconstructed Rustlings source.
5. Use `h` for a three-stage hint, `r` to retry, `l` for the mission map, and `n` after completion. `x` resets the current mission.
6. Earlier repairs are included in every later build. Editing or resetting an earlier mission invalidates its completion evidence; recheck from that mission onward.

A mission is cleared by compilation and regression evidence. There is no `completed = true` flag to edit. Like ordinary Rustlings, the course is an open learning environment, not a tamper-resistant exam: changing tests, the verifier, caches, or solution fixtures is outside the exercise contract.

The `repair!` token wrapper transports a function into its original module and `impl`. The wrapper itself does not expand in the small exercise binary. The verifier inserts its contents at the recorded source range and Cargo compiles it with the original project dependencies. Compiler diagnostics therefore point into `target/workshop/engine/`; repair the corresponding mission file, not that generated copy. The header and source map identify the original function. This arrangement preserves the existing checker and exercise dependency manifest.

## Campaign map

| Chapter | Missions | Responsibility | Badge |
| --- | --- | --- | --- |
| [Catalog](01_catalog/README.md) | 001–007 | Turn declarative course metadata into stable exercise identities, source paths, and Cargo targets. | The Cartographer |
| [Checker](02_checker/README.md) | 008–016 | Restore the exact compile, test, lint, and execution contract used to award completion. | The Gatekeeper |
| [Distribution](03_distribution/README.md) | 017–022 | Restore initialization, packaged assets, reset, and solution materialization. | The Quartermaster |
| [Progress](04_progress/README.md) | 023–039 | Restore persistence, idempotent status accounting, selection, aggregate checking, and completion. | The Archivist |
| [Terminal](05_terminal/README.md) | 040–048 | Restore bounded terminal output, progress, paths, and links. | The Signal Keeper |
| [Navigation](06_navigation/README.md) | 049–066 | Restore scrolling, filtering, search, selection mapping, and the list lifecycle. | The Navigator |
| [Watch](07_watch/README.md) | 067–084 | Restore debouncing, event ownership, input coordination, reruns, and watch/list transitions. | The Watchkeeper |
| [Editors](08_editors/README.md) | 085–096 | Restore process boundaries, editor selection, asynchronous startup, and Zellij pane management. | The Toolsmith |
| [Authoring](09_authoring/README.md) | 097–110 | Restore the original community-course creation and author-quality gates. | The Course Architect |
| [Launch](10_launch/README.md) | 111–112 | Restore embedding and launch dispatch, then assemble the unchanged upstream project from all repairs. | The Maintainer |

Badges are chapter names that help you remember the responsibilities. Rustlings' existing completion counter is the score. Reflection and transfer questions are practice prompts, not hidden automatic gates.

## What you build

A build combines the immutable pinned project, all earlier learner repairs, the current repair, and the still-supplied future implementation. Each mission replaces a disjoint source range. Later missions never overwrite earlier repairs. Solutions use earlier reference solutions so authors can validate them independently of learner progress.

This is incremental **restoration**, not a greenfield skeleton. Imports, type definitions, constants, small accessors, upstream tests, and functions not assigned as repair ranges remain supplied scaffold. The complete source still ships in the reference snapshot; every source range and scaffold boundary is inspectable in `01_catalog/missions.json`. All runtime subsystems are encountered. The course does not claim the learner types every line.

The grader runs the original project's Clippy gate and tests, plus additive contract probes for catalog defaults, paths, Cargo arguments, nonzero exits, compile/test/runtime rejection, persistence, selection, scroll boundaries, watch events, and author-name validation. It does not compare your code to a solution string to decide normal mission completion.

The reference solutions reconstruct every archived file byte for byte. Equivalent learner implementations may differ textually while satisfying the same tested contracts. This is not a proof of equivalence for every possible execution. The final reference export is exactly the pinned Rustlings implementation; the learner export contains the actual cumulative repairs, never a substituted reference answer.

## Final boss: run the restored checker

See [final boss: run the restored checker](01_catalog/export.md).

## Diagnose a stuck mission

See [diagnose a stuck mission](01_catalog/troubleshooting.md).

## Architecture description record

See [architecture description record](01_catalog/architecture.md).

## Maintainer validation

See [maintainer validation](01_catalog/maintenance.md).

## Source map

See [source map](01_catalog/source-map.md).
