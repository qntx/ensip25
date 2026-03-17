//! Typed error definitions for the ENSIP-25 SDK.

/// The primary error type for all ENSIP-25 SDK operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The input bytes are too short to contain a valid ERC-7930 header.
    #[error("erc7930: buffer too short (need at least 6 bytes, got {len})")]
    BufferTooShort {
        /// Actual byte length provided.
        len: usize,
    },

    /// The ERC-7930 version field is unsupported.
    #[error("erc7930: unsupported version {version:#06x}")]
    UnsupportedVersion {
        /// The version value found in the header.
        version: u16,
    },

    /// The declared lengths inside an ERC-7930 address exceed the buffer.
    #[error("erc7930: truncated payload (expected {expected} bytes, have {available})")]
    TruncatedPayload {
        /// Number of bytes the header says should follow.
        expected: usize,
        /// Number of bytes actually remaining.
        available: usize,
    },

    /// Both `ChainReferenceLength` and `AddressLength` are zero, which is
    /// invalid per ERC-7930.
    #[error("erc7930: both chain reference and address are empty")]
    EmptyAddress,

    /// A hex string could not be decoded.
    #[error("hex decode error: {0}")]
    Hex(#[from] alloy_primitives::hex::FromHexError),

    /// An `agentId` contains forbidden characters (`[` or `]`).
    #[error("agent id must not contain '[' or ']': {agent_id:?}")]
    InvalidAgentId {
        /// The offending agent ID string.
        agent_id: String,
    },

    /// An ENS text-record lookup failed.
    #[cfg(feature = "provider")]
    #[error("ens error: {0}")]
    Ens(#[from] alloy_ens::EnsError),

    /// An ERC-8004 SDK error.
    #[cfg(feature = "erc8004")]
    #[error("erc8004 error: {0}")]
    Erc8004(#[from] erc8004::Error),
}

/// A convenience type alias used throughout the SDK.
pub type Result<T> = core::result::Result<T, Error>;
