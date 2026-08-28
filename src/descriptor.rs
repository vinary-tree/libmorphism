use crate::{Completeness, EffectSet, MorphismId, Precision, Provenance, Signature};

/// An immutable semantic description of one transformation.
///
/// The stable descriptor identifier is assigned by the producing trust domain. When it is used as
/// a content identity, the producer must bind every field relevant to its semantic policy.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MorphismDescriptor {
    id: MorphismId,
    signature: Signature,
    effects: EffectSet,
    precision: Precision,
    completeness: Completeness,
    provenance: Provenance,
}

impl MorphismDescriptor {
    /// Constructs an immutable descriptor.
    #[must_use]
    pub const fn new(
        id: MorphismId,
        signature: Signature,
        effects: EffectSet,
        precision: Precision,
        completeness: Completeness,
        provenance: Provenance,
    ) -> Self {
        Self {
            id,
            signature,
            effects,
            precision,
            completeness,
            provenance,
        }
    }

    /// Constructs an exact, complete, effect-free identity descriptor.
    ///
    /// The returned descriptor is still unvalidated. Exact publication requires accepted
    /// [`crate::LawEvidence`] just like every other exact descriptor.
    #[must_use]
    pub const fn identity(id: MorphismId, domain: crate::DomainId, provenance: Provenance) -> Self {
        Self::new(
            id,
            Signature::identity(domain),
            EffectSet::NONE,
            Precision::Exact,
            Completeness::Complete,
            provenance,
        )
    }

    /// Returns the stable descriptor identity.
    #[must_use]
    pub const fn id(&self) -> MorphismId {
        self.id
    }

    /// Returns the source and target signature.
    #[must_use]
    pub const fn signature(&self) -> Signature {
        self.signature
    }

    /// Returns the conservative effect declaration.
    #[must_use]
    pub const fn effects(&self) -> EffectSet {
        self.effects
    }

    /// Returns the precision claim.
    #[must_use]
    pub const fn precision(&self) -> Precision {
        self.precision
    }

    /// Returns the completeness claim.
    #[must_use]
    pub const fn completeness(&self) -> Completeness {
        self.completeness
    }

    /// Returns the descriptor provenance.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }
}
