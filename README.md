# libmorphism

libmorphism is the formal, implementation-independent contract layer for composable Vinary
transformations. It defines what typed composition, identities, declared effects, precision,
completeness, lawful algebraic structure, indexed families, and validated witnesses must mean
before a production Rust interface is admitted.

The repository is intentionally formal-only at this milestone. Category theory is used as a
design language and a source of laws, not as a runtime object hierarchy. The future Rust surface
must refine these proofs with static endpoint types, concrete algebra traits, and zero-cost
witnesses. It must not add dynamic dispatch, allocate categorical wrappers in hot loops, or imply
that every indexed feed is a fibration.

## Start here

- [Composition and algebra theory](docs/theory/composition-and-algebra.md)
- [Ownership and optimization architecture](docs/architecture/ownership-and-pipeline.md)
- [Verification model](docs/science/verification-model.md)
- [Performance and concurrency](docs/engineering/performance-and-concurrency.md)
- [Trust and resource safety](docs/security/trust-and-resource-safety.md)
- [Formal verification workflow](docs/usage/formal-workflow.md)
- [Formal source guide](formal/README.md)
- [Diagram catalog](docs/diagrams/README.md)

Run `scripts/verify-formal.sh all` before adding or changing implementation-facing semantics.
The runner places every heavy proof command in a 4 GiB, no-swap systemd scope. Run
`scripts/verify-docs.sh` after every documentation change.

## Campaign role

libmorphism owns cross-project semantic vocabulary and proofs. It does not own lattice data
structures, weighted equations, e-graphs, parser generation, graph kernels, or orchestration.
Those responsibilities remain with `llattice`, Rigail, Replete, PraTTaIL, libvgraph, and
lling-llang respectively. This boundary lets each library optimize its concrete representation
without importing a universal runtime abstraction.

The formal milestone is tracked by pgmcp task `vco-e1-formal-contracts` under epic
`vinary-categorical-optimization-campaign`.

## License

Apache-2.0. See [LICENSE](LICENSE).
