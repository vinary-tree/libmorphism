# Formal Verification Guide

The formal development establishes the laws that any future libmorphism implementation must
refine. It deliberately combines proof, exhaustive transition exploration, and independent
countermodels so that one tool is never asked to establish every kind of claim.

## Proof layers

| Layer | Source | Establishes |
|---|---|---|
| Rocq | `rocq/Composition.v` | Typed partial composition, identities, associativity, effects, precision, completeness, and proof-carrying validation |
| Rocq | `rocq/Algebra.v` | Monoid, semilattice, lattice, semiring, homomorphism, and order-embedding laws |
| Rocq | `rocq/IndexedFamilies.v` | Indexed feeds as dependent families and a constructive non-fibration counterexample |
| Rocq | `rocq/Countermodels.v` | Concrete refutations of invalid algebraic identifications |
| Rocq | `rocq/RustRefinement.v` | Stable identifiers, Rust descriptor mapping, evidence binding, and private composition and validation witnesses |
| TLA+ and TLC | `tla/PrecisionLifecycle.tla` | Exhaustive validation, composition, publication, rejection, and cancellation lifecycle |
| TLAPS | `tla/PrecisionLifecycle.tla` | Transition-local exactness and witness implications used by the lifecycle |
| Z3 | `smt/contracts.smt2` | Independent unsatisfiability checks and satisfiable countermodels |

Every named Rocq result used as evidence ends with `Print Assumptions`. The verification runner
requires exactly eighteen reports of `Closed under the global context` and rejects `Axiom`,
`Parameter`, `Admitted`, or `admit` in proof sources. This is a syntactic guard plus Rocq's own
kernel check; it is not a substitute for reviewing theorem statements.

## Reproduce the evidence

```text
scripts/verify-formal.sh rocq
scripts/verify-formal.sh tla
scripts/verify-formal.sh tlaps
scripts/verify-formal.sh z3
scripts/verify-formal.sh all
```

Each invocation self-enters a headless systemd scope with `MemoryMax=4G`, `MemorySwapMax=0`,
`CPUQuota=400%`, and `TasksMax=64`. Java-based TLA+ checks also receive a 1 GiB heap limit.
Evidence is captured under `target/verification/`.

## Trust statement

Rocq's kernel checks constructive proofs. TLC exhaustively explores the finite lifecycle model but
does not prove unbounded implementation behavior. TLAPS checks three logical kernels in the TLA+
module. Z3 validates deliberately separate first-order encodings. The trusted computing base is
therefore the toolchain, each formalization, and the future refinement argument—not merely a green
command exit.

The production mapping is specified in the
[Rust refinement argument](../docs/science/rust-refinement.md) and indexed by the
[machine-checkable correspondence matrix](refinement.tsv). An implementation claim is accepted
only when the formal gate and every mapped Rust test pass together.
