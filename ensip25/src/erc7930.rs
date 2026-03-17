//! ERC-7930 Interoperable Address encoding and decoding.
//!
//! [ERC-7930](https://eips.ethereum.org/EIPS/eip-7930) defines a compact binary
//! format that binds a chain identifier and an address into a single payload.
//!
//! This module provides [`InteropAddress`] for constructing, encoding, decoding,
//! and displaying interoperable addresses — the building block for ENSIP-25
//! text record keys.
//!
//! # Wire format
//!
//! ```text
//! ┌─────────┬───────────┬──────────────────────┬────────────────┬───────────────┬─────────┐
//! │ Version │ ChainType │ ChainReferenceLength │ ChainReference │ AddressLength │ Address │
//! │ 2 bytes │ 2 bytes   │ 1 byte               │ variable       │ 1 byte        │ variable│
//! └─────────┴───────────┴──────────────────────┴────────────────┴───────────────┴─────────┘
//! ```

use core::fmt;

use alloy_primitives::{Address, hex};

use crate::error::{Error, Result};

/// Current ERC-7930 version.
const VERSION_1: u16 = 0x0001;

/// CASA namespace for EVM chains.
const CHAIN_TYPE_EVM: u16 = 0x0000;

/// An ERC-7930 interoperable address.
///
/// Represents a chain-specific address in the compact binary format defined by
/// the specification. Use [`InteropAddress::evm`] for the common EVM case or
/// [`InteropAddress::decode`] to parse raw bytes.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InteropAddress {
    /// Protocol version (`0x0001` for v1).
    pub version: u16,
    /// CASA namespace identifier (e.g. `0x0000` for EVM).
    pub chain_type: u16,
    /// Binary chain reference (e.g. big-endian chain ID for EVM).
    pub chain_ref: Vec<u8>,
    /// Binary address (e.g. 20-byte EVM address).
    pub address: Vec<u8>,
}

impl InteropAddress {
    /// Create an EVM interoperable address for the given chain ID and address.
    ///
    /// The chain ID is encoded as a minimal big-endian integer (no leading
    /// zero bytes), matching the ERC-7930 / CAIP-350 EVM profile.
    #[must_use]
    pub fn evm(chain_id: u64, address: Address) -> Self {
        Self {
            version: VERSION_1,
            chain_type: CHAIN_TYPE_EVM,
            chain_ref: minimal_be_bytes(chain_id),
            address: address.to_vec(),
        }
    }

    /// Create an EVM interoperable address **without** a chain reference.
    ///
    /// This is valid per ERC-7930 (chain reference length = 0).
    #[must_use]
    pub fn evm_no_chain(address: Address) -> Self {
        Self {
            version: VERSION_1,
            chain_type: CHAIN_TYPE_EVM,
            chain_ref: Vec::new(),
            address: address.to_vec(),
        }
    }

    /// Decode an interoperable address from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is malformed or uses an unsupported
    /// version.
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        // Minimum: version(2) + chain_type(2) + chain_ref_len(1) + addr_len(1) = 6
        if bytes.len() < 6 {
            return Err(Error::BufferTooShort { len: bytes.len() });
        }

        let version = u16::from_be_bytes([bytes[0], bytes[1]]);
        if version != VERSION_1 {
            return Err(Error::UnsupportedVersion { version });
        }

        let chain_type = u16::from_be_bytes([bytes[2], bytes[3]]);
        let chain_ref_len = bytes[4] as usize;

        let addr_len_offset = 5 + chain_ref_len;
        if bytes.len() < addr_len_offset + 1 {
            return Err(Error::TruncatedPayload {
                expected: addr_len_offset + 1,
                available: bytes.len(),
            });
        }

        let addr_len = bytes[addr_len_offset] as usize;
        let total = addr_len_offset + 1 + addr_len;
        if bytes.len() < total {
            return Err(Error::TruncatedPayload {
                expected: total,
                available: bytes.len(),
            });
        }

        if chain_ref_len == 0 && addr_len == 0 {
            return Err(Error::EmptyAddress);
        }

        let chain_ref = bytes[5..5 + chain_ref_len].to_vec();
        let address = bytes[addr_len_offset + 1..total].to_vec();

        Ok(Self {
            version,
            chain_type,
            chain_ref,
            address,
        })
    }

    /// Decode an interoperable address from a hex string (with or without
    /// `0x` prefix).
    ///
    /// # Errors
    ///
    /// Returns an error if the hex is invalid or the payload is malformed.
    pub fn from_hex(s: &str) -> Result<Self> {
        let bytes: Vec<u8> = hex::decode(s)?;
        Self::decode(&bytes)
    }

    /// Encode this interoperable address to raw bytes.
    ///
    /// # Panics
    ///
    /// Panics if `chain_ref` or `address` length exceeds 255 bytes.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let chain_ref_len = self.chain_ref.len();
        let addr_len = self.address.len();
        let mut buf = Vec::with_capacity(6 + chain_ref_len + addr_len);

        buf.extend_from_slice(&self.version.to_be_bytes());
        buf.extend_from_slice(&self.chain_type.to_be_bytes());
        buf.push(u8::try_from(chain_ref_len).expect("chain_ref length exceeds u8::MAX"));
        buf.extend_from_slice(&self.chain_ref);
        buf.push(u8::try_from(addr_len).expect("address length exceeds u8::MAX"));
        buf.extend_from_slice(&self.address);

        buf
    }

    /// Format as a lowercase hex string **with** `0x` prefix.
    #[must_use]
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.encode()))
    }

    /// Returns `true` if this is an EVM-type address (chain type `0x0000`).
    #[must_use]
    pub const fn is_evm(&self) -> bool {
        self.chain_type == CHAIN_TYPE_EVM
    }

    /// Try to extract the EVM chain ID from the chain reference.
    ///
    /// Returns `None` if the chain reference is empty or longer than 8 bytes.
    #[must_use]
    pub fn evm_chain_id(&self) -> Option<u64> {
        if self.chain_ref.is_empty() || self.chain_ref.len() > 8 {
            return None;
        }
        let mut padded = [0u8; 8];
        let offset = 8 - self.chain_ref.len();
        padded[offset..].copy_from_slice(&self.chain_ref);
        Some(u64::from_be_bytes(padded))
    }

    /// Try to extract the 20-byte EVM address.
    ///
    /// Returns `None` if the address is not exactly 20 bytes.
    #[must_use]
    pub fn evm_address(&self) -> Option<Address> {
        if self.address.len() == 20 {
            Some(Address::from_slice(&self.address))
        } else {
            None
        }
    }
}

