use soroban_sdk::contracterror;

/// Errors returned by the CommitmentPool contract.
///
/// Each variant maps to a specific failure mode in the privacy pool lifecycle.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Commitment already exists in the pool — duplicate deposits are rejected
    /// to prevent state corruption in the Merkle tree.
    DuplicateCommitment = 1,

    /// Nullifier has already been spent — this prevents double-spend attacks.
    /// Each nullifier can only be used once across the lifetime of the contract.
    NullifierAlreadySpent = 2,

    /// ZK proof verification failed — the provided Groth16 proof did not
    /// satisfy the verification equation against the registered verification key.
    InvalidProof = 3,

    /// Amount mismatch between the proof's public inputs and the requested
    /// transfer amount.
    AmountMismatch = 4,

    /// Caller not authorized for this operation — administrative functions
    /// require the contract admin to authenticate.
    Unauthorized = 5,

    /// Contract is paused — all deposit and withdrawal operations are
    /// suspended until the admin unpauses the contract.
    ContractPaused = 6,
}
