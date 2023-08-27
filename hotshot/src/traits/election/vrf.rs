use hotshot_types::traits::signature_key::EncodedPublicKey;

#[allow(deprecated)]
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug, marker::PhantomData, num::NonZeroU64};

// TODO wrong palce for this
/// the sortition committee size parameter
pub const SORTITION_PARAMETER: u64 = 100;

// TODO compatibility this function's impl into a trait
// TODO do we necessariy want the units of stake to be a u64? or generics
/// The stake table for VRFs
#[derive(Serialize, Deserialize, Debug)]
pub struct VRFStakeTable<VRF, VRFHASHER, VRFPARAMS> {
    /// the mapping of id -> stake
    mapping: BTreeMap<EncodedPublicKey, NonZeroU64>,
    /// total stake present
    total_stake: NonZeroU64,
    /// PhantomData
    _pd: PhantomData<(VRF, VRFHASHER, VRFPARAMS)>,
}

impl<VRF, VRFHASHER, VRFPARAMS> Clone for VRFStakeTable<VRF, VRFHASHER, VRFPARAMS> {
    fn clone(&self) -> Self {
        Self {
            mapping: self.mapping.clone(),
            total_stake: self.total_stake,
            _pd: PhantomData,
        }
    }
}

// impl<VRF, VRFHASHER, VRFPARAMS> VRFStakeTable<VRF, VRFHASHER, VRFPARAMS> {
//     /// get total stake
//     #[must_use]
//     pub fn get_all_stake(&self) -> NonZeroU64 {
//         self.total_stake
//     }
// }

// impl<VRF, VRFHASHER, VRFPARAMS> VRFStakeTable<VRF, VRFHASHER, VRFPARAMS>
// where
//     VRF: Vrf,
//     VRFPARAMS: Bls12Parameters,
//     <VRFPARAMS as Bls12Parameters>::G1Parameters: TEHashToGroup,
//     VRF::PublicKey: Clone,
// {
//     /// get total stake
//     /// # Panics
//     /// If converting non-zero stake into `NonZeroU64` fails
//     pub fn get_stake<SIGSCHEME>(&self, pk: &JfPubKey<SIGSCHEME>) -> Option<NonZeroU64>
//     where
//         SIGSCHEME: SignatureScheme<
//             VerificationKey = VRF::PublicKey,
//             PublicParameter = (),
//             MessageUnit = u8,
//         >,
//         SIGSCHEME::VerificationKey: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//         SIGSCHEME::SigningKey: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//         SIGSCHEME::Signature: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//     {
//         let encoded = pk.to_bytes();
//         let stake = self.mapping.get(&encoded).map(|val| val.get());
//         stake.and_then(NonZeroU64::new)
//     }
// }

// /// the vrf implementation
// #[derive(Derivative)]
// #[derivative(Debug, Eq, PartialEq)]
// pub struct VrfImpl<TYPES, LEAF: LeafType<NodeType = TYPES>, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS>
// where
//     VRF: Vrf + Sync + Send,
//     TYPES: NodeType,
// {
//     /// the stake table
//     #[derivative(Debug = "ignore", PartialEq = "ignore")]
//     stake_table: VRFStakeTable<VRF, VRFHASHER, VRFPARAMS>,
//     /// the proof params
//     #[derivative(Debug = "ignore", PartialEq = "ignore")]
//     proof_parameters: VRF::PublicParameter,
//     /// the rng
//     #[derivative(PartialEq = "ignore")]
//     prng: std::sync::Arc<std::sync::Mutex<rand_chacha::ChaChaRng>>,
//     /// the committee parameter
//     sortition_parameter: NonZeroU64,
//     /// the chain commitment seed
//     chain_seed: [u8; 32],
//     /// pdf cache
//     #[derivative(PartialEq = "ignore")]
//     _sortition_cache: std::sync::Arc<std::sync::Mutex<HashMap<BinomialQuery, Ratio<BigUint>>>>,

//     /// phantom data
//     _pd: PhantomData<(TYPES, LEAF, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS)>,
// }

// impl<TYPES, LEAF: LeafType<NodeType = TYPES>, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS> Clone
//     for VrfImpl<TYPES, LEAF, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS>
// where
//     VRF: Vrf<PublicParameter = ()> + Sync + Send,
//     TYPES: NodeType,
// {
//     fn clone(&self) -> Self {
//         Self {
//             stake_table: self.stake_table.clone(),
//             proof_parameters: (),
//             prng: self.prng.clone(),
//             sortition_parameter: self.sortition_parameter,
//             chain_seed: self.chain_seed,
//             _sortition_cache: Arc::default(),
//             _pd: PhantomData,
//         }
//     }
// }

// /// TODO doc me
// #[derive(Serialize, Deserialize, Clone)]
// pub struct VRFVoteToken<PUBKEY, PROOF> {
//     /// The public key assocaited with this token
//     pub pub_key: PUBKEY,
//     /// The list of signatures
//     pub proof: PROOF,
//     /// The number of signatures that are valid
//     /// TODO (ct) this should be the sorition outbput
//     pub count: NonZeroU64,
// }

