# Diagnose a stuck mission

| Observation | Meaning and next action |
| --- | --- |
| `LOCKED: complete or recheck ...` | An earlier source, verifier, or toolchain changed. Recheck the indicated mission and continue in order. |
| Error in `target/workshop/engine/src/...` | This is the real reconstructed code. Fix the mapped function in the mission file. |
| A missing binding or incompatible type | Complete the operation in the mapped repair region; use Rust's type and ownership explanation. |
| Compilation succeeds but a test fails | Investigate state, boundaries, command arguments, and exit status. Read the named contract probe. |
| `INFRA_TIMEOUT` or `INFRA_ERROR` | The verifier could not judge the repair. Check tool availability, network access for uncached dependencies, and machine load, then retry. Infrastructure failures never become cached rejections. |
| Another course check owns the compiler lock | Wait for that check to finish and retry. The OS releases the lock automatically if its verifier exits; the lock file itself can remain. |
| A course file or reference input was deleted | Reinitialize a separate course directory with the same fork binary and restore the missing fixture. Preserve your edited missions. |
| An editor save changes several missions | Check them in order. Saved completion flags do not replace source-specific verification receipts. |

`target/workshop` contains disposable build artifacts, reports, and source-specific completion receipts. Removing it requires preparation and rechecking solved missions; it never removes learner source.

## Read the original failure

The course checks compilation, behavioral contracts and then strict Clippy. Rust's errors and suggestions are preserved. An `Editable location` links a generated source coordinate back to the corresponding mission line. The `Repair` line identifies the active region even when the failure is in a caller or a contract test.

For type, ownership and API mistakes, use the compiler's explanation and applicable suggestions. For a compiling logic bug, read the named failing test, its input, and the actual/expected values. A compiler cannot infer the intended algorithm from every wrong result. The course supplies contract evidence, not replacement implementations. Ordinary lint suggestions become the final cleanup step after behavior passes.

## Expand a saved backtrace

Every rejected check saves the full, unedited tool report in `target/workshop/diagnostics/`. The default terminal view keeps the cause and assertions while hiding numbered stack frames. After a failure, inspect that same report without recompiling:

```sh
rustlings workshop trace catalog_defaults 1
rustlings workshop trace catalog_defaults 2
rustlings workshop trace catalog_defaults 3
rustlings workshop trace catalog_defaults full
```

In a source checkout use `cargo run --locked -- workshop trace ...`. Each numeric level reveals another 20 frames **per panic**. `full` shows the original report unchanged. The report belongs to the last failed check; rerun the mission after editing its source. A compile error normally has no runtime backtrace.

The same depth can be selected for the ordinary Rustlings view on POSIX shells:

```sh
RUST_BACKTRACE=2 cargo run --locked -- --no-editor
```

In PowerShell, set `$env:RUST_BACKTRACE = '2'` before launching Rustlings. Changing this display setting does not invalidate completed prerequisite receipts.

Numeric depth is a **course display convention**. Native Rust recognizes `0`, a short trace with `1`, and `full`; `2`, `3`, etc. do not natively select deeper traces. The course captures native full traces and limits their presentation. Original source, exported Rustlings and ordinary courses retain Rust's native semantics. No additional stack frames are invented, and no solutions are printed.
