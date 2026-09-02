# Diagram Catalog

| Diagram | Editable source | Purpose |
|---|---|---|
| Algebra boundaries | [algebra-boundaries.puml](algebra-boundaries.puml) | Distinguishes monoids, semilattices, lattices, semirings, and valid implication edges |
| Precision lifecycle | [precision-lifecycle.puml](precision-lifecycle.puml) | Shows validation, no-promotion composition, publication, rejection, and cancellation |
| Ownership boundaries | [ownership-boundaries.puml](ownership-boundaries.puml) | Assigns semantic and runtime responsibilities across the Vinary libraries |
| Optimization pipeline | [optimization-pipeline.puml](optimization-pipeline.puml) | Shows the complete parse-to-witness optimization flow |
| Formal-first sequence | [formal-first-sequence.puml](formal-first-sequence.puml) | Shows the mandatory evidence gates before implementation |
| Rust refinement | [rust-refinement.puml](rust-refinement.puml) | Shows candidate validation, evidence authority, typed endpoints, and composition witnesses |
| Release provenance | [release-provenance.puml](release-provenance.puml) | Shows exact-tag validation, immutable publication, and public readback |

Run `scripts/render-diagrams.sh` after changing a PlantUML source. Commit each editable `.puml`
file with its current `.svg` rendering. `scripts/verify-docs.sh` renders all diagrams headlessly and
checks them with `vinary-doc-lint`.