// impl<PUBKEY, PROOF> Hash for VRFVoteToken<PUBKEY, PROOF>
// where
//     PUBKEY: serde::Serialize,
//     PROOF: serde::Serialize,
// {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         bincode_opts().serialize(&self.pub_key).unwrap().hash(state);
//         bincode_opts().serialize(&self.proof).unwrap().hash(state);
//         self.count.hash(state);
//     }
// }

// impl<PUBKEY, PROOF> Debug for VRFVoteToken<PUBKEY, PROOF> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("VRFVoteToken")
//             .field("pub_key", &std::any::type_name::<PUBKEY>())
//             .field("proof", &std::any::type_name::<PROOF>())
//             .field("count", &self.count)
//             .finish()
//     }
// }

// impl<PUBKEY, PROOF> PartialEq for VRFVoteToken<PUBKEY, PROOF>
// where
//     PUBKEY: serde::Serialize,
//     PROOF: serde::Serialize,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.count == other.count
//             && bincode_opts().serialize(&self.pub_key).unwrap()
//                 == bincode_opts().serialize(&other.pub_key).unwrap()
//             && bincode_opts().serialize(&self.proof).unwrap()
//                 == bincode_opts().serialize(&other.proof).unwrap()
//     }
// }

// impl<PUBKEY, PROOF> VoteToken for VRFVoteToken<PUBKEY, PROOF>
// where
//     PUBKEY: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
//     PROOF: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
// {
//     fn vote_count(&self) -> NonZeroU64 {
//         self.count
//     }
// }

// impl<PUBKEY, PROOF> Committable for VRFVoteToken<PUBKEY, PROOF>
// where
//     PUBKEY: serde::Serialize,
//     PROOF: serde::Serialize,
// {
//     fn commit(&self) -> Commitment<Self> {
//         RawCommitmentBuilder::new(std::any::type_name::<Self>())
//             .u64(self.count.get())
//             .var_size_bytes(bincode_opts().serialize(&self.pub_key).unwrap().as_slice())
//             .var_size_bytes(bincode_opts().serialize(&self.proof).unwrap().as_slice())
//             .finalize()
//     }

//     fn tag() -> String {
//         tag::VRF_VOTE_TOKEN.to_string()
//     }
// }

// // KEY is VRFPubKey
// impl<VRFHASHER, VRFPARAMS, VRF, SIGSCHEME, TYPES, LEAF: LeafType<NodeType = TYPES>>
//     Membership<TYPES> for VrfImpl<TYPES, LEAF, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS>
// where
//     SIGSCHEME: SignatureScheme<PublicParameter = (), MessageUnit = u8> + Sync + Send + 'static,
//     SIGSCHEME::VerificationKey: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//     SIGSCHEME::SigningKey: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//     SIGSCHEME::Signature: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//     VRF: Vrf<
//             PublicParameter = (),
//             Input = Vec<u8>,
//             Output = Vec<u8>,
//             PublicKey = SIGSCHEME::VerificationKey,
//             SecretKey = SIGSCHEME::SigningKey,
//         > + Sync
//         + Send
//         + 'static,
//     VRF::Proof: Clone + Sync + Send + Serialize + for<'a> Deserialize<'a>,
//     VRF::PublicParameter: Sync + Send,
//     VRFHASHER: digest::Digest + Clone + Sync + Send + 'static,
//     VRFPARAMS: Sync + Send + Bls12Parameters,
//     <VRFPARAMS as Bls12Parameters>::G1Parameters: TEHashToGroup,
//     TYPES: NodeType<
//         VoteTokenType = VRFVoteToken<VRF::PublicKey, VRF::Proof>,
//         ElectionConfigType = VRFStakeTableConfig,
//         SignatureKey = JfPubKey<SIGSCHEME>,
//     >,
// {
//     // pubkey -> unit of stake
//     type StakeTable = VRFStakeTable<VRF, VRFHASHER, VRFPARAMS>;

//     // FIXED STAKE
//     // just return the state
//     fn get_stake_table(
//         &self,
//         _view_number: TYPES::Time,
//         _state: &TYPES::StateType,
//     ) -> Self::StakeTable {
//         self.stake_table.clone()
//     }

//     fn get_leader(&self, view_number: TYPES::Time) -> JfPubKey<SIGSCHEME> {
//         // TODO fst2 (ct) this is round robin, we should make this dependent on
//         // the VRF + some source of randomness

//         // TODO for now do by stake table of how much stake each
//         // participant has
//         let mapping = &self.stake_table.mapping;
//         let index = ((*view_number) as usize) % mapping.len();
//         let encoded = mapping.keys().nth(index).unwrap();
//         SignatureKey::from_bytes(encoded).unwrap()
//     }

