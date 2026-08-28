//! Representation and auto-trait regression checks for the allocation-free core.

use core::mem::{needs_drop, size_of};

use libmorphism::{
    ArtifactId, CompositionSummary, CompositionWitness, DomainId, EffectSet, ID_BYTE_LEN,
    LawEvidence, MorphismDescriptor, MorphismId, Signature, ValidatedMorphism, VerifierId,
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
