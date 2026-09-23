# Audit review and remediation — 2026-09-23

The audit identified real release-blocking failures: the native author checker timed out, its descendants survived, onboarding installed the wrong course, and failing TODO starters were being used as stronger grading evidence than they justify. Its recommendations were mostly sound. Two interpretations need correction: a whole displayed function is not the amount of missing code, and absence of tests in a particular file does not prove absence of integration coverage.

## Evidence and scope

Reviewed input: `rustlings-audit-2026-09-23.zip`, SHA-256 `bbd7a538afb0b6c10f815dc9552eb6e5d9f67bf7895b87cc11a87c9938633c51`. The audited course commit is `1b871b5706c358ab9d31337f2e3dec3a493dbc0d`; its original engine reference remains `a650509c789da1656f813392b16aa1fa043b7f3e`.

The review inspected the source, archives, mission manifest, original and added tests, and actual [failed Actions run 35855861938](https://github.com/QuasarRay/rustlings/actions/runs/35855861938). Remediation is preserved in small dependent pull requests, starting at [PR #16](https://github.com/QuasarRay/rustlings/pull/16). Merge in dependency order; none of these PRs was merged as part of the review.

The engine source additions are hooks for this course's command policy, maintenance command, and initialized lockfile. Original engine statements and algorithms remain present. The pinned archive and reference solutions remain unchanged; generated probes, process policy, hints, and maintenance tools are excluded from the exported original project. Onboarding and workflow corrections address the audit's distribution failures.

## Finding-by-finding disposition

“Confirmed” describes the audited commit, not a claim that the flaw remains in the corrected stack. “Qualified” records a narrower conclusion supported by the evidence.

| ID | Assessment | Resolution or justified boundary |
| --- | --- | --- |
| H-01 | Confirmed | Repair the two failing gates and add course CI. Main remains unchanged until the PR stack is merged; successful PR checks must not be described as a green main. |
| H-02 | Confirmed | Native author threads now share one course compiler permit, with a budget that accommodates nested verification. Cold Linux author validation passes at the execution-policy checkpoint. |
| H-03 | Confirmed | Shared process ownership terminates descendant trees on timeout and reaps the child; a sleeping-grandchild test checks the failure path. |
| H-04 | Confirmed | A 600-second host command encloses one 540-second verifier budget, including lock waiting, Clippy and tests. The old 30-second policy remains only for ordinary courses and the original export. |
| H-05 | Confirmed | Keep the original author scheduler, but permit only one course compiler/checker command at a time; default Cargo build parallelism is two. |
| H-06 | Confirmed | Preparation is a convenience. Cold CI validates correctness without relying on prewarmed artifacts. |
| H-07 | Qualified, real coverage gaps | The per-file count was reproducible, but initialization and run dispatch already had integration coverage, and macros execute during compilation. Add explicit contracts and per-mission mutation evidence for the actual gaps. |
| H-08 | Confirmed | Separate starter rejection from behavioral evidence. Require compiling faults, wrong-value examples, named test evidence, and a passing reference. |
| H-09 | Confirmed | Root onboarding starts with this fork's advanced course and source build commands. Upstream instructions are labeled as upstream. |
| H-10 | Confirmed | Website setup uses this repository and its binary instead of installing the crates.io beginner course. |
| H-11 | Qualified | Rust 1.88 was a valid engine minimum but an insufficient course requirement. Declare course 1.89 separately, check it before compiling the verifier, test both minima, and use the course requirement in release validation. |
| H-12 | Confirmed | Commit the author lockfile, distribute its identical initialized copy, use locked Cargo commands, and check agreement in preflight. Windows checkout rules preserve their bytes. |
| H-13 | Confirmed, overlaps H-03 | The course host and nested verifier now use the same process-tree ownership helper. |
| M-01 | Confirmed | Quote the pane identifier as Markdown code. The Markdown check passes. |
| M-02 | Confirmed cost, not a correctness defect | Full independent audits are inherently more expensive than one repair. Preserve complete-input caching, provide focused mutation ranges and platform smoke checks, and require the exhaustive gate for author/release validation. |
| M-03 | Confirmed | Expand contracts beyond metadata and scrolling to materialization, progress, UI, watch orchestration, editors, and author validation. |
| M-04 | Confirmed | Exercise input/state transitions and terminal cleanup through test-only boundaries; use real child processes with an isolated fake Zellij. Live terminal/editor compatibility remains a manual boundary. |
| M-05 | Measurement was misinterpreted | The audit counted displayed source ranges. The 182-line initializer omitted two lines; the 149-line launcher omitted one. Actual changed reference lines had a maximum of 48 and median six. Narrow 14 real large gaps: the new maximum is 16 and median four. |
| M-06 | Confirmed | Each mission now has independently requested contract, diagnostic-region, and optional reference-operation hints. |
| M-07 | Partly justified curriculum judgment | Reduce actual reconstruction labor and retain surrounding code as context. Architecture checkpoints and transfer questions remain unscored practice, not a claim that compiler success proves design skill. |
| M-08 | Correct observation, deliberate scaffold | The CLI argument declarations remain supplied source. The source map and architecture record explicitly state that boundary; the launch mission and original integration tests exercise their connection to dispatch. Requiring every declaration to be retyped would change the chosen restoration scope. |
| M-09 | Confirmed claim limitation | Distinguish source-range exposure, tested contracts, negative examples, and exact reference identity. None proves universal behavioral equivalence or learner mastery. |
| M-10 | Confirmed identity ambiguity | Add separate course name, version, repository, minimum Rust, distribution metadata and `workshop info`. The preserved engine package version remains 6.5.0. |
| M-11 | Confirmed | Contributor instructions now document restoration invariants, source/probe boundaries and canonical author commands. |
| M-12 | Confirmed | Add actual course Rust 1.89 and original host Rust 1.88 jobs, including required components. |
| M-13 | Confirmed | Correct website path filters to match descendants and repair the deployment output reference. |
| M-14 | Confirmed | Add independent Markdown CI that includes root documents and chapter guides on PRs. |
| M-15 | Confirmed | The release hook invokes the portable canonical gate, including exact reconstruction, compiling mutations, and native author checks. |
| M-16 | Confirmed | Reconcile generated inputs, delete stale files, reject/fix generated symlink entries, and preserve expected source files. Tests demonstrate stale-input removal. |
| M-17 | Confirmed | Fingerprint OS/architecture, tool identity, compiler/build environment and Cargo configuration, including Windows Cargo-home discovery. Input equality still protects against hash collisions. |
| M-18 | Confirmed | Distinguish `REJECTED`, `LOCKED`, `INFRA_ERROR`, and `INFRA_TIMEOUT`, with separate exit codes. Infrastructure exits do not certify a repair. |
| M-19 | Confirmed | Add Windows/macOS reference, starter, solution and export smoke checks alongside exhaustive Linux checks. These caught real Windows line-ending defects and newer Clippy incompatibilities. |
| M-20 | Confirmed | The canonical driver is Rust and exposed as `rustlings workshop`; Bash is only a convenience wrapper. |
| M-21 | Confirmed | Document a single preparation command that performs preflight and reference verification, followed by the normal Rustlings experience. |
| M-22 | Confirmed usability issue | Rejections identify the editable mission and mapped original module before showing raw reconstructed-source diagnostics. The troubleshooting guide explains both paths. |
| L-01 | Confirmed | Separate learner instructions, architecture, maintenance, export, troubleshooting and source-map documents. |
| L-02 | Confirmed | Root and website entry points identify the same course and link to its architecture record. |
| L-03 | Confirmed hardening opportunity | Pin workflow actions to verified commit SHAs. |
| L-04 | Confirmed hardening opportunity | Check the official Zola archive digest before extraction. |
| L-05 | Confirmed | Add portable preflight for the course toolchain, Cargo, Clippy, rustfmt, Git and lockfile agreement; document the native linker requirement. |

## Verification and limits

The execution-policy checkpoint [run 35864736565](https://github.com/QuasarRay/rustlings/actions/runs/35864736565) passed the full cold Linux course audit, Windows and macOS smoke checks, and the engine's Rust 1.88 job. Subsequent PRs add the expanded behavioral suite and mandatory mutation gate; their own Checks and uploaded evidence are the authority for those later commits.

The expanded reference suite passes locally on Rust 1.89: strict Clippy, 45 debug unit tests, eight original integration tests, and the focused release-profile solution-visibility contract. The author gate requires 112 independent starter rejections, 112 passing canonical repairs, byte equality for 266 archived files, and 131 compiling negative examples. See [maintainer validation](maintenance.md) for report formats, focused commands, and how to distinguish an escaped fault from an invalid mutant. The current mutation run and latest composed CI result must be checked before merging the stack.

The 14 narrowed starters were each independently rejected with all other repairs correct. Manifest/starter agreement and the 112 three-stage hint entries were checked. Markdown and relative document links were checked. Windows-style checkout simulation exposed and verified byte preservation for archives and the two course lockfiles.

This work does not change the archived engine's known assumptions, certify ISO conformance, turn open Rustlings exercises into a secure exam, or prove correctness for all terminal environments and possible learner implementations. Those are scope boundaries, not failures concealed by a passing reference run.