//     // what this is doing:
//     // -
//     fn make_vote_token(
//         // TODO see if we can make this take &mut self
//         // because we're using a mutable prng
//         &self,
//         view_number: TYPES::Time,
//         private_key: &(SIGSCHEME::SigningKey, SIGSCHEME::VerificationKey),
//     ) -> Result<Option<TYPES::VoteTokenType>, ElectionError> {
//         let pub_key = JfPubKey::<SIGSCHEME>::from_native(private_key.1.clone());
//         let Some(replicas_stake) = self.stake_table.get_stake(&pub_key) else { return Ok(None) };

//         let view_seed = generate_view_seed::<TYPES, VRFHASHER>(view_number, &self.chain_seed);

//         let proof = Self::internal_get_vrf_proof(
//             &private_key.0,
//             &self.proof_parameters,
//             &mut self.prng.lock().unwrap(),
//             &view_seed,
//         )?;

//         let selected_stake = Self::internal_get_sortition_for_proof(
//             &self.proof_parameters,
//             &proof,
//             self.stake_table.get_all_stake(),
//             replicas_stake,
//             self.sortition_parameter,
//         );

//         match selected_stake {
//             Some(count) => {
//                 // TODO (ct) this can fail, return Result::Err
//                 let proof = VRF::prove(
//                     &self.proof_parameters,
//                     &private_key.0,
//                     &view_seed,
//                     &mut *self.prng.lock().unwrap(),
//                 )
//                 .unwrap();

//                 Ok(Some(VRFVoteToken {
//                     pub_key: private_key.1.clone(),
//                     proof,
//                     count,
//                 }))
//             }
//             None => Ok(None),
//         }
//     }

//     fn validate_vote_token(
//         &self,
//         view_number: TYPES::Time,
//         pub_key: JfPubKey<SIGSCHEME>,
//         token: Checked<TYPES::VoteTokenType>,
//     ) -> Result<Checked<TYPES::VoteTokenType>, ElectionError> {
//         match token {
//             Checked::Unchecked(token) => {
//                 let stake: Option<NonZeroU64> = self.stake_table.get_stake(&pub_key);
//                 let view_seed =
//                     generate_view_seed::<TYPES, VRFHASHER>(view_number, &self.chain_seed);
//                 if let Some(stake) = stake {
//                     Self::internal_check_sortition(
//                         &pub_key.pk,
//                         &self.proof_parameters,
//                         &token.proof,
//                         self.stake_table.get_all_stake(),
//                         stake,
//                         self.sortition_parameter,
//                         token.count,
//                         &view_seed,
//                     )
//                     .map(|c| match c {
//                         Checked::Inval(_) => Checked::Inval(token),
//                         Checked::Valid(_) => Checked::Valid(token),
//                         Checked::Unchecked(_) => Checked::Unchecked(token),
//                     })
//                 } else {
//                     // TODO better error
//                     Err(ElectionError::StubError)
//                 }
//             }
//             already_checked => Ok(already_checked),
//         }
//     }

//     fn create_election(keys: Vec<JfPubKey<SIGSCHEME>>, config: TYPES::ElectionConfigType) -> Self {
//         // This all needs to be refactored. For one thing, having the stake table - even an initial
//         // stake table - hardcoded like this is flat-out broken. This is, obviously, an artifact
//         let genesis_seed = [0u8; 32];
//         VrfImpl::with_initial_stake(keys, &config, genesis_seed)
//     }

//     fn default_election_config(num_nodes: u64) -> TYPES::ElectionConfigType {
//         let mut stake = Vec::new();
//         let units_of_stake_per_node = NonZeroU64::new(100).unwrap();
//         for _ in 0..num_nodes {
//             stake.push(units_of_stake_per_node);
//         }
//         VRFStakeTableConfig {
//             sortition_parameter: NonZeroU64::new(SORTITION_PARAMETER).unwrap(),
//             distribution: stake,
//         }
//     }

//     fn success_threshold(&self) -> NonZeroU64 {
//         NonZeroU64::new(((u64::from(self.sortition_parameter) * 2) / 3) + 1).unwrap()
//     }

//     fn failure_threshold(&self) -> NonZeroU64 {
//         NonZeroU64::new(((u64::from(self.sortition_parameter)) / 3) + 1).unwrap()
//     }
//     /// TODO if we ever come back to using this, we'll need to change this
//     /// this stub is incorrect as it stands right now
//     fn get_committee(
//         &self,
//         _view_number: <TYPES as NodeType>::Time,
//     ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
//         self.stake_table
//             .mapping
//             .keys()
//             .clone()
//             .filter_map(<TYPES as NodeType>::SignatureKey::from_bytes)
//             .collect()
//     }
// }

