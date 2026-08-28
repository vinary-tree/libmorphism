//! Executable correspondence between public Rust behavior and the Rocq refinement theorems.

#[allow(dead_code)]
mod common;

use common::{
    TestVerifier, VerificationFailure, artifact, descriptor, domain, evidence_for, morphism_id,
    validate_descriptor, verifier_id,
};
use libmorphism::{
    Completeness, CompositionCheck, Domain, DomainId, EffectSet, EndpointRole, ID_BYTE_LEN,
    LawEvidence, LawKind, MorphismCandidate, Precision, Signature, TypedMorphism, ValidationError,
    check_composition, compose_typed, validate_composition, validate_morphism,
};

#[test]
fn signature_equality_matches_the_formal_decider() {
    let domains = [domain(0), domain(1), domain(2), domain(255)];
    for left_source in domains {
        for left_target in domains {
            for right_source in domains {
                for right_target in domains {
                    let left = Signature::new(left_source, left_target);
                    let right = Signature::new(right_source, right_target);
                    let oracle = left_source == right_source && left_target == right_target;
                    assert_eq!(left == right, oracle);
                }
            }
        }
    }
}

#[test]
fn precision_composition_matches_conjunction() {
    let values = [Precision::Exact, Precision::SoundApproximation];
    for left in values {
        for right in values {
            assert_eq!(
                left.compose(right).is_exact(),
                left.is_exact() && right.is_exact()
            );
        }
    }
}

#[test]
fn completeness_composition_matches_conjunction() {
    let values = [Completeness::Complete, Completeness::Incomplete];
    for left in values {
        for right in values {
            assert_eq!(
                left.compose(right).is_complete(),
                left.is_complete() && right.is_complete()
            );
        }
    }
}

#[test]
fn composition_rejects_every_mismatched_middle_endpoint() {
    let domains = [domain(0), domain(1), domain(2), domain(3)];
    for before_target in domains {
        for after_source in domains {
            let before = descriptor(
                1,
                domain(10),
                before_target,
                EffectSet::NONE,
                Precision::SoundApproximation,
                Completeness::Incomplete,
            );
            let after = descriptor(
                2,
                after_source,
                domain(11),
                EffectSet::NONE,
                Precision::SoundApproximation,
                Completeness::Incomplete,
            );
            let check = check_composition(&after, &before);
            if before_target == after_source {
                assert!(matches!(check, CompositionCheck::Compatible(_)));
            } else {
                let mismatch = check.mismatch().expect("mismatch must be diagnosed");
                assert_eq!(mismatch.before_target(), before_target);
                assert_eq!(mismatch.after_source(), after_source);
            }
        }
    }
}

#[test]
fn composition_preserves_outer_endpoints() {
    let before = descriptor(
        3,
        domain(1),
        domain(2),
        EffectSet::READS_STATE,
        Precision::Exact,
        Completeness::Incomplete,
    );
    let after = descriptor(
        4,
        domain(2),
        domain(3),
        EffectSet::WRITES_STATE,
        Precision::SoundApproximation,
        Completeness::Complete,
    );
    let summary = check_composition(&after, &before)
        .into_result()
        .expect("middle endpoint agrees");
    assert_eq!(summary.signature(), Signature::new(domain(1), domain(3)));
    assert_eq!(
        summary.effects(),
        EffectSet::READS_STATE.union(EffectSet::WRITES_STATE)
    );
    assert_eq!(summary.precision(), Precision::SoundApproximation);
    assert_eq!(summary.completeness(), Completeness::Incomplete);
}

#[test]
fn composition_provenance_is_before_then_after() {
    let before = descriptor(
        5,
        domain(1),
        domain(2),
        EffectSet::NONE,
        Precision::SoundApproximation,
        Completeness::Incomplete,
    );
    let after = descriptor(
        6,
        domain(2),
        domain(3),
        EffectSet::NONE,
        Precision::SoundApproximation,
        Completeness::Incomplete,
    );
    let summary = check_composition(&after, &before)
        .into_result()
        .expect("middle endpoint agrees");
    assert_eq!(summary.provenance().before(), before.provenance());
    assert_eq!(summary.provenance().after(), after.provenance());
}

