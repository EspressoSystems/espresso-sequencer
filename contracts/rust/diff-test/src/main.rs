use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ed_on_bn254::{EdwardsConfig as EdOnBn254Config, Fq as FqEd254};
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_poly::domain::radix2::Radix2EvaluationDomain;
use ark_poly::EvaluationDomain;
use ark_std::rand::{rngs::StdRng, Rng, SeedableRng};
use clap::{Parser, ValueEnum};
use diff_test_bn254::ParsedG2Point;

use ethers::{
    abi::{AbiDecode, AbiEncode, Address},
    types::{Bytes, U256},
};
use hotshot_contract_adapter::{
    jellyfish::*,
    light_client::{ParsedLightClientState, ParsedStakeTableState},
};
use hotshot_state_prover::mock_ledger::{
    gen_plonk_proof_for_test, MockLedger, MockSystemParam, STAKE_TABLE_CAPACITY_FOR_TEST,
};
use hotshot_types::utils::epoch_from_block_number;
use jf_pcs::prelude::Commitment;
use jf_plonk::proof_system::structs::{Proof, VerifyingKey};
use jf_plonk::proof_system::PlonkKzgSnark;
use jf_plonk::{
    testing_apis::Verifier,
    transcript::{PlonkTranscript, SolidityTranscript},
};
use jf_signature::bls_over_bn254::{hash_to_curve, KeyPair as BLSKeyPair, Signature};
use jf_signature::constants::CS_ID_BLS_BN254;
use jf_signature::schnorr::KeyPair as SchnorrKeyPair;
use sha3::Keccak256;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Identifier for the functions to invoke in Jellyfish
    #[arg(value_enum)]
    action: Action,
    /// Optional arguments for the `action`
    #[arg(value_parser, num_args = 1.., value_delimiter = ' ')]
    args: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
    /// Get ark_poly::Radix2EvaluationDomain::new()
    NewPolyEvalDomain,
    /// Get ark_poly::Radix2EvaluationDomain::elements()
    EvalDomainElements,
    /// Get some poly evals during jf_plonk::prepare_pcs_info()
    EvalDataGen,
    /// Get jf_plonk::Transcript::append_message()
    TranscriptAppendMsg,
    /// Get jf_plonk::Transcript::append_challenge()
    TranscriptAppendField,
    /// Get jf_plonk::Transcript::append_commitment()
    TranscriptAppendGroup,
    /// Get jf_plonk::Transcript::get_and_append_challenge()
    TranscriptGetChal,
    /// Get jf_plonk::Transcript::append_vk_and_pub_input()
    TranscriptAppendVkAndPi,
    /// Get jf_plonk::Transcript::append_proof_evaluations()
    TranscriptAppendProofEvals,
    /// Return the Plonk Verifier related constants
    PlonkConstants,
    /// Get jf_plonk::Verifier::compute_challenges()
    PlonkComputeChal,
    /// Get jf_plonk::Verifier::batch_verify()
    PlonkVerify,
    /// Get a random, dummy proof with correct format
    DummyProof,
    /// Test only logic
    TestOnly,
    /// Generate Client Wallet
    GenClientWallet,
    /// Generate internal hash values for the BLS signature scheme
    GenBLSHashes,
    /// Generate BLS keys and a signature
    GenBLSSig,
    /// Generate some random point in G2
    GenRandomG2Point,
    /// Get mock genesis light client state
    MockGenesis,
    /// Get a consecutive finalized light client states
    MockConsecutiveFinalizedStates,
    /// Get a light client state that skipped a few blocks
    MockSkipBlocks,
    /// Compute the epoch number from block height
    EpochCompute,
}