// /// checks that the expected aomunt of stake matches the VRF output
// /// TODO this can be optimized most likely
// fn check_bin_idx(
//     expected_amount_of_stake: u64,
//     replicas_stake: u64,
//     total_stake: u64,
//     sortition_parameter: u64,
//     unnormalized_seed: &[u8; 32],
//     cache: &mut HashMap<BinomialQuery, Ratio<BigUint>>,
// ) -> Option<bool> {
//     let bin_idx = find_bin_idx(
//         replicas_stake,
//         total_stake,
//         sortition_parameter,
//         unnormalized_seed,
//         cache,
//     );
//     bin_idx.map(|idx| idx == NonZeroU64::new(expected_amount_of_stake).unwrap())
// }

// /// generates the seed from algorand paper
// /// baseed on `view_number` and a constant as of now, but in the future will be other things
// /// this is a stop-gap
// fn generate_view_seed<TYPES: NodeType, HASHER: digest::Digest>(
//     view_number: TYPES::Time,
//     vrf_seed: &[u8; 32],
// ) -> [u8; 32] {
//     let mut hasher = HASHER::new();
//     hasher.update(vrf_seed);
//     hasher.update(view_number.deref().to_le_bytes());
//     let mut output = [0u8; 32];
//     output.copy_from_slice(hasher.finalize().as_ref());
//     output
// }

// /// represents a binomial query made by sortition
// /// `B(stake_attempt; replicas_stake; sortition_parameter / total_stake)`
// #[derive(Hash, Eq, PartialEq, Clone, Debug)]
// pub struct BinomialQuery {
//     /// the number of heads
//     stake_attempt: u64,
//     /// the total number of coin flips
//     replicas_stake: u64,
//     /// the total amount of stake
//     total_stake: u64,
//     /// the sortition parameter
//     sortition_parameter: u64,
// }

// impl BinomialQuery {
//     /// get the committee parameter
//     /// for this query
//     #[must_use]
//     pub fn get_p(&self) -> Ratio<BigUint> {
//         let sortition_parameter_big: BigUint = BigUint::from(self.sortition_parameter);
//         let total_stake_big: BigUint = BigUint::from(self.total_stake);
//         Ratio::new(sortition_parameter_big, total_stake_big)
//     }
// }

// #[instrument]
// fn calculate_threshold_from_cache(
//     previous_calculation: Option<(BinomialQuery, Ratio<BigUint>)>,
//     query: BinomialQuery,
// ) -> Option<Ratio<BigUint>> {
//     if let Some((previous_query, previous_result)) = previous_calculation {
//         let expected_previous_query = BinomialQuery {
//             stake_attempt: query.stake_attempt - 1,
//             ..query
//         };
//         if previous_query == expected_previous_query {
//             let permutation = Ratio::new(
//                 BigUint::from(query.replicas_stake - query.stake_attempt + 1),
//                 BigUint::from(query.stake_attempt),
//             );
//             let p = query.get_p();
//             assert!(p.numer() < p.denom());
//             let reciprocal = Ratio::recip(&(Ratio::from_integer(BigUint::from(1_u32)) - p.clone()));
//             let result = previous_result * p * reciprocal * permutation;
//             assert!(result.numer() < result.denom());

//             return Some(result);
//         }
//     }
//     calculate_threshold(query)
// }

// // Calculates B(j; w; p) where B means bernoulli distribution.
// // That is: run w trials, with p probability of success for each trial, and return the probability
// // of j successes.
// // p = tau / W, where tau is the sortition parameter (controlling committee size)
// // this is the only usage of W and tau
// //
// // Translation:
// // stake_attempt: our guess at what the stake might be. This is j
// // replicas_stake: the units of stake owned by the replica. This is w
// // total_stake: the units of stake owned in total. This is W
// // sorition_parameter: the parameter controlling the committee size. This is tau
// //
// // TODO (ct) better error handling
// // returns none if one of our calculations fails
// //
// // TODO keep data around from last iteration so less calculation is needed
// // TODO test this "correct/simple" implementation against any optimized version
// #[instrument]
// // fn calculate_threshold(stake_attempt: u32, replicas_stake: u64, total_stake: u64, sortition_parameter: u64) -> Option<Ratio<BigUint>> {
// fn calculate_threshold(query: BinomialQuery) -> Option<Ratio<BigUint>> {
//     let stake_attempt = query.stake_attempt;
//     tracing::info!("Running calculate threshold");
//     // TODO (ct) better error handling
//     if stake_attempt > query.replicas_stake {
//         error!("j is larger than amount of stake we are allowed");
//         return None;
//     }

//     let sortition_parameter_big: BigUint = BigUint::from(query.sortition_parameter);
//     let total_stake_big: BigUint = BigUint::from(query.total_stake);
//     let one_big = BigUint::from(1_u32);

//     // this is the p parameter for the bernoulli distribution
//     let p = Ratio::new(sortition_parameter_big, total_stake_big);

//     assert!(p.numer() <= p.denom());

//     info!("p is {p:?}");

//     // number of tails in bernoulli
//     let failed_num = query.replicas_stake - stake_attempt;

