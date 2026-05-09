use soroban_sdk::contracterror;

/// Errors returned by the Groth16Verifier contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VerifierError {
    /// The proof bytes are malformed or invalid length
    InvalidProofFormat = 1,
    /// The public inputs are invalid
    InvalidPublicInputs = 2,
    /// The verification key is not registered
    VkNotRegistered = 3,
    /// Pairing check failed — proof is invalid
    PairingCheckFailed = 4,
    /// Caller not authorized
    Unauthorized = 5,
}
