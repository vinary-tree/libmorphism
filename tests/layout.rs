//! Representation and auto-trait regression checks for the allocation-free core.

use core::mem::{needs_drop, size_of};
use std::{hint::black_box, thread};

use libmorphism::{
    ArtifactId, Completeness, CompositionSummary, CompositionWitness, DomainId, EffectSet,
    ID_BYTE_LEN, LawEvidence, MorphismDescriptor, MorphismId, Precision, Provenance, Signature,
    ValidatedMorphism, VerifierId, check_composition,
};

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn core_representations_are_fixed_size_and_allocation_free() {
    assert_eq!(size_of::<DomainId>(), ID_BYTE_LEN);
    assert_eq!(size_of::<MorphismId>(), ID_BYTE_LEN);
    assert_eq!(size_of::<ArtifactId>(), ID_BYTE_LEN);
    assert_eq!(size_of::<VerifierId>(), ID_BYTE_LEN);
    assert_eq!(size_of::<Signature>(), 2 * ID_BYTE_LEN);
    assert_eq!(size_of::<EffectSet>(), 1);

    assert!(size_of::<MorphismDescriptor>() <= 160);
    assert!(size_of::<LawEvidence>() <= 112);
    assert!(size_of::<CompositionSummary>() <= 168);

    assert!(!needs_drop::<MorphismDescriptor>());
    assert!(!needs_drop::<LawEvidence>());
    assert!(!needs_drop::<ValidatedMorphism>());
    assert!(!needs_drop::<CompositionWitness>());
}

#[test]
fn authority_bearing_values_are_send_and_sync() {
    assert_send_sync::<ValidatedMorphism>();
    assert_send_sync::<CompositionWitness>();
}

#[test]
fn composition_check_is_stack_constant_on_a_small_stack() {
    let worker = thread::Builder::new()
        .name("libmorphism-stack-safety".into())
        .stack_size(64 * 1024)
        .spawn(|| {
            let middle = DomainId::new([2; ID_BYTE_LEN]);
            let before = MorphismDescriptor::new(
                MorphismId::new([1; ID_BYTE_LEN]),
                Signature::new(DomainId::new([1; ID_BYTE_LEN]), middle),
                EffectSet::READS_STATE,
                Precision::Exact,
                Completeness::Complete,
                Provenance::new(ArtifactId::new([1; ID_BYTE_LEN]), 1, 1),
            );
            let after = MorphismDescriptor::new(
                MorphismId::new([2; ID_BYTE_LEN]),
                Signature::new(middle, DomainId::new([3; ID_BYTE_LEN])),
                EffectSet::WRITES_STATE,
                Precision::SoundApproximation,
                Completeness::Incomplete,
                Provenance::new(ArtifactId::new([2; ID_BYTE_LEN]), 1, 2),
            );

            for _ in 0..250_000 {
                let check = black_box(check_composition(black_box(&after), black_box(&before)));
                assert!(check.is_compatible());
            }
        })
        .expect("small-stack worker must start");
    worker.join().expect("small-stack worker must finish");
}
