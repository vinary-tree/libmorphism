use core::{error::Error, fmt};

use crate::{
    DomainId, EvidenceVerifier, LawEvidence, LawKind, MorphismDescriptor, MorphismId, Precision,
    Signature, VerifierId,
};

/// Which endpoint disagreed with the immutable descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureRole {
    /// The source endpoint.
    Source,
    /// The target endpoint.
    Target,
}

/// A claimed signature that disagrees with the immutable descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignatureMismatch {
    role: SignatureRole,
    claimed: DomainId,
    descriptor: DomainId,
}

impl SignatureMismatch {
    /// Returns which endpoint disagreed.
    #[must_use]
    pub const fn role(self) -> SignatureRole {
        self.role
    }

    /// Returns the candidate's claimed endpoint identifier.
    #[must_use]
    pub const fn claimed(self) -> DomainId {
        self.claimed
    }

    /// Returns the immutable descriptor's endpoint identifier.
    #[must_use]
    pub const fn descriptor(self) -> DomainId {
        self.descriptor
    }
}

/// Evidence supports a different law than the validator requires.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceKindMismatch {
    expected: LawKind,
    actual: LawKind,
}

impl EvidenceKindMismatch {
    /// Returns the required law kind.
    #[must_use]
    pub const fn expected(self) -> LawKind {
        self.expected
    }

    /// Returns the supplied law kind.
    #[must_use]
    pub const fn actual(self) -> LawKind {
        self.actual
    }
}

/// Evidence is bound to a different descriptor identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceSubjectMismatch {
    expected: MorphismId,
    actual: MorphismId,
}

impl EvidenceSubjectMismatch {
    /// Returns the descriptor identity being validated.
    #[must_use]
    pub const fn expected(self) -> MorphismId {
        self.expected
    }

    /// Returns the descriptor identity named by the evidence.
    #[must_use]
    pub const fn actual(self) -> MorphismId {
        self.actual
    }
}

/// Evidence is bound to a different verifier identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceVerifierMismatch {
    expected: VerifierId,
    actual: VerifierId,
}

impl EvidenceVerifierMismatch {
    /// Returns the active verifier identity.
    #[must_use]
    pub const fn expected(self) -> VerifierId {
        self.expected
    }

    /// Returns the verifier identity named by the evidence.
    #[must_use]
    pub const fn actual(self) -> VerifierId {
        self.actual
    }
}

/// A failure to turn an untrusted candidate into a validated morphism.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationError<E> {
    /// The candidate duplicates the descriptor endpoints incorrectly.
    SignatureMismatch(SignatureMismatch),
    /// An exact descriptor did not carry exact-denotation evidence.
    MissingExactEvidence,
    /// Evidence supports a different law.
    EvidenceKindMismatch(EvidenceKindMismatch),
    /// Evidence names a different descriptor.
    EvidenceSubjectMismatch(EvidenceSubjectMismatch),
    /// Evidence names a different verifier.
    EvidenceVerifierMismatch(EvidenceVerifierMismatch),
    /// The selected verifier rejected the policy or proof artifact.
    EvidenceRejected(E),
}

impl<E: fmt::Display> fmt::Display for ValidationError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignatureMismatch(_) => {
                formatter.write_str("candidate signature does not match descriptor")
            }
            Self::MissingExactEvidence => {
                formatter.write_str("exact descriptor is missing exact-denotation evidence")
            }
            Self::EvidenceKindMismatch(_) => formatter.write_str("evidence supports the wrong law"),
            Self::EvidenceSubjectMismatch(_) => {
                formatter.write_str("evidence is bound to the wrong descriptor")
            }
            Self::EvidenceVerifierMismatch(_) => {
                formatter.write_str("evidence is bound to the wrong verifier")
            }
            Self::EvidenceRejected(error) => write!(formatter, "evidence rejected: {error}"),
        }
    }
}

impl<E> Error for ValidationError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EvidenceRejected(error) => Some(error),
            _ => None,
        }
    }
}

/// An untrusted descriptor, duplicated endpoint claim, and optional exactness evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MorphismCandidate {
    descriptor: MorphismDescriptor,
    claimed_signature: Signature,
    exact_evidence: Option<LawEvidence>,
}

impl MorphismCandidate {
    /// Constructs an untrusted candidate.
    #[must_use]
    pub const fn new(
        descriptor: MorphismDescriptor,
        claimed_signature: Signature,
        exact_evidence: Option<LawEvidence>,
    ) -> Self {
        Self {
            descriptor,
            claimed_signature,
            exact_evidence,
        }
    }

