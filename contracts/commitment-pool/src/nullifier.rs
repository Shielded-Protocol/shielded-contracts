use soroban_sdk::{contracttype, BytesN, Env};

/// Storage key for nullifier spent state.
#[contracttype]
#[derive(Clone)]
pub enum NullifierKey {
    /// Tracks whether a specific nullifier has been spent.
    Spent(BytesN<32>),
}

/// NullifierSet prevents double-spending by tracking which nullifiers
/// have been consumed.
///
/// Each nullifier is derived from the secret used to create a commitment.
/// Once a nullifier is revealed during withdrawal, it is permanently marked
/// as spent, preventing the same commitment from being withdrawn twice.
pub struct NullifierSet;

impl NullifierSet {
    /// Mark a nullifier as spent. This is irreversible.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `nullifier` - The 32-byte nullifier hash to mark as spent
    pub fn mark_spent(env: &Env, nullifier: &BytesN<32>) {
        env.storage()
            .persistent()
            .set(&NullifierKey::Spent(nullifier.clone()), &true);
    }

    /// Check whether a nullifier has already been spent.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `nullifier` - The 32-byte nullifier hash to check
    ///
    /// # Returns
    /// `true` if the nullifier has been spent, `false` otherwise
    pub fn is_spent(env: &Env, nullifier: &BytesN<32>) -> bool {
        env.storage()
            .persistent()
            .get(&NullifierKey::Spent(nullifier.clone()))
            .unwrap_or(false)
    }
}
