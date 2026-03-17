//! # ENSIP-25: AI Agent Registry ENS Name Verification
//!
//! A type-safe Rust SDK for [ENSIP-25](https://docs.ens.domains/ensip/25) —
//! verify the bidirectional link between ENS names and AI agent identities
//! registered in on-chain registries such as
//! [ERC-8004](https://eips.ethereum.org/EIPS/eip-8004).
//!
//! ## How it works
//!
//! ENSIP-25 defines a parameterized ENS text record key:
//!
//! ```text
//! agent-registration[<registry>][<agentId>]
//! ```
//!
//! Where `<registry>` is the [ERC-7930](https://eips.ethereum.org/EIPS/eip-7930)
//! interoperable address of the on-chain registry and `<agentId>` is the
//! agent's unique identifier. If the ENS name owner sets this text record to
//! any non-empty value, the association is considered verified.
//!
//! ## Feature flags
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `provider` | off | Enables on-chain verification via alloy provider |
//! | `erc8004` | off | Adds ERC-8004 integration (implies `provider`) |
//! | `serde` | off | Derives `Serialize` / `Deserialize` on core types |
//!
//! ## Quick Start — Offline key construction
//!
//! ```rust
//! use ensip25::record_key::evm_record_key;
//!
//! let registry: alloy_primitives::Address =
//!     "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432".parse().unwrap();
//!
//! let key = evm_record_key(1, registry, 42).unwrap();
//! assert_eq!(
//!     key,
//!     "agent-registration[0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432][42]"
//! );
//! ```
//!
//! ## Quick Start — On-chain verification (`provider` feature)
//!
//! ```rust,ignore
//! use alloy::providers::ProviderBuilder;
//! use ensip25::verify::verify;
//!
//! let provider = ProviderBuilder::new()
//!     .connect_http("https://eth.llamarpc.com".parse()?);
//!
//! let registry = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432".parse()?;
//! let status = verify(&provider, "vitalik.eth", 1, registry, 42).await?;
//! println!("verified: {}", status.is_verified());
//! ```
//!
//! ## Quick Start — ERC-8004 integration (`erc8004` feature)
//!
//! ```rust,ignore
//! use alloy::providers::ProviderBuilder;
//! use ensip25::verify::verify_agent;
//!
//! let provider = ProviderBuilder::new()
//!     .connect_http("https://eth.llamarpc.com".parse()?);
//!
//! let status = verify_agent(
//!     &provider,
//!     erc8004::Network::EthereumMainnet,
//!     42,
//!     "vitalik.eth",
//! ).await?;
//! ```

pub mod erc7930;
pub mod error;
pub mod record_key;
#[cfg(feature = "provider")]
pub mod verify;

#[cfg(feature = "erc8004")]
pub use erc8004;
pub use error::{Error, Result};
