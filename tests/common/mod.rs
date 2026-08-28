use std::{cell::Cell, error::Error, fmt};

use libmorphism::{
    ArtifactId, Completeness, DomainId, EffectSet, EvidenceVerifier, ID_BYTE_LEN, LawEvidence,
    LawKind, MorphismCandidate, MorphismDescriptor, MorphismId, Precision, Provenance, Signature,
    ValidatedMorphism, VerifierId, validate_morphism,
};

pub const POLICY_VERSION: u64 = 7;

pub fn domain(byte: u8) -> DomainId {
    DomainId::new([byte; ID_BYTE_LEN])
}

pub fn morphism_id(byte: u8) -> MorphismId {
    MorphismId::new([byte; ID_BYTE_LEN])
}

pub fn artifact(byte: u8) -> ArtifactId {
    ArtifactId::new([byte; ID_BYTE_LEN])
}

pub fn verifier_id(byte: u8) -> VerifierId {
    VerifierId::new([byte; ID_BYTE_LEN])
}

pub fn descriptor(
    tag: u8,
    source: DomainId,
    target: DomainId,
    effects: EffectSet,
    precision: Precision,
    completeness: Completeness,
) -> MorphismDescriptor {
    MorphismDescriptor::new(
        morphism_id(tag),
        Signature::new(source, target),
        effects,
        precision,
        completeness,
        Provenance::new(artifact(tag.wrapping_add(64)), 3, u64::from(tag)),
    )
}

pub fn evidence_for(descriptor: &MorphismDescriptor, verifier: VerifierId) -> LawEvidence {
    LawEvidence::new(
        descriptor.id(),
        LawKind::ExactDenotation,
        verifier,
        POLICY_VERSION,
        artifact(240),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerificationFailure;

impl fmt::Display for VerificationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("test verifier rejected evidence")
    }
}

impl Error for VerificationFailure {}

pub struct TestVerifier {
    id: VerifierId,
    accepts: bool,
    calls: Cell<usize>,
}

impl TestVerifier {
    pub fn accepting(id: VerifierId) -> Self {
        Self {
            id,
            accepts: true,
            calls: Cell::new(0),
        }
    }

    pub fn rejecting(id: VerifierId) -> Self {
        Self {
            id,
            accepts: false,
            calls: Cell::new(0),
        }
    }

    pub fn calls(&self) -> usize {
        self.calls.get()
    }
}

impl EvidenceVerifier for TestVerifier {
    type Error = VerificationFailure;

    fn verifier_id(&self) -> VerifierId {
        self.id
    }

    fn verify(
        &self,
        _descriptor: &MorphismDescriptor,
        evidence: &LawEvidence,
    ) -> Result<(), Self::Error> {
        self.calls.set(self.calls.get() + 1);
        if self.accepts && evidence.policy_version() == POLICY_VERSION {
            Ok(())
        } else {
            Err(VerificationFailure)
        }
    }
}

pub fn validate_descriptor(
    descriptor: MorphismDescriptor,
    verifier: &TestVerifier,
) -> ValidatedMorphism {
    let candidate = if descriptor.precision().is_exact() {
        let evidence = evidence_for(&descriptor, verifier.verifier_id());
        MorphismCandidate::from_descriptor(descriptor).with_exact_evidence(evidence)
    } else {
        MorphismCandidate::from_descriptor(descriptor)
    };
    validate_morphism(candidate, verifier).expect("test descriptor should validate")
}
