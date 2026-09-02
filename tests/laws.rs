//! Exhaustive finite-domain tests for the algebraic laws used by composition.

#[allow(dead_code)]
mod common;

use common::{descriptor, domain};
use libmorphism::{
    Completeness, CompositionSummary, EffectSet, MorphismDescriptor, Precision, Provenance,
    Signature, check_composition,
};

#[test]
fn effect_union_is_a_monoid() {
    for left_bits in 0..=EffectSet::ALL.bits() {
        let left = EffectSet::from_bits(left_bits).expect("enumerated bits are valid");
        assert_eq!(EffectSet::NONE.union(left), left);
        assert_eq!(left.union(EffectSet::NONE), left);
        assert_eq!(left.union(left), left, "union is also idempotent");

        for right_bits in 0..=EffectSet::ALL.bits() {
            let right = EffectSet::from_bits(right_bits).expect("enumerated bits are valid");
            assert_eq!(left.union(right), right.union(left), "union is commutative");
            for third_bits in 0..=EffectSet::ALL.bits() {
                let third = EffectSet::from_bits(third_bits).expect("enumerated bits are valid");
                assert_eq!(
                    left.union(right).union(third),
                    left.union(right.union(third)),
                    "union is associative"
                );
            }
        }
    }
    assert!(EffectSet::from_bits(EffectSet::ALL.bits() | 0b1000_0000).is_err());
}

#[test]
fn precision_and_completeness_composition_are_monoids() {
    let precisions = [Precision::Exact, Precision::SoundApproximation];
    for first in precisions {
        assert_eq!(Precision::Exact.compose(first), first);
        assert_eq!(first.compose(Precision::Exact), first);
        for second in precisions {
            for third in precisions {
                assert_eq!(
                    first.compose(second).compose(third),
                    first.compose(second.compose(third))
                );
            }
        }
    }

    let completeness = [Completeness::Complete, Completeness::Incomplete];
    for first in completeness {
        assert_eq!(Completeness::Complete.compose(first), first);
        assert_eq!(first.compose(Completeness::Complete), first);
        for second in completeness {
            for third in completeness {
                assert_eq!(
                    first.compose(second).compose(third),
                    first.compose(second.compose(third))
                );
            }
        }
    }
}

fn reify_summary(tag: u8, summary: &CompositionSummary) -> MorphismDescriptor {
    MorphismDescriptor::new(
        common::morphism_id(tag),
        summary.signature(),
        summary.effects(),
        summary.precision(),
        summary.completeness(),
        Provenance::new(common::artifact(tag), 1, u64::from(tag)),
    )
}

fn assert_same_semantics(left: &CompositionSummary, right: &CompositionSummary) {
    assert_eq!(left.signature(), right.signature());
    assert_eq!(left.effects(), right.effects());
    assert_eq!(left.precision(), right.precision());
    assert_eq!(left.completeness(), right.completeness());
}

#[test]
fn composition_observables_are_associative() {
    let precisions = [Precision::Exact, Precision::SoundApproximation];
    let completeness = [Completeness::Complete, Completeness::Incomplete];

    for first_effects in 0..=EffectSet::ALL.bits() {
        for second_effects in 0..=EffectSet::ALL.bits() {
            for third_effects in 0..=EffectSet::ALL.bits() {
                for first_precision in precisions {
                    for second_precision in precisions {
                        for third_precision in precisions {
                            for first_completeness in completeness {
                                for second_completeness in completeness {
                                    for third_completeness in completeness {
                                        let first = descriptor(
                                            1,
                                            domain(1),
                                            domain(2),
                                            EffectSet::from_bits(first_effects)
                                                .expect("enumerated first effects are valid"),
                                            first_precision,
                                            first_completeness,
                                        );
                                        let second = descriptor(
                                            2,
                                            domain(2),
                                            domain(3),
                                            EffectSet::from_bits(second_effects)
                                                .expect("enumerated second effects are valid"),
                                            second_precision,
                                            second_completeness,
                                        );
                                        let third = descriptor(
                                            3,
                                            domain(3),
                                            domain(4),
                                            EffectSet::from_bits(third_effects)
                                                .expect("enumerated third effects are valid"),
                                            third_precision,
                                            third_completeness,
                                        );

                                        let second_after_first = check_composition(&second, &first)
                                            .into_result()
                                            .expect("first middle endpoint agrees");
                                        let left = check_composition(
                                            &third,
                                            &reify_summary(4, &second_after_first),
                                        )
                                        .into_result()
                                        .expect("left-grouped middle endpoint agrees");

                                        let third_after_second = check_composition(&third, &second)
                                            .into_result()
                                            .expect("second middle endpoint agrees");
                                        let right = check_composition(
                                            &reify_summary(5, &third_after_second),
                                            &first,
                                        )
                                        .into_result()
                                        .expect("right-grouped middle endpoint agrees");

                                        assert_same_semantics(&left, &right);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn identity_descriptors_preserve_semantic_observables() {
    let arrow = descriptor(
        9,
        domain(1),
        domain(2),
        EffectSet::READS_STATE.union(EffectSet::ALLOCATES),
        Precision::SoundApproximation,
        Completeness::Incomplete,
    );
    let left_identity = MorphismDescriptor::identity(
        common::morphism_id(10),
        domain(1),
        Provenance::new(common::artifact(10), 1, 0),
    );
    let right_identity = MorphismDescriptor::identity(
        common::morphism_id(11),
        domain(2),
        Provenance::new(common::artifact(11), 1, 1),
    );

    let left = check_composition(&arrow, &left_identity)
        .into_result()
        .expect("left identity composes");
    let right = check_composition(&right_identity, &arrow)
        .into_result()
        .expect("right identity composes");
    for summary in [&left, &right] {
        assert_eq!(summary.signature(), arrow.signature());
        assert_eq!(summary.effects(), arrow.effects());
        assert_eq!(summary.precision(), arrow.precision());
        assert_eq!(summary.completeness(), arrow.completeness());
    }
}

#[test]
fn signature_constructor_preserves_source_and_target_roles() {
    let signature = Signature::new(domain(1), domain(2));
    assert_eq!(signature.source(), domain(1));
    assert_eq!(signature.target(), domain(2));
    assert_ne!(signature.source(), signature.target());
}
