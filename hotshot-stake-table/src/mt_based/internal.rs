//! Utilities and internals for maintaining a local stake table

use super::config::{u256_to_field, Digest, FieldType, TREE_BRANCH};
use ark_ff::{Field, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{hash::Hash, sync::Arc, vec, vec::Vec};
use ethereum_types::U256;
use hotshot_types::traits::stake_table::StakeTableError;
use jf_primitives::{crhf::CRHF, signatures::bls_over_bn254};
use jf_utils::canonical;
use serde::{Deserialize, Serialize};
use tagged_base64::tagged;

/// Common trait bounds for generic key type `K` for [`PersistentMerkleNode`]
pub trait Key:
    Clone + CanonicalSerialize + CanonicalDeserialize + PartialEq + Eq + IntoFields<FieldType> + Hash
{
}
impl<T> Key for T where
    T: Clone
        + CanonicalSerialize
        + CanonicalDeserialize
        + PartialEq
        + Eq
        + IntoFields<FieldType>
        + Hash
{
}

/// A trait that converts into a field element.
/// Help avoid "cannot impl foreign traits on foreign types" problem
pub trait IntoFields<F: Field> {
    fn into_fields(self) -> [F; 2];
}

impl IntoFields<FieldType> for FieldType {
    fn into_fields(self) -> [FieldType; 2] {
        [FieldType::default(), self]
    }
}

impl IntoFields<FieldType> for bls_over_bn254::VerKey {
    fn into_fields(self) -> [FieldType; 2] {
        let bytes = jf_utils::to_bytes!(&self.to_affine()).unwrap();
        let x = <ark_bn254::Fq as PrimeField>::from_le_bytes_mod_order(&bytes[..32]);
        let y = <ark_bn254::Fq as PrimeField>::from_le_bytes_mod_order(&bytes[32..]);
        [x, y]
    }
}

/// A persistent merkle tree tailored for the stake table.
/// Generic over the key type `K`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound = "K: Key")]
pub(crate) enum PersistentMerkleNode<K: Key> {
    Empty,
    Branch {
        #[serde(with = "canonical")]
        comm: FieldType,
        children: [Arc<PersistentMerkleNode<K>>; TREE_BRANCH],
        num_keys: usize,
        total_stakes: U256,
    },
    Leaf {
        #[serde(with = "canonical")]
        comm: FieldType,
        #[serde(with = "canonical")]
        key: K,
        value: U256,
    },
}

/// A compressed Merkle node for Merkle path
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MerklePathEntry<K> {
    Branch {
        pos: usize,
        #[serde(with = "canonical")]
        siblings: [FieldType; TREE_BRANCH - 1],
    },
    Leaf {
        key: K,
        value: U256,
    },
}
/// Path from a Merkle root to a leaf
pub type MerklePath<K> = Vec<MerklePathEntry<K>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// An existential proof
pub struct MerkleProof<K> {
    /// Index for the given key
    pub index: usize,
    /// A Merkle path for the given leaf
    pub path: MerklePath<K>,
}

impl<K: Key> MerkleProof<K> {
    pub fn tree_height(&self) -> usize {
        self.path.len() - 1
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn get_key(&self) -> Option<&K> {
        match self.path.first() {
            Some(MerklePathEntry::Leaf { key, value: _ }) => Some(key),
            _ => None,
        }
    }

    pub fn get_value(&self) -> Option<&U256> {
        match self.path.first() {
            Some(MerklePathEntry::Leaf { key: _, value }) => Some(value),
            _ => None,
        }
    }

    pub fn get_key_value(&self) -> Option<(&K, &U256)> {
        match self.path.first() {
            Some(MerklePathEntry::Leaf { key, value }) => Some((key, value)),
            _ => None,
        }
    }

    pub fn compute_root(&self) -> Result<FieldType, StakeTableError> {
        match self.path.first() {
            Some(MerklePathEntry::Leaf { key, value }) => {
                let mut input = [FieldType::default(); 3];
                input[..2].copy_from_slice(&(*key).clone().into_fields()[..]);
                input[2] = u256_to_field(value);
                let init = Digest::evaluate(input).map_err(|_| StakeTableError::RescueError)?[0];
                self.path
                    .iter()
                    .skip(1)
                    .fold(Ok(init), |result, node| match node {
                        MerklePathEntry::Branch { pos, siblings } => match result {
                            Ok(comm) => {
                                let mut input = [FieldType::from(0); TREE_BRANCH];
                                input[..*pos].copy_from_slice(&siblings[..*pos]);
                                input[*pos] = comm;
                                input[pos + 1..].copy_from_slice(&siblings[*pos..]);
                                let comm = Digest::evaluate(input)
                                    .map_err(|_| StakeTableError::RescueError)?[0];
                                Ok(comm)
                            }
                            Err(_) => unreachable!(),
                        },
                        _ => Err(StakeTableError::MalformedProof),
                    })
            }
            _ => Err(StakeTableError::MalformedProof),
        }
    }

    pub fn verify(&self, comm: &MerkleCommitment) -> Result<(), StakeTableError> {
        if self.tree_height() != comm.tree_height() || !self.compute_root()?.eq(comm.digest()) {
            Err(StakeTableError::VerificationError)
        } else {
            Ok(())
        }
    }
}

#[tagged("MERKLE_COMM")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, CanonicalSerialize, CanonicalDeserialize)]
/// A succint commitment for Merkle tree
pub struct MerkleCommitment {
    /// Merkle tree digest
    comm: FieldType,
    /// Height of a tree
    height: usize,
    /// Number of leaves
    size: usize,
}

impl MerkleCommitment {
    pub fn new(comm: FieldType, height: usize, size: usize) -> Self {
        Self { comm, height, size }
    }

    pub fn digest(&self) -> &FieldType {
        &self.comm
    }

    pub fn tree_height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<K: Key> PersistentMerkleNode<K> {
    /// Returns the succint commitment of this subtree
    pub fn commitment(&self) -> FieldType {
        match self {
            PersistentMerkleNode::Empty => FieldType::from(0),
            PersistentMerkleNode::Branch {
                comm,
                children: _,
                num_keys: _,
                total_stakes: _,
            } => *comm,
            PersistentMerkleNode::Leaf {
                comm,
                key: _,
                value: _,
            } => *comm,
        }
    }

    /// Returns the total number of keys in this subtree
    pub fn num_keys(&self) -> usize {
        match self {
            PersistentMerkleNode::Empty => 0,
            PersistentMerkleNode::Branch {
                comm: _,
                children: _,
                num_keys,
                total_stakes: _,
            } => *num_keys,
            PersistentMerkleNode::Leaf {
                comm: _,
                key: _,
                value: _,
            } => 1,
        }
    }

    /// Returns the total stakes in this subtree
    pub fn total_stakes(&self) -> U256 {
        match self {
            PersistentMerkleNode::Empty => U256::zero(),
            PersistentMerkleNode::Branch {
                comm: _,
                children: _,
                num_keys: _,
                total_stakes,
            } => *total_stakes,
            PersistentMerkleNode::Leaf {
                comm: _,
                key: _,
                value,
            } => *value,
        }
    }

    /// Returns the stakes withhelded by a public key, None if the key is not registered.
    pub fn simple_lookup(&self, height: usize, path: &[usize]) -> Result<U256, StakeTableError> {
        match self {
            PersistentMerkleNode::Empty => Err(StakeTableError::KeyNotFound),
            PersistentMerkleNode::Branch {
                comm: _,
                children,
                num_keys: _,
                total_stakes: _,
            } => children[path[height - 1]].simple_lookup(height - 1, path),
            PersistentMerkleNode::Leaf {
                comm: _,
                key: _,
                value,
            } => Ok(*value),
        }
    }

    /// Returns a Merkle proof to the given location
    pub fn lookup(&self, height: usize, path: &[usize]) -> Result<MerkleProof<K>, StakeTableError> {
        match self {
            PersistentMerkleNode::Empty => Err(StakeTableError::KeyNotFound),
            PersistentMerkleNode::Branch {
                comm: _,
                children,
                num_keys: _,
                total_stakes: _,
            } => {
                let pos = path[height - 1];
                let mut proof = children[pos].lookup(height - 1, path)?;
                let siblings = children
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != pos)
                    .map(|(_, node)| node.commitment())
                    .collect::<Vec<_>>();
                proof.path.push(MerklePathEntry::Branch {
                    pos,
                    siblings: siblings.try_into().unwrap(),
                });
                Ok(proof)
            }
            PersistentMerkleNode::Leaf {
                comm: _,
                key,
                value,
            } => Ok(MerkleProof {
                index: from_merkle_path(path),
                path: vec![MerklePathEntry::Leaf {
                    key: key.clone(),
                    value: *value,
                }],
            }),
        }
    }

    /// Imagine that the keys in this subtree is sorted, returns the first key such that
    /// the prefix sum of withholding stakes is greater or equal the given `stake_number`.
    /// Useful for key sampling weighted by withholding stakes
    pub fn get_key_by_stake(&self, mut stake_number: U256) -> Option<(&K, &U256)> {
        if stake_number >= self.total_stakes() {
            None
        } else {
            match self {
                PersistentMerkleNode::Empty => None,
                PersistentMerkleNode::Branch {
                    comm: _,
                    children,
                    num_keys: _,
                    total_stakes: _,
                } => {
                    let mut ptr = 0;
                    while stake_number >= children[ptr].total_stakes() {
                        stake_number -= children[ptr].total_stakes();
                        ptr += 1;
                    }
                    children[ptr].get_key_by_stake(stake_number)
                }
                PersistentMerkleNode::Leaf {
                    comm: _,
                    key,
                    value,
                } => Some((key, value)),
            }
        }
    }

    /// Insert a new `key` into the Merkle tree
    pub fn register(
        &self,
        height: usize,
        path: &[usize],
        key: &K,
        value: U256,
    ) -> Result<Arc<Self>, StakeTableError> {
        if height == 0 {
            if matches!(self, PersistentMerkleNode::Empty) {
                let mut input = [FieldType::default(); 3];
                input[..2].copy_from_slice(&(*key).clone().into_fields()[..]);
                input[2] = u256_to_field(&value);
                Ok(Arc::new(PersistentMerkleNode::Leaf {
                    comm: Digest::evaluate(input).map_err(|_| StakeTableError::RescueError)?[0],
                    key: key.clone(),
                    value,
                }))
            } else {
                Err(StakeTableError::ExistingKey)
            }
        } else {
            let mut children = if let &PersistentMerkleNode::Branch {
                comm: _,
                children,
                num_keys: _,
                total_stakes: _,
            } = &self
            {
                children.clone()
            } else {
                [0; TREE_BRANCH].map(|_| Arc::new(PersistentMerkleNode::Empty))
            };
            children[path[height - 1]] =
                children[path[height - 1]].register(height - 1, path, key, value)?;
            let num_keys = children.iter().map(|child| child.num_keys()).sum();
            let total_stakes = children
                .iter()
                .map(|child| child.total_stakes())
                .fold(U256::zero(), |sum, val| sum + val);
            let comm = Digest::evaluate(children.clone().map(|child| child.commitment()))
                .map_err(|_| StakeTableError::RescueError)?[0];
            Ok(Arc::new(PersistentMerkleNode::Branch {
                comm,
                children,
                num_keys,
                total_stakes,
            }))
        }
    }

    /// Update the stake of the `key` with `(negative ? -1 : 1) * delta`.
    /// Return the updated stake
    pub fn update(
        &self,
        height: usize,
        path: &[usize],
        key: &K,
        delta: U256,
        negative: bool,
    ) -> Result<(Arc<Self>, U256), StakeTableError> {
        match self {
            PersistentMerkleNode::Empty => Err(StakeTableError::KeyNotFound),
            PersistentMerkleNode::Branch {
                comm: _,
                children,
                num_keys: _,
                total_stakes: _,
            } => {
                let mut children = children.clone();
                let value: U256;
                (children[path[height - 1]], value) =
                    children[path[height - 1]].update(height - 1, path, key, delta, negative)?;
                let num_keys = children.iter().map(|child| child.num_keys()).sum();
                let total_stakes = children
                    .iter()
                    .map(|child| child.total_stakes())
                    .fold(U256::zero(), |sum, val| sum + val);
                let comm = Digest::evaluate(children.clone().map(|child| child.commitment()))
                    .map_err(|_| StakeTableError::RescueError)?[0];
                Ok((
                    Arc::new(PersistentMerkleNode::Branch {
                        comm,
                        children,
                        num_keys,
                        total_stakes,
                    }),
                    value,
                ))
            }
            PersistentMerkleNode::Leaf {
                comm: _,
                key: node_key,
                value: old_value,
            } => {
                if key == node_key {
                    let value = if negative {
                        old_value
                            .checked_sub(delta)
                            .ok_or(StakeTableError::InsufficientFund)
                    } else {
                        old_value
                            .checked_add(delta)
                            .ok_or(StakeTableError::StakeOverflow)
                    }?;
                    let mut input = [FieldType::default(); 3];
                    input[..2].copy_from_slice(&(*key).clone().into_fields()[..]);
                    input[2] = u256_to_field(&value);
                    Ok((
                        Arc::new(PersistentMerkleNode::Leaf {
                            comm: Digest::evaluate(input)
                                .map_err(|_| StakeTableError::RescueError)?[0],
                            key: key.clone(),
                            value,
                        }),
                        value,
                    ))
                } else {
                    Err(StakeTableError::MismatchedKey)
                }
            }
        }
    }

    /// Set the stake of `key` to be `value`.
    /// Return the previous stake
    pub fn set_value(
        &self,
        height: usize,
        path: &[usize],
        key: &K,
        value: U256,
    ) -> Result<(Arc<Self>, U256), StakeTableError> {
        match self {
            PersistentMerkleNode::Empty => Err(StakeTableError::KeyNotFound),
            PersistentMerkleNode::Branch {
                comm: _,
                children,
                num_keys: _,
                total_stakes: _,
            } => {
                let mut children = children.clone();
                let old_value: U256;
                (children[path[height - 1]], old_value) =
                    children[path[height - 1]].set_value(height - 1, path, key, value)?;
                let num_keys = children.iter().map(|child| child.num_keys()).sum();
                if num_keys == 0 {
                    Ok((Arc::new(PersistentMerkleNode::Empty), value))
                } else {
                    let total_stakes = children
                        .iter()
                        .map(|child| child.total_stakes())
                        .fold(U256::zero(), |sum, val| sum + val);
                    let comm = Digest::evaluate(children.clone().map(|child| child.commitment()))
                        .map_err(|_| StakeTableError::RescueError)?[0];
                    Ok((
                        Arc::new(PersistentMerkleNode::Branch {
                            comm,
                            children,
                            num_keys,
                            total_stakes,
                        }),
                        old_value,
                    ))
                }
            }
            PersistentMerkleNode::Leaf {
                comm: _,
                key: cur_key,
                value: old_value,
            } => {
                if key == cur_key {
                    let mut input = [FieldType::default(); 3];
                    input[..2].copy_from_slice(&(*key).clone().into_fields()[..]);
                    input[2] = u256_to_field(&value);
                    Ok((
                        Arc::new(PersistentMerkleNode::Leaf {
                            comm: Digest::evaluate(input)
                                .map_err(|_| StakeTableError::RescueError)?[0],
                            key: key.clone(),
                            value,
                        }),
                        *old_value,
                    ))
                } else {
                    Err(StakeTableError::MismatchedKey)
                }
            }
        }
    }
}

/// An owning iterator over the (key, value) entries of a `PersistentMerkleNode`
/// Traverse using post-order: children from left to right, finally visit the current.
pub struct IntoIter<K: Key> {
    unvisited: Vec<Arc<PersistentMerkleNode<K>>>,
    num_visited: usize,
}

impl<K: Key> IntoIter<K> {
    /// create a new merkle tree iterator from a `root`.
    /// This (abstract) `root` can be an internal node of a larger tree, our iterator
    /// will iterate over all of its children.
    pub(crate) fn new(root: Arc<PersistentMerkleNode<K>>) -> Self {
        Self {
            unvisited: vec![root],
            num_visited: 0,
        }
    }
}

impl<K: Key> Iterator for IntoIter<K> {
    type Item = (K, U256);
    fn next(&mut self) -> Option<Self::Item> {
        if self.unvisited.is_empty() {
            return None;
        }

        let visiting = (**self.unvisited.last()?).clone();
        match visiting {
            PersistentMerkleNode::Empty => None,
            PersistentMerkleNode::Leaf {
                comm: _,
                key,
                value,
            } => {
                self.unvisited.pop();
                self.num_visited += 1;
                Some((key, value))
            }
            PersistentMerkleNode::Branch {
                comm: _,
                children,
                num_keys: _,
                total_stakes: _,
            } => {
                self.unvisited.pop();
                // put the left-most child to the last, so it is visited first.
                self.unvisited.extend(children.into_iter().rev());
                self.next()
            }
        }
    }
}

impl<K: Key> IntoIterator for PersistentMerkleNode<K> {
    type Item = (K, U256);
    type IntoIter = self::IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(Arc::new(self))
    }
}

/// Convert an index to a list of Merkle path branches
pub fn to_merkle_path(idx: usize, height: usize) -> Vec<usize> {
    let mut pos = idx;
    let mut ret: Vec<usize> = vec![];
    for _ in 0..height {
        ret.push(pos % TREE_BRANCH);
        pos /= TREE_BRANCH;
    }
    ret
}

/// Convert a list of Merkle path branches back to an index
pub fn from_merkle_path(path: &[usize]) -> usize {
    path.iter()
        .fold((0, 1), |(pos, mul), branch| {
            (pos + mul * branch, mul * TREE_BRANCH)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::super::config;
    use super::{to_merkle_path, PersistentMerkleNode};
    use ark_std::{
        rand::{Rng, RngCore},
        sync::Arc,
        vec,
        vec::Vec,
    };
    use ethereum_types::U256;
    use jf_utils::test_rng;

    type Key = ark_bn254::Fq;

    #[test]
    fn test_persistent_merkle_tree() {
        let height = 3;
        let mut roots = vec![Arc::new(PersistentMerkleNode::<Key>::Empty)];
        let path = (0..10)
            .map(|idx| to_merkle_path(idx, height))
            .collect::<Vec<_>>();
        let keys = (0..10).map(Key::from).collect::<Vec<_>>();
        // Insert key (0..10) with associated value 100 to the persistent merkle tree
        for (i, key) in keys.iter().enumerate() {
            roots.push(
                roots
                    .last()
                    .unwrap()
                    .register(height, &path[i], key, U256::from(100))
                    .unwrap(),
            );
        }
        // Check that if the insertion is perform correctly
        for i in 0..10 {
            assert!(roots[i].simple_lookup(height, &path[i]).is_err());
            assert_eq!(i, roots[i].num_keys());
            assert_eq!(
                U256::from((i as u64 + 1) * 100),
                roots[i + 1].total_stakes()
            );
            assert_eq!(
                U256::from(100),
                roots[i + 1].simple_lookup(height, &path[i]).unwrap()
            );
        }
        // test get_key_by_stake
        keys.iter().enumerate().for_each(|(i, key)| {
            assert_eq!(
                key,
                roots
                    .last()
                    .unwrap()
                    .get_key_by_stake(U256::from(i as u64 * 100 + i as u64 + 1))
                    .unwrap()
                    .0
            );
        });

        // test for `lookup` and Merkle proof
        for i in 0..10 {
            let proof = roots.last().unwrap().lookup(height, &path[i]).unwrap();
            assert_eq!(height, proof.tree_height());
            assert_eq!(&keys[i], proof.get_key().unwrap());
            assert_eq!(&U256::from(100), proof.get_value().unwrap());
            assert_eq!(
                roots.last().unwrap().commitment(),
                proof.compute_root().unwrap()
            );
        }

        // test for `set_value`
        // `set_value` with wrong key should fail
        assert!(roots
            .last()
            .unwrap()
            .set_value(height, &path[2], &keys[1], U256::from(100))
            .is_err());
        // A successful `set_value`
        let (new_root, value) = roots
            .last()
            .unwrap()
            .set_value(height, &path[2], &keys[2], U256::from(90))
            .unwrap();
        roots.push(new_root);
        assert_eq!(U256::from(100), value);
        assert_eq!(
            U256::from(90),
            roots
                .last()
                .unwrap()
                .simple_lookup(height, &path[2])
                .unwrap()
        );
        assert_eq!(U256::from(990), roots.last().unwrap().total_stakes());

        // test for `update`
        // `update` with a wrong key should fail
        assert!(roots
            .last()
            .unwrap()
            .update(height, &path[3], &keys[0], U256::from(10), false)
            .is_err());
        // `update` that results in a negative stake should fail
        assert!(roots
            .last()
            .unwrap()
            .update(height, &path[3], &keys[3], U256::from(200), true)
            .is_err());
        // A successful `update`
        let (new_root, value) = roots
            .last()
            .unwrap()
            .update(height, &path[2], &keys[2], U256::from(10), false)
            .unwrap();
        roots.push(new_root);
        assert_eq!(U256::from(100), value);
        assert_eq!(
            value,
            roots
                .last()
                .unwrap()
                .simple_lookup(height, &path[2])
                .unwrap()
        );
        assert_eq!(U256::from(1000), roots.last().unwrap().total_stakes());
    }

    #[test]
    fn test_mt_iter() {
        let height = 3;
        let capacity = config::TREE_BRANCH.pow(height);
        let mut rng = test_rng();

        for _ in 0..5 {
            let num_keys = rng.gen_range(1..capacity);
            let keys: Vec<Key> = (0..num_keys).map(|i| Key::from(i as u64)).collect();
            let paths = (0..num_keys)
                .map(|idx| to_merkle_path(idx, height as usize))
                .collect::<Vec<_>>();
            let amounts: Vec<U256> = (0..num_keys).map(|_| U256::from(rng.next_u64())).collect();

            // register all `num_keys` of (key, amount) pair.
            let mut root = Arc::new(PersistentMerkleNode::<Key>::Empty);
            for i in 0..num_keys {
                root = root
                    .register(height as usize, &paths[i], &keys[i], amounts[i])
                    .unwrap();
            }
            for (i, (k, v)) in (*root).clone().into_iter().enumerate() {
                assert_eq!((k, v), (keys[i], amounts[i]));
            }
        }
    }
}
