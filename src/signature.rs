use crate::DomainId;

/// The source and target domains of a transformation.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Signature {
    source: DomainId,
    target: DomainId,
}

impl Signature {
    /// Constructs a signature from distinct source and target roles.
    #[must_use]
    pub const fn new(source: DomainId, target: DomainId) -> Self {
        Self { source, target }
    }

    /// Constructs the identity signature for one domain.
    #[must_use]
    pub const fn identity(domain: DomainId) -> Self {
        Self::new(domain, domain)
    }

    /// Returns the source domain.
    #[must_use]
    pub const fn source(self) -> DomainId {
        self.source
    }

    /// Returns the target domain.
    #[must_use]
    pub const fn target(self) -> DomainId {
        self.target
    }
}
