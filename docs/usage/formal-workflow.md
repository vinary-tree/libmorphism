# Formal Verification Workflow

## Prerequisites

The verification suite expects Rocq with `coqc`, the TLA+ `tla2sany` and `tlc` launchers, TLAPS
with `tlapm`, Z3, systemd user scopes, ripgrep, PlantUML, `jq`, and `vinary-doc-lint`. The scripts
fail closed when a required executable is absent.

## Verify a change

Run the complete semantic gate from the repository root:

```text
scripts/verify-formal.sh all
```

For a focused edit, select one layer:

```text
scripts/verify-formal.sh rocq
scripts/verify-formal.sh tla
scripts/verify-formal.sh tlaps
scripts/verify-formal.sh z3
```

The script re-executes itself under systemd, so callers do not need to construct the memory scope.
Do not bypass the wrapper for routine work. Captured logs appear under `target/verification/`.

For documentation:

```text
scripts/verify-docs.sh
```

That command renders every PlantUML source headlessly and requires `vinary-doc-lint` to report no
diagnostics and no pending changes.

## Change protocol

1. Define the semantic claim and its observable consequences in the relevant theory document.
2. Add or revise the Rocq theorem without axioms or admissions.
3. If the claim concerns lifecycle or concurrency, revise the TLA+ behavior and invariants.
4. Add a negative model for each plausible but rejected strengthening.
5. Run the focused gate while iterating, then run `all`.
6. Update the refinement and ownership documentation.
7. Run the documentation gate and `pgmcp bug-gate` before committing.
8. Record theorem names, model counts, solver outcomes, resource limits, and the commit in pgmcp.

Changing a TLC state count requires explaining whether the reachable behavior changed or merely its
encoding. Changing an expected Z3 `sat` or `unsat` result requires statement review, not only a
script update.

## Add a future Rust implementation

The current repository intentionally has no Cargo package. Before adding one, create a formal
refinement task and document:

- which endpoint errors become unrepresentable statically and which remain dynamic;
- how private validation-witness construction refines the Rocq dependent pair;
- how effect, precision, and completeness values are represented without layout ambiguity;
- which law-bearing traits are sealed and how implementations are tested;
- which APIs are allocation-free and monomorphized on hot paths;
- how serialization revalidates untrusted data; and
- which bounded and exhaustive checks connect implementation behavior to the formal model.

No Rust API should precede that mapping.

## Diagnose resource failures

Use `systemctl --user status` on the unit name printed by `systemd-run` and inspect the captured
log. A process killed by the 4 GiB cap has failed verification. Do not rerun it uncapped. Reduce the
model, select a more appropriate proof strategy, or request a reviewed cap change with evidence.

Heap profiling is always headless. Record with `heaptrack --record-only`, then inspect the capture
using `heaptrack_print`.
