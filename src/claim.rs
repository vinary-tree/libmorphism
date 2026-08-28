/// Whether a transformation's reported result is exact or conservatively approximate.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Precision {
    /// The result denotes exactly the claimed semantics.
    Exact,
    /// The result is sound but may lose precision.
    #[default]
    SoundApproximation,
}

impl Precision {
    /// Composes two precision claims without allowing promotion.
    #[must_use]
    pub const fn compose(self, other: Self) -> Self {
        match (self, other) {
            (Self::Exact, Self::Exact) => Self::Exact,
            _ => Self::SoundApproximation,
        }
    }

    /// Returns whether this is an exactness claim.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether `proposed` is a legal degradation of this claim.
    #[must_use]
    pub const fn permits_degradation_to(self, proposed: Self) -> bool {
        matches!(self, Self::Exact) || matches!(proposed, Self::SoundApproximation)
    }
}

/// Whether a transformation considered every result required by its contract.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Completeness {
    /// Every required result was considered.
    Complete,
    /// Some required results may not have been considered.
    #[default]
    Incomplete,
}

impl Completeness {
    /// Composes two completeness claims without allowing promotion.
    #[must_use]
    pub const fn compose(self, other: Self) -> Self {
        match (self, other) {
            (Self::Complete, Self::Complete) => Self::Complete,
            _ => Self::Incomplete,
        }
    }

    /// Returns whether this is a completeness claim.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}