#[allow(clippy::type_complexity)]
fn main() {
    let cli = Cli::parse();

    match cli.action {
        Action::NewPolyEvalDomain => {
            if cli.args.len() != 1 {
                panic!("Should provide arg1=logSize");
            }
            let log_size = cli.args[0].parse::<u32>().unwrap();

            let domain = Radix2EvaluationDomain::<Fr>::new(2u32.pow(log_size) as usize).unwrap();
            let res = (
                field_to_u256(domain.size_inv),
                field_to_u256(domain.group_gen),
            );
            println!("{}", res.encode_hex());
        }
        Action::EvalDomainElements => {
            if cli.args.len() != 2 {
                panic!("Should provide arg1=logSize, arg2=length");
            }
            let log_size = cli.args[0].parse::<u32>().unwrap();
            let length = cli.args[1].parse::<usize>().unwrap();

            let domain = Radix2EvaluationDomain::<Fr>::new(2u32.pow(log_size) as usize).unwrap();
            let res = domain
                .elements()
                .take(length)
                .map(field_to_u256)
                .collect::<Vec<_>>();
            println!("{}", res.encode_hex());
        }
        Action::EvalDataGen => {
            if cli.args.len() != 3 {
                panic!("Should provide arg1=logSize, arg2=zeta, arg3=publicInput");
            }

            let log_size = cli.args[0].parse::<u32>().unwrap();
            let zeta = u256_to_field::<Fr>(cli.args[1].parse::<U256>().unwrap());
            let pi_u256: [U256; 11] = AbiDecode::decode_hex(&cli.args[2]).unwrap();
            let pi: Vec<Fr> = pi_u256.into_iter().map(u256_to_field).collect();

            let verifier = Verifier::<Bn254>::new(2u32.pow(log_size) as usize).unwrap();
            let (vanish_eval, lagrange_one, _, pi_eval) = verifier
                .compute_poly_evals_for_pcs_info(&zeta, &pi)
                .unwrap();
            let res = (
                field_to_u256(vanish_eval),
                field_to_u256(lagrange_one),
                field_to_u256(pi_eval),
            );
            println!("{}", res.encode_hex());
        }
        Action::TranscriptAppendMsg => {
            if cli.args.len() != 2 {
                panic!("Should provide arg1=transcript, arg2=message");
            }
            let t_parsed = cli.args[0].parse::<ParsedTranscript>().unwrap();
            let msg = {
                let parsed: Bytes = AbiDecode::decode_hex(&cli.args[1]).unwrap();
                parsed.0.to_vec()
            };

            let mut t: SolidityTranscript = t_parsed.into();
            <SolidityTranscript as PlonkTranscript<Fr>>::append_message(&mut t, &[], &msg).unwrap();
            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TranscriptAppendField => {
            if cli.args.len() != 2 {
                panic!("Should provide arg1=transcript, arg2=fieldElement");
            }
            let t_parsed = cli.args[0].parse::<ParsedTranscript>().unwrap();
            let field = u256_to_field::<Fr>(cli.args[1].parse::<U256>().unwrap());

            let mut t: SolidityTranscript = t_parsed.into();
            t.append_field_elem::<Bn254>(&[], &field).unwrap();
            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TranscriptAppendGroup => {
            if cli.args.len() != 2 {
                panic!("Should provide arg1=transcript, arg2=groupElement");
            }

            let t_parsed = cli.args[0].parse::<ParsedTranscript>().unwrap();
            let point: G1Affine = cli.args[1].parse::<ParsedG1Point>().unwrap().into();

            let mut t: SolidityTranscript = t_parsed.into();
            t.append_commitment::<Bn254, ark_bn254::g1::Config>(&[], &Commitment::from(point))
                .unwrap();
            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TranscriptGetChal => {
            if cli.args.len() != 1 {
                panic!("Should provide arg1=transcript");
            }

            let t_parsed = cli.args[0].parse::<ParsedTranscript>().unwrap();

            let mut t: SolidityTranscript = t_parsed.into();
            let chal = t.get_challenge::<Bn254>(&[]).unwrap();

            let updated_t: ParsedTranscript = t.into();
            let res = (updated_t, field_to_u256(chal));
            println!("{}", res.encode_hex());
        }
        Action::TranscriptAppendVkAndPi => {
            if cli.args.len() != 3 {
                panic!("Should provide arg1=transcript, arg2=verifyingKey, arg3=publicInput");
            }

            let t_parsed = cli.args[0].parse::<ParsedTranscript>().unwrap();
            let vk_parsed = cli.args[1].parse::<ParsedVerifyingKey>().unwrap();
            let pi_u256: Vec<U256> = AbiDecode::decode_hex(&cli.args[2]).unwrap();
            let pi: Vec<Fr> = pi_u256.into_iter().map(u256_to_field).collect();

            let mut t: SolidityTranscript = t_parsed.into();
            let vk: VerifyingKey<Bn254> = vk_parsed.into();
            t.append_vk_and_pub_input(&vk, &pi).unwrap();

            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TranscriptAppendProofEvals => {
            if cli.args.len() != 1 {
                panic!("Should provide arg1=transcript");
            }

            let mut rng = jf_utils::test_rng();

            let t_parsed = cli.args[0].parse::<ParsedTranscript>().unwrap();
            let proof_parsed = ParsedPlonkProof::dummy_with_rand_proof_evals(&mut rng);
            let proof: Proof<Bn254> = proof_parsed.clone().into();

            let mut t: SolidityTranscript = t_parsed.into();
            <SolidityTranscript as PlonkTranscript<Fq>>::append_proof_evaluations::<Bn254>(
                &mut t,
                &proof.poly_evals,
            )
            .unwrap();

            let t_updated: ParsedTranscript = t.into();
            let res = (t_updated, proof_parsed);
            println!("{}", res.encode_hex());
        }
        Action::PlonkConstants => {
            let coset_k = coset_k();
            let open_key = open_key();

            let res = (
                field_to_u256::<Fr>(coset_k[1]),
                field_to_u256::<Fr>(coset_k[2]),
                field_to_u256::<Fr>(coset_k[3]),
                field_to_u256::<Fr>(coset_k[4]),
                // NOTE: be EXTRA careful here!! Solidity's BN254.G2Point: Fp2 = x0 * u + x1
                // whereas in rust: Fp2 = x0 + x1 * u
                field_to_u256::<Fq>(open_key.beta_h.x().unwrap().c1),
                field_to_u256::<Fq>(open_key.beta_h.x().unwrap().c0),
                field_to_u256::<Fq>(open_key.beta_h.y().unwrap().c1),
                field_to_u256::<Fq>(open_key.beta_h.y().unwrap().c0),
            );
            println!("{}", res.encode_hex());
        }
        Action::PlonkComputeChal => {
            if cli.args.len() != 4 {
                panic!("Should provide arg1=verifyingKey, arg2=publicInput, arg3=proof, arg4=extraTranscriptInitMsg");
            }

            let vk = cli.args[0].parse::<ParsedVerifyingKey>().unwrap().into();
            let pi_u256: [U256; 11] = AbiDecode::decode_hex(&cli.args[1]).unwrap();
            let pi: Vec<Fr> = pi_u256.into_iter().map(u256_to_field).collect();
            let proof: Proof<Bn254> = cli.args[2].parse::<ParsedPlonkProof>().unwrap().into();
            let msg = {
                let parsed: Bytes = AbiDecode::decode_hex(&cli.args[3]).unwrap();
                parsed.0.to_vec()
            };

            let chal: ParsedChallenges =
                Verifier::<Bn254>::compute_challenges::<SolidityTranscript>(
                    &[&vk],
                    &[&pi],
                    &proof.into(),
                    &Some(msg),
                )
                .unwrap()
                .into();
            println!("{}", (chal,).encode_hex());
        }
        Action::PlonkVerify => {
            let (proof, vk, public_input, _, _): (
                Proof<Bn254>,
                VerifyingKey<Bn254>,
                Vec<Fr>,
                Option<Vec<u8>>, // won't use extraTranscriptMsg
                usize,           // won't use circuit size
            ) = gen_plonk_proof_for_test(1)[0].clone();

            // ensure they are correct params
            assert!(PlonkKzgSnark::batch_verify::<SolidityTranscript>(
                &[&vk],
                &[&public_input],
                &[&proof],
                &[None]
            )
            .is_ok());

            let vk_parsed: ParsedVerifyingKey = vk.into();
            let mut pi_parsed = [U256::default(); 11];
            assert_eq!(public_input.len(), 11);
            for (i, pi) in public_input.into_iter().enumerate() {
                pi_parsed[i] = field_to_u256(pi);
            }
            let proof_parsed: ParsedPlonkProof = proof.into();

            let res = (vk_parsed, pi_parsed, proof_parsed);
            println!("{}", res.encode_hex());
        }
        Action::DummyProof => {
            let mut rng = jf_utils::test_rng();
            if !cli.args.is_empty() {
                let seed = cli.args[0].parse::<u64>().unwrap();
                rng = StdRng::seed_from_u64(seed);
            }
            let proof = ParsedPlonkProof::dummy(&mut rng);
            println!("{}", (proof,).encode_hex());
        }
        Action::TestOnly => {
            println!("args: {:?}", cli.args);
        }
        Action::GenClientWallet => {
            if cli.args.len() != 2 {
                panic!("Should provide arg1=senderAddress arg2=seed");
            }

            // Use seed from cli to generate different bls keys
            let seed_value: u8 = cli.args[1].parse::<u8>().unwrap();
            let seed = [seed_value; 32];
            let mut rng = StdRng::from_seed(seed);

            let sender_address = cli.args[0].parse::<Address>().unwrap();
            let sender_address_bytes = AbiEncode::encode(sender_address);

            // Generate the Schnorr key
            let schnorr_key_pair: SchnorrKeyPair<EdOnBn254Config> =
                SchnorrKeyPair::generate(&mut rng);
            let schnorr_ver_key = schnorr_key_pair.ver_key();
            let schnorr_ver_key_affine = schnorr_ver_key.to_affine();
            let schnorr_pk_x = field_to_u256::<FqEd254>(schnorr_ver_key_affine.x);
            let schnorr_pk_y = field_to_u256::<FqEd254>(schnorr_ver_key_affine.y);

            // Generate the BLS ver key
            let key_pair = BLSKeyPair::generate(&mut rng);
            let vk = key_pair.ver_key();
            let vk_g2_affine: G2Affine = vk.to_affine();

            let vk_parsed: ParsedG2Point = vk_g2_affine.into();

            // Sign the ethereum address with the BLS key
            let sig: Signature = key_pair.sign(&sender_address_bytes, CS_ID_BLS_BN254);
            let sig_affine_point = sig.sigma.into_affine();
            let sig_parsed: ParsedG1Point = sig_affine_point.into();

            let res = (
                sig_parsed,
                vk_parsed,
                schnorr_pk_x,
                schnorr_pk_y,
                sender_address,
            );
            println!("{}", res.encode_hex());
        }
        Action::GenRandomG2Point => {
            if cli.args.len() != 1 {
                panic!("Should provide arg1=exponent");
            }

            let exponent: u64 = cli.args[0].parse::<u64>().unwrap();
            let mut point = G2Affine::generator();
            point = (point * Fr::from(exponent)).into();
            let point_parsed: ParsedG2Point = point.into();
            let res = point_parsed;
            println!("{}", (res.encode_hex()));
        }
        Action::MockGenesis => {
            if cli.args.len() != 1 {
                panic!("Should provide arg1=numInitValidators");
            }
            let num_init_validators = cli.args[0].parse::<u64>().unwrap();

            let pp = MockSystemParam::init();
            let ledger = MockLedger::init(pp, num_init_validators as usize);

            let res: (ParsedLightClientState, ParsedStakeTableState) = (
                ledger.light_client_state().into(),
                ledger.voting_stake_table_state().into(),
            );
            println!("{}", res.encode_hex());
        }
        Action::MockConsecutiveFinalizedStates => {
            if cli.args.len() != 1 {
                panic!("Should provide arg1=numInitValidators");
            }
            let num_init_validators = cli.args[0].parse::<u64>().unwrap();

            let pp = MockSystemParam::init();
            let mut ledger = MockLedger::init(pp, num_init_validators as usize);

            let mut new_states: Vec<ParsedLightClientState> = vec![];
            let mut proofs: Vec<ParsedPlonkProof> = vec![];
            let mut next_st_states: Vec<ParsedStakeTableState> = vec![];

            for _ in 1..4 {
                // random number of notarized but not finalized block
                if ledger.rng.gen_bool(0.5) {
                    let num_non_blk = ledger.rng.gen_range(0..5);
                    for _ in 0..num_non_blk {
                        ledger.elapse_without_block();
                    }
                }

                ledger.elapse_with_block();

                let (pi, proof) = ledger.gen_state_proof();
                next_st_states.push(pi.next_st_state.into());
                new_states.push(pi.lc_state.into());
                proofs.push(proof.into());
            }

            let res = (new_states, next_st_states, proofs);
            println!("{}", res.encode_hex());
        }
        Action::MockSkipBlocks => {
            if cli.args.is_empty() || cli.args.len() > 2 {
                panic!("Should provide arg1=numBlockSkipped,arg2(opt)=requireValidProof");
            }

            let num_block_skipped = cli.args[0].parse::<u32>().unwrap();
            let require_valid_proof: bool = if cli.args.len() == 2 {
                cli.args[1].parse::<bool>().unwrap()
            } else {
                true
            };

            let pp = MockSystemParam::init();
            let mut ledger = MockLedger::init(pp, STAKE_TABLE_CAPACITY_FOR_TEST / 2);

            for _ in 0..num_block_skipped {
                ledger.elapse_with_block();
            }

            let res = if require_valid_proof {
                let (pi, proof) = ledger.gen_state_proof();
                let state_parsed: ParsedLightClientState = pi.lc_state.into();
                let proof_parsed: ParsedPlonkProof = proof.into();
                let next_stake_table: ParsedStakeTableState = pi.next_st_state.into();
                (state_parsed, next_stake_table, proof_parsed)
            } else {
                let state_parsed = ledger.light_client_state().into();
                let proof_parsed = ParsedPlonkProof::dummy(&mut ledger.rng);
                let next_stake_table: ParsedStakeTableState =
                    ledger.next_stake_table_state().into();
                (state_parsed, next_stake_table, proof_parsed)
            };
            println!("{}", res.encode_hex());
        }
        Action::GenBLSHashes => {
            if cli.args.len() != 1 {
                panic!("Should provide arg1=message");
            }

            // Same as in the hash_to_curve function
            // See https://github.com/EspressoSystems/jellyfish/blob/6c2c08f4e966fd1d454d48bcf30bd41a952f9f76/primitives/src/signatures/bls_over_bn254.rs#L310
            let hasher_init = &[1u8];
            let hasher = <DefaultFieldHasher<Keccak256> as HashToField<Fq>>::new(hasher_init);

            let message_bytes = cli.args[0].parse::<Bytes>().unwrap();

            let field_elem: Fq = hasher.hash_to_field(&message_bytes, 1)[0];
            let fq_u256 = field_to_u256::<Fq>(field_elem);
            let hash_to_curve_elem: G1Affine = hash_to_curve::<Keccak256>(&message_bytes).into();
            let hash_to_curve_elem_parsed: ParsedG1Point = hash_to_curve_elem.into();

            let res = (fq_u256, hash_to_curve_elem_parsed);
            println!("{}", res.encode_hex());
        }
        Action::GenBLSSig => {
            let mut rng = jf_utils::test_rng();

            if cli.args.len() != 1 {
                panic!("Should provide arg1=message");
            }
            let message_bytes = cli.args[0].parse::<Bytes>().unwrap();

            // Generate the BLS ver key
            let key_pair = BLSKeyPair::generate(&mut rng);
            let vk = key_pair.ver_key();
            let vk_g2_affine: G2Affine = vk.to_affine();
            let vk_parsed: ParsedG2Point = vk_g2_affine.into();

            // Sign the message
            let sig: Signature = key_pair.sign(&message_bytes, CS_ID_BLS_BN254);
            let sig_affine_point = sig.sigma.into_affine();
            let sig_parsed: ParsedG1Point = sig_affine_point.into();

            let res = (vk_parsed, sig_parsed);
            println!("{}", res.encode_hex());
        }
        Action::EpochCompute => {
            if cli.args.len() != 2 {
                panic!("Should provide arg1=blockNum, arg2=blocksPerEpoch");
            }
            let block_num = cli.args[0].parse::<u64>().unwrap();
            let epoch_height = cli.args[1].parse::<u64>().unwrap();

            let res = epoch_from_block_number(block_num, epoch_height);
            println!("{}", (res,).encode_hex());
        }
    };
}
