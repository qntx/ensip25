//! ERC-7930 Interoperable Address encoding/decoding example.
//!
//! Run with: `cargo run --example erc7930_encode`
//!
//! Uses official ENSIP-25 test data:
//! - Registry: ERC-8004 `IdentityRegistry` on Ethereum Mainnet
//! - Expected: `0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432`

use alloy_primitives::address;
use ensip25::erc7930::InteropAddress;

#[allow(clippy::print_stdout)]
fn main() {
    // ERC-8004 IdentityRegistry on Ethereum mainnet (chain_id = 1)
    let registry = address!("8004A169FB4a3325136EB29fA0ceB6D2e539a432");
    let addr = InteropAddress::evm(1, registry);

    // Encode to hex string
    let hex = addr.to_hex();
    println!("ERC-7930 Encoded Registry: {hex}");

    // Verify against official ENSIP-25 example
    assert_eq!(
        hex,
        "0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432"
    );
    println!("✓ Matches official ENSIP-25 example");

    // Decode from hex
    let decoded = InteropAddress::from_hex(&hex).expect("valid hex");
    assert_eq!(addr, decoded);
    println!("✓ Roundtrip encode/decode successful");

    // Access individual fields
    println!("\nParsed fields:");
    println!("  Version: {:#06x}", addr.version);
    println!("  Chain type: {} (EVM)", addr.chain_type);
    println!(
        "  Chain ref: 0x{} (chain_id = 1)",
        alloy_primitives::hex::encode(&addr.chain_ref)
    );
    println!(
        "  Address: 0x{}",
        alloy_primitives::hex::encode(&addr.address)
    );
}
