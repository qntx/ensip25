//! ENSIP-25 text record key construction.
//!
//! [ENSIP-25](https://docs.ens.domains/ensip/25) defines a parameterized ENS
//! text record key that links a registry entry to an ENS name:
//!
//! ```text
//! agent-registration[<registry>][<agentId>]
//! ```
//!
//! Where `<registry>` is the [ERC-7930](https://eips.ethereum.org/EIPS/eip-7930)
//! interoperable address of the registry contract, and `<agentId>` is the
//! registry-defined agent identifier.

use alloy_primitives::Address;

use crate::{
    erc7930::InteropAddress,
    error::{Ensip25Error, Result},
};

/// Build an ENSIP-25 text record key from a pre-built [`InteropAddress`] and
/// an agent ID string.
///
/// # Errors
///
/// Returns [`Ensip25Error::InvalidAgentId`] if `agent_id` contains `[` or `]`.
///
/// # Examples
///
/// ```
/// use ensip25::erc7930::InteropAddress;
/// use ensip25::record_key::record_key;
///
/// let registry: alloy_primitives::Address =
///     "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432".parse().unwrap();
/// let ia = InteropAddress::evm(1, registry);
///
/// let key = record_key(&ia, "167").unwrap();
/// assert_eq!(
///     key,
///     "agent-registration[0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432][167]"
/// );
/// ```
pub fn record_key(registry: &InteropAddress, agent_id: &str) -> Result<String> {
    validate_agent_id(agent_id)?;
    Ok(format!(
        "agent-registration[{}][{agent_id}]",
        registry.to_hex()?
    ))
}

/// Convenience: build an ENSIP-25 text record key for an EVM registry.
///
/// Encodes the registry address as an ERC-7930 interoperable address with the
/// given `chain_id`, then formats the full key.
///
/// # Errors
///
/// Returns [`Ensip25Error::InvalidAgentId`] if `agent_id` contains `[` or `]`.
///
/// # Examples
///
/// ```
/// use ensip25::record_key::evm_record_key;
///
/// let registry: alloy_primitives::Address =
///     "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432".parse().unwrap();
///
/// let key = evm_record_key(1, registry, 167).unwrap();
/// assert_eq!(
///     key,
///     "agent-registration[0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432][167]"
/// );
/// ```
pub fn evm_record_key(chain_id: u64, registry: Address, agent_id: u64) -> Result<String> {
    let ia = InteropAddress::evm(chain_id, registry);
    let id_str = agent_id.to_string();
    record_key(&ia, &id_str)
}

/// Validate that an agent ID does not contain forbidden characters.
fn validate_agent_id(agent_id: &str) -> Result<()> {
    if agent_id.contains('[') || agent_id.contains(']') {
        return Err(Ensip25Error::InvalidAgentId {
            agent_id: agent_id.to_owned(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ENSIP-25 spec example: agent 167 in the ERC-8004 `IdentityRegistry` on
    /// Ethereum mainnet.
    #[test]
    fn ensip25_spec_example() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let key = evm_record_key(1, registry, 167).expect("valid key");
        assert_eq!(
            key,
            "agent-registration[0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432][167]"
        );
    }

    /// The blog post example uses agent 42.
    #[test]
    fn blog_example_agent_42() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let key = evm_record_key(1, registry, 42).expect("valid key");
        assert_eq!(
            key,
            "agent-registration[0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432][42]"
        );
    }

    /// Agent ID with brackets is rejected.
    #[test]
    fn invalid_agent_id_brackets() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let ia = InteropAddress::evm(1, registry);
        assert!(record_key(&ia, "foo[bar]").is_err());
    }

    /// Agent ID with only `[` is rejected.
    #[test]
    fn invalid_agent_id_open_bracket() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let ia = InteropAddress::evm(1, registry);
        assert!(record_key(&ia, "foo[").is_err());
    }

    /// Agent ID with only `]` is rejected.
    #[test]
    fn invalid_agent_id_close_bracket() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let ia = InteropAddress::evm(1, registry);
        assert!(record_key(&ia, "]bar").is_err());
    }

    /// Empty agent ID is accepted (valid per spec).
    #[test]
    fn empty_agent_id_is_valid() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let ia = InteropAddress::evm(1, registry);
        let key = record_key(&ia, "").expect("valid key");
        assert!(key.ends_with("[]"));
    }

    /// Generic `record_key` with a pre-built `InteropAddress`.
    #[test]
    fn generic_record_key() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let ia = InteropAddress::evm(1, registry);
        let key = record_key(&ia, "42").expect("valid key");
        assert!(key.starts_with("agent-registration[0x"));
        assert!(key.ends_with("][42]"));
    }
}
