use soroban_sdk::{contracttype, Bytes, BytesN, Env};

const TREE_DEPTH: u32 = 20;

#[contracttype]
#[derive(Clone)]
pub enum MerkleKey {
    Node(u32, u32),
    LeafExists(BytesN<32>),
    NextIndex,
}

/// Sparse Merkle tree stored in Soroban persistent storage.
/// Uses SHA256 as placeholder hash.
/// TODO: Replace with Poseidon host function when CAP-0075 lands on mainnet.
pub struct MerkleTree;

impl MerkleTree {
    pub fn insert(env: &Env, leaf: &BytesN<32>) -> u32 {
        let index: u32 = env.storage().persistent()
            .get(&MerkleKey::NextIndex).unwrap_or(0);

        env.storage().persistent().set(&MerkleKey::Node(0, index), leaf);
        env.storage().persistent().set(&MerkleKey::LeafExists(leaf.clone()), &true);

        let mut current_index = index;
        let mut current_hash = leaf.clone();

        for level in 0..TREE_DEPTH {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            let sibling: BytesN<32> = env.storage().persistent()
                .get(&MerkleKey::Node(level, sibling_index))
                .unwrap_or(Self::zero_hash(env));

            let (left, right) = if current_index % 2 == 0 {
                (current_hash, sibling)
            } else {
                (sibling, current_hash)
            };

            current_hash = Self::hash_pair(env, &left, &right);
            current_index /= 2;
            env.storage().persistent()
                .set(&MerkleKey::Node(level + 1, current_index), &current_hash);
        }

        env.storage().persistent().set(&MerkleKey::NextIndex, &(index + 1));
        index
    }

    pub fn root(env: &Env) -> BytesN<32> {
        env.storage().persistent()
            .get(&MerkleKey::Node(TREE_DEPTH, 0))
            .unwrap_or(Self::zero_hash(env))
    }

    pub fn has_leaf(env: &Env, leaf: &BytesN<32>) -> bool {
        env.storage().persistent()
            .get(&MerkleKey::LeafExists(leaf.clone()))
            .unwrap_or(false)
    }

    /// SHA256(left || right) placeholder.
    /// TODO: Replace with Poseidon host function when CAP-0075 lands on mainnet.
    fn hash_pair(env: &Env, left: &BytesN<32>, right: &BytesN<32>) -> BytesN<32> {
        let mut combined = Bytes::new(env);
        combined.extend_from_array(&left.to_array());
        combined.extend_from_array(&right.to_array());
        env.crypto().sha256(&combined).into()
    }

    fn zero_hash(env: &Env) -> BytesN<32> {
        BytesN::from_array(env, &[0u8; 32])
    }
}
