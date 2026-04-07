//! On-chain ENSIP-25 verification.
//!
//! This module provides functions to verify the bidirectional attestation
//! between an ENS name and an AI agent registered in an on-chain registry
//! (e.g. ERC-8004).
//!
//! Requires the `provider` feature.

use alloy::providers::Provider;
use alloy_ens::ProviderEnsExt as _;
use alloy_primitives::Address;

use crate::{error::Result, record_key::evm_record_key};

/// The result of an ENSIP-25 verification check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VerificationStatus {
    /// The ENS name has the `agent-registration[…][…]` text record set to a
    /// non-empty value — the association is confirmed.
    Verified,
    /// The ENS text record does not exist or is empty — the ENS name owner has
    /// **not** confirmed the relationship.
    EnsRecordMissing,
}

impl VerificationStatus {
    /// Returns `true` if the verification succeeded.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

impl core::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Verified => f.write_str("verified"),
            Self::EnsRecordMissing => f.write_str("ENS record missing"),
        }
    }
}

/// Verify the ENSIP-25 link from an EVM registry entry to an ENS name.
///
/// This performs step 2-4 of the ENSIP-25 verification flow:
///
/// 1. Constructs the text record key using ERC-7930 encoding.
/// 2. Resolves the text record on `ens_name` via the provider.
/// 3. Returns [`VerificationStatus::Verified`] if the value is non-empty.
///
/// # Parameters
///
/// - `provider` — Any alloy provider connected to Ethereum mainnet (or the
///   chain where ENS is deployed).
/// - `ens_name` — The ENS name to check (e.g. `"vitalik.eth"`).
/// - `chain_id` — The EIP-155 chain ID where the registry is deployed.
/// - `registry` — The registry contract address.
/// - `agent_id` — The agent's numeric ID within the registry.
///
/// # Errors
///
/// Returns an error if the text record key cannot be constructed or if the
/// ENS lookup fails at the RPC level.
pub async fn verify<P: Provider>(
    provider: &P,
    ens_name: &str,
    chain_id: u64,
    registry: Address,
    agent_id: u64,
) -> Result<VerificationStatus> {
    let key = evm_record_key(chain_id, registry, agent_id)?;

    match provider.lookup_txt(ens_name, &key).await {
        Ok(value) if !value.is_empty() => Ok(VerificationStatus::Verified),
        Ok(_) | Err(alloy_ens::EnsError::ResolveTxtRecord(_)) => {
            Ok(VerificationStatus::EnsRecordMissing)
        }
        Err(e) => Err(e.into()),
    }
}

/// Verify an agent registered on a known ERC-8004 network.
///
/// This is a convenience wrapper around [`verify`] that uses the
/// [`erc8004::Network`] to determine the chain ID and registry address
/// automatically.
///
/// # Errors
///
/// Returns an error if the ENS lookup fails at the RPC level.
#[cfg(feature = "erc8004")]
pub async fn verify_agent<P: Provider>(
    provider: &P,
    network: erc8004::Network,
    agent_id: u64,
    ens_name: &str,
) -> Result<VerificationStatus> {
    let chain_id = network.chain_id();
    let registry = network.addresses().identity;
    verify(provider, ens_name, chain_id, registry, agent_id).await
}
