# Chapter 10: The Maintainer

Restore embedding and launch dispatch, then assemble the unchanged upstream project from all repairs.

This chapter assumes you can already use Rust ownership, traits, iterators, error handling, and tests. The new work is understanding and repairing an application architecture.

## Play loop

1. Read the contract and the surrounding real implementation. Predict the failure before running it.
2. Run the named mission. Read the compiler error or failed regression test.
3. Repair only the Rust between the repair markers, then save.
4. Ask for a hint with `h` if needed. Compare the revealed solution after completion.
5. Explain the data flow and failure mode aloud, then press `n`.

## Missions

| Mission | Kind | Restore | Source |
| --- | --- | --- | --- |
| 111 [embedded_macro](embedded_macro.rs) | TODO | Restore a stage of the real procedural macro: ordered exercises, solutions, directory indices, inputs, and READMEs must remain aligned. | `rustlings-macros/src/lib.rs::include_files` |
| 112 [launch_dispatch](launch_dispatch.rs) | TODO | Restore the missing launch stage, preserving priority commands, initialization checks, normal CLI dispatch, and exit status. Then reconstruct and run the complete checker. | `src/main.rs::main` |

## Chapter checkpoint

Can you trace an edit from the filesystem event to Cargo and back to the progress file? Which code is supplied scaffold and which code did you restore?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Explain how to author another subject using Rustlings dev new. Keep changes to the subject in its own course; do not change this final Rustlings implementation.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
