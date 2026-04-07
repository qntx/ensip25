//! ENSIP-25 text record key construction example.
//!
//! Run with: `cargo run --example record_key`
//!
//! Uses official ENSIP-25 test data:
//! - ENS Name: `ens-registration-agent.ses.eth`
//! - Agent ID: `26433`
//! - Registry: ERC-8004 `IdentityRegistry` on Ethereum Mainnet

use alloy_primitives::address;
use ensip25::record_key::evm_record_key;

#[cfg(feature = "provider")]
use alloy as _;
#[cfg(feature = "provider")]
use alloy_ens as _;
#[cfg(feature = "erc8004")]
use erc8004 as _;
#[cfg(feature = "serde")]
use serde as _;
use thiserror as _;
use tokio as _;

#[expect(clippy::print_stdout, reason = "example demonstrates CLI output")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Official ENSIP-25 test data
    // ENS Name: ens-registration-agent.ses.eth
    // Agent ID: 26433
    // Registry: ERC-8004 IdentityRegistry on Ethereum mainnet
    let registry = address!("8004A169FB4a3325136EB29fA0ceB6D2e539a432");
    let chain_id = 1u64;
    let agent_id = 26433u64;

    // Build the ENSIP-25 text record key
    let key = evm_record_key(chain_id, registry, agent_id)?;

    println!("Text record key: {key}");

    // Verify against official ENSIP-25 example
    if key != "agent-registration[0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432][26433]"
    {
        return Err("record key does not match official ENSIP-25 example".into());
    }
    println!("✓ Matches official ENSIP-25 example");

    Ok(())
}
