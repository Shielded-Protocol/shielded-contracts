use soroban_sdk::contracterror;

/// Errors returned by the ComplianceRegistry contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ComplianceError {
    /// Viewing key already registered for this commitment
    AlreadyRegistered = 1,
    /// No viewing key found for this commitment
    NotFound = 2,
    /// Caller is not the designated auditor
    NotAuditor = 3,
    /// Caller not authorized
    Unauthorized = 4,
}
