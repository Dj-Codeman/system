use serde::{Deserialize, Serialize};

/// Represents different types of generic warnings.
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Serialize, Deserialize)]
pub enum Warnings {
    /// Generic warning.
    Warning,
    /// Warning indicating an outdated version.
    OutdatedVersion,
    /// Warning indicating a misaligned chunk.
    MisAlignedChunk,
    /// Warning indicating failure to delete a file.
    FileNotDeleted,
    /// Warning indicating a lost connection.
    ConnectionLost,
    /// Warning indicating resource exhaustion.
    ResourceExhaustion,
    /// Warning indicating unexpected behavior.
    UnexpectedBehavior,
    /// Warning indicating unexpected configuration.
    UnexpectedConfiguration,
}
