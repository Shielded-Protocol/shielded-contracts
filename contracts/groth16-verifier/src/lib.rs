#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, Vec};

mod errors;
use errors::VerifierError;

/// # Groth16 Verification on BN254
///
/// ## Verification Equation
///
/// A valid Groth16 proof π = (A, B, C) over BN254 satisfies:
///
///   e(A, B) = e(α, β) · Π e(aᵢ · Lᵢ, γ) · e(C, δ)
///
/// Which can be rewritten as a single multi-pairing check:
///
///   e(-A, B) · e(α, β) · e(Σ(aᵢ · Lᵢ), γ) · e(C, δ) = 1
///
/// Where:
///   - e(·,·)  = optimal Ate pairing on BN254
///   - A ∈ G₁  = proof element (first group)
///   - B ∈ G₂  = proof element (second group)
///   - C ∈ G₁  = proof element
///   - α ∈ G₁  = verification key element
///   - β ∈ G₂  = verification key element
///   - γ ∈ G₂  = verification key element
///   - δ ∈ G₂  = verification key element
///   - Lᵢ ∈ G₁ = verification key points for public inputs
///   - aᵢ      = public input scalars
///
/// ## Implementation Notes
///
/// This contract uses BN254 host functions from CAP-0074.
/// The BLS12-381 curve operations are available via soroban_sdk
/// crypto module. BN254 support is pending full host function availability.
/// Until then, this contract provides the verification interface
/// and structure, with the actual pairing operations stubbed.

#[contracttype]
#[derive(Clone)]
enum VkKey {
    /// Admin address
    Admin,
    /// Registered verification key by its hash
    Vk(BytesN<32>),
}

#[contract]
pub struct Groth16Verifier;

#[contractimpl]
impl Groth16Verifier {
    /// Initialize the verifier with an admin address.
    pub fn initialize(env: Env, admin: Address) -> Result<(), VerifierError> {
        if env.storage().instance().has(&VkKey::Admin) {
            return Err(VerifierError::Unauthorized);
        }
        admin.require_auth();
        env.storage().instance().set(&VkKey::Admin, &admin);
        Ok(())
    }

    /// Register a verification key by storing its serialized form.
    ///
    /// Only the admin can register verification keys.
    pub fn register_vk(
        env: Env,
        vk_hash: BytesN<32>,
        vk_data: Bytes,
    ) -> Result<(), VerifierError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&VkKey::Admin)
            .ok_or(VerifierError::Unauthorized)?;
        admin.require_auth();

        env.storage()
            .persistent()
            .set(&VkKey::Vk(vk_hash), &vk_data);
        Ok(())
    }

    /// Check if a verification key is registered.
    pub fn is_vk_registered(env: Env, vk_hash: BytesN<32>) -> bool {
        env.storage()
            .persistent()
            .has(&VkKey::Vk(vk_hash))
    }

    /// Verify a Groth16 proof against registered verification key.
    ///
    /// # Arguments
    /// * `proof` - Serialized proof bytes (A, B, C points)
    /// * `public_inputs` - Vector of 32-byte public input scalars
    /// * `vk_hash` - Hash of the verification key to use
    ///
    /// # Returns
    /// `true` if the proof is valid, `false` otherwise
    pub fn verify(
        env: Env,
        proof: Bytes,
        public_inputs: Vec<BytesN<32>>,
        vk_hash: BytesN<32>,
    ) -> Result<bool, VerifierError> {
        // Ensure the verification key is registered
        if !env.storage().persistent().has(&VkKey::Vk(vk_hash.clone())) {
            return Err(VerifierError::VkNotRegistered);
        }

        // Load the verification key data
        let _vk_data: Bytes = env
            .storage()
            .persistent()
            .get(&VkKey::Vk(vk_hash))
            .unwrap();

        // Validate proof format (expected: 3 G1/G2 points)
        // BN254 G1 point = 64 bytes, G2 point = 128 bytes
        // Proof = A(G1) + B(G2) + C(G1) = 64 + 128 + 64 = 256 bytes
        if proof.len() < 256 {
            return Err(VerifierError::InvalidProofFormat);
        }

        if public_inputs.is_empty() {
            return Err(VerifierError::InvalidPublicInputs);
        }

        // TODO: Implement actual BN254 pairing check using CAP-0074 host functions.
        //
        // The verification algorithm:
        // 1. Deserialize proof into points A ∈ G₁, B ∈ G₂, C ∈ G₁
        // 2. Deserialize verification key (α, β, γ, δ, IC[])
        // 3. Compute vk_x = IC[0] + Σ(public_inputs[i] * IC[i+1])
        // 4. Check: e(-A, B) · e(α, β) · e(vk_x, γ) · e(C, δ) == 1
        //
        // When BN254 host functions land (CAP-0074), replace this stub:
        // let result = env.crypto().bn254_pairing_check(&pairs);

        let _ = _vk_data;

        // Placeholder: return true for development/testing
        // SECURITY: This MUST be replaced with actual verification before mainnet
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_initialize_and_register_vk() {
        let env = Env::default();
        let contract_id = env.register(Groth16Verifier, ());
        let client = Groth16VerifierClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        env.mock_all_auths();

        client.initialize(&admin);

        let vk_hash = BytesN::from_array(&env, &[1u8; 32]);
        let vk_data = Bytes::from_slice(&env, &[0u8; 512]);
        client.register_vk(&vk_hash, &vk_data);

        assert!(client.is_vk_registered(&vk_hash));
    }
}