#[test]
fn composition_witnesses_are_constructed_exactly_on_success() {
    let verifier = TestVerifier::accepting(verifier_id(9));
    let domains = [domain(0), domain(1), domain(2), domain(3)];
    for before_target in domains {
        for after_source in domains {
            let before = validate_descriptor(
                descriptor(
                    7,
                    domain(10),
                    before_target,
                    EffectSet::NONE,
                    Precision::SoundApproximation,
                    Completeness::Incomplete,
                ),
                &verifier,
            );
            let after = validate_descriptor(
                descriptor(
                    8,
                    after_source,
                    domain(11),
                    EffectSet::NONE,
                    Precision::SoundApproximation,
                    Completeness::Incomplete,
                ),
                &verifier,
            );
            assert_eq!(
                validate_composition(&after, &before).is_ok(),
                before_target == after_source
            );
        }
    }
    assert_eq!(
        verifier.calls(),
        0,
        "approximate validation is verifier-free"
    );
}

#[test]
fn exact_candidates_require_accepted_bound_evidence() {
    let descriptor = descriptor(
        12,
        domain(1),
        domain(2),
        EffectSet::NONE,
        Precision::Exact,
        Completeness::Complete,
    );
    let active_id = verifier_id(20);
    let accepting = TestVerifier::accepting(active_id);

    let missing = validate_morphism(
        MorphismCandidate::from_descriptor(descriptor.clone()),
        &accepting,
    );
    assert!(matches!(
        missing,
        Err(ValidationError::MissingExactEvidence)
    ));

    let wrong_kind = LawEvidence::new(
        descriptor.id(),
        LawKind::Identity,
        active_id,
        common::POLICY_VERSION,
        artifact(1),
    );
    assert!(matches!(
        validate_morphism(
            MorphismCandidate::from_descriptor(descriptor.clone()).with_exact_evidence(wrong_kind),
            &accepting,
        ),
        Err(ValidationError::EvidenceKindMismatch(_))
    ));

    let wrong_subject = LawEvidence::new(
        morphism_id(99),
        LawKind::ExactDenotation,
        active_id,
        common::POLICY_VERSION,
        artifact(2),
    );
    assert!(matches!(
        validate_morphism(
            MorphismCandidate::from_descriptor(descriptor.clone())
                .with_exact_evidence(wrong_subject),
            &accepting,
        ),
        Err(ValidationError::EvidenceSubjectMismatch(_))
    ));

    let wrong_verifier = LawEvidence::new(
        descriptor.id(),
        LawKind::ExactDenotation,
        verifier_id(21),
        common::POLICY_VERSION,
        artifact(3),
    );
    assert!(matches!(
        validate_morphism(
            MorphismCandidate::from_descriptor(descriptor.clone())
                .with_exact_evidence(wrong_verifier),
            &accepting,
        ),
        Err(ValidationError::EvidenceVerifierMismatch(_))
    ));

    assert_eq!(accepting.calls(), 0, "binding checks precede verifier work");

    let rejecting = TestVerifier::rejecting(active_id);
    let rejected = validate_morphism(
        MorphismCandidate::from_descriptor(descriptor.clone())
            .with_exact_evidence(evidence_for(&descriptor, active_id)),
        &rejecting,
    );
    assert_eq!(
        rejected,
        Err(ValidationError::EvidenceRejected(VerificationFailure))
    );
    assert_eq!(rejecting.calls(), 1);

    let accepted = validate_morphism(
        MorphismCandidate::from_descriptor(descriptor.clone())
            .with_exact_evidence(evidence_for(&descriptor, active_id)),
        &accepting,
    )
    .expect("fully bound evidence should validate");
    assert_eq!(accepted.descriptor(), &descriptor);
    assert!(accepted.exact_evidence().is_some());
    assert_eq!(accepting.calls(), 1);
}

