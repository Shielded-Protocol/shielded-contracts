#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env};

mod errors;
use errors::ProxyBlendError;

/// Storage keys for the ProxyBlend contract.
#[contracttype]
#[derive(Clone)]
enum ProxyKey {
    Admin,
    CommitmentPoolId,
    BlendPoolId,
    Paused,
}

/// ProxyBlend wraps the Blend lending protocol interface to enable
/// shielded (private) lending operations.
///
/// Users can deposit, withdraw, and borrow from Blend pools while
/// maintaining transaction privacy through the shielded commitment pool.
#[contract]
pub struct ProxyBlend;

#[contractimpl]
impl ProxyBlend {
    /// Initializes the proxy with admin and pool addresses.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `admin` - The address of the contract administrator.
    /// * `commitment_pool_id` - The address of the shielded commitment pool.
    /// * `blend_pool_id` - The address of the Blend lending pool.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful initialization.
    ///
    /// # Errors
    ///
    /// Returns `ProxyBlendError::Unauthorized` if the contract is already initialized.
    pub fn initialize(
        env: Env,
        admin: Address,
        commitment_pool_id: Address,
        blend_pool_id: Address,
    ) -> Result<(), ProxyBlendError> {
        if env.storage().instance().has(&ProxyKey::Admin) {
            return Err(ProxyBlendError::Unauthorized);
        }
        admin.require_auth();
        env.storage().instance().set(&ProxyKey::Admin, &admin);
        env.storage().instance().set(&ProxyKey::CommitmentPoolId, &commitment_pool_id);
        env.storage().instance().set(&ProxyKey::BlendPoolId, &blend_pool_id);
        env.storage().instance().set(&ProxyKey::Paused, &false);
        Ok(())
    }

    /// Deposits into the Blend lending pool using a shielded withdrawal proof.
    ///
    /// Flow: User proves ownership of shielded funds → proxy withdraws
    /// from commitment pool → deposits into Blend pool.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `nullifier` - The nullifier corresponding to the shielded commitment being spent.
    /// * `proof` - The ZK proof proving ownership of the commitment.
    /// * `amount` - The amount of tokens to deposit.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` upon successful deposit.
    ///
    /// # Errors
    ///
    /// Returns `ProxyBlendError::ContractPaused` if the proxy contract is paused.
    pub fn shielded_deposit(
        env: Env,
        nullifier: BytesN<32>,
        proof: Bytes,
        amount: i128,
    ) -> Result<(), ProxyBlendError> {
        Self::require_not_paused(&env)?;

        // TODO: Verify proof via commitment pool's withdraw
        // TODO: Deposit into Blend pool
        let _ = (nullifier, proof, amount);

        Ok(())
    }

    /// Withdraws from the Blend lending pool into a new shielded commitment.
    ///
    /// Flow: Proxy withdraws from Blend → creates new commitment
    /// in the shielded pool.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `commitment` - The 32-byte hash of the new commitment to create.
    /// * `amount` - The amount of tokens to withdraw.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` upon successful withdrawal.
    ///
    /// # Errors
    ///
    /// Returns `ProxyBlendError::ContractPaused` if the proxy contract is paused.
    pub fn shielded_withdraw(
        env: Env,
        commitment: BytesN<32>,
        amount: i128,
    ) -> Result<(), ProxyBlendError> {
        Self::require_not_paused(&env)?;

        // TODO: Withdraw from Blend pool
        // TODO: Deposit into commitment pool with new commitment
        let _ = (commitment, amount);

        Ok(())
    }

    /// Borrows from the Blend lending pool using shielded collateral.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `collateral_nullifier` - The nullifier corresponding to the collateral commitment.
    /// * `collateral_proof` - The ZK proof proving ownership of the collateral.
    /// * `borrow_amount` - The amount of tokens to borrow.
    /// * `borrow_commitment` - The new commitment hash for the borrowed funds.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` upon successful borrow operation.
    ///
    /// # Errors
    ///
    /// Returns `ProxyBlendError::ContractPaused` if the proxy contract is paused.
    pub fn shielded_borrow(
        env: Env,
        collateral_nullifier: BytesN<32>,
        collateral_proof: Bytes,
        borrow_amount: i128,
        borrow_commitment: BytesN<32>,
    ) -> Result<(), ProxyBlendError> {
        Self::require_not_paused(&env)?;

        // TODO: Verify collateral proof
        // TODO: Interact with Blend pool to borrow
        // TODO: Deposit borrowed amount as new commitment
        let _ = (collateral_nullifier, collateral_proof, borrow_amount, borrow_commitment);

        Ok(())
    }

    fn require_not_paused(env: &Env) -> Result<(), ProxyBlendError> {
        let paused: bool = env.storage().instance()
            .get(&ProxyKey::Paused).unwrap_or(false);
        if paused {
            return Err(ProxyBlendError::ContractPaused);
        }
        Ok(())
    }
}
