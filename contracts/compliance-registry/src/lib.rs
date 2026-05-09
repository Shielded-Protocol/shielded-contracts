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
    /// Initialize the registry with an admin address.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ComplianceError> {
        if env.storage().instance().has(&RegistryKey::Admin) {
            return Err(ComplianceError::Unauthorized);
        }
        admin.require_auth();
        env.storage().instance().set(&RegistryKey::Admin, &admin);
        Ok(())
    }

    /// Set the auditor address. Only admin can call this.
    pub fn set_auditor(env: Env, auditor: Address) -> Result<(), ComplianceError> {
        let admin: Address = env.storage().instance()
            .get(&RegistryKey::Admin).ok_or(ComplianceError::Unauthorized)?;
        admin.require_auth();
        env.storage().instance().set(&RegistryKey::Auditor, &auditor);
        Ok(())
    }

    /// Register an encrypted viewing key for a commitment.
    ///
    /// The viewing key is encrypted with the auditor's public key,
    /// allowing only the auditor to decrypt transaction details.
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

    /// Get the encrypted viewing key for a commitment.
    /// Only the auditor can retrieve viewing keys.
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
