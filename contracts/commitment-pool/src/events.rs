use soroban_sdk::{contracttype, Address, BytesN, Env, Symbol};

/// Event data emitted when a new commitment is deposited into the pool.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deposited {
    pub commitment: BytesN<32>,
    pub index: u32,
    pub timestamp: u64,
}

/// Event data emitted when a withdrawal is processed from the pool.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withdrawn {
    pub nullifier: BytesN<32>,
    pub recipient: Address,
    pub amount: i128,
}

/// Event data emitted when the contract is paused.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paused {
    pub by: Address,
}

/// Event data emitted when the contract is unpaused.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unpaused {
    pub by: Address,
}

/// Emit a Deposited event with the commitment, its Merkle index, and the ledger timestamp.
pub fn emit_deposited(env: &Env, commitment: &BytesN<32>, index: u32) {
    let event = Deposited {
        commitment: commitment.clone(),
        index,
        timestamp: env.ledger().timestamp(),
    };
    env.events()
        .publish((Symbol::new(env, "deposited"),), event);
}

/// Emit a Withdrawn event with the nullifier, recipient, and amount.
pub fn emit_withdrawn(env: &Env, nullifier: &BytesN<32>, recipient: &Address, amount: i128) {
    let event = Withdrawn {
        nullifier: nullifier.clone(),
        recipient: recipient.clone(),
        amount,
    };
    env.events()
        .publish((Symbol::new(env, "withdrawn"),), event);
}

/// Emit a Paused event indicating who paused the contract.
pub fn emit_paused(env: &Env, by: &Address) {
    let event = Paused { by: by.clone() };
    env.events().publish((Symbol::new(env, "paused"),), event);
}

/// Emit an Unpaused event indicating who unpaused the contract.
pub fn emit_unpaused(env: &Env, by: &Address) {
    let event = Unpaused { by: by.clone() };
    env.events()
        .publish((Symbol::new(env, "unpaused"),), event);
}