impl fmt::Debug for InteropAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InteropAddress")
            .field("version", &format_args!("{:#06x}", self.version))
            .field("chain_type", &format_args!("{:#06x}", self.chain_type))
            .field("chain_ref", &hex::encode(&self.chain_ref))
            .field("address", &hex::encode(&self.address))
            .finish()
    }
}

impl fmt::Display for InteropAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl core::str::FromStr for InteropAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_hex(s)
    }
}

/// Encode a `u64` as minimal big-endian bytes (no leading zeros).
fn minimal_be_bytes(value: u64) -> Vec<u8> {
    if value == 0 {
        return vec![0];
    }
    let bytes = value.to_be_bytes();
    let skip = bytes.iter().position(|&b| b != 0).unwrap_or(7);
    bytes[skip..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ERC-7930 Example 1: Ethereum mainnet address (chain ID 1).
    ///
    /// ```text
    /// 0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432
    /// ```
    #[test]
    fn encode_evm_mainnet() {
        let addr: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid address");
        let ia = InteropAddress::evm(1, addr);
        assert_eq!(
            ia.to_hex(),
            "0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432"
        );
    }

    /// ERC-7930 Example 3: EVM address without chain reference.
    ///
    /// ```text
    /// 0x000100000014d8da6bf26964af9d7eed9e03e53415d37aa96045
    /// ```
    #[test]
    fn encode_evm_no_chain() {
        let addr: Address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
            .parse()
            .expect("valid address");
        let ia = InteropAddress::evm_no_chain(addr);
        assert_eq!(
            ia.to_hex(),
            "0x000100000014d8da6bf26964af9d7eed9e03e53415d37aa96045"
        );
    }

    /// Roundtrip: encode → decode → encode produces identical output.
    #[test]
    fn roundtrip() {
        let addr: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid address");
        let original = InteropAddress::evm(1, addr);
        let bytes = original.encode();
        let decoded = InteropAddress::decode(&bytes).expect("decode ok");
        assert_eq!(original, decoded);
        assert_eq!(original.to_hex(), decoded.to_hex());
    }

    /// Decode from hex string (with 0x prefix).
    #[test]
    fn from_hex_with_prefix() {
        let ia =
            InteropAddress::from_hex("0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432")
                .expect("decode ok");
        assert_eq!(ia.version, 0x0001);
        assert_eq!(ia.chain_type, 0x0000);
        assert_eq!(ia.evm_chain_id(), Some(1));
        assert_eq!(
            ia.evm_address(),
            Some(
                "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
                    .parse()
                    .expect("valid")
            )
        );
    }

    /// `FromStr` trait works.
    #[test]
    fn from_str() {
        let ia: InteropAddress = "0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432"
            .parse()
            .expect("parse ok");
        assert!(ia.is_evm());
    }

    /// Display produces the same hex as `to_hex`.
    #[test]
    fn display() {
        let addr: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid address");
        let ia = InteropAddress::evm(1, addr);
        assert_eq!(format!("{ia}"), ia.to_hex());
    }

    /// Too-short buffer is rejected.
    #[test]
    fn decode_too_short() {
        assert!(InteropAddress::decode(&[0, 1, 0]).is_err());
    }

    /// Unsupported version is rejected.
    #[test]
    fn decode_unsupported_version() {
        let err = InteropAddress::decode(&[0, 2, 0, 0, 0, 1, 0xFF]).unwrap_err();
        assert!(err.to_string().contains("unsupported version"));
    }

    /// Both lengths zero is rejected.
    #[test]
    fn decode_empty_address() {
        // version=1, chain_type=0, chain_ref_len=0, addr_len=0
        let err = InteropAddress::decode(&[0, 1, 0, 0, 0, 0]).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    /// ENSIP-25 example: ERC-8004 `IdentityRegistry` on Ethereum mainnet.
    ///
    /// The spec says the text record key for agent 167 should use:
    /// `0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432`
    #[test]
    fn ensip25_example_registry() {
        let registry: Address = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432"
            .parse()
            .expect("valid");
        let ia = InteropAddress::evm(1, registry);
        assert_eq!(
            ia.to_hex(),
            "0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432"
        );
    }

    /// Minimal BE encoding for various chain IDs.
    #[test]
    fn minimal_be_encoding() {
        assert_eq!(minimal_be_bytes(0), vec![0]);
        assert_eq!(minimal_be_bytes(1), vec![1]);
        assert_eq!(minimal_be_bytes(255), vec![255]);
        assert_eq!(minimal_be_bytes(256), vec![1, 0]);
        assert_eq!(minimal_be_bytes(11_155_111), vec![0xAA, 0x36, 0xA7]);
    }
}