    /// Constructs a candidate whose duplicated signature agrees with its descriptor.
    #[must_use]
    pub fn from_descriptor(descriptor: MorphismDescriptor) -> Self {
        let claimed_signature = descriptor.signature();
        Self::new(descriptor, claimed_signature, None)
    }

    /// Attaches an untrusted exact-denotation evidence record.
    #[must_use]
    pub fn with_exact_evidence(mut self, evidence: LawEvidence) -> Self {
        self.exact_evidence = Some(evidence);
        self
    }

    /// Returns the immutable descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &MorphismDescriptor {
        &self.descriptor
    }

    /// Returns the independently supplied endpoint claim.
    #[must_use]
    pub const fn claimed_signature(&self) -> Signature {
        self.claimed_signature
    }

    /// Returns the untrusted exact-denotation evidence, when supplied.
    #[must_use]
    pub const fn exact_evidence(&self) -> Option<&LawEvidence> {
        self.exact_evidence.as_ref()
    }
}

/// A descriptor admitted through [`validate_morphism`].
///
/// Fields are private so safe callers cannot fabricate validation authority.
///
/// ```compile_fail
/// use libmorphism::ValidatedMorphism;
/// let forged = ValidatedMorphism { /* private fields */ };
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedMorphism {
    descriptor: MorphismDescriptor,
    exact_evidence: Option<LawEvidence>,
}

impl ValidatedMorphism {
    /// Returns the validated descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &MorphismDescriptor {
        &self.descriptor
    }

    /// Returns verifier-accepted exact-denotation evidence.
    ///
    /// Approximate descriptors always return `None`, even if their untrusted candidate carried an
    /// irrelevant evidence record.
    #[must_use]
    pub const fn exact_evidence(&self) -> Option<&LawEvidence> {
        self.exact_evidence.as_ref()
    }

    /// Consumes the witness and returns the immutable descriptor.
    #[must_use]
    pub fn into_descriptor(self) -> MorphismDescriptor {
        self.descriptor
    }
}

/// Validates an untrusted morphism candidate.
///
/// Approximate candidates do not call the verifier and discard irrelevant exactness evidence.
/// Exact candidates require evidence bound to the descriptor, the exact-denotation law, and the
/// active verifier before verifier-specific checks run.
///
/// # Errors
///
/// Returns [`ValidationError`] at the first failed trust boundary.
pub fn validate_morphism<V>(
    candidate: MorphismCandidate,
    verifier: &V,
) -> Result<ValidatedMorphism, ValidationError<V::Error>>
where
    V: EvidenceVerifier,
{
    let MorphismCandidate {
        descriptor,
        claimed_signature,
        exact_evidence,
    } = candidate;

    let descriptor_signature = descriptor.signature();
    if claimed_signature.source() != descriptor_signature.source() {
        return Err(ValidationError::SignatureMismatch(SignatureMismatch {
            role: SignatureRole::Source,
            claimed: claimed_signature.source(),
            descriptor: descriptor_signature.source(),
        }));
    }
    if claimed_signature.target() != descriptor_signature.target() {
        return Err(ValidationError::SignatureMismatch(SignatureMismatch {
            role: SignatureRole::Target,
            claimed: claimed_signature.target(),
            descriptor: descriptor_signature.target(),
        }));
    }

    if descriptor.precision() == Precision::SoundApproximation {
        return Ok(ValidatedMorphism {
            descriptor,
            exact_evidence: None,
        });
    }

    let evidence = exact_evidence.ok_or(ValidationError::MissingExactEvidence)?;
    if evidence.kind() != LawKind::ExactDenotation {
        return Err(ValidationError::EvidenceKindMismatch(
            EvidenceKindMismatch {
                expected: LawKind::ExactDenotation,
                actual: evidence.kind(),
            },
        ));
    }
    if evidence.subject() != descriptor.id() {
        return Err(ValidationError::EvidenceSubjectMismatch(
            EvidenceSubjectMismatch {
                expected: descriptor.id(),
                actual: evidence.subject(),
            },
        ));
    }
    let verifier_id = verifier.verifier_id();
    if evidence.verifier() != verifier_id {
        return Err(ValidationError::EvidenceVerifierMismatch(
            EvidenceVerifierMismatch {
                expected: verifier_id,
                actual: evidence.verifier(),
            },
        ));
    }
    verifier
        .verify(&descriptor, &evidence)
        .map_err(ValidationError::EvidenceRejected)?;

    Ok(ValidatedMorphism {
        descriptor,
        exact_evidence: Some(evidence),
    })
}
