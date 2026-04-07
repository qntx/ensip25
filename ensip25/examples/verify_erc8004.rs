//! On-chain ENSIP-25 verification using ERC-8004 integration.
//!
//! Run with: `cargo run --example verify_erc8004 --features erc8004`
//!
//! Uses official ENSIP-25 test data (verified in both directions):
//! - ENS Name: `ens-registration-agent.ses.eth`
//! - Agent ID: `26433`
//! - Registry: ERC-8004 `IdentityRegistry` on Ethereum Mainnet
//!
//! This example demonstrates the simplified `verify_agent` API that
//! automatically resolves `chain_id` and registry address from the
//! `erc8004::Network` enum.

use alloy::providers::ProviderBuilder;
use alloy_ens as _;
use alloy_primitives as _;
use ensip25::verify::verify_agent;
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

    // Official ENSIP-25 test data
    let ens_name = "ens-registration-agent.ses.eth";
    let agent_id = 26433u64;

    println!("Verifying ENSIP-25 attestation (ERC-8004 API)...");
    println!("  ENS Name: {ens_name}");
    println!("  Agent ID: {agent_id}");
    println!("  Network: EthereumMainnet (auto-resolved)");
    println!();

    // Verify using the simplified ERC-8004 API
    // No need to manually specify chain_id or registry address!
    let status = verify_agent(
        &provider,
        erc8004::Network::EthereumMainnet,
        agent_id,
        ens_name,
    )
    .await?;

    if status.is_verified() {
        println!("✓ Attestation verified: {ens_name} → Agent #{agent_id}");
        println!("✓ Verification loop closed (verified in both directions)");
    } else {
        println!("✗ Attestation NOT found: {ens_name} has not set the agent-registration record");
    }

    Ok(())
}
