# Rust Refinement Argument

## Purpose and scope

The Rust package is a refinement of the constructive semantics in `formal/rocq`, not a second
source of truth. A *refinement* preserves every observation admitted by the abstract model while
choosing a concrete representation suitable for production. This document fixes that mapping
before the implementation is admitted.

libmorphism remains a semantic-law crate. It does not plan optimization passes, execute them,
schedule work, transport application binary interface payloads, manipulate e-graphs, or own
project-specific reports. Those responsibilities remain outside this package.

## Representation mapping

| Formal concept | Rust representation | Refinement obligation |
|---|---|---|
| `Object` | `DomainId` and `Domain` marker types | Equality is stable, deterministic, and independent of allocation or process identity. |
| `Signature` | `Signature { source, target }` | Source and target remain distinct and equality agrees with the formal decider. |
| `Effects` | `EffectSet` bit set | Empty is the identity and union is associative, commutative, and idempotent. Unknown bits cannot enter through the checked constructor. |
| `Precision` | two-case `Precision` enumeration | Composition is conjunction: exactness cannot be promoted. |
| `Completeness` | two-case `Completeness` enumeration | Composition is conjunction: completeness cannot be promoted. |
| `Morphism` | immutable `MorphismDescriptor` | Signature, effects, claims, and provenance are observed without hidden mutation. |
| partial `compose` | `check_composition` | A result exists exactly when the middle endpoint agrees; its signature uses the outer endpoints. |
| dependent validation witness | private-field `ValidatedMorphism` | Safe public code cannot construct the witness without running validation. |
| exact confirmation | verifier-accepted `LawEvidence` | Evidence is bound to the descriptor identifier, law kind, verifier, and policy. |
| typed endpoint witness | `TypedMorphism<Source, Target>` | Marker identifiers must equal the validated dynamic signature before construction. |

Stable identifiers are role-specific 256-bit values. They are not pointers, hashes selected by
this crate, or global registries. The caller chooses an identity scheme appropriate to its trust
domain and passes the resulting bytes explicitly.

## Exactness trust boundary

`Precision::Exact` is a strong semantic claim. A candidate may carry that claim, but it becomes a
`ValidatedMorphism` only when all of the following hold:

1. the candidate's claimed signature equals the immutable descriptor signature;
2. exact candidates carry `LawEvidence` of kind `ExactDenotation`;
3. the evidence subject equals the descriptor identifier;
4. the evidence verifier identifier equals the active verifier identifier; and
5. the verifier accepts both the policy version and proof artifact.

Non-exact candidates do not require exactness evidence. This asymmetry prevents accidental
promotion while keeping conservative transformations cheap. Evidence verification is generic and
monomorphized; the hot path does not require dynamic dispatch or heap allocation.

```text
procedure VALIDATE-MORPHISM(candidate, verifier)
    require candidate.claimed_signature = candidate.descriptor.signature
    if candidate.descriptor.precision = Exact
        require candidate.evidence is present
        require candidate.evidence.kind = ExactDenotation
        require candidate.evidence.subject = candidate.descriptor.id
        require candidate.evidence.verifier = verifier.id
        require verifier accepts(candidate.descriptor, candidate.evidence)
    return private ValidatedMorphism(candidate.descriptor, accepted evidence)
end procedure
```

The pseudocode is literate: each `require` names a separately testable rejection boundary, and the
last line explains why callers cannot bypass those boundaries with a struct literal.

## Composition refinement

For transformations `before : A -> B` and `after : B -> C`, `check_composition(after, before)`
returns the descriptor summary `A -> C`. The argument order deliberately matches ordinary function
composition.

```math
\mathrm{effects}(g \circ f)
  = \mathrm{effects}(f) \cup \mathrm{effects}(g)
```

```math
\mathrm{exact}(g \circ f)
  = \mathrm{exact}(f) \land \mathrm{exact}(g)
```

```math
\mathrm{complete}(g \circ f)
  = \mathrm{complete}(f) \land \mathrm{complete}(g)
```

Provenance remains ordered as `(before, after)`. It is not a mathematical commutative set: audit
consumers must be able to reconstruct which transformation ran first. `CompositionCheck` is a
diagnostic result and does not confer authority. `CompositionWitness` has private fields and is
created only by the dynamic validator or the statically typed composition function.

The witness covers one binary edge. libmorphism does not allocate an unbounded provenance trace;
lling-llang owns arbitrary plan sequences and flattens their binary evidence in stable execution
order. The proved associative observations used for regrouping are the outer signature, effect
union, precision, and completeness. Pipeline provenance order remains an orchestration invariant.

## Static endpoint layer

`TypedMorphism<Source, Target>` uses zero-sized marker types implementing `Domain`. The wrapper
retains a dynamic `ValidatedMorphism` because independently produced artifacts still need stable
runtime identities. Its constructor checks both marker identifiers once. Consequently, code that
accepts `TypedMorphism<A, B>` cannot accidentally substitute `TypedMorphism<A, C>`.

This is a hybrid refinement:

- marker types make endpoint mismatches visible to the Rust type checker inside a compiled
  pipeline;
- stable identifiers preserve interoperability across compilation units and persisted artifacts;
  and
- private witnesses preserve validation authority across both layers.

The phantom marker is representation-free. It adds no allocation and no per-value storage.

## Traceability and evidence

[`formal/refinement.tsv`](../../formal/refinement.tsv) is the machine-checkable correspondence
matrix. Each of its twelve rows names one formal claim, one Rust symbol, and one test. The
verification script
fails when a referenced path, theorem, symbol, or test disappears.

The correspondence tests are intentionally independent of implementation helpers. They enumerate
the finite precision, completeness, effect, and endpoint domains and compare public behavior with
small test oracles. This is not a proof that Rust implements Rocq's calculus for every possible
program; it is an executable refinement argument for the finite representations selected here.

The formal proof remains authoritative for the general law. The Rust tests establish that the
chosen encodings and constructors expose the proved behavior.

## Non-goals

- Serialization is not enabled in the initial package. A future codec must revalidate untrusted
  descriptors and may not deserialize private witnesses.
- Algebraic structures such as lattices and semirings are documented by the formal contract but
  are not forced into one universal runtime trait hierarchy.
- `LawEvidence` records verifier results; libmorphism does not define a proof-file format or trust
  a particular theorem prover.
- The crate does not claim that every indexed feed is a fibration.

## Threats to validity

- A stable identifier can be assigned incorrectly by a caller. The crate checks binding and
  equality, not the caller's naming policy.
- A verifier implementation is part of the trusted computing base for accepted exactness claims.
- Exhaustive tests cover finite value domains, but verifier-specific proof formats require their
  own test suites.
- Safe Rust privacy protects constructors; code using `unsafe`, process corruption, or a modified
  crate artifact lies outside this argument.
