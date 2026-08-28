//! Validate exact descriptors and compose them through static endpoint markers.

use core::fmt;

use libmorphism::{
    ArtifactId, Completeness, Domain, DomainId, EffectSet, EvidenceVerifier, ID_BYTE_LEN,
    LawEvidence, LawKind, MorphismCandidate, MorphismDescriptor, MorphismId, Precision, Provenance,
    Signature, TypedMorphism, VerifierId, compose_typed, validate_morphism,
};

struct Source;
struct Middle;
struct Target;

impl Domain for Source {
    const ID: DomainId = DomainId::new([1; ID_BYTE_LEN]);
}

impl Domain for Middle {
    const ID: DomainId = DomainId::new([2; ID_BYTE_LEN]);
}

impl Domain for Target {
    const ID: DomainId = DomainId::new([3; ID_BYTE_LEN]);
}

#[derive(Debug)]
struct PolicyError;

impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("proof artifact or policy version is not trusted")
    }
}

impl std::error::Error for PolicyError {}

struct Policy {
    id: VerifierId,
    trusted_proof: ArtifactId,
}

impl EvidenceVerifier for Policy {
    type Error = PolicyError;

    fn verifier_id(&self) -> VerifierId {
        self.id
    }

    fn verify(
        &self,
        _descriptor: &MorphismDescriptor,
        evidence: &LawEvidence,
    ) -> Result<(), Self::Error> {
        if evidence.policy_version() == 1 && evidence.proof_artifact() == self.trusted_proof {
            Ok(())
        } else {
            Err(PolicyError)
        }
    }
}

fn validated<From, To>(tag: u8, effects: EffectSet, policy: &Policy) -> TypedMorphism<From, To>
where
    From: Domain,
    To: Domain,
{
    let descriptor = MorphismDescriptor::new(
        MorphismId::new([tag; ID_BYTE_LEN]),
        Signature::new(From::ID, To::ID),
        effects,
        Precision::Exact,
        Completeness::Complete,
        Provenance::new(ArtifactId::new([tag.wrapping_add(16); ID_BYTE_LEN]), 1, 0),
    );
    let evidence = LawEvidence::new(
        descriptor.id(),
        LawKind::ExactDenotation,
        policy.id,
        1,
        policy.trusted_proof,
    );
    let candidate = MorphismCandidate::from_descriptor(descriptor).with_exact_evidence(evidence);
    let dynamic = validate_morphism(candidate, policy).expect("example policy accepts the proof");
    TypedMorphism::try_new(dynamic).expect("descriptor IDs agree with marker IDs")
}

fn main() {
    let policy = Policy {
        id: VerifierId::new([90; ID_BYTE_LEN]),
        trusted_proof: ArtifactId::new([91; ID_BYTE_LEN]),
    };
    let before = validated::<Source, Middle>(10, EffectSet::READS_STATE, &policy);
    let after = validated::<Middle, Target>(11, EffectSet::EMITS_EVIDENCE, &policy);

    let witness = compose_typed(&after, &before);
    assert_eq!(
        witness.summary().signature(),
        Signature::new(Source::ID, Target::ID)
    );
    assert_eq!(
        witness.summary().effects(),
        EffectSet::READS_STATE.union(EffectSet::EMITS_EVIDENCE)
    );
}