//     // TODO cancel things out (avoid calculating factorial)
//     // TODO can just do division
//     let num_permutations = Ratio::new(
//         factorial(query.replicas_stake),
//         factorial(stake_attempt) * factorial(failed_num),
//     );

//     info!("num permutations is {num_permutations:?}, failed_num is {failed_num:?}");

//     let one = Ratio::from_integer(one_big);

//     // TODO can keep results from last try
//     let result = num_permutations
//         * (p.pow(i32::try_from(stake_attempt).ok()?)
//             * (one - p).pow(i32::try_from(failed_num).ok()?));

//     assert!(result.numer() < result.denom());

//     info!("result is is {result:?}");

//     Some(result)
// }

// /// compute i! as a biguint
// fn factorial(mut i: u64) -> BigUint {
//     if i == 0 {
//         return BigUint::from(1u32);
//     }

//     let mut result = BigUint::from(1u32);
//     while i > 0 {
//         result *= i;
//         i -= 1;
//     }
//     result
// }

// /// find the amount of stake we rolled.
// /// NOTE: in the future this requires a view numb
// /// Returns None if zero stake was rolled
// #[instrument]
// fn find_bin_idx(
//     replicas_stake: u64,
//     total_stake: u64,
//     sortition_parameter: u64,
//     unnormalized_seed: &[u8; 32],
//     cache: &mut HashMap<BinomialQuery, Ratio<BigUint>>,
// ) -> Option<NonZeroU64> {
//     let unnormalized_seed = BigUint::from_bytes_le(unnormalized_seed);
//     let normalized_seed = Ratio::new(unnormalized_seed, BigUint::from(2_u32).pow(256));
//     assert!(normalized_seed.numer() < normalized_seed.denom());
//     let mut j: u64 = 0;

//     // [j, j+1)
//     // [cdf(j),cdf(j+1))

//     // left_threshold corresponds to the sum of all bernoulli distributions
//     // from i in 0 to j: B(i; replicas_stake; p). Where p is calculated later and corresponds to
//     // algorands paper
//     let mut left_threshold = Ratio::from_integer(BigUint::from(0u32));

//     loop {
//         // check cache

//         // if cache miss, feed in with previous val from cache
//         // that *probably* exists

//         assert!(left_threshold.numer() < left_threshold.denom());
//         let query = BinomialQuery {
//             stake_attempt: j + 1,
//             replicas_stake,
//             total_stake,
//             sortition_parameter,
//         };

//         let bin_val = {
//             // we already computed this value
//             if let Some(result) = cache.get(&query) {
//                 result.clone()
//             } else {
//                 // we haven't computed this value, but maybe
//                 // we already computed the previous value

//                 let mut maybe_old_query = query.clone();
//                 maybe_old_query.stake_attempt -= 1;
//                 let old_result = cache
//                     .get(&maybe_old_query)
//                     .map(|x| (maybe_old_query, x.clone()));
//                 let result = calculate_threshold_from_cache(old_result, query.clone())?;
//                 cache.insert(query, result.clone());
//                 result
//             }
//         };

//         // corresponds to right range from apper
//         let right_threshold = left_threshold + bin_val.clone();

//         // debugging info. Unnecessary
//         {
//             let right_threshold_float = ToPrimitive::to_f64(&right_threshold.clone());
//             let bin_val_float = ToPrimitive::to_f64(&bin_val.clone());
//             let normalized_seed_float = ToPrimitive::to_f64(&normalized_seed.clone());
//             info!("rightthreshold: {right_threshold_float:?}, bin: {bin_val_float:?}, seed: {normalized_seed_float:?}");
//         }

//         // from i in 0 to j + 1: B(i; replicas_stake; p)
//         if normalized_seed < right_threshold {
//             match j {
//                 0 => return None,
//                 _ => return Some(NonZeroU64::new(j).unwrap()),
//             }
//         }
//         left_threshold = right_threshold;
//         j += 1;
//     }
// }

