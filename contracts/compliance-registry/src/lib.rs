#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env};

mod errors;
use errors::ComplianceError;

/// Storage keys for the compliance registry.
#[contracttype]
#[derive(Clone)]
enum RegistryKey {
    Admin,
    Auditor,
    ViewingKey(BytesN<32>),
}

/// ComplianceRegistry stores encrypted viewing keys per commitment.
///
/// This allows designated auditors to decrypt and view transaction details
/// for regulatory compliance, while maintaining privacy from the general public.
#[contract]
pub struct ComplianceRegistry;

#[contractimpl]
impl ComplianceRegistry {
    /// Initializes the registry with an admin address.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `admin` - The address of the contract administrator.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful initialization.
    ///
    /// # Errors
    ///
    /// Returns `ComplianceError::Unauthorized` if the contract is already initialized.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ComplianceError> {
        if env.storage().instance().has(&RegistryKey::Admin) {
            return Err(ComplianceError::Unauthorized);
        }
        admin.require_auth();
        env.storage().instance().set(&RegistryKey::Admin, &admin);
        Ok(())
    }

    /// Sets the auditor address. Only the admin can call this function.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `auditor` - The address of the designated auditor.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` upon successful update of the auditor address.
    ///
    /// # Errors
    ///
    /// Returns `ComplianceError::Unauthorized` if the caller is not the admin.
    pub fn set_auditor(env: Env, auditor: Address) -> Result<(), ComplianceError> {
        let admin: Address = env.storage().instance()
            .get(&RegistryKey::Admin).ok_or(ComplianceError::Unauthorized)?;
        admin.require_auth();
        env.storage().instance().set(&RegistryKey::Auditor, &auditor);
        Ok(())
    }

    /// Registers an encrypted viewing key for a commitment.
    ///
    /// The viewing key is encrypted with the auditor's public key,
    /// allowing only the auditor to decrypt transaction details.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `commitment` - The 32-byte hash of the commitment.
    /// * `encrypted_key` - The encrypted viewing key.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` upon successful registration.
    ///
    /// # Errors
    ///
    /// Returns `ComplianceError::AlreadyRegistered` if a key is already registered for this commitment.
    pub fn register_viewing_key(
        env: Env,
        commitment: BytesN<32>,
        encrypted_key: Bytes,
    ) -> Result<(), ComplianceError> {
        if env.storage().persistent().has(&RegistryKey::ViewingKey(commitment.clone())) {
            return Err(ComplianceError::AlreadyRegistered);
        }
        env.storage().persistent()
            .set(&RegistryKey::ViewingKey(commitment), &encrypted_key);
        Ok(())
    }

    /// Gets the encrypted viewing key for a commitment.
    /// 
    /// Only the auditor can retrieve viewing keys.
    ///
    /// # Arguments
    ///
    /// * `env` - The execution environment.
    /// * `commitment` - The 32-byte hash of the commitment.
    ///
    /// # Returns
    ///
    /// Returns the encrypted viewing key as `Bytes`.
    ///
    /// # Errors
    ///
    /// * Returns `ComplianceError::NotAuditor` if the caller is not the registered auditor.
    /// * Returns `ComplianceError::NotFound` if no viewing key exists for the given commitment.
    pub fn get_viewing_key(
        env: Env,
        commitment: BytesN<32>,
    ) -> Result<Bytes, ComplianceError> {
        let auditor: Address = env.storage().instance()
            .get(&RegistryKey::Auditor).ok_or(ComplianceError::NotAuditor)?;
        auditor.require_auth();

        env.storage().persistent()
            .get(&RegistryKey::ViewingKey(commitment))
            .ok_or(ComplianceError::NotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_register_and_retrieve_viewing_key() {
        let env = Env::default();
        let contract_id = env.register(ComplianceRegistry, ());
        let client = ComplianceRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let auditor = Address::generate(&env);
        env.mock_all_auths();

        client.initialize(&admin);
        client.set_auditor(&auditor);

        let commitment = BytesN::from_array(&env, &[1u8; 32]);
        let key = Bytes::from_slice(&env, b"encrypted_viewing_key_data");
        client.register_viewing_key(&commitment, &key);

        let retrieved = client.get_viewing_key(&commitment);
        assert_eq!(retrieved, key);
    }
}
