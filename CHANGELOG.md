# Changelog

All notable changes to libmorphism are recorded here. This project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.2 - 2026-09-02

This is the first crates.io release and the first GitHub release published under
repository-enforced release immutability. It contains the semantic contract introduced by the
earlier GitHub validation releases.

### Fixed

- Enabled repository-enforced immutable releases before publication.
- Changed GitHub publication to create a draft, attach every checksummed asset, publish the draft,
  and reject a public record unless it reports `immutable: true`.

## 0.1.1 - 2026-09-02 (GitHub-only validation release)

This GitHub-only validation release establishes the dependency-free, allocation-free, `no_std`
semantic contract shared by composable Vinary transformations. It was not published to crates.io.

### Added

- Fixed-size domain, morphism, verifier, and artifact identifiers.
- Immutable descriptors for signatures, conservative effects, precision, completeness, and
  provenance.
- Pure dynamic composition checks plus private-field validation and composition witnesses.
- Monomorphized evidence verification and zero-storage typed domain markers.
- Rocq proofs, TLA+ model checking and theorem proofs, Z3 countermodels, exhaustive finite-domain
  law tests, randomized refinement properties, and a small-stack stress test.
- Reproducible documentation rendering and fail-closed immutable release gates.

### Fixed

- Kept intermediate release artifacts beneath ignored, repository-backed `target/` storage so
  repeated packaging and registry reproduction operate only on clean source trees.

### Release status

- Published checksummed GitHub assets before the repository immutable-release setting was enabled.
  GitHub therefore reports this release as mutable; version 0.1.2 supersedes it.

## 0.1.0 - 2026-09-02 (unpublished validation candidate)

- Preserved the immutable validation tag that exposed the release-artifact path defect.
- Completed all non-publishing contract, Rust, MSRV, and Rocq gates; no registry or GitHub release
  was created from this candidate.