#[test]
fn non_exact_candidates_validate_without_exactness_evidence() {
    let descriptor = descriptor(
        13,
        domain(1),
        domain(2),
        EffectSet::ALLOCATES,
        Precision::SoundApproximation,
        Completeness::Complete,
    );
    let verifier = TestVerifier::rejecting(verifier_id(30));
    let irrelevant = LawEvidence::new(
        morphism_id(200),
        LawKind::Identity,
        verifier_id(201),
        0,
        artifact(202),
    );
    let validated = validate_morphism(
        MorphismCandidate::from_descriptor(descriptor.clone()).with_exact_evidence(irrelevant),
        &verifier,
    )
    .expect("sound approximation requires no exactness evidence");
    assert_eq!(validated.descriptor(), &descriptor);
    assert_eq!(validated.exact_evidence(), None);
    assert_eq!(verifier.calls(), 0);
}

struct A;
struct B;
struct C;

impl Domain for A {
    const ID: DomainId = DomainId::new([1; ID_BYTE_LEN]);
}

impl Domain for B {
    const ID: DomainId = DomainId::new([2; ID_BYTE_LEN]);
}

impl Domain for C {
    const ID: DomainId = DomainId::new([3; ID_BYTE_LEN]);
}

#[test]
fn typed_wrapper_requires_marker_endpoint_ids() {
    let verifier = TestVerifier::accepting(verifier_id(40));
    let validated = validate_descriptor(
        descriptor(
            14,
            A::ID,
            B::ID,
            EffectSet::NONE,
            Precision::SoundApproximation,
            Completeness::Complete,
        ),
        &verifier,
    );

    let typed = TypedMorphism::<A, B>::try_new(validated.clone())
        .expect("matching marker IDs should construct a wrapper");
    assert_eq!(typed.as_validated(), &validated);

    let source_error = TypedMorphism::<C, B>::try_new(validated.clone())
        .expect_err("wrong source marker must fail");
    assert_eq!(source_error.role(), EndpointRole::Source);
    assert_eq!(source_error.expected(), C::ID);
    assert_eq!(source_error.actual(), A::ID);

    let target_error =
        TypedMorphism::<A, C>::try_new(validated).expect_err("wrong target marker must fail");
    assert_eq!(target_error.role(), EndpointRole::Target);
    assert_eq!(target_error.expected(), C::ID);
    assert_eq!(target_error.actual(), B::ID);
}

#[test]
fn typed_composition_uses_the_shared_marker_as_a_witness() {
    let verifier = TestVerifier::accepting(verifier_id(41));
    let before = TypedMorphism::<A, B>::try_new(validate_descriptor(
        descriptor(
            15,
            A::ID,
            B::ID,
            EffectSet::READS_STATE,
            Precision::SoundApproximation,
            Completeness::Complete,
        ),
        &verifier,
    ))
    .expect("before endpoints agree");
    let after = TypedMorphism::<B, C>::try_new(validate_descriptor(
        descriptor(
            16,
            B::ID,
            C::ID,
            EffectSet::EMITS_EVIDENCE,
            Precision::SoundApproximation,
            Completeness::Incomplete,
        ),
        &verifier,
    ))
    .expect("after endpoints agree");

    let witness = compose_typed(&after, &before);
    assert_eq!(witness.summary().signature(), Signature::new(A::ID, C::ID));
    assert_eq!(
        witness.summary().effects(),
        EffectSet::READS_STATE.union(EffectSet::EMITS_EVIDENCE)
    );
}

#[test]
fn signature_mismatch_precedes_evidence_validation() {
    let descriptor = descriptor(
        17,
        domain(1),
        domain(2),
        EffectSet::NONE,
        Precision::Exact,
        Completeness::Complete,
    );
    let verifier = TestVerifier::accepting(verifier_id(42));
    let result = validate_morphism(
        MorphismCandidate::new(descriptor, Signature::new(domain(9), domain(2)), None),
        &verifier,
    );
    assert!(matches!(result, Err(ValidationError::SignatureMismatch(_))));
    assert_eq!(verifier.calls(), 0);
}
