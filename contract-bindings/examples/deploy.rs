use contract_bindings::bindings::{
    erc20_permit_mock::ERC20PermitMock,
    polygon_zk_evm::PolygonZkEVM,
    polygon_zk_evm_bridge::PolygonZkEVMBridge,
    polygon_zk_evm_global_exit_root::PolygonZkEVMGlobalExitRoot,
    shared_types::{BatchData, InitializePackedParameters},
    verifier_rollup_helper_mock::VerifierRollupHelperMock,
};
use ethers::{
    abi::Tokenize,
    contract::Contract,
    prelude::{ContractFactory, SignerMiddleware},
    providers::{Middleware, Provider},
    signers::{coins_bip39::English, MnemonicBuilder, Signer},
    types::BlockNumber,
    utils::parse_ether,
};
use ethers_solc::HardhatArtifact;
use hex::FromHex;
use std::{fs, ops::Mul, path::Path, sync::Arc, time::Duration};

async fn deploy_artifact<M: Middleware, T: Tokenize>(
    artifact: HardhatArtifact,
    client: &Arc<M>,
    args: T,
) -> Contract<M> {
    let factory = ContractFactory::new(
        artifact.abi.into(),
        artifact.bytecode.unwrap().into_bytes().unwrap(),
        client.clone(),
    );
    factory.deploy(args).unwrap().send().await.unwrap()
}

/// A convenience function to deploy a contract given it's Name.
async fn deploy_by_name<M: Middleware, T: Tokenize>(
    name: &str,
    client: &Arc<M>,
    args: T,
) -> Contract<M> {
    let matches: Vec<_> = glob::glob(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(format!(
                "zkevm-contracts/artifacts/contracts/**/{name}.json",
            ))
            .to_str()
            .unwrap(),
    )
    .unwrap()
    .map(|entry| entry.unwrap())
    .collect();

    let path = if matches.len() == 1 {
        matches[0].clone()
    } else {
        panic!("Not one exact match for contract {name}: {matches:?}");
    };

    let file = fs::File::open(&path).unwrap_or_else(|_| panic!("Unable to open path {:?}", path));
    let artifact = serde_json::from_reader::<_, HardhatArtifact>(file).unwrap();
    deploy_artifact(artifact, client, args).await
}

#[async_std::main]
async fn main() {
    let mut args = std::env::args();
    args.next().unwrap(); // skip program name
    tracing_subscriber::fmt::init();

    let mut provider = Provider::try_from("http://localhost:8545").unwrap();
    provider.set_interval(Duration::from_millis(10));

    // Deploy contracts like in zkevm-contracts/test/contracts/proofOfEfficiency.js, but wihout proxies.

    let mnemonic = MnemonicBuilder::<English>::default()
        .phrase("test test test test test test test test test test test junk");

    let deployer = mnemonic
        .clone()
        .build()
        .unwrap()
        .with_chain_id(provider.get_chainid().await.unwrap().as_u64());
    let trusted_aggregator = mnemonic
        .clone()
        .index(1u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(provider.get_chainid().await.unwrap().as_u64());
    let trusted_sequencer = mnemonic
        .clone()
        .index(2u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(provider.get_chainid().await.unwrap().as_u64());
    let admin = mnemonic
        .index(3u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(provider.get_chainid().await.unwrap().as_u64());

    let client = Arc::new(SignerMiddleware::new(provider.clone(), deployer.clone()));
    let trusted_sequencer_client =
        Arc::new(SignerMiddleware::new(provider.clone(), trusted_sequencer));

    let verifier: VerifierRollupHelperMock<_> =
        deploy_by_name("VerifierRollupHelperMock", &client, ())
            .await
            .into();

    let matic_token_initial_balance = parse_ether("20000000").unwrap();
    let matic: ERC20PermitMock<_> = deploy_by_name(
        "ERC20PermitMock",
        &client,
        (
            "Matic Token".to_string(),
            "MATIC".to_string(),
            client.address(),
            matic_token_initial_balance,
        ),
    )
    .await
    .into();

    let global_exit_root: PolygonZkEVMGlobalExitRoot<_> =
        deploy_by_name("PolygonZkEVMGlobalExitRoot", &client, ())
            .await
            .into();
    let bridge: PolygonZkEVMBridge<_> = deploy_by_name("PolygonZkEVMBridge", &client, ())
        .await
        .into();

    let rollup: PolygonZkEVM<_> = deploy_by_name("PolygonZkEVM", &client, ()).await.into();

    global_exit_root
        .initialize(rollup.address(), bridge.address())
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    let network_id_mainnet = 0;
    bridge
        .initialize(
            network_id_mainnet,
            global_exit_root.address(),
            rollup.address(),
        )
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    let genesis_root =
        <[u8; 32]>::from_hex("0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
    let trusted_sequencer_url = "http://zkevm-json-rpc:8123";
    let network_name = "zkevm";
    rollup
        .initialize(
            global_exit_root.address(),
            matic.address(),
            verifier.address(),
            bridge.address(),
            InitializePackedParameters {
                admin: admin.address(),
                force_batch_allowed: true,
                chain_id: 1000,
                trusted_sequencer: trusted_sequencer_client.address(),
                pending_state_timeout: 10,
                trusted_aggregator: trusted_aggregator.address(),
                trusted_aggregator_timeout: 10,
            },
            genesis_root,
            trusted_sequencer_url.to_string(),
            network_name.to_string(),
        )
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    // Fund sequencer address with Matic tokens.
    matic
        .transfer(
            trusted_sequencer_client.address(),
            parse_ether("100").unwrap(),
        )
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    // Try to sequence a batch
    let l2_tx_data = hex::decode("1234").unwrap();
    let matic_amount = rollup.get_current_batch_fee().await.unwrap().mul(2u64);
    let current_timestamp = provider
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .unwrap()
        .timestamp;
    let batch = BatchData {
        transactions: l2_tx_data.into(),
        global_exit_root: [0u8; 32],
        timestamp: current_timestamp.as_u64(),
        min_forced_timestamp: 0u64,
    };

    // Approve Matic
    let matic_trusted: ERC20PermitMock<_> = matic.connect(trusted_sequencer_client.clone()).into();
    matic_trusted
        .approve(rollup.address(), matic_amount)
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    let rollup_trusted: PolygonZkEVM<_> = rollup.connect(trusted_sequencer_client).into();
    let receipt = rollup_trusted
        .sequence_batches(vec![batch])
        .send()
        .await
        .unwrap()
        .await
        .unwrap();
    assert_eq!(receipt.unwrap().status, Some(1u64.into()));
}
