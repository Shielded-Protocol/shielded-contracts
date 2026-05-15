# shielded-contracts

[![CI](https://github.com/Shielded-Protocol/shielded-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/Shielded-Protocol/shielded-contracts/actions/workflows/ci.yml)
[![Stellar Wave](https://img.shields.io/badge/Stellar-Wave-blue)](https://drips.network/wave/stellar)
[![Issues](https://img.shields.io/github/issues/Shielded-Protocol/shielded-contracts)](https://github.com/Shielded-Protocol/shielded-contracts/issues)

The on-chain smart contracts for [Shielded Protocol](https://github.com/Shielded-Protocol). Written in **Rust** and compiled to WebAssembly for deployment on [Stellar's Soroban](https://soroban.stellar.org) smart contract platform.

---

## What these contracts do

When a user wants to deposit or withdraw privately, they interact with these contracts:

1. **Deposit** → User sends tokens to the `commitment-pool`. The pool records a cryptographic commitment (a hash of the user's secret) in an on-chain Merkle tree. The actual identity and amount stay private.

2. **Withdraw** → User generates a zero-knowledge proof off-chain (in the browser) then submits it to the pool. The `groth16-verifier` checks the proof is valid, the pool checks the nullifier hasn't been used before, then releases the funds to the recipient.

3. **Compliance** → Users can register encrypted viewing keys in the `compliance-registry`, granting specific auditors the ability to verify particular commitments without seeing the full transaction history.

---

## Contracts

| Contract | Description | Status |
|---|---|---|
| `commitment-pool` | Core pool contract. Handles deposits, withdrawals, the Merkle tree, and nullifier tracking. | ✅ Tests passing |
| `groth16-verifier` | Verifies Groth16 proofs on-chain using Stellar's BN254 host functions. Called by the pool during withdrawals. | 🔧 Scaffold |
| `compliance-registry` | Stores and retrieves encrypted viewing key grants per commitment. Auditors use this to verify positions. | ✅ Complete |
| `proxy-blend` | Shielded wrapper around the [Blend](https://blend.capital) lending protocol. Allows private lending/borrowing. | 🔧 Scaffold |

---

## Architecture

```
User
 │
 ├─ deposit(amount, commitment)
 │         │
 │         ▼
 │   CommitmentPool
 │   ├── Merkle tree (Poseidon, depth 20, 1M capacity)
 │   │     └── insert(commitment) → new root
 │   └── emit CommitmentInserted(commitment, index, root)
 │
 └─ withdraw(proof, nullifier, recipient, root)
           │
           ▼
     CommitmentPool
     ├── Groth16Verifier.verify(proof, public_signals)
     │         └── BN254 host functions (CAP-0074)
     ├── check nullifier_set.contains(nullifier) == false
     ├── nullifier_set.insert(nullifier)
     └── transfer(amount, token, recipient)

ComplianceRegistry (independent contract)
└── grant_viewing_key(commitment, auditor, encrypted_key)
└── get_viewing_key(commitment, auditor) → encrypted_key
```

---

## Prerequisites

- **Rust** stable toolchain
- **wasm32 target:** `rustup target add wasm32-unknown-unknown`
- **Stellar CLI:** `cargo install --locked stellar-cli --features opt`

---

## Running locally

```bash
git clone https://github.com/Shielded-Protocol/shielded-contracts
cd shielded-contracts

# Run all tests
cargo test --all
# Expected: test result: ok. 5 passed; 0 failed

# Test a specific contract
cargo test -p shielded-commitment-pool
cargo test -p shielded-compliance-registry
```

---

## Building for deployment

```bash
# Build all contracts to WASM
cargo build --target wasm32-unknown-unknown --release

# Or use the Stellar CLI helper (handles optimization)
stellar contract build
```

Compiled WASM files appear in `target/wasm32-unknown-unknown/release/`:

```
shielded_commitment_pool.wasm
shielded_groth16_verifier.wasm
shielded_compliance_registry.wasm
shielded_proxy_blend.wasm
```

---

## Deploying to Stellar testnet

```bash
# 1. Create and fund a deployer account
stellar keys generate --global deployer --network testnet
stellar keys fund deployer --network testnet

# 2. Deploy the Groth16 verifier first (the pool depends on it)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/shielded_groth16_verifier.wasm \
  --source deployer \
  --network testnet
# → Save the returned contract address as VERIFIER_ADDRESS

# 3. Deploy the commitment pool, passing the verifier address
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/shielded_commitment_pool.wasm \
  --source deployer \
  --network testnet
# → Save the returned contract address as POOL_ADDRESS

# 4. Deploy the compliance registry
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/shielded_compliance_registry.wasm \
  --source deployer \
  --network testnet
```

---

## Directory structure

```
contracts/
├── commitment-pool/     ← Core deposit/withdraw logic + Merkle tree
├── groth16-verifier/    ← On-chain BN254 Groth16 proof verifier
├── compliance-registry/ ← Encrypted viewing key storage
└── proxy-blend/         ← Shielded Blend protocol wrapper
```

---

## Soroban and BN254

Stellar's [CAP-0074](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0074.md) adds native BN254 elliptic curve operations as Soroban host functions. This makes on-chain Groth16 proof verification practical — the verifier only needs 3 pairing operations regardless of circuit complexity.

Without native host functions, verifying a Groth16 proof on a smart contract platform would be prohibitively expensive.

---

## Security

See [SECURITY.md](./SECURITY.md). Please do **not** open public GitHub issues for security vulnerabilities — use the private disclosure process described there.

Key security properties:
- The Merkle tree uses Poseidon hash, which is ZK-friendly and collision-resistant
- Nullifiers are stored permanently — a note can only be spent once
- The verifying key in the contract must match the proving key distributed to users

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

Browse [Wave-ready issues →](https://github.com/Shielded-Protocol/shielded-contracts/issues?q=label%3Astatus%3Awave-ready+state%3Aopen)

Good first issues are labeled `layer:contracts` and `difficulty:easy`.

---

## Related repos

| Repo | Description |
|---|---|
| [shielded-circuits](https://github.com/Shielded-Protocol/shielded-circuits) | Circom ZK circuits that generate the proofs these contracts verify |
| [shielded-sdk](https://github.com/Shielded-Protocol/shielded-sdk) | TypeScript SDK for interacting with these contracts from the browser |
| [shielded-app](https://github.com/Shielded-Protocol/shielded-app) | Frontend UI |

---

## License

MIT
