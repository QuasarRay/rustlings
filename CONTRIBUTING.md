# Contributing to the restoration course

Read the [architecture record](exercises/01_catalog/architecture.md) and [author validation guide](exercises/01_catalog/maintenance.md) first. The original upstream guidance below is background for ordinary Rustlings courses.

1. Preserve the pinned archive and exact reference reconstruction. Do not silently change the implementation being taught.
2. Keep mission IDs, names, source ranges, exercise/solution paths, Cargo targets, and registry metadata consistent. Repair ranges must not overlap.
3. Give each mission a specific contract and diagnostic hints. Keep the active gap small; surrounding source is supplied context.
4. Add behavioral probes, including a compiling wrong implementation that demonstrates rejection. A forbidden `todo!()` alone is not behavioral evidence.
5. Keep support inputs registered so `rustlings init` distributes them. Test a fresh installation when changing packaging.
6. Keep `dev/Cargo.lock` and `exercises/01_catalog/exercises.lock` identical. Update dependencies deliberately.
7. Run `cargo run --locked -- workshop audit`. CI also checks the course MSRV and Windows/macOS smoke paths.

## Upstream guidance: Contributing to Rustlings

First off, thanks for taking the time to contribute! ❤️

## LLM Usage Policy

Planning to use an LLM to contribute to Rustlings?
Please follow the rules:

- ✅ Allowed: Personal use. For example: research and help during debugging.
- ⚠️ Disclosure required: Code contributions that are fully or partially LLM-generated.
  Maintainers may be less inclined to review such contributions.
- ❌ Banned: Generating comments, issues or PR descriptions. We want to talk to you, not your LLM.

For more information, refer to the [LLM Usage Policy](https://forge.rust-lang.org/policies/llm-usage.html) of `rust-lang/rust`.

## Quick Reference

I want to …

- _report a bug!_ ➡️ [open an issue](#issues)
- _fix a bug!_ ➡️ [open a pull request](#pull-requests)
- _implement a new feature!_ ➡️ [open an issue to discuss it first, then a pull request](#issues)
- _add an exercise!_ ➡️ [read this](#adding-an-exercise)
- _update an outdated exercise!_ ➡️ [open a pull request](#pull-requests)

## Issues

You can [open an issue](https://github.com/rust-lang/rustlings/issues/new).
If you're reporting a bug, please include the output of the following commands:

- `cargo --version`
- `rustlings --version`
- `ls -la`
- Your OS name and version

## Pull Requests

You are welcome to open a pull request, but unless it is small and trivial, **please open an issue to discuss your idea first** 🙏🏼

Opening a pull request is as easy as forking the repository and committing your changes.
If you need any help with it or face any Git related problems, don't hesitate to ask for help 🤗

It may take time to review your pull request.
Please be patient 😇

When updating an exercise, check if its solution needs to be updated.

## Adding An Exercise

- Name the file `exercises/yourTopic/yourTopicN.rs`.
- Make sure to put in some helpful links, and link to sections of The Book in `exercises/yourTopic/README.md`.
- In the exercise, add a `// TODO: …` comment where user changes are required.
- Add a solution at `solutions/yourTopic/yourTopicN.rs` with comments explaining it.
- Add the [metadata for your exercise](#exercise-metadata) in the `rustlings-macros/info.toml` file.
- [Open a pull request](#pull-requests).

### Exercise Metadata

The exercise metadata should contain the following:

```toml
[[exercises]]
name = "yourTopicN"
dir = "yourTopic"
hint = """
A useful (multi-line) hint for your exercise.
Include links to a section in The Book or a documentation page."""
```

If your exercise doesn't contain any test, add `test = false` to the exercise metadata.
But adding tests is recommended.
