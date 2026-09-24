# Integrated external assessment kernel

The normal Rustlings compile/test/Clippy and watch workflow remains the student
interface. When the trainer sets `NCP_GRADER_CONFIG` to an absolute JSON path,
each successful exercise program must also pass an independent external grader.
Unknown exercise names, malformed receipts, absent required facets, a stale
attempt, wrong exercise, wrong execution class, missing domain or failed coupling
cannot be accepted. The receipt is read after the child exits successfully and
within the configured deadline.

Example trainer configuration (paths and masks are supplied by the course):

```json
{
  "protocol": "ncp-grade-v1",
  "program": "/srv/ncp/bin/ncp-grade",
  "args": ["--bank", "/srv/ncp/private/bank.json"],
  "timeout_secs": 120,
  "exercises": {"e01a": {"id": 0, "tier": 1, "required_facets": 7}}
}
```

The grader reads one JSON request on stdin containing `protocol`, `name`,
`exercise` and a nonzero `attempt`. It returns exactly one JSON receipt with those
fields and `tier`, `ops`, `net`, `infra`, `coupled`, `provenance`,
`observed_facets`, `feedback`. Domain statuses are pending=0, fail=1, pass=2,
blocked=3. Execution classes are model=1, emulated=2, upstream=3, product=4;
they require an exact match, not an assumed fidelity hierarchy.

`kernel.rs` is the production decision source. The Verus generator inserts only
function contracts into those exact bodies, hashes its input and rejects an
export without a contract. Kani checks the same crate. There are no admitted
proofs, external-body annotations or disabled overflow/unwinding checks.
`reserve` is also used by the native topology compiler: it accepts a positive
resource request exactly when remaining capacity permits it, adds that exact
quantity without overflow, and otherwise leaves usage unchanged.
These functions contain no loops; their symbolic checks cover their full
machine-integer input widths. The facet proof quantifies over all 64 bit positions.

Reproduce: run `cargo kani -p ncp-assessment-kernel`; generate a Verus input with
`python3 lab-kernel/proofs/generate_verus.py OUTPUT.rs`, then pass that file to
the pinned Verus binary. The workflow records both tools' output.

## Trust and scope

The kernel is not a complete proof of Rustlings, JSON parsing, subprocesses,
timekeeping, observation authenticity, the OS or any NVIDIA product. The Verus
source transformation, Rust compiler and verifier trusted bases are assumptions.
The `advance` function is available for an authoritative trainer progression
ledger; the existing Rustlings local UI progress file is not such a ledger.

The receipt nonce prevents accidental stale/cross-exercise acceptance; it is not
a signature or an authentication scheme. A hostile student running under the
trainer's identity can change files, environment variables, program binaries or
the grader. For assessment, run student code in an unprivileged Incus container,
keep the observer and configuration outside it, and grade independently of the
student's editable Rustlings progress file. Export prompts and submitted code,
not hidden oracle data. Without `NCP_GRADER_CONFIG`, ordinary Rustlings courses
keep their existing behavior; the course launcher must enforce its presence.

The external runner currently bounds the deadline and accepted output size but
does not sandbox the trusted grader or kill its entire process tree. Use a
service/cgroup boundary for a grader that spawns descendants. A proof of the
decision functions is not a proof that an arbitrary grader tells the truth.
# Proof-generation boundary

The generator rejects unmodeled top-level items, duplicate/missing functions,
attributes, calls and proof-bypass constructs before emitting a Verus file. Its
checks use exceptions and remain active under `python -O`. The supported syntax
is deliberately the current pure scalar kernel; an extension requires a reviewed
parser and contract change. Verus runs with `--no-cheating` in CI. The generator
and specifications remain trusted; this does not prove the Python transformer.
