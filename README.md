# Shielded Contracts

Privacy-preserving smart contracts for the Shielded Protocol, built on [Stellar Soroban](https://soroban.stellar.org).

## Overview

Shielded Protocol enables private, compliant DeFi transactions on Stellar using zero-knowledge proofs (Groth16 over BN254). Users can deposit assets into a shielded pool, transfer privately, and withdraw — all while maintaining regulatory compliance through encrypted viewing keys.

## Architecture

```
┌─────────────────────┐     ┌──────────────────────┐
│  CommitmentPool     │────▶│  Groth16Verifier     │
│  - deposit()        │     │  - verify()          │
│  - withdraw()       │     │  - register_vk()     │
│  - get_root()       │     └──────────────────────┘
│  - is_nullifier_    │
│    spent()          │     ┌──────────────────────┐
└─────────┬───────────┘     │  ComplianceRegistry  │
          │                 │  - register_viewing  │
          │                 │    _key()            │
          ▼                 │  - get_viewing_key() │
┌─────────────────────┐     └──────────────────────┘
│  ProxyBlend         │
│  - shielded_deposit │     
│  - shielded_withdraw│     
│  - shielded_borrow  │     
└─────────────────────┘
```

## Contracts

| Contract | Description |
|---|---|
| **commitment-pool** | Core privacy pool — Merkle tree of commitments, nullifier tracking, deposit/withdraw with ZK proof verification |
| **groth16-verifier** | On-chain Groth16 proof verifier using BN254 curve operations |
| **compliance-registry** | Encrypted viewing key registry for regulatory compliance |
| **proxy-blend** | Shielded proxy for Blend lending protocol integration |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Stellar CLI](https://soroban.stellar.org/docs/getting-started/setup) v25+
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Build

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Test

```bash
cargo test --all
```

### Format & Lint

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## Security

See [SECURITY.md](./SECURITY.md) for our security policy and how to report vulnerabilities.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Apache-2.0 — see [LICENSE](./LICENSE) for details.
