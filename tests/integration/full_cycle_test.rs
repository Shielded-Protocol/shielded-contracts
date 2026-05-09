//! Integration test: full deposit → transfer → withdraw cycle.
//!
//! This test validates the end-to-end privacy flow:
//! 1. Initialize all contracts
//! 2. Deposit a commitment into the pool
//! 3. Verify the Merkle root updates
//! 4. Check nullifier is not yet spent
//! 5. (Withdraw would require actual ZK proof generation)

#[cfg(test)]
mod tests {
    use commitment_pool::CommitmentPool;
    use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env};

    #[test]
    fn test_full_deposit_cycle() {
        let env = Env::default();
        let pool_id = env.register(CommitmentPool, ());
        let pool = commitment_pool::CommitmentPoolClient::new(&env, &pool_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let verifier = Address::generate(&env);

        env.mock_all_auths();

        // Step 1: Initialize
        pool.initialize(&admin, &token, &verifier);

        // Step 2: Deposit a commitment
        let commitment_1 = BytesN::from_array(&env, &[1u8; 32]);
        let note_1 = Bytes::from_slice(&env, b"encrypted_note_1");
        pool.deposit(&commitment_1, &note_1);

        // Step 3: Verify root changed from zero
        let root_after_1 = pool.get_root();
        let zero_root = BytesN::from_array(&env, &[0u8; 32]);
        assert_ne!(root_after_1, zero_root, "Root should change after deposit");

        // Step 4: Deposit a second commitment
        let commitment_2 = BytesN::from_array(&env, &[2u8; 32]);
        let note_2 = Bytes::from_slice(&env, b"encrypted_note_2");
        pool.deposit(&commitment_2, &note_2);

        // Root should change again
        let root_after_2 = pool.get_root();
        assert_ne!(root_after_2, root_after_1, "Root should change with each deposit");

        // Step 5: Verify nullifier is not spent
        let nullifier = BytesN::from_array(&env, &[3u8; 32]);
        assert!(
            !pool.is_nullifier_spent(&nullifier),
            "Nullifier should not be spent before withdrawal"
        );

        // Step 6: Verify duplicate commitment is rejected
        let result = pool.try_deposit(&commitment_1, &note_1);
        assert!(result.is_err(), "Duplicate commitment should be rejected");
    }

    #[test]
    fn test_pause_unpause_cycle() {
        let env = Env::default();
        let pool_id = env.register(CommitmentPool, ());
        let pool = commitment_pool::CommitmentPoolClient::new(&env, &pool_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let verifier = Address::generate(&env);

        env.mock_all_auths();
        pool.initialize(&admin, &token, &verifier);

        // Pause the contract
        pool.pause();

        // Deposit should fail while paused
        let commitment = BytesN::from_array(&env, &[1u8; 32]);
        let note = Bytes::from_slice(&env, b"encrypted_note");
        let result = pool.try_deposit(&commitment, &note);
        assert!(result.is_err(), "Deposit should fail while paused");

        // Unpause
        pool.unpause();

        // Deposit should succeed again
        pool.deposit(&commitment, &note);
    }
}
