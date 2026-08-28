use core::{error::Error, fmt, ops};

/// A compact conservative declaration of observable transformation effects.
///
/// Effects compose by bit-set union. An implementation may do less than it declares, but doing
/// more invalidates any scheduling decision based on the declaration.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct EffectSet(u8);

impl EffectSet {
    /// No declared effects; the identity for union.
    pub const NONE: Self = Self(0);
    /// Reads shared or externally observable state.
    pub const READS_STATE: Self = Self(1 << 0);
    /// Writes shared or externally observable state.
    pub const WRITES_STATE: Self = Self(1 << 1);
    /// Performs allocation visible to resource accounting.
    pub const ALLOCATES: Self = Self(1 << 2);
    /// Emits evidence whose presence or order may be observable.
    pub const EMITS_EVIDENCE: Self = Self(1 << 3);
    /// Every effect understood by this version of the crate.
    pub const ALL: Self = Self(
        Self::READS_STATE.0 | Self::WRITES_STATE.0 | Self::ALLOCATES.0 | Self::EMITS_EVIDENCE.0,
    );

    /// Constructs a set when every bit has defined semantics.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidEffectBits`] when `bits` contains an unknown bit.
    pub const fn from_bits(bits: u8) -> Result<Self, InvalidEffectBits> {
        let invalid = bits & !Self::ALL.0;
        if invalid == 0 {
            Ok(Self(bits))
        } else {
            Err(InvalidEffectBits { invalid })
        }
    }

    /// Returns the canonical bit representation.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Returns the union of two conservative declarations.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns whether this set contains every bit in `required`.
    #[must_use]
    pub const fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    /// Returns whether the two declarations share at least one effect.
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    /// Returns whether no effect is declared.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl ops::BitOr for EffectSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl ops::BitOrAssign for EffectSet {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

/// Unknown bits supplied to [`EffectSet::from_bits`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InvalidEffectBits {
    invalid: u8,
}

impl InvalidEffectBits {
    /// Returns only the unsupported bits.
    #[must_use]
    pub const fn invalid(self) -> u8 {
        self.invalid
    }
}

impl fmt::Display for InvalidEffectBits {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown effect bits: 0b{:08b}", self.invalid)
    }
}

impl Error for InvalidEffectBits {}
