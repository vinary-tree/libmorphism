use core::{error::Error, fmt, marker::PhantomData};

use crate::{
    CompositionWitness, DomainId, ValidatedMorphism,
    composition::{CompositionWitness as PrivateCompositionWitness, compose_summary},
};

/// A zero-storage marker for a stable semantic domain.
///
/// Implementations must keep `ID` stable across processes and artifacts that exchange typed
/// descriptors. The crate checks this identifier against a validated dynamic signature before a
/// typed wrapper is constructed.
pub trait Domain {
    /// The domain's stable semantic identity.
    const ID: DomainId;
}

/// Which endpoint failed a typed-wrapper check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointRole {
    /// The source endpoint.
    Source,
    /// The target endpoint.
    Target,
}

/// A validated descriptor whose runtime endpoint disagrees with its marker type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypedEndpointError {
    role: EndpointRole,
    expected: DomainId,
    actual: DomainId,
}

impl TypedEndpointError {
    /// Returns which endpoint disagreed.
    #[must_use]
    pub const fn role(self) -> EndpointRole {
        self.role
    }

    /// Returns the marker type's stable identifier.
    #[must_use]
    pub const fn expected(self) -> DomainId {
        self.expected
    }

    /// Returns the validated descriptor's identifier.
    #[must_use]
    pub const fn actual(self) -> DomainId {
        self.actual
    }
}

impl fmt::Display for TypedEndpointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "typed {:?} endpoint does not match its domain marker",
            self.role
        )
    }
}

impl Error for TypedEndpointError {}

/// A validated morphism with zero-storage static endpoint markers.
///
/// Construction checks the marker identifiers once. Safe code cannot change the validated inner
/// descriptor or fabricate the wrapper's private fields.
pub struct TypedMorphism<Source, Target> {
    inner: ValidatedMorphism,
    marker: PhantomData<fn() -> (Source, Target)>,
}

impl<Source, Target> TypedMorphism<Source, Target>
where
    Source: Domain,
    Target: Domain,
{
    /// Checks the dynamic endpoints and constructs a typed wrapper.
    ///
    /// # Errors
    ///
    /// Returns [`TypedEndpointError`] for the first marker identifier that disagrees.
    pub fn try_new(inner: ValidatedMorphism) -> Result<Self, TypedEndpointError> {
        let signature = inner.descriptor().signature();
        if signature.source() != Source::ID {
            return Err(TypedEndpointError {
                role: EndpointRole::Source,
                expected: Source::ID,
                actual: signature.source(),
            });
        }
        if signature.target() != Target::ID {
            return Err(TypedEndpointError {
                role: EndpointRole::Target,
                expected: Target::ID,
                actual: signature.target(),
            });
        }
        Ok(Self {
            inner,
            marker: PhantomData,
        })
    }

    /// Borrows the dynamically validated morphism.
    #[must_use]
    pub const fn as_validated(&self) -> &ValidatedMorphism {
        &self.inner
    }

    /// Consumes the typed wrapper without invalidating its inner witness.
    #[must_use]
    pub fn into_validated(self) -> ValidatedMorphism {
        self.inner
    }
}

impl<Source, Target> Clone for TypedMorphism<Source, Target> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<Source, Target> fmt::Debug for TypedMorphism<Source, Target> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedMorphism")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}

impl<Source, Target> PartialEq for TypedMorphism<Source, Target> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<Source, Target> Eq for TypedMorphism<Source, Target> {}

/// Composes typed validated morphisms without a redundant runtime endpoint comparison.
///
/// Both wrappers were constructed by checking the same `Middle::ID`, so the middle endpoint is
/// unrepresentable as a mismatch in safe code.
#[must_use]
pub fn compose_typed<Source, Middle, Target>(
    after: &TypedMorphism<Middle, Target>,
    before: &TypedMorphism<Source, Middle>,
) -> CompositionWitness
where
    Source: Domain,
    Middle: Domain,
    Target: Domain,
{
    PrivateCompositionWitness::new(compose_summary(
        after.as_validated().descriptor(),
        before.as_validated().descriptor(),
    ))
}
