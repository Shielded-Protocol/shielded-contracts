use soroban_sdk::contracterror;

/// Errors returned by the ProxyBlend contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ProxyBlendError {
    /// The shielded proof is invalid
    InvalidProof = 1,
    /// Insufficient shielded balance
    InsufficientBalance = 2,
    /// Blend protocol interaction failed
    BlendError = 3,
    /// Caller not authorized
    Unauthorized = 4,
    /// Contract is paused
    ContractPaused = 5,
}
