use crate::{
    bindings::{
        erc20_permit_mock::ERC20PermitMock, hot_shot::HotShot, polygon_zk_evm::PolygonZkEVM,
        polygon_zk_evm_bridge::PolygonZkEVMBridge,
        polygon_zk_evm_global_exit_root::PolygonZkEVMGlobalExitRoot,
        polygon_zk_evm_global_exit_root_l2::PolygonZkEVMGlobalExitRootL2,
        polygon_zk_evm_timelock::PolygonZkEVMTimelock, verifier::Verifier,
        verifier_rollup_helper_mock::VerifierRollupHelperMock,
    },
    polygon_zk_evm::InitializePackedParameters,
};
use ethers::{
    abi::Tokenize,
    contract::Contract,
    prelude::{ContractFactory, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
    types::{Address, TransactionRequest, U256},
    utils::{get_contract_address, parse_ether},
};
use ethers_solc::HardhatArtifact;
use hex::FromHex;
use std::{fs, path::Path, sync::Arc, time::Duration};

type EthMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

#[async_trait::async_trait]
pub trait Deploy<M: Middleware> {
    async fn deploy_contract<T: Tokenize + Send>(client: &Arc<M>, args: T) -> Self;
}

/// Creates a deploy function for the contract.
///
/// If the contract is in a subdirectory of the "artifacts/contracts" directory,
/// the subdirectory relative to the "artifacts/contracts" directory must be
/// passed as first argument.
macro_rules! mk_deploy {
    ($prefix: expr, $contract:ident) => {
        #[async_trait::async_trait]
        impl<M: Middleware> Deploy<M> for $contract<M> {
            async fn deploy_contract<T: Tokenize + Send>(client: &Arc<M>, args: T) -> Self {
                // Ideally we would make our bindings generator script inline
                // the contract bytecode somewhere in this crate, then the
                // heuristic for finding the hardhat artifact below would no
                // longer be necessary.
                let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join(format!(
                        "{}/{}.sol/{}.json",
                        $prefix,
                        stringify!($contract),
                        stringify!($contract)
                    ));
                let file = fs::File::open(&path)
                    .unwrap_or_else(|_| panic!("Unable to open path {:?}", path));
                let artifact = serde_json::from_reader::<_, HardhatArtifact>(file).unwrap();
                let contract: $contract<M> = deploy_artifact(artifact, client, args).await.into();
                tracing::info!(
                    "Deployed {} at {:?}",
                    stringify!($contract),
                    contract.address()
                );
                contract
            }
        }
    };
}

// If other contracts need to be deployed, add them here.
const ZKEVM_CONTRACTS_PREFIX: &str = "zkevm-contracts/artifacts/contracts/";
mk_deploy!(ZKEVM_CONTRACTS_PREFIX, PolygonZkEVM);
mk_deploy!(ZKEVM_CONTRACTS_PREFIX, PolygonZkEVMBridge);
mk_deploy!(ZKEVM_CONTRACTS_PREFIX, PolygonZkEVMGlobalExitRootL2);
mk_deploy!(ZKEVM_CONTRACTS_PREFIX, PolygonZkEVMGlobalExitRoot);
mk_deploy!(ZKEVM_CONTRACTS_PREFIX, PolygonZkEVMTimelock);
mk_deploy!(ZKEVM_CONTRACTS_PREFIX.to_owned() + "/verifiers", Verifier);
mk_deploy!(
    ZKEVM_CONTRACTS_PREFIX.to_owned() + "/mocks",
    VerifierRollupHelperMock
);
mk_deploy!(
    ZKEVM_CONTRACTS_PREFIX.to_owned() + "/mocks",
    ERC20PermitMock
);

/// Deploy a contract from a hardhat artifact.
pub async fn deploy_artifact<M: Middleware, T: Tokenize>(
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

#[derive(Debug, Clone)]
pub struct TestClient {
    pub index: u32,
    pub provider: Arc<EthMiddleware>,
}

// We may want to use different names once we deploy a customized system without
// trusted parties.
#[derive(Debug, Clone)]
pub struct TestClients {
    // Account to use when deploying contracts.
    pub deployer: TestClient,
    // The block_driver client shouldn't be used for anything else to avoid nonce issues.
    pub block_driver: TestClient,
    // Various funded accounts that tests can use.
    pub funded: Vec<TestClient>,
}
impl TestClients {
    pub fn new(provider: &Provider<Http>, chain_id: u64) -> Self {
        Self {
            deployer: get_test_client(0, provider, chain_id),
            funded: vec![
                get_test_client(1, provider, chain_id),
                get_test_client(2, provider, chain_id),
                get_test_client(3, provider, chain_id),
            ],
            block_driver: get_test_client(4, provider, chain_id),
        }
    }
}

pub fn get_test_client(index: u32, provider: &Provider<Http>, chain_id: u64) -> TestClient {
    let mnemonic = MnemonicBuilder::<English>::default()
        .phrase("test test test test test test test test test test test junk");
    let provider = Arc::new(SignerMiddleware::new(
        provider.clone(),
        mnemonic
            .index(index)
            .unwrap()
            .build()
            .unwrap()
            .with_chain_id(chain_id),
    ));
    TestClient { provider, index }
}

/// A system of hermez smart contracts for testing purposes.
#[derive(Debug, Clone)]
pub struct TestHermezContracts {
    pub hotshot: HotShot<EthMiddleware>,
    pub rollup: PolygonZkEVM<EthMiddleware>,
    pub bridge: PolygonZkEVMBridge<EthMiddleware>,
    pub global_exit_root: PolygonZkEVMGlobalExitRoot<EthMiddleware>,
    pub verifier: VerifierRollupHelperMock<EthMiddleware>,
    pub matic: ERC20PermitMock<EthMiddleware>,
    pub gen_block_number: u64,
    pub clients: TestClients,
    pub provider: Provider<Http>,
}

impl TestHermezContracts {
    /// Connect to a system of deployed contracts for testing purposes.
    pub async fn connect(
        provider: impl AsRef<str>,
        hotshot_address: Address,
        rollup_address: Address,
        bridge_address: Address,
        global_exit_root_address: Address,
        verifier_address: Address,
        matic_address: Address,
    ) -> Self {
        let mut provider = Provider::try_from(provider.as_ref()).unwrap();
        provider.set_interval(Duration::from_millis(10));
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let clients = TestClients::new(&provider, chain_id);
        let deployer = clients.deployer.provider.clone();
        let hotshot = HotShot::new(hotshot_address, deployer.clone());
        let rollup = PolygonZkEVM::new(rollup_address, deployer.clone());
        let mut block_num = 0;

        // Iterate over all block numbers to figure out when the rollup contract
        // was deployed.
        let gen_block_number = loop {
            if let Ok(bytes) = provider
                .get_code(rollup_address, Some(block_num.into()))
                .await
            {
                if !bytes.is_empty() {
                    break block_num;
                }
            }

            block_num += 1;
        };

        Self {
            hotshot,
            rollup,
            bridge: PolygonZkEVMBridge::new(bridge_address, deployer.clone()),
            global_exit_root: PolygonZkEVMGlobalExitRoot::new(
                global_exit_root_address,
                deployer.clone(),
            ),
            verifier: VerifierRollupHelperMock::new(verifier_address, deployer.clone()),
            matic: ERC20PermitMock::new(matic_address, deployer.clone()),
            gen_block_number,
            clients,
            provider,
        }
    }

    /// Deploy the system of contracts for testing purposes.
    pub async fn deploy(provider: impl AsRef<str>, trusted_sequencer: impl AsRef<str>) -> Self {
        let mut provider = Provider::try_from(provider.as_ref()).unwrap();
        provider.set_interval(Duration::from_millis(10));

        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let clients = TestClients::new(&provider, chain_id);
        let deployer = clients.deployer.provider.clone();

        let hotshot = HotShot::deploy(deployer.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();

        let verifier = VerifierRollupHelperMock::deploy_contract(&deployer, ()).await;

        let matic_token_initial_balance = parse_ether("20000000").unwrap();
        let matic = ERC20PermitMock::deploy_contract(
            &deployer,
            (
                "Matic Token".to_string(),
                "MATIC".to_string(),
                deployer.address(),
                matic_token_initial_balance,
            ),
        )
        .await;

        // We need to pass the addresses to the GER constructor.
        let nonce = provider
            .get_transaction_count(deployer.address(), None)
            .await
            .unwrap();
        let precalc_bridge_address = get_contract_address(deployer.address(), nonce + 1);
        let precalc_rollup_address = get_contract_address(deployer.address(), nonce + 2);

        let global_exit_root = PolygonZkEVMGlobalExitRoot::deploy_contract(
            &deployer,
            (precalc_rollup_address, precalc_bridge_address),
        )
        .await;

        let bridge = PolygonZkEVMBridge::deploy_contract(&deployer, ()).await;
        assert_eq!(bridge.address(), precalc_bridge_address);

        let chain_id = U256::from(1001);
        let fork_id = U256::from(1);
        let rollup = PolygonZkEVM::deploy_contract(
            &deployer,
            (
                global_exit_root.address(),
                matic.address(),
                verifier.address(),
                bridge.address(),
                hotshot.address(),
                chain_id,
                fork_id,
            ),
        )
        .await;
        assert_eq!(rollup.address(), precalc_rollup_address);

        // Remember the genesis block number where the rollup contract was deployed.
        let gen_block_number = provider.get_block_number().await.unwrap().as_u64();

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

        // Genesis root from ./config/test.genesis.config.json in zkevm-node repo.
        //
        // Note that setting a wrong value currently does not seem to have any
        // noticeable effect, which is suspicious.
        let genesis_root = <[u8; 32]>::from_hex(
            "5c8df6a4b7748c1308a60c5380a2ff77deb5cfee3bf4fba76eef189d651d4558",
        )
        .unwrap();
        let network_name = "zkevm".to_string();
        let version = "0.0.1".to_string();

        // Note we currently use the deployer account for everything because the
        // zkevm-contracts geth L1 image still deploys like that.
        rollup
            .initialize(
                InitializePackedParameters {
                    admin: deployer.address(),
                    trusted_sequencer: deployer.address(),
                    pending_state_timeout: 10,
                    trusted_aggregator: deployer.address(),
                    trusted_aggregator_timeout: 10,
                },
                genesis_root,
                trusted_sequencer.as_ref().into(),
                network_name,
                version,
            )
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        // Approve rollup to move Matic in deployer account.
        matic
            .approve(rollup.address(), U256::MAX)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        Self {
            hotshot,
            rollup,
            bridge,
            global_exit_root,
            verifier,
            matic,
            gen_block_number,
            clients,
            provider,
        }
    }

    // The functions to create blocks below look a bit weird. This is due to
    // lifetime issues I encountered when spawning the periodic task. It's
    // surely possible to write this in a more concise way.

    /// A helper function to mine a block
    pub async fn mine_block(&self) {
        Self::do_mine_block(self.clients.block_driver.provider.clone()).await;
    }

    async fn do_mine_block(client: Arc<EthMiddleware>) {
        client
            .send_transaction(
                TransactionRequest::new()
                    .to(client.address())
                    .value(0)
                    .from(client.address()),
                None,
            )
            .await
            .unwrap();
    }

    /// A helper function to mine `n` blocks
    pub async fn mine_blocks(&self, n: u64) {
        for _ in 0..n {
            self.mine_block().await;
        }
    }

    /// A helper function to mine blocks periodically
    pub async fn mine_blocks_periodic(
        &self,
        interval: Duration,
    ) -> async_std::task::JoinHandle<()> {
        async_std::task::spawn(Self::do_mine_blocks_periodic(
            self.clients.block_driver.provider.clone(),
            interval,
        ))
    }

    // This function is here because async closures are unstable.
    async fn do_mine_blocks_periodic(client: Arc<EthMiddleware>, interval: Duration) {
        loop {
            Self::do_mine_block(client.clone()).await;
            async_std::task::sleep(interval).await
        }
    }
}
