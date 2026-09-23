# Diagnose a stuck mission

| Observation | Meaning and next action |
| --- | --- |
| `LOCKED: complete or recheck ...` | An earlier source, verifier, or toolchain changed. Recheck the indicated mission and continue in order. |
| Error in `target/workshop/engine/src/...` | This is the real reconstructed code. Fix the mapped function in the mission file. |
| `todo!` or a missing binding | Implement the omitted operation; removing the marker without restoring behavior is insufficient. |
| Compilation succeeds but a test fails | Investigate state, boundaries, command arguments, and exit status. Read the named contract probe. |
| `INFRA_TIMEOUT` or `INFRA_ERROR` | The verifier could not judge the repair. Check tool availability, network access for uncached dependencies, and machine load, then retry. Infrastructure failures never become cached rejections. |
| Another course check owns the compiler lock | Wait for that check to finish and retry. The OS releases the lock automatically if its verifier exits; the lock file itself can remain. |
| A course file or reference input was deleted | Reinitialize a separate course directory with the same fork binary and restore the missing fixture. Preserve your edited missions. |
| An editor save changes several missions | Check them in order. Saved completion flags do not replace source-specific verification receipts. |

`target/workshop` contains disposable build artifacts, reports, and source-specific completion receipts. Removing it requires preparation and rechecking solved missions; it never removes learner source.
