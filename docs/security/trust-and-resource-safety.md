# Trust Boundaries and Resource Safety

## Assets and adversaries

The protected assets are semantic correctness, exactness claims, completeness claims, validation
evidence, deterministic output, memory availability, and reproducible builds. Inputs may include
untrusted grammars, source programs, dictionary or distance feeds, rewrite rules, serialized
e-graphs, CPG data, plugin adapters, and resource estimates.

The threat model includes malformed input, a buggy adapter, an unsound rewrite, false exactness,
hash-collision denial of service, combinatorial saturation, integer overflow, unbounded task
creation, and memory exhaustion.

## Boundary rules

1. Parsed or deserialized values are data, never validation witnesses.
2. Endpoint identifiers are checked before dynamic composition.
3. Exactness and completeness can degrade but cannot self-promote.
4. Exact publication requires an independently constructed witness.
5. Algebraic law markers are sealed or constructed through reviewed implementations; callers
   cannot assert laws by setting a boolean.
6. Resource estimates are conservative, checked, and enforced before expansion.
7. E-graph rewrites carry stable identities, side conditions, and provenance.
8. Parallel tasks publish only through a deterministic commit boundary.

## Witness construction

The formal constructor uses a decidable validation predicate and returns a dependent pair only
when the predicate succeeds. A Rust refinement should make witness fields private and expose a
fallible validator. Deserialization must reconstruct the candidate and rerun validation; it must
not deserialize an authority-bearing witness directly.

Witnesses should bind at least the candidate digest, source and target identities, policy version,
rewrite-set digest, precision, completeness, and validation procedure version. If any bound field
changes, the witness is stale.

## Resource containment

Formal proofs are part of the threat surface because solvers and compilers can consume unbounded
memory. Repository scripts self-enter systemd scopes with a hard resident-memory limit, no swap,
bounded CPU, and bounded task count. Output is captured for audit. A failed or killed scope is a
failed gate, never an implicit pass.

Production orchestration should apply the same hierarchy: a campaign budget contains per-stage
budgets, which contain per-task budgets. Replete saturation needs node, e-class, iteration, time,
and memory ceilings. Parser generation needs grammar-size and recursion limits. Feed ingestion
needs item and payload limits. Graph analysis must validate dense-index and allocation arithmetic
before traversal.

## Concurrency hazards

An effect declaration is conservative authority, not merely documentation. Two operations may
share a wave only if their declared reads and writes are compatible or their state is demonstrably
partitioned. Cancellation must not publish partial mutations. Panics or task failures must leave
the last witnessed object intact.

Evidence order can itself be observable. Concurrent workers therefore return local evidence
buffers, and the coordinator merges those buffers in stable plan order after successful
validation.

## Supply-chain and migration boundary

Replete and PraTTaIL integrations remain blocked until their standalone migrations and review gates
complete. Temporary path dependencies or copying active in-progress code would bypass provenance
and make later reconciliation ambiguous. Rigail adapters must target its accepted standalone API,
not the historical embedded copy.

No crate is refactored merely to increase the number of shared dependencies. A dependency is
admitted only when ownership, law compatibility, performance, versioning, and failure behavior are
explicit.
