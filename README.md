# shielded-contracts

> Smart contracts for the Shielded Protocol, built on Stellar Soroban.

Part of [shielded-protocol](https://github.com/Shielded-Protocol) — 
private, compliant DeFi on Stellar.

[![CI](https://github.com/Shielded-Protocol/shielded-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/Shielded-Protocol/shielded-contracts/actions)
[![Stellar Wave](https://img.shields.io/badge/Stellar-Wave-blue)](https://drips.network/wave/stellar)

## What this does

Shielded Protocol enables private, compliant DeFi transactions on Stellar using zero-knowledge proofs (Groth16 over BN254). Users can deposit assets into a shielded pool, transfer privately, and withdraw — all while maintaining regulatory compliance through encrypted viewing keys.

The core privacy logic is implemented in Rust and deployed as Soroban smart contracts, ensuring the integrity of the Merkle tree and preventing double-spends via nullifier sets.

## Quickstart

```bash
cargo build --release --target wasm32-unknown-unknown
cargo test --all
```

## Architecture

[Link to shielded-docs](https://github.com/Shielded-Protocol/shielded-docs)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).  
Browse [Wave-ready issues](../../issues?q=label%3Astatus%3Awave-ready).

## License

MIT
