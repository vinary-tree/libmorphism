use crate::ArtifactId;

/// Stable provenance for one transformation descriptor.
///
/// `revision` identifies the artifact's version, while `sequence` provides a stable order within
/// that revision. Their interpretation belongs to the producing trust domain.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Provenance {
    artifact: ArtifactId,
    revision: u64,
    sequence: u64,
}

impl Provenance {
    /// Constructs provenance for one immutable artifact revision and sequence.
    #[must_use]
    pub const fn new(artifact: ArtifactId, revision: u64, sequence: u64) -> Self {
        Self {
            artifact,
            revision,
            sequence,
        }
    }

    /// Returns the artifact identity.
    #[must_use]
    pub const fn artifact(self) -> ArtifactId {
        self.artifact
    }

    /// Returns the artifact revision.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// Returns the stable sequence within the revision.
    #[must_use]
    pub const fn sequence(self) -> u64 {
        self.sequence
    }
}
