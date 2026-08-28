use crate::{
    Completeness, EffectSet, MorphismDescriptor, Precision, Provenance, Signature,
    ValidatedMorphism,
};

/// The incompatible middle endpoints of a requested composition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndpointMismatch {
    before_target: crate::DomainId,
    after_source: crate::DomainId,
}

impl EndpointMismatch {
    /// Returns the target produced by the transformation that would run first.
    #[must_use]
    pub const fn before_target(self) -> crate::DomainId {
        self.before_target
    }

    /// Returns the source required by the transformation that would run second.
    #[must_use]
    pub const fn after_source(self) -> crate::DomainId {
        self.after_source
    }
}

/// Ordered provenance for one binary composition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompositionProvenance {
    before: Provenance,
    after: Provenance,
}

impl CompositionProvenance {
    /// Returns the provenance of the transformation executed first.
    #[must_use]
    pub const fn before(self) -> Provenance {
        self.before
    }

    /// Returns the provenance of the transformation executed second.
    #[must_use]
    pub const fn after(self) -> Provenance {
        self.after
    }
}

/// The semantic result of one compatible binary composition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionSummary {
    signature: Signature,
    effects: EffectSet,
    precision: Precision,
    completeness: Completeness,
    provenance: CompositionProvenance,
}

impl CompositionSummary {
    /// Returns the outer endpoints of the composition.
    #[must_use]
    pub const fn signature(&self) -> Signature {
        self.signature
    }

    /// Returns the union of both effect declarations.
    #[must_use]
    pub const fn effects(&self) -> EffectSet {
        self.effects
    }

    /// Returns the no-promotion precision result.
    #[must_use]
    pub const fn precision(&self) -> Precision {
        self.precision
    }

    /// Returns the no-promotion completeness result.
    #[must_use]
    pub const fn completeness(&self) -> Completeness {
        self.completeness
    }

    /// Returns binary provenance in execution order.
    #[must_use]
    pub const fn provenance(&self) -> CompositionProvenance {
        self.provenance
    }
}

/// A non-authoritative diagnostic result from [`check_composition`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompositionCheck {
    /// The middle endpoint agrees and the contained summary is well-formed.
    Compatible(CompositionSummary),
    /// The middle endpoint disagrees.
    EndpointMismatch(EndpointMismatch),
}

impl CompositionCheck {
    /// Returns whether the composition is compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible(_))
    }

    /// Borrows the compatible summary, if present.
    #[must_use]
    pub const fn summary(&self) -> Option<&CompositionSummary> {
        match self {
            Self::Compatible(summary) => Some(summary),
            Self::EndpointMismatch(_) => None,
        }
    }

    /// Returns the mismatch, if present.
    #[must_use]
    pub const fn mismatch(&self) -> Option<EndpointMismatch> {
        match self {
            Self::Compatible(_) => None,
            Self::EndpointMismatch(mismatch) => Some(*mismatch),
        }
    }

    /// Converts the diagnostic into an ordinary result.
    pub fn into_result(self) -> Result<CompositionSummary, EndpointMismatch> {
        match self {
            Self::Compatible(summary) => Ok(summary),
            Self::EndpointMismatch(mismatch) => Err(mismatch),
        }
    }
}

pub(crate) fn compose_summary(
    after: &MorphismDescriptor,
    before: &MorphismDescriptor,
) -> CompositionSummary {
    CompositionSummary {
        signature: Signature::new(before.signature().source(), after.signature().target()),
        effects: before.effects().union(after.effects()),
        precision: before.precision().compose(after.precision()),
        completeness: before.completeness().compose(after.completeness()),
        provenance: CompositionProvenance {
            before: before.provenance(),
            after: after.provenance(),
        },
    }
}

/// Checks `after` composed after `before` without constructing validation authority.
#[must_use]
pub fn check_composition(
    after: &MorphismDescriptor,
    before: &MorphismDescriptor,
) -> CompositionCheck {
    let before_target = before.signature().target();
    let after_source = after.signature().source();
    if before_target == after_source {
        CompositionCheck::Compatible(compose_summary(after, before))
    } else {
        CompositionCheck::EndpointMismatch(EndpointMismatch {
            before_target,
            after_source,
        })
    }
}

/// Proof-carrying evidence that two validated descriptors are composable.
///
/// Fields are private; use [`validate_composition`] or [`crate::compose_typed`].
///
/// ```compile_fail
/// use libmorphism::CompositionWitness;
/// let forged = CompositionWitness { /* private fields */ };
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionWitness {
    summary: CompositionSummary,
}

impl CompositionWitness {
    pub(crate) const fn new(summary: CompositionSummary) -> Self {
        Self { summary }
    }

    /// Borrows the witnessed composition summary.
    #[must_use]
    pub const fn summary(&self) -> &CompositionSummary {
        &self.summary
    }

    /// Consumes the witness and returns its summary.
    #[must_use]
    pub fn into_summary(self) -> CompositionSummary {
        self.summary
    }
}

/// Validates the dynamic endpoints of two already validated morphisms.
///
/// # Errors
///
/// Returns [`EndpointMismatch`] when `before`'s target differs from `after`'s source.
pub fn validate_composition(
    after: &ValidatedMorphism,
    before: &ValidatedMorphism,
) -> Result<CompositionWitness, EndpointMismatch> {
    check_composition(after.descriptor(), before.descriptor())
        .into_result()
        .map(CompositionWitness::new)
}
