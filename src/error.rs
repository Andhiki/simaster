use std::process::ExitCode;

/// Errors returned by the CLI.
///
/// The walking skeleton maps every error to exit code 1. Later milestones
/// map auth, network, upstream, and permission failures to exit codes 3-6
/// as described in `docs/CLI-CONTRACT.md`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The command exists but its behavior has not landed yet.
    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),

    /// Configuration is missing or invalid.
    #[error("configuration error: {0}")]
    Config(String),

    /// An input/output operation failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// JSON serialization or deserialization failed.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Any other failure with a human-readable message.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Process exit code for this error.
    #[must_use]
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::FAILURE
    }
}
