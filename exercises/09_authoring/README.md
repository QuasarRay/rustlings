# Chapter 9: The Course Architect

Restore the original community-course creation and author-quality gates.

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
| 097 [author_names](author_names.rs) | BUG | Permit alphanumeric characters and underscores in exercise names. Report the first forbidden character. | `src/dev/check.rs::forbidden_char` |
| 098 [author_manifest](author_manifest.rs) | TODO | Generate the expected binary registration and compare it against the manifest without modifying that manifest. | `src/dev/check.rs::check_cargo_toml` |
| 099 [author_catalog](author_catalog.rs) | TODO | Restore one validation stage for unique names, hints, paths, TODO guidance, and declared test presence. | `src/dev/check.rs::check_info_file_exercises` |
| 100 [author_file_inventory](author_file_inventory.rs) | TODO | Reject undeclared files and excessive directory nesting while allowing chapter READMEs and declared inputs. | `src/dev/check.rs::check_unexpected_files` |
| 101 [author_unsolved](author_unsolved.rs) | TODO | Run every non-exempt starter and reject an already solved exercise; propagate worker failures. | `src/dev/check.rs::check_exercises_unsolved` |
| 102 [author_format](author_format.rs) | TODO | Validate the format version and combine metadata, directory, and unsolved-starter checks. | `src/dev/check.rs::check_exercises` |
| 103 [author_solutions](author_solutions.rs) | TODO | Restore one solution-validation stage, preserving required-solution handling, strict Clippy, formatting, and unexpected-file checks. | `src/dev/check.rs::check_solutions` |
| 104 [author_gate](author_gate.rs) | TODO | Enforce catalog-size and manifest constraints before validating starters and solutions. | `src/dev/check.rs::check` |
| 105 [author_directories](author_directories.rs) | TODO | Create a relative project directory and attach its path to any failure. | `src/dev/new.rs::create_rel_dir` |
| 106 [author_files](author_files.rs) | TODO | Write generated project content to the named relative file and retain actionable error context. | `src/dev/new.rs::write_rel_file` |
| 107 [author_new_course](author_new_course.rs) | TODO | Restore one community-course initialization stage, including the original optional Git behavior. | `src/dev/new.rs::new` |
| 108 [author_update_manifest](author_update_manifest.rs) | TODO | Regenerate only the bin list and preserve the rest of the existing manifest. | `src/dev/update.rs::update_cargo_toml` |
| 109 [author_update](author_update.rs) | TODO | Update the proper manifest for official development or a community course without redesigning either workflow. | `src/dev/update.rs::update` |
| 110 [author_dispatch](author_dispatch.rs) | TODO | Dispatch New, Check, and Update through their original handlers and preserve the debug-build restriction. | `src/dev.rs::run` |

## Chapter checkpoint

How can a green learner checker coexist with a broken course? Why must authors test both deliberately failing starters and passing solutions?

Explain these before taking the next chapter. These reflection questions are not automatically scored; the Rust repairs are.

## Transfer to another subject

Design three assessed tasks for a subject you know: one TODO, one behavioral bug, and one integration failure. Specify independent rejection and success evidence.

The [course guide and architecture record](../README.md) describe the source-restoration model, the final export, and its limits.
