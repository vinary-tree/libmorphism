//! Property-based refinement checks derived from the closed formal invariants.

#[allow(dead_code)]
mod common;

use common::TestVerifier;
use libmorphism::{
    ArtifactId, Completeness, DomainId, EffectSet, LawEvidence, LawKind, MorphismCandidate,
    MorphismDescriptor, MorphismId, Precision, Provenance, Signature, VerifierId,
    check_composition, validate_morphism,
};
use proptest::prelude::*;

fn precision(exact: bool) -> Precision {
    if exact {
        Precision::Exact
    } else {
        Precision::SoundApproximation
    }
}

fn completeness(complete: bool) -> Completeness {
    if complete {
        Completeness::Complete
    } else {
        Completeness::Incomplete
    }
}

fn defined_effects(bits: u8) -> EffectSet {
    EffectSet::from_bits(bits & EffectSet::ALL.bits())
        .expect("masking by ALL must remove every undefined effect bit")
}

fn distinct_domain(mut bytes: [u8; 32]) -> DomainId {
    bytes[0] ^= u8::MAX;
    DomainId::new(bytes)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1_024))]

    #[test]
    fn composition_refines_endpoint_and_no_promotion_invariants(
        source in any::<[u8; 32]>(),
        middle in any::<[u8; 32]>(),
        target in any::<[u8; 32]>(),
        endpoints_agree in any::<bool>(),
        before_effect_bits in any::<u8>(),
        after_effect_bits in any::<u8>(),
        before_exact in any::<bool>(),
        after_exact in any::<bool>(),
        before_complete in any::<bool>(),
        after_complete in any::<bool>(),
    ) {
        let source = DomainId::new(source);
        let middle = DomainId::new(middle);
        let target = DomainId::new(target);
        let after_source = if endpoints_agree {
            middle
        } else {
            distinct_domain(middle.into_bytes())
        };
        let before_effects = defined_effects(before_effect_bits);
        let after_effects = defined_effects(after_effect_bits);
        let before_precision = precision(before_exact);
        let after_precision = precision(after_exact);
        let before_completeness = completeness(before_complete);
        let after_completeness = completeness(after_complete);
        let before_provenance = Provenance::new(ArtifactId::new([11; 32]), 3, 5);
        let after_provenance = Provenance::new(ArtifactId::new([13; 32]), 7, 11);
        let before = MorphismDescriptor::new(
            MorphismId::new([17; 32]),
            Signature::new(source, middle),
            before_effects,
            before_precision,
            before_completeness,
            before_provenance,
        );
        let after = MorphismDescriptor::new(
            MorphismId::new([19; 32]),
            Signature::new(after_source, target),
            after_effects,
            after_precision,
            after_completeness,
            after_provenance,
        );

        let check = check_composition(&after, &before);
        prop_assert_eq!(check.is_compatible(), endpoints_agree);
        if endpoints_agree {
            let summary = check
                .into_result()
                .expect("the generated middle endpoints agree");
            prop_assert_eq!(summary.signature(), Signature::new(source, target));
            prop_assert_eq!(summary.effects(), before_effects.union(after_effects));
            prop_assert_eq!(
                summary.precision(),
                before_precision.compose(after_precision)
            );
            prop_assert_eq!(
                summary.completeness(),
                before_completeness.compose(after_completeness)
            );
            prop_assert_eq!(summary.provenance().before(), before_provenance);
            prop_assert_eq!(summary.provenance().after(), after_provenance);
            prop_assert_eq!(summary.precision().is_exact(), before_exact && after_exact);
            prop_assert_eq!(
                summary.completeness().is_complete(),
                before_complete && after_complete
            );
        } else {
            let mismatch = check
                .mismatch()
                .expect("the generated middle endpoints disagree");
            prop_assert_eq!(mismatch.before_target(), middle);
            prop_assert_eq!(mismatch.after_source(), after_source);
        }
    }

    #[test]
    fn effect_union_refines_the_bounded_join_monoid(
        first_bits in any::<u8>(),
        second_bits in any::<u8>(),
        third_bits in any::<u8>(),
    ) {
        let first = defined_effects(first_bits);
        let second = defined_effects(second_bits);
        let third = defined_effects(third_bits);

        prop_assert_eq!(EffectSet::NONE.union(first), first);
        prop_assert_eq!(first.union(EffectSet::NONE), first);
        prop_assert_eq!(first.union(first), first);
        prop_assert_eq!(first.union(second), second.union(first));
        prop_assert_eq!(
            first.union(second).union(third),
            first.union(second.union(third))
        );
        prop_assert!(first.union(second).contains(first));
        prop_assert!(first.union(second).contains(second));
    }

    #[test]
    fn approximate_validation_ignores_non_authoritative_exact_evidence(
        source in any::<[u8; 32]>(),
        target in any::<[u8; 32]>(),
        descriptor_id in any::<[u8; 32]>(),
        evidence_subject in any::<[u8; 32]>(),
        verifier_bytes in any::<[u8; 32]>(),
        proof_artifact in any::<[u8; 32]>(),
        effect_bits in any::<u8>(),
        policy_version in any::<u64>(),
    ) {
        let descriptor = MorphismDescriptor::new(
            MorphismId::new(descriptor_id),
            Signature::new(DomainId::new(source), DomainId::new(target)),
            defined_effects(effect_bits),
            Precision::SoundApproximation,
            Completeness::Incomplete,
            Provenance::new(ArtifactId::new([23; 32]), 1, 0),
        );
        let verifier_id = VerifierId::new(verifier_bytes);
        let irrelevant_evidence = LawEvidence::new(
            MorphismId::new(evidence_subject),
            LawKind::ExactDenotation,
            verifier_id,
            policy_version,
            ArtifactId::new(proof_artifact),
        );
        let verifier = TestVerifier::rejecting(verifier_id);
        let candidate = MorphismCandidate::from_descriptor(descriptor)
            .with_exact_evidence(irrelevant_evidence);

        let validated = validate_morphism(candidate, &verifier)
            .expect("sound approximations do not require exactness evidence");
        prop_assert_eq!(validated.descriptor().precision(), Precision::SoundApproximation);
        prop_assert!(validated.exact_evidence().is_none());
        prop_assert_eq!(verifier.calls(), 0);
    }
}