// impl<TYPES, LEAF: LeafType<NodeType = TYPES>, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS>
//     VrfImpl<TYPES, LEAF, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS>
// where
//     SIGSCHEME: SignatureScheme<PublicParameter = (), MessageUnit = u8> + Sync + Send,
//     SIGSCHEME::VerificationKey: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//     SIGSCHEME::SigningKey: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//     SIGSCHEME::Signature: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
//     VRF: Vrf<
//             PublicParameter = (),
//             Input = [u8; 32],
//             Output = [u8; 32],
//             PublicKey = SIGSCHEME::VerificationKey,
//             SecretKey = SIGSCHEME::SigningKey,
//         > + Sync
//         + Send,
//     VRF::Proof: Clone + Sync + Send + Serialize + for<'a> Deserialize<'a>,
//     VRF::PublicParameter: Sync + Send,
//     VRFHASHER: digest::Digest + Clone + Sync + Send,
//     VRFPARAMS: Sync + Send + Bls12Parameters,
//     <VRFPARAMS as Bls12Parameters>::G1Parameters: TEHashToGroup,
//     TYPES: NodeType,
// {
//     /// create stake table with this initial stake
//     /// # Panics
//     /// TODO
//     #[must_use]
//     pub fn with_initial_stake(
//         known_nodes: Vec<JfPubKey<SIGSCHEME>>,
//         config: &VRFStakeTableConfig,
//         genesis_seed: [u8; 32],
//     ) -> Self {
//         assert_eq!(known_nodes.iter().len(), config.distribution.len());
//         let key_with_stake = known_nodes
//             .into_iter()
//             .map(|x| x.to_bytes())
//             .zip(config.distribution.clone())
//             .collect();
//         VrfImpl {
//             stake_table: {
//                 let st = VRFStakeTable {
//                     mapping: key_with_stake,
//                     total_stake: NonZeroU64::new(config.distribution.iter().map(|x| x.get()).sum())
//                         .unwrap(),
//                     _pd: PhantomData,
//                 };
//                 st
//             },
//             proof_parameters: (),
//             chain_seed: genesis_seed,
//             prng: Arc::new(Mutex::new(ChaChaRng::from_seed(Default::default()))),
//             _pd: PhantomData,
//             sortition_parameter: config.sortition_parameter,
//             _sortition_cache: Arc::default(),
//         }
//     }

//     /// stateless delegate for VRF proof generation
//     /// # Errors
//     ///

//     fn internal_get_vrf_proof(
//         private_key: &SIGSCHEME::SigningKey,
//         proof_param: &VRF::PublicParameter,
//         to_refactor: &mut rand_chacha::ChaChaRng,
//         vrf_in_seed: &VRF::Input,
//     ) -> Result<VRF::Proof, hotshot_types::traits::election::ElectionError> {
//         VRF::prove(proof_param, private_key, vrf_in_seed, to_refactor)
//             .map_err(|_| ElectionError::StubError)
//     }

//     /// stateless delegate for VRF sortition generation
//     fn internal_get_sortition_for_proof(
//         proof_param: &VRF::PublicParameter,
//         proof: &VRF::Proof,
//         total_stake: NonZeroU64,
//         voter_stake: NonZeroU64,
//         sortition_parameter: NonZeroU64,
//     ) -> Option<NonZeroU64> {
//         // TODO (ct) this can fail, return result::err
//         let hash = VRF::evaluate(proof_param, proof).unwrap();
//         let mut cache: HashMap<BinomialQuery, Ratio<BigUint>> = HashMap::new();

//         find_bin_idx(
//             u64::from(voter_stake),
//             u64::from(total_stake),
//             sortition_parameter.into(),
//             &hash,
//             &mut cache,
//         )
//     }

//     /// stateless delegate for VRF sortition confirmation
//     /// # Errors
//     /// if the proof is malformed
//     #[allow(clippy::too_many_arguments)]
//     fn internal_check_sortition(
//         public_key: &SIGSCHEME::VerificationKey,
//         proof_param: &VRF::PublicParameter,
//         proof: &VRF::Proof,
//         total_stake: NonZeroU64,
//         voter_stake: NonZeroU64,
//         sortition_parameter: NonZeroU64,
//         sortition_claim: NonZeroU64,
//         vrf_in_seed: &VRF::Input,
//     ) -> Result<Checked<()>, hotshot_types::traits::election::ElectionError> {
//         if let Ok(true) = VRF::verify(proof_param, proof, public_key, vrf_in_seed) {
//             let seed = VRF::evaluate(proof_param, proof).map_err(|_| ElectionError::StubError)?;
//             if let Some(res) = check_bin_idx(
//                 u64::from(sortition_claim),
//                 u64::from(voter_stake),
//                 u64::from(total_stake),
//                 u64::from(sortition_parameter),
//                 &seed,
//                 &mut HashMap::new(),
//             ) {
//                 if res {
//                     Ok(Checked::Valid(()))
//                 } else {
//                     Ok(Checked::Inval(()))
//                 }
//             } else {
//                 Ok(Checked::Unchecked(()))
//             }
//         } else {
//             Ok(Checked::Inval(()))
//         }
//     }

//     /// Stateless method to produce VRF proof and sortition for a given view number
//     /// # Errors
//     ///
//     pub fn get_sortition_proof(
//         private_key: &SIGSCHEME::SigningKey,
//         proof_param: &VRF::PublicParameter,
//         chain_seed: &VRF::Input,
//         view_number: TYPES::Time,
//         total_stake: NonZeroU64,
//         voter_stake: NonZeroU64,
//         sortition_parameter: NonZeroU64,
//     ) -> Result<(VRF::Proof, Option<NonZeroU64>), hotshot_types::traits::election::ElectionError>
//     {
//         let mut rng = ChaChaRng::from_seed(Default::default()); // maybe use something else that isn't deterministic?
//         let view_seed = generate_view_seed::<TYPES, VRFHASHER>(view_number, chain_seed);
//         let proof = Self::internal_get_vrf_proof(private_key, proof_param, &mut rng, &view_seed)?;
//         let sortition = Self::internal_get_sortition_for_proof(
//             proof_param,
//             &proof,
//             total_stake,
//             voter_stake,
//             sortition_parameter,
//         );
//         Ok((proof, sortition))
//     }

