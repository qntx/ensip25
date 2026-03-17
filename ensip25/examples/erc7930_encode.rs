//! ERC-7930 Interoperable Address encoding/decoding example.
//!
//! Run with: `cargo run --example erc7930_encode`

use alloy_primitives::address;
use ensip25::erc7930::InteropAddress;

#[allow(clippy::print_stdout)]
fn main() {
    // Create an EVM interoperable address (Ethereum mainnet)
    let addr = InteropAddress::evm(1, address!("8004A169FB4a3325136EB29fA0ceB6D2e539a432"));

    // Encode to hex string
    let hex = addr.to_hex();
    println!("Encoded: {hex}");
    // Output: 0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432

    // Decode from hex
    let decoded = InteropAddress::from_hex(&hex).expect("valid hex");
    assert_eq!(addr, decoded);

    // Display uses hex format
    println!("Display: {addr}");

    // Access individual fields
    println!("Chain type: {}", addr.chain_type);
    println!(
        "Chain ref: 0x{}",
        alloy_primitives::hex::encode(&addr.chain_ref)
    );
    println!(
        "Address: 0x{}",
        alloy_primitives::hex::encode(&addr.address)
    );
}
