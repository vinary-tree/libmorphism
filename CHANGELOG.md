# Changelog

All notable changes to libmorphism are recorded here. This project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 - 2026-09-02

The first public release establishes the dependency-free, allocation-free, `no_std` semantic
contract shared by composable Vinary transformations.

### Added

- Fixed-size domain, morphism, verifier, and artifact identifiers.
- Immutable descriptors for signatures, conservative effects, precision, completeness, and
  provenance.
- Pure dynamic composition checks plus private-field validation and composition witnesses.
- Monomorphized evidence verification and zero-storage typed domain markers.
- Rocq proofs, TLA+ model checking and theorem proofs, Z3 countermodels, exhaustive finite-domain
  law tests, randomized refinement properties, and a small-stack stress test.
- Reproducible documentation rendering and fail-closed immutable release gates.
