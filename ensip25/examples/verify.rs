//! On-chain ENSIP-25 verification example.
//!
//! Run with: `cargo run --example verify --features provider`
//!
//! Requires an Ethereum RPC endpoint (defaults to public endpoint).

use alloy::providers::ProviderBuilder;
use alloy_primitives::address;
use ensip25::verify::verify;

#[allow(clippy::print_stdout)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Ethereum mainnet
    let provider = ProviderBuilder::new()
        .connect("https://eth.llamarpc.com")
        .await?;

    // ERC-8004 IdentityRegistry on Ethereum mainnet
    let registry = address!("8004A169FB4a3325136EB29fA0ceB6D2e539a432");
    let chain_id = 1u64;
    let agent_id = 167u64;
    let ens_name = "example.eth";

    // Verify the ENSIP-25 link
    let status = verify(&provider, ens_name, chain_id, registry, agent_id).await?;

    if status.is_verified() {
        println!("✓ {ens_name} has verified agent #{agent_id}");
    } else {
        println!("✗ {ens_name} has NOT set the agent-registration record");
    }

    Ok(())
}
