use crate::{ArtifactId, MorphismDescriptor, MorphismId, VerifierId};

/// The semantic law supported by a proof artifact.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LawKind {
    /// A transformation behaves as an identity on its domain.
    Identity,
    /// A composition operation is associative for its declared observations.
    Associativity,
    /// A transformation exactly denotes its claimed semantics.
    ExactDenotation,
    /// A transformation's effect declaration conservatively covers its behavior.
    EffectSoundness,
}

/// Evidence bound to a descriptor, verifier, policy, and proof artifact.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LawEvidence {
    subject: MorphismId,
    kind: LawKind,
    verifier: VerifierId,
    policy_version: u64,
    proof_artifact: ArtifactId,
}

impl LawEvidence {
    /// Constructs an untrusted evidence record.
    ///
    /// Construction records a claim but does not validate it. Authority is conferred only when an
    /// [`EvidenceVerifier`] accepts the record during morphism validation.
    #[must_use]
    pub const fn new(
        subject: MorphismId,
        kind: LawKind,
        verifier: VerifierId,
        policy_version: u64,
        proof_artifact: ArtifactId,
    ) -> Self {
        Self {
            subject,
            kind,
            verifier,
            policy_version,
            proof_artifact,
        }
    }

    /// Returns the descriptor identity to which the evidence is bound.
    #[must_use]
    pub const fn subject(&self) -> MorphismId {
        self.subject
    }

    /// Returns the supported law.
    #[must_use]
    pub const fn kind(&self) -> LawKind {
        self.kind
    }

    /// Returns the expected verifier identity.
    #[must_use]
    pub const fn verifier(&self) -> VerifierId {
        self.verifier
    }

    /// Returns the verifier policy version.
    #[must_use]
    pub const fn policy_version(&self) -> u64 {
        self.policy_version
    }

    /// Returns the stable proof-artifact identity.
    #[must_use]
    pub const fn proof_artifact(&self) -> ArtifactId {
        self.proof_artifact
    }
}

/// A monomorphized trust boundary for law evidence.
///
/// Implementations decide which policy versions and proof artifacts they trust. The validator
/// independently checks subject, law kind, and verifier identity before calling [`Self::verify`].
pub trait EvidenceVerifier {
    /// A verifier-specific rejection reason.
    type Error;

    /// Returns this verifier's stable trust-domain identity.
    fn verifier_id(&self) -> VerifierId;

    /// Verifies the evidence against the complete immutable descriptor.
    ///
    /// # Errors
    ///
    /// Returns a verifier-specific reason when the policy or proof artifact is unacceptable.
    fn verify(
        &self,
        descriptor: &MorphismDescriptor,
        evidence: &LawEvidence,
    ) -> Result<(), Self::Error>;
}