//     /// Stateless method to verify VRF proof and sortition for a given view number
//     /// # Errors
//     ///
//     #[allow(clippy::too_many_arguments)]
//     pub fn check_sortition_proof(
//         public_key: &JfPubKey<SIGSCHEME>,
//         proof_param: &VRF::PublicParameter,
//         proof: &VRF::Proof,
//         total_stake: NonZeroU64,
//         voter_stake: NonZeroU64,
//         sortition_parameter: NonZeroU64,
//         sortition_claim: NonZeroU64,
//         chain_seed: &VRF::Input,
//         view_number: TYPES::Time,
//     ) -> Result<bool, hotshot_types::traits::election::ElectionError> {
//         let view_seed = generate_view_seed::<TYPES, VRFHASHER>(view_number, chain_seed);
//         Self::internal_check_sortition(
//             &public_key.pk,
//             proof_param,
//             proof,
//             total_stake,
//             voter_stake,
//             sortition_parameter,
//             sortition_claim,
//             &view_seed,
//         )
//         .map(|c| matches!(c, Checked::Valid(_)))
//     }
// }

// impl<TYPES, LEAF: LeafType<NodeType = TYPES>> TestableElection<TYPES>
//     for VrfImpl<TYPES, LEAF, BLSSignatureScheme, BLSVRFScheme, Hasher, Param381>
// where
//     TYPES: NodeType<
//         VoteTokenType = VRFVoteToken<
//             BLSVerKey,
//             BLSSignature,
//         >,
//         ElectionConfigType = VRFStakeTableConfig,
//         SignatureKey = JfPubKey<BLSSignatureScheme>,
//     >,
// {
//     fn generate_test_vote_token() -> TYPES::VoteTokenType {
//         VRFVoteToken {
//             count: NonZeroU64::new(1234).unwrap(),
//             proof: BLSSignature::default(),
//             pub_key: BLSVerKey::default(),
//         }
//     }
// }

// /// configuration specifying the stake table
// #[derive(Clone, Serialize, Deserialize, core::fmt::Debug)]
// pub struct VRFStakeTableConfig {
//     /// the committee size parameter
//     pub sortition_parameter: NonZeroU64,
//     /// the ordered distribution of stake across nodes
//     pub distribution: Vec<NonZeroU64>,
// }

// impl Default for VRFStakeTableConfig {
//     fn default() -> Self {
//         VRFStakeTableConfig {
//             sortition_parameter: NonZeroU64::new(SORTITION_PARAMETER).unwrap(),
//             distribution: Vec::new(),
//         }
//     }
// }

// impl ElectionConfig for VRFStakeTableConfig {}

// Tests have been commented out, so `mod tests` isn't used.
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ark_bls12_381::Parameters as Param381;
//     use ark_std::test_rng;

//     use blake3::Hasher;
//     use hotshot_types::{
//         data::ViewNumber,
//         traits::{
//             block_contents::dummy::{DummyBlock, DummyTransaction},
//             consensus_type::validating_consensus::ValidatingConsensus,
//             state::dummy::DummyState,
//         },
//     };
//     use jf_primitives::{
//         signatures::{
//             bls::{BLSSignature, BLSVerKey},
//             BLSSignatureScheme,
//         },
//         vrf::blsvrf::BLSVRFScheme,
//     };
//     use std::{num::NonZeroUsize, time::Duration};

//     #[derive(
//         Copy,
//         Clone,
//         Debug,
//         Default,
//         Hash,
//         PartialEq,
//         Eq,
//         PartialOrd,
//         Ord,
//         serde::Serialize,
//         serde::Deserialize,
//     )]
//     struct TestTypes;
//     impl NodeType for TestTypes {
//         // TODO (da) can this be SequencingConsensus?
//         type ConsensusType = ValidatingConsensus;
//         type Time = ViewNumber;
//         type BlockType = DummyBlock;
//         type SignatureKey = JfPubKey<BLSSignatureScheme>;
//         type VoteTokenType = VRFVoteToken<
//             BLSVerKey<ark_bls12_381::Parameters>,
//             BLSSignature<ark_bls12_381::Parameters>,
//         >;
//         type Transaction = DummyTransaction;
//         type ElectionConfigType = VRFStakeTableConfig;
//         type StateType = DummyState;
//     }

