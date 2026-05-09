# shielded-contracts

> Rust/Soroban smart contracts for Shielded Protocol.

Part of [Shielded Protocol](https://github.com/Shielded-Protocol) —
private, compliant DeFi on Stellar.

[![CI](https://github.com/Shielded-Protocol/shielded-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/Shielded-Protocol/shielded-contracts/actions/workflows/ci.yml)
[![Stellar Wave](https://img.shields.io/badge/Stellar-Wave-blue)](https://drips.network/wave/stellar)
[![Issues](https://img.shields.io/github/issues/Shielded-Protocol/shielded-contracts)](https://github.com/Shielded-Protocol/shielded-contracts/issues)

---

## Contracts

| Contract | Description | Status |
|---|---|---|
| `commitment-pool` | Core pool: deposits, withdrawals, Merkle tree (depth 20), nullifier set | ✅ Tests passing |
| `groth16-verifier` | On-chain Groth16 proof verifier using BN254 host functions | 🔧 Scaffold |
| `compliance-registry` | Encrypted viewing key grant/revoke per commitment | ✅ Complete |
| `proxy-blend` | Shielded wrapper for Blend lending protocol | 🔧 Scaffold |

---

## Architecture

```
CommitmentPool
├── MerkleTree        (Poseidon-based, depth 20, 1M capacity)
├── NullifierSet      (prevents double-spend)
└── Groth16Verifier   (cross-contract proof verification)
    └── BN254 host functions (CAP-0074, Protocol X-Ray)

ComplianceRegistry
└── ViewingKeyGrant   (encrypted per commitment, auditor-specific)

ShieldedProxyBlend
└── Blend Pool        (shielded supply/borrow interface)
```

# Prerequisites: Rust stable, wasm32-unknown-unknown target, Stellar CLI

rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt

# Clone and test
git clone https://github.com/Shielded-Protocol/shielded-contracts
cd shielded-contracts
cargo test --all

Expected output: test result: ok. 5 passed; 0 failed

Build for deployment
cargo build --target wasm32-unknown-unknown --release

# Or use Stellar CLI
stellar contract build

Deploy to testnet
stellar keys generate --global deployer --network testnet
stellar keys fund deployer --network testnet

stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/shielded_commitment_pool.wasm \
  --source deployer \
  --network testnet

Contributing
See CONTRIBUTING.md.
Browse Wave-ready issues —
all labeled with difficulty and estimated time.

Security
See SECURITY.md. Do not file public issues for
security vulnerabilities.

License
MIT
