#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Bytes, BytesN, Env, Vec};

mod errors;
mod events;
mod merkle;
mod nullifier;

use errors::Error;
use events::{emit_deposited, emit_paused, emit_unpaused, emit_withdrawn};
use merkle::MerkleTree;
use nullifier::NullifierSet;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    TokenId,
    Paused,
    VerifierId,
}

#[contract]
pub struct CommitmentPool;

#[contractimpl]
impl CommitmentPool {
    /// Initialize the commitment pool with admin, token, and verifier contract addresses.
    pub fn initialize(
        env: Env,
        admin: Address,
        token_id: Address,
        verifier_id: Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::Unauthorized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenId, &token_id);
        env.storage().instance().set(&DataKey::VerifierId, &verifier_id);
        env.storage().instance().set(&DataKey::Paused, &false);
        Ok(())
    }

    /// Deposit a commitment into the shielded pool.
    ///
    /// The commitment is a hash of (secret, nullifier, amount, token).
    /// The encrypted_note allows the recipient to decrypt the note details.
    pub fn deposit(
        env: Env,
        commitment: BytesN<32>,
        encrypted_note: Bytes,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;

        // Validate the commitment is new (not duplicate)
        if MerkleTree::has_leaf(&env, &commitment) {
            return Err(Error::DuplicateCommitment);
        }

        // Insert into Merkle tree and get the leaf index
        let index = MerkleTree::insert(&env, &commitment);

        // Store encrypted note for off-chain retrieval
        let _ = encrypted_note; // TODO: store in events or off-chain indexer

        // Emit Deposited event
        emit_deposited(&env, &commitment, index);

        Ok(())
    }

    /// Withdraw from the pool by providing a valid ZK proof.
    ///
    /// The proof demonstrates knowledge of a valid commitment in the Merkle tree
    /// without revealing which commitment is being spent.
    pub fn withdraw(
        env: Env,
        nullifier: BytesN<32>,
        proof: Bytes,
        recipient: Address,
        amount: i128,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;

        // Check nullifier not previously used
        if NullifierSet::is_spent(&env, &nullifier) {
            return Err(Error::NullifierAlreadySpent);
        }

        // Get current Merkle root for proof verification
        let root = MerkleTree::root(&env);

        // Build public inputs: [root, nullifier]
        let mut public_inputs: Vec<BytesN<32>> = Vec::new(&env);
        public_inputs.push_back(root);
        public_inputs.push_back(nullifier.clone());

        // Get verifier contract address
        let verifier_id: Address = env
            .storage()
            .instance()
            .get(&DataKey::VerifierId)
            .unwrap();

        // TODO: Wire cross-contract call to groth16-verifier
        // let verifier = groth16_verifier::Client::new(&env, &verifier_id);
        // let vk_hash = ...;
        // if !verifier.verify(&proof, &public_inputs, &vk_hash) {
        //     return Err(Error::InvalidProof);
        // }
        let _ = (proof, verifier_id, public_inputs);

        // Mark nullifier as spent (irreversible)
        NullifierSet::mark_spent(&env, &nullifier);

        // Transfer tokens to recipient
        let token_id: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenId)
            .unwrap();
        let token_client = token::Client::new(&env, &token_id);
        token_client.transfer(&env.current_contract_address(), &recipient, &amount);

        // Emit Withdrawn event
        emit_withdrawn(&env, &nullifier, &recipient, amount);

        Ok(())
    }

    /// Returns the current Merkle root of the commitment tree.
    pub fn get_root(env: Env) -> BytesN<32> {
        MerkleTree::root(&env)
    }

    /// Checks whether a nullifier has already been spent.
    pub fn is_nullifier_spent(env: Env, nullifier: BytesN<32>) -> bool {
        NullifierSet::is_spent(&env, &nullifier)
    }

    /// Pause the contract. Only the admin can call this.
    pub fn pause(env: Env) -> Result<(), Error> {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin).ok_or(Error::Unauthorized)?;
        admin.require_auth();
        env.storage().instance().set(&DataKey::Paused, &true);
        emit_paused(&env, &admin);
        Ok(())
    }

    /// Unpause the contract. Only the admin can call this.
    pub fn unpause(env: Env) -> Result<(), Error> {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin).ok_or(Error::Unauthorized)?;
        admin.require_auth();
        env.storage().instance().set(&DataKey::Paused, &false);
        emit_unpaused(&env, &admin);
        Ok(())
    }

    fn require_not_paused(env: &Env) -> Result<(), Error> {
        let paused: bool = env.storage().instance()
            .get(&DataKey::Paused).unwrap_or(false);
        if paused {
            return Err(Error::ContractPaused);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_deposit_and_get_root() {
        let env = Env::default();
        let contract_id = env.register(CommitmentPool, ());
        let client = CommitmentPoolClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let verifier = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin, &token, &verifier);

        let commitment = BytesN::from_array(&env, &[1u8; 32]);
        let note = Bytes::from_slice(&env, &[0u8; 64]);

        client.deposit(&commitment, &note);

        let root = client.get_root();
        assert_ne!(root, BytesN::from_array(&env, &[0u8; 32]));
    }

    #[test]
    fn test_duplicate_commitment_rejected() {
        let env = Env::default();
        let contract_id = env.register(CommitmentPool, ());
        let client = CommitmentPoolClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let verifier = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin, &token, &verifier);

        let commitment = BytesN::from_array(&env, &[1u8; 32]);
        let note = Bytes::from_slice(&env, &[0u8; 64]);

        client.deposit(&commitment, &note);

        // Second deposit with same commitment should fail
        let result = client.try_deposit(&commitment, &note);
        assert!(result.is_err());
    }

    #[test]
    fn test_nullifier_tracking() {
        let env = Env::default();
        let contract_id = env.register(CommitmentPool, ());
        let client = CommitmentPoolClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let verifier = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin, &token, &verifier);

        let nullifier = BytesN::from_array(&env, &[2u8; 32]);
        assert!(!client.is_nullifier_spent(&nullifier));
    }
}
