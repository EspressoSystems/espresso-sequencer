use crate::{
    bindings::{
        erc20_permit_mock::ERC20PermitMock, polygon_zk_evm::PolygonZkEVM,
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
    types::{TransactionRequest, U256},
    utils::parse_ether,
};
use ethers_solc::HardhatArtifact;
use hex::FromHex;
use std::{fs, path::Path, sync::Arc, time::Duration};

type EthMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

#[async_trait::async_trait]
pub trait Deploy<M: Middleware> {
    async fn deploy<T: Tokenize + Send>(client: &Arc<M>, args: T) -> Self;
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
            async fn deploy<T: Tokenize + Send>(client: &Arc<M>, args: T) -> Self {
                // Ideally we would make our bindings generator script inline
                // the contract bytecode somewhere in this crate, then the
                // heuristic for finding the hardhat artifact below would no
                // longer be necessary.
                let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join(format!(
                        "zkevm-contracts/artifacts/contracts/{}/{}.sol/{}.json",
                        $prefix,
                        stringify!($contract),
                        stringify!($contract)
                    ));
                let file = fs::File::open(&path)
                    .unwrap_or_else(|_| panic!("Unable to open path {:?}", path));
                let artifact = serde_json::from_reader::<_, HardhatArtifact>(file).unwrap();
                let contract: $contract<M> = deploy_artifact(artifact, client, args).await.into();
                tracing::info!(
                    "Deployed {} at {}",
                    stringify!($contract),
                    contract.address()
                );
                contract
            }
        }
    };
}

// If other contracts need to be deployed, add them here.
mk_deploy!("", PolygonZkEVM);
mk_deploy!("", PolygonZkEVMBridge);
mk_deploy!("", PolygonZkEVMGlobalExitRootL2);
mk_deploy!("", PolygonZkEVMGlobalExitRoot);
mk_deploy!("", PolygonZkEVMTimelock);
mk_deploy!("verifiers", Verifier);
mk_deploy!("mocks", VerifierRollupHelperMock);
mk_deploy!("mocks", ERC20PermitMock);

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

// We may want to use different names once we deploy a customized system without
// trusted parties.
#[derive(Debug, Clone)]
pub struct TestClients {
    pub deployer: Arc<EthMiddleware>,
    pub trusted_aggregator: Arc<EthMiddleware>,
    pub trusted_sequencer: Arc<EthMiddleware>,
    pub admin: Arc<EthMiddleware>,
    // The block_driver client shouldn't be used for anything else to avoid nonce issues.
    pub block_driver: Arc<EthMiddleware>,
}
impl TestClients {
    pub fn new(provider: &Provider<Http>, chain_id: u64) -> Self {
        Self {
            deployer: get_test_client(0, provider, chain_id),
            trusted_aggregator: get_test_client(1, provider, chain_id),
            trusted_sequencer: get_test_client(2, provider, chain_id),
            admin: get_test_client(3, provider, chain_id),
            block_driver: get_test_client(4, provider, chain_id),
        }
    }
}

fn get_test_client(index: u32, provider: &Provider<Http>, chain_id: u64) -> Arc<EthMiddleware> {
    let mnemonic = MnemonicBuilder::<English>::default()
        .phrase("test test test test test test test test test test test junk");
    Arc::new(SignerMiddleware::new(
        provider.clone(),
        mnemonic
            .index(index)
            .unwrap()
            .build()
            .unwrap()
            .with_chain_id(chain_id),
    ))
}

/// A system of hermez smart contracts for testing purposes.
#[derive(Debug, Clone)]
pub struct TestHermezContracts {
    pub rollup: PolygonZkEVM<EthMiddleware>,
    pub bridge: PolygonZkEVMBridge<EthMiddleware>,
    pub global_exit_root: PolygonZkEVMGlobalExitRoot<EthMiddleware>,
    pub verifier: VerifierRollupHelperMock<EthMiddleware>,
    pub matic: ERC20PermitMock<EthMiddleware>,
    pub clients: TestClients,
    pub provider: Provider<Http>,
}

impl TestHermezContracts {
    /// Deploy the system of contracts for testing purposes.
    pub async fn deploy(provider: impl AsRef<str>, trusted_sequencer: impl AsRef<str>) -> Self {
        let mut provider = Provider::try_from(provider.as_ref()).unwrap();
        provider.set_interval(Duration::from_millis(10));

        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let clients = TestClients::new(&provider, chain_id);

        let verifier = VerifierRollupHelperMock::deploy(&clients.deployer, ()).await;

        let matic_token_initial_balance = parse_ether("20000000").unwrap();
        let matic = ERC20PermitMock::deploy(
            &clients.deployer,
            (
                "Matic Token".to_string(),
                "MATIC".to_string(),
                clients.deployer.address(),
                matic_token_initial_balance,
            ),
        )
        .await;

        let global_exit_root = PolygonZkEVMGlobalExitRoot::deploy(&clients.deployer, ()).await;
        let bridge = PolygonZkEVMBridge::deploy(&clients.deployer, ()).await;
        let rollup = PolygonZkEVM::deploy(&clients.deployer, ()).await;

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

        // Genesis root from ./config/test.genesis.config.json in zkevm-node repo.
        let genesis_root = <[u8; 32]>::from_hex(
            "bf34f9a52a63229e90d1016011655bc12140bba5b771817b88cbf340d08dcbde",
        )
        .unwrap();
        let network_name = "zkevm";

        // Note that the test zkevm-node expects all wallets to be the deployer
        // wallet (account 0), except for the aggregator wallet (account 1).
        rollup
            .initialize(
                global_exit_root.address(),
                matic.address(),
                verifier.address(),
                bridge.address(),
                InitializePackedParameters {
                    // admin: clients.admin.address(),
                    admin: clients.deployer.address(),
                    force_batch_allowed: true,
                    chain_id: 1001,
                    // trusted_sequencer: clients.trusted_sequencer.address(),
                    trusted_sequencer: clients.deployer.address(),
                    pending_state_timeout: 10,
                    trusted_aggregator: clients.trusted_aggregator.address(),
                    // trusted_aggregator: clients.deployer.address(),
                    trusted_aggregator_timeout: 10,
                },
                genesis_root,
                trusted_sequencer.as_ref().into(),
                network_name.to_string(),
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
            rollup,
            bridge,
            global_exit_root,
            verifier,
            matic,
            clients,
            provider,
        }
    }

    // The functions to create blocks below look a bit weird. This is due to
    // lifetime issues I encountered when spawning the periodic task. It's
    // surely possible to write this in a more concise way.

    /// A helper function to mine a block
    pub async fn mine_block(&self) {
        Self::do_mine_block(self.clients.block_driver.clone()).await;
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
            self.clients.block_driver.clone(),
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
