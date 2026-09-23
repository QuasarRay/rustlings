# Reference solutions: Rebuild the Checker

These repairs contain the exact source intervals from the pinned Rustlings implementation. The normal initialized-course workflow reveals each solution after its mission is complete. Read the [course guide and architecture description](../exercises/README.md) before using them.

A solution check composes earlier reference repairs, so an author can test it independently of learner progress. Copying a solution's repair into its matching exercise still uses the exercise binary identity and therefore still checks the learner prerequisite chain. The `_sol` executable is an explicitly separate reference check.

The source archive and solutions are intentionally inspectable, as in ordinary Rustlings. They are not secret exam material. Explain the design and failure mode before comparing code.

For maintainer validation, run `bash exercises/01_catalog/audit.sh` and then `cargo dev check --require-solutions` from the checkout root. The audit proves that every planted defect fails in isolation and that all reference repairs reconstruct the pinned source byte for byte.
