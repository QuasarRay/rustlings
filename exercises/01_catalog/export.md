# Final boss: run the restored checker

After clearing every mission, compile the small export driver and export your implementation to a new directory:

```sh
rustc --edition=2024 exercises/01_catalog/grader.rs -o target/workshop/grader-cli
target/workshop/grader-cli . export ../restored-rustlings
cargo test --locked --manifest-path ../restored-rustlings/Cargo.toml
cargo run --manifest-path ../restored-rustlings/Cargo.toml -- --help
```

On Windows the driver executable has the usual `.exe` suffix. Run the engine's author gate **from the exported root**, because Rustlings resolves several paths against its working directory:

```sh
cd ../restored-rustlings
cargo dev check --require-solutions
```

The export is the original checker project with your restored functions and its original beginner-course data. It deliberately contains no workshop grader, course execution policy, or extra probes. Its original exercises provide a useful independent subject on which to use the checker you restored. The host fork adds an opt-in-by-course command policy; existing engine logic and the reference archive remain intact.

For an author-only reference export, append `solutions` to the export command. That explicitly uses reference repairs and does not grant learner progress.
