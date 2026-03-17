# ensip25

[![CI][ci-badge]][ci-url]
[![crates.io][crate-badge]][crate-url]
[![docs.rs][doc-badge]][doc-url]
[![License][license-badge]][license-url]
[![Rust][rust-badge]][rust-url]

[ci-badge]: https://github.com/qntx/ensip25/actions/workflows/rust.yml/badge.svg
[ci-url]: https://github.com/qntx/ensip25/actions/workflows/rust.yml
[crate-badge]: https://img.shields.io/crates/v/ensip25.svg
[crate-url]: https://crates.io/crates/ensip25
[doc-badge]: https://img.shields.io/docsrs/ensip25.svg
[doc-url]: https://docs.rs/ensip25
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
[license-url]: LICENSE-MIT
[rust-badge]: https://img.shields.io/badge/rust-edition%202024-orange.svg
[rust-url]: https://doc.rust-lang.org/edition-guide/

**Type-safe Rust SDK for [ENSIP-25](https://docs.ens.domains/ensip/25) — verify the bidirectional link between ENS names and AI agent identities registered in on-chain registries such as [ERC-8004](https://eips.ethereum.org/EIPS/eip-8004).**

ENSIP-25 defines a parameterized ENS text record that links an ENS name to an agent registry entry:

```text
agent-registration[<registry>][<agentId>]
```

This SDK provides:

- **ERC-7930 encoding/decoding** — compact binary interoperable addresses
- **Text record key construction** — deterministic ENSIP-25 key formatting
- **On-chain verification** — ENS text record lookup via alloy provider
- **ERC-8004 integration** — one-call verification using the [`erc8004`](https://crates.io/crates/erc8004) crate

## Quick Start

### Offline key construction (no network)

```rust
use ensip25::record_key::evm_record_key;

let registry: alloy_primitives::Address =
    "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432".parse()?;

let key = evm_record_key(1, registry, 42)?;
assert_eq!(
    key,
    "agent-registration[0x000100000101148004a169fb4a3325136eb29fa0ceb6d2e539a432][42]"
);
```

### On-chain verification (`provider` feature)

```rust
use alloy::providers::ProviderBuilder;
use ensip25::verify::verify;

let provider = ProviderBuilder::new()
    .connect_http("https://eth.llamarpc.com".parse()?);

let registry = "0x8004A169FB4a3325136EB29fA0ceB6D2e539a432".parse()?;
let status = verify(&provider, "vitalik.eth", 1, registry, 42).await?;
println!("verified: {}", status.is_verified());
```

### ERC-8004 integration (`erc8004` feature)

```rust
use alloy::providers::ProviderBuilder;
use ensip25::verify::verify_agent;

let provider = ProviderBuilder::new()
    .connect_http("https://eth.llamarpc.com".parse()?);

let status = verify_agent(
    &provider,
    erc8004::Network::EthereumMainnet,
    42,
    "vitalik.eth",
).await?;
```

## Architecture

| Module | Description |
| --- | --- |
| **[`erc7930`](ensip25/src/erc7930.rs)** | ERC-7930 Interoperable Address encode/decode/Display — pure computation, no I/O |
| **[`record_key`](ensip25/src/record_key.rs)** | ENSIP-25 text record key construction — string formatting only |
| **[`verify`](ensip25/src/verify.rs)** | On-chain ENS text record resolution and verification (requires `provider` feature) |
| **[`error`](ensip25/src/error.rs)** | Unified error type covering ERC-7930 parsing, ENS resolution, and verification |

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `provider` | off | Enables `verify` module with on-chain ENS lookup via alloy |
| `erc8004` | off | Adds `verify_agent` convenience + re-exports `erc8004` crate (implies `provider`) |
| `serde` | off | Derives `Serialize` / `Deserialize` on core types |

## Design

- **Zero-dependency core** — `erc7930` and `record_key` modules depend only on `alloy-primitives` + `thiserror`
- **Provider-generic** — works with any alloy transport (HTTP, WebSocket, IPC)
- **Strict linting** — `pedantic` + `nursery` + `correctness` (deny)
- **Spec-compliant** — test vectors derived from ENSIP-25 and ERC-7930 specification examples

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project shall be dual-licensed as above, without any additional terms or conditions.

---

<div align="center">

A **[QNTX](https://qntx.fun)** open-source project.

<a href="https://qntx.fun"><img alt="QNTX" width="369" src="https://raw.githubusercontent.com/qntx/.github/main/profile/qntx-banner.svg" /></a>

<!--prettier-ignore-->
Code is law. We write both.

</div>
