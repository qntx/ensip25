//! On-chain ENSIP-25 verification example.
//!
//! Run with: `cargo run --example verify --features provider`
//!
//! Uses official ENSIP-25 test data (verified in both directions):
//! - ENS Name: `ens-registration-agent.ses.eth`
//! - Agent ID: `26433`
//! - Registry: ERC-8004 `IdentityRegistry` on Ethereum Mainnet

use alloy::providers::ProviderBuilder;
use alloy_ens as _;
use alloy_primitives::address;
use ensip25::verify::verify;
#[cfg(feature = "erc8004")]
use erc8004 as _;
#[cfg(feature = "serde")]
use serde as _;
use thiserror as _;

#[expect(clippy::print_stdout, reason = "example demonstrates CLI output")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Ethereum mainnet
    let provider = ProviderBuilder::new()
        .connect("https://eth.llamarpc.com")
        .await?;

    // Official ENSIP-25 test data (verified in both directions)
    let ens_name = "ens-registration-agent.ses.eth";
    let agent_id = 26433u64;
    let registry = address!("8004A169FB4a3325136EB29fA0ceB6D2e539a432");
    let chain_id = 1u64;

    println!("Verifying ENSIP-25 attestation...");
    println!("  ENS Name: {ens_name}");
    println!("  Agent ID: {agent_id}");
    println!("  Registry: ERC-8004 @ Ethereum Mainnet");
    println!();

    // Verify the ENSIP-25 link (ENS → Agent)
    let status = verify(&provider, ens_name, chain_id, registry, agent_id).await?;

    if status.is_verified() {
        println!("✓ Attestation verified: {ens_name} → Agent #{agent_id}");
    } else {
        println!("✗ Attestation NOT found: {ens_name} has not set the agent-registration record");
    }

    Ok(())
}