//     fn gen_vrf_impl<LEAF: LeafType<NodeType = TestTypes>>(
//         num_nodes: usize,
//     ) -> (
//         VrfImpl<
//             TestTypes,
//             LEAF,
//             BLSSignatureScheme,
//             BLSVRFScheme<Param381>,
//             Hasher,
//             Param381,
//         >,
//         Vec<(
//             jf_primitives::signatures::bls::BLSSignKey<Param381>,
//             jf_primitives::signatures::bls::BLSVerKey<Param381>,
//         )>,
//     ) {
//         let mut known_nodes = Vec::new();
//         let mut keys = Vec::new();
//         let rng = &mut test_rng();
//         let mut stake_distribution = Vec::new();
//         let stake_per_node = NonZeroU64::new(100).unwrap();
//         let genesis_seed = [0u8; 32];
//         for _i in 0..num_nodes {
//             let (sk, pk) = BLSSignatureScheme::<Param381>::key_gen(&(), rng).unwrap();
//             keys.push((sk.clone(), pk.clone()));
//             known_nodes.push(JfPubKey::from_native(pk.clone()));
//             stake_distribution.push(stake_per_node);
//         }
//         let stake_table = VrfImpl::with_initial_stake(
//             known_nodes,
//             &VRFStakeTableConfig {
//                 sortition_parameter: std::num::NonZeroU64::new(SORTITION_PARAMETER).unwrap(),
//                 distribution: stake_distribution,
//             },
//             genesis_seed,
//         );
//         (stake_table, keys)
//     }

//     pub fn check_if_valid<T>(token: &Checked<T>) -> bool {
//         match token {
//             Checked::Valid(_) => true,
//             Checked::Inval(_) | Checked::Unchecked(_) => false,
//         }
//     }

//     // #[test]
//     // pub fn test_sortition() {
//     //     setup_logging();
//     //     let (vrf_impl, keys) = gen_vrf_impl::<ValidatingLeaf<TestTypes>>(10);
//     //     let views = 100;

//     //     for view in 0..views {
//     //         for (node_idx, (sk, pk)) in keys.iter().enumerate() {
//     //             let token_result = vrf_impl
//     //                 .make_vote_token(ViewNumber::new(view), &(sk.clone(), pk.clone()))
//     //                 .unwrap();
//     //             match token_result {
//     //                 Some(token) => {
//     //                     let count = token.count;
//     //                     let result = vrf_impl
//     //                         .validate_vote_token(
//     //                             ViewNumber::new(view),
//     //                             JfPubKey::from_native(pk.clone()),
//     //                             Checked::Unchecked(token),
//     //                         )
//     //                         .unwrap();
//     //                     let result_is_valid = check_if_valid(&result);
//     //                     error!("view {view:?}, node_idx {node_idx:?}, stake {count:?} ");
//     //                     assert!(result_is_valid);
//     //                 }
//     //                 _ => continue,
//     //             }
//     //         }
//     //     }
//     // }

//     #[test]
//     pub fn test_factorial() {
//         assert_eq!(factorial(0), BigUint::from(1u32));
//         assert_eq!(factorial(1), BigUint::from(1u32));
//         assert_eq!(factorial(2), BigUint::from(2u32));
//         assert_eq!(factorial(3), BigUint::from(6u32));
//         assert_eq!(factorial(4), BigUint::from(24u32));
//         assert_eq!(factorial(5), BigUint::from(120u32));
//     }

//     // TODO add failure case

//     #[test]
//     fn network_config_is_serializable() {
//         // validate that `RunResults` can be serialized
//         // Note that there is currently an issue with `VRFPubKey` where it can't be serialized with toml
//         // so instead we only test with serde_json
//         let key =
//             <JfPubKey<BLSSignatureScheme> as TestableSignatureKey>::generate_test_key(1);
//         let pub_key = JfPubKey::<BLSSignatureScheme>::from_private(&key);
//         let mut config = hotshot_centralized_server::NetworkConfig {
//             config: hotshot_types::HotShotConfig {
//                 election_config: Some(super::VRFStakeTableConfig {
//                     distribution: vec![NonZeroU64::new(1).unwrap()],
//                     sortition_parameter: NonZeroU64::new(1).unwrap(),
//                 }),
//                 known_nodes: vec![pub_key],
//                 execution_type: hotshot_types::ExecutionType::Incremental,
//                 total_nodes: NonZeroUsize::new(1).unwrap(),
//                 min_transactions: 1,
//                 max_transactions: NonZeroUsize::new(1).unwrap(),
//                 next_view_timeout: 1,
//                 timeout_ratio: (1, 1),
//                 round_start_delay: 1,
//                 start_delay: 1,
//                 num_bootstrap: 1,
//                 propose_min_round_time: Duration::from_secs(1),
//                 propose_max_round_time: Duration::from_secs(1),
//             },
//             ..Default::default()
//         };
//         serde_json::to_string(&config).unwrap();
//         assert!(toml::to_string(&config).is_err());

//         // validate that this is indeed a `pub_key` issue
//         config.config.known_nodes.clear();
//         serde_json::to_string(&config).unwrap();
//         toml::to_string(&config).unwrap();
//     }
// }
