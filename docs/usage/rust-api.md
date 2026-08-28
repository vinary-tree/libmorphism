# Rust API Workflow

## What the crate admits

libmorphism has two data layers and one authority layer:

1. `MorphismDescriptor`, `MorphismCandidate`, `LawEvidence`, and `CompositionCheck` are ordinary
   data. Callers may construct and inspect them, but their presence proves nothing.
2. `ValidatedMorphism` and `CompositionWitness` have private fields. Only validation functions
   construct them.
3. `TypedMorphism<Source, Target>` adds zero-storage static endpoint markers after checking their
   stable identifiers against a `ValidatedMorphism`.

The distinction is deliberate. Parsing, deserialization, caching, or receiving a descriptor from
another crate must not grant publication authority.

## Define stable domains

A domain marker implements `Domain` with one 256-bit identifier:

```rust
use libmorphism::{Domain, DomainId, ID_BYTE_LEN};

struct ParsedProgram;

impl Domain for ParsedProgram {
    const ID: DomainId = DomainId::new([1; ID_BYTE_LEN]);
}
```

The repeated byte is only an abbreviated example. Production identifiers need a reviewed naming
policy. When crates exchange artifacts, the same semantics must have the same identifier, and
different semantics must not share one.

## Describe and validate a morphism

Constructing `LawEvidence` only records an untrusted claim. An `EvidenceVerifier` supplies the
trust policy. Exact validation proceeds in a fixed fail-fast order:

```text
procedure VALIDATE(candidate, verifier)
    compare the independently claimed source endpoint
    compare the independently claimed target endpoint
    if the descriptor is a sound approximation
        return a witness without invoking the exactness verifier
    require exact-denotation evidence
    require the ExactDenotation law kind
    require the descriptor subject identity
    require the active verifier identity
    invoke the verifier for policy and proof-artifact checks
    return a private ValidatedMorphism
end procedure
```

The endpoint, law-kind, subject, and verifier checks occur before potentially expensive proof
verification. A sound approximation cannot promote itself to exact, but it also pays no exactness
verification cost.

The complete compiled example is
[`examples/validated_composition.rs`](../../examples/validated_composition.rs). It constructs two
exact, complete descriptors, validates their bound evidence, adds static endpoint markers, and
calls `compose_typed`.

## Choose a composition path

Use `check_composition` for ordinary descriptors received from an untrusted or dynamically typed
boundary. It returns diagnostic data and checks the middle identifier.

Use `validate_composition` for two `ValidatedMorphism` values. It repeats the dynamic middle check
and returns a private composition witness on success.

Use `compose_typed` for `TypedMorphism<A, B>` and `TypedMorphism<B, C>`. Both wrappers already
proved equality with `B::ID`, so the function constructs the witness without another endpoint
comparison. Generic dispatch is monomorphized and the marker types occupy no value storage.

## Interpret a binary witness

A `CompositionWitness` covers one ordered pair. Its `CompositionProvenance` stores `before` and
`after` explicitly, effect declarations combine by union, and precision and completeness combine
by conjunction.

libmorphism does not allocate an unbounded pipeline trace. lling-llang owns the plan and its full
provenance sequence; it validates each edge or uses typed edges and then flattens evidence in
stable plan order. This boundary preserves an allocation-free semantic core without losing
pipeline-level audit data.

## Treat identifiers as security inputs

The crate compares canonical bytes but does not compute them. A producing trust domain should
make `MorphismId` content-address all descriptor fields relevant to its policy, including endpoint
semantics, effects, precision, completeness, provenance, rewrite set, and procedure version when
those fields affect the claim. `LawEvidence::subject` then binds the proof to that identity.

No serialization feature exists in version 0.1. A future codec may deserialize only ordinary data
and must rerun validation; it must never deserialize private witnesses as authority.

## Verify a change

Run all implementation checks through the bounded wrapper:

```text
scripts/verify-rust.sh
```

The wrapper uses one Cargo job, a 4 GiB resident-memory cap, no swap, and repository-local build,
log, and temporary paths. It checks formatting, `no_std`, all targets, Clippy, tests, rustdoc, and
the formal-to-Rust correspondence matrix.
