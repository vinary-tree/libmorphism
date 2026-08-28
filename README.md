# libmorphism

libmorphism is the formally grounded, `no_std` semantic-law crate for composable Vinary
transformations. It provides compact endpoint identities, effect and result claims, immutable
descriptors, verifier-bound law evidence, dynamic composition checks, and private validation
witnesses. A zero-storage `Domain` marker layer makes already validated endpoints statically
composable.

Category theory is used as a design language and a source of machine-checked laws, not as a
runtime object hierarchy. The Rust core is dependency-free and allocation-free: it uses fixed-size
identifiers, compact enums and bit sets, monomorphized verifiers, and private-field witnesses. It
does not add dynamic dispatch, allocate categorical wrappers in hot loops, or imply that every
indexed feed is a fibration.

## Start here

- [Composition and algebra theory](docs/theory/composition-and-algebra.md)
- [Ownership and optimization architecture](docs/architecture/ownership-and-pipeline.md)
- [Verification model](docs/science/verification-model.md)
- [Performance and concurrency](docs/engineering/performance-and-concurrency.md)
- [Trust and resource safety](docs/security/trust-and-resource-safety.md)
- [Formal verification workflow](docs/usage/formal-workflow.md)
- [Rust API workflow](docs/usage/rust-api.md)
- [Rust refinement argument](docs/science/rust-refinement.md)
- [Formal source guide](formal/README.md)
- [Diagram catalog](docs/diagrams/README.md)

Run `scripts/verify-formal.sh all` before changing implementation-facing semantics, then run
`scripts/verify-rust.sh` and `scripts/verify-docs.sh`. Every wrapper uses a 4 GiB, no-swap systemd
scope and keeps its build, proof, log, and temporary state under the ignored repository `target/`
tree on persistent storage.

## Campaign role

libmorphism owns cross-project semantic vocabulary and proofs. It does not own lattice data
structures, weighted equations, e-graphs, parser generation, graph kernels, or orchestration.
Those responsibilities remain with `llattice`, Rigail, Replete, PraTTaIL, libvgraph, and
lling-llang respectively. This boundary lets each library optimize its concrete representation
without importing a universal runtime abstraction.

The formal baseline is tracked by pgmcp task `vco-e1-formal-contracts`; the production crate is
tracked by `vco-e1-libmorphism`, both under epic `vinary-categorical-optimization-campaign`.

## License

Apache-2.0. See [LICENSE](LICENSE).
