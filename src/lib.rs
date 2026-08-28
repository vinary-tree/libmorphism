//! Formally grounded semantic contracts for composable transformations.
//!
//! libmorphism separates three concerns:
//!
//! - compact immutable descriptions of a transformation's endpoints and claims;
//! - pure diagnostic composition checks; and
//! - private-field witnesses that safe callers can obtain only through validation.
//!
//! The crate is `no_std`, dependency-free, and allocation-free. Category-theoretic laws are
//! established in the repository's Rocq development and connected to this implementation by
//! exhaustive finite-domain tests.

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_code)]

mod claim;
mod composition;
mod descriptor;
mod effect;
mod evidence;
mod id;
mod provenance;
mod signature;
mod typed;
mod validation;

pub use claim::{Completeness, Precision};
pub use composition::{
    CompositionCheck, CompositionProvenance, CompositionSummary, CompositionWitness,
    EndpointMismatch, check_composition, validate_composition,
};
pub use descriptor::MorphismDescriptor;
pub use effect::{EffectSet, InvalidEffectBits};
pub use evidence::{EvidenceVerifier, LawEvidence, LawKind};
pub use id::{ArtifactId, DomainId, ID_BYTE_LEN, MorphismId, VerifierId};
pub use provenance::Provenance;
pub use signature::Signature;
pub use typed::{Domain, EndpointRole, TypedEndpointError, TypedMorphism, compose_typed};
pub use validation::{
    EvidenceKindMismatch, EvidenceSubjectMismatch, EvidenceVerifierMismatch, MorphismCandidate,
    SignatureMismatch, SignatureRole, ValidatedMorphism, ValidationError, validate_morphism,
};
