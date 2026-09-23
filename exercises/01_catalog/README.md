# Chapter 1: The Cartographer

Turn declarative course metadata into stable exercise identities, source paths, and Cargo targets.

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
| 001 [catalog_paths](catalog_paths.rs) | TODO | Build an exercise path for both a root-level file and a directory. Preserve the exercises/ prefix and .rs suffix. | `src/info_file.rs::path` |
| 002 [catalog_defaults](catalog_defaults.rs) | BUG | Missing test metadata must enable tests. Missing strict_clippy metadata must remain false. | `src/info_file.rs::default_true` |
| 003 [catalog_loading](catalog_loading.rs) | TODO | Prefer a local community catalog. Fall back to the embedded catalog only for NotFound; reject malformed or empty local catalogs. | `src/info_file.rs::parse` |
| 004 [solution_paths](solution_paths.rs) | TODO | Produce the matching solutions/ path without losing the optional directory or file extension. | `src/exercise.rs::sol_path` |
| 005 [manifest_boundaries](manifest_boundaries.rs) | BUG | Locate only the contents of bin = [...]. Report missing delimiters; offsets are UTF-8 byte offsets. | `src/cargo_toml.rs::bins_start_end_ind` |
| 006 [manifest_targets](manifest_targets.rs) | TODO | Register each exercise and only solutions that exist. Respect path prefixes and preserve catalog order. | `src/cargo_toml.rs::append_bins` |
| 007 [manifest_update](manifest_update.rs) | TODO | Replace the bin-list contents while preserving all manifest text before and after the list. | `src/cargo_toml.rs::updated_cargo_toml` |

## Chapter checkpoint

Why does a catalog contain names and flags instead of executable Rust callbacks? What breaks if catalog order changes after saving progress?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Create a paper catalog for a SQL course: name, input fixture, expected behavior, hint, and solution. Keep it as a design exercise; the assessed code remains Rustlings.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
