//! Error types for the devtools component.

/// Errors in devtools operations.
#[derive(Debug, thiserror::Error)]
pub enum DevtoolsError {
    /// Invalid build/commit spec.
    #[error("invalid spec: {0}")]
    InvalidSpec(String),

    /// Chroot operation failed.
    #[error("chroot error: {0}")]
    Chroot(String),

    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
