use core::fmt;

/// The byte length of every stable identifier.
///
/// The 256-bit width accommodates content-addressed identities without prescribing a hash
/// function or naming authority.
pub const ID_BYTE_LEN: usize = 32;

macro_rules! define_id {
    ($(#[$metadata:meta])* $name:ident) => {
        $(#[$metadata])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; ID_BYTE_LEN]);

        impl $name {
            /// Constructs an identifier from its canonical bytes.
            #[must_use]
            pub const fn new(bytes: [u8; ID_BYTE_LEN]) -> Self {
                Self(bytes)
            }

            /// Borrows the identifier's canonical bytes.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; ID_BYTE_LEN] {
                &self.0
            }

            /// Returns the identifier's canonical bytes.
            #[must_use]
            pub const fn into_bytes(self) -> [u8; ID_BYTE_LEN] {
                self.0
            }
        }

        impl From<[u8; ID_BYTE_LEN]> for $name {
            fn from(bytes: [u8; ID_BYTE_LEN]) -> Self {
                Self::new(bytes)
            }
        }

        impl AsRef<[u8; ID_BYTE_LEN]> for $name {
            fn as_ref(&self) -> &[u8; ID_BYTE_LEN] {
                self.as_bytes()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                for byte in self.0 {
                    write!(formatter, "{byte:02x}")?;
                }
                Ok(())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(concat!(stringify!($name), "(\""))?;
                fmt::Display::fmt(self, formatter)?;
                formatter.write_str("\")")
            }
        }
    };
}

define_id!(
    /// Stable identity of a semantic source or target domain.
    DomainId
);

define_id!(
    /// Stable identity of an immutable morphism descriptor.
    MorphismId
);

define_id!(
    /// Stable identity of a provenance or proof artifact.
    ArtifactId
);

define_id!(
    /// Stable identity of an evidence verifier and its trust domain.
    VerifierId
);
