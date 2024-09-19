use anyhow::{ensure, Context};
use async_std::sync::Arc;
use clap::{builder::OsStr, Parser, ValueEnum};
use contract_bindings::{
    erc1967_proxy::ERC1967Proxy,
    fee_contract::FeeContract,
    hot_shot::HotShot,
    light_client::{LightClient, LIGHTCLIENT_ABI},
    light_client_mock::LIGHTCLIENTMOCK_ABI,
    light_client_state_update_vk::LightClientStateUpdateVK,
    light_client_state_update_vk_mock::LightClientStateUpdateVKMock,
    plonk_verifier_2::PlonkVerifier2,
};
use derive_more::Display;
use ethers::{
    prelude::*, signers::coins_bip39::English, solc::artifacts::BytecodeObject, utils::hex,
};
use futures::future::{BoxFuture, FutureExt};
use hotshot_contract_adapter::light_client::{
    LightClientConstructorArgs, ParsedLightClientState, ParsedStakeTableState,
};
use std::{collections::HashMap, io::Write, ops::Deref};
use url::Url;

/// Set of predeployed contracts.
#[derive(Clone, Debug, Parser)]
pub struct DeployedContracts {
    /// Use an already-deployed HotShot.sol instead of deploying a new one.
    #[clap(long, env = Contract::HotShot)]
    hotshot: Option<Address>,

    /// Use an already-deployed PlonkVerifier.sol instead of deploying a new one.
    #[clap(long, env = Contract::PlonkVerifier)]
    plonk_verifier: Option<Address>,

    /// Use an already-deployed LightClientStateUpdateVK.sol instead of deploying a new one.
    #[clap(long, env = Contract::StateUpdateVK)]
    light_client_state_update_vk: Option<Address>,

    /// Use an already-deployed LightClient.sol instead of deploying a new one.
    #[clap(long, env = Contract::LightClient)]
    light_client: Option<Address>,

    /// Use an already-deployed LightClient.sol proxy instead of deploying a new one.
    #[clap(long, env = Contract::LightClientProxy)]
    light_client_proxy: Option<Address>,

    /// Use an already-deployed FeeContract.sol instead of deploying a new one.
    #[clap(long, env = Contract::FeeContract)]
    fee_contract: Option<Address>,

    /// Use an already-deployed FeeContract.sol proxy instead of deploying a new one.
    #[clap(long, env = Contract::FeeContractProxy)]
    fee_contract_proxy: Option<Address>,
}

/// An identifier for a particular contract.
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub enum Contract {
    #[display(fmt = "ESPRESSO_SEQUENCER_HOTSHOT_ADDRESS")]
    HotShot,
    #[display(fmt = "ESPRESSO_SEQUENCER_PLONK_VERIFIER_ADDRESS")]
    PlonkVerifier,
    #[display(fmt = "ESPRESSO_SEQUENCER_LIGHT_CLIENT_STATE_UPDATE_VK_ADDRESS")]
    StateUpdateVK,
    #[display(fmt = "ESPRESSO_SEQUENCER_LIGHT_CLIENT_ADDRESS")]
    LightClient,
    #[display(fmt = "ESPRESSO_SEQUENCER_LIGHT_CLIENT_PROXY_ADDRESS")]
    LightClientProxy,
    #[display(fmt = "ESPRESSO_SEQUENCER_FEE_CONTRACT_ADDRESS")]
    FeeContract,
    #[display(fmt = "ESPRESSO_SEQUENCER_FEE_CONTRACT_PROXY_ADDRESS")]
    FeeContractProxy,
}

impl From<Contract> for OsStr {
    fn from(c: Contract) -> OsStr {
        c.to_string().into()
    }
}

/// Cache of contracts predeployed or deployed during this current run.
#[derive(Debug, Clone, Default)]
pub struct Contracts(HashMap<Contract, Address>);

impl From<DeployedContracts> for Contracts {
    fn from(deployed: DeployedContracts) -> Self {
        let mut m = HashMap::new();
        if let Some(addr) = deployed.hotshot {
            m.insert(Contract::HotShot, addr);
        }
        if let Some(addr) = deployed.plonk_verifier {
            m.insert(Contract::PlonkVerifier, addr);
        }
        if let Some(addr) = deployed.light_client_state_update_vk {
            m.insert(Contract::StateUpdateVK, addr);
        }
        if let Some(addr) = deployed.light_client {
            m.insert(Contract::LightClient, addr);
        }
        if let Some(addr) = deployed.light_client_proxy {
            m.insert(Contract::LightClientProxy, addr);
        }
        if let Some(addr) = deployed.fee_contract {
            m.insert(Contract::FeeContract, addr);
        }
        if let Some(addr) = deployed.fee_contract_proxy {
            m.insert(Contract::FeeContractProxy, addr);
        }
        Self(m)
    }
}

impl Contracts {
    pub fn new() -> Self {
        Contracts(HashMap::new())
    }

    pub fn get_contract_address(&self, contract: Contract) -> Option<Address> {
        self.0.get(&contract).copied()
    }
    /// Deploy a contract by calling a function.
    ///
    /// The `deploy` function will be called only if contract `name` is not already deployed;
    /// otherwise this function will just return the predeployed address. The `deploy` function may
    /// access this [`Contracts`] object, so this can be used to deploy contracts recursively in
    /// dependency order.
    pub async fn deploy_fn(
        &mut self,
        name: Contract,
        deploy: impl FnOnce(&mut Self) -> BoxFuture<'_, anyhow::Result<Address>>,
    ) -> anyhow::Result<Address> {
        if let Some(addr) = self.0.get(&name) {
            tracing::info!("skipping deployment of {name}, already deployed at {addr:#x}");
            return Ok(*addr);
        }
        tracing::info!("deploying {name}");
        let addr = deploy(self).await?;
        tracing::info!("deployed {name} at {addr:#x}");

        self.0.insert(name, addr);
        Ok(addr)
    }

    /// Deploy a contract by executing its deploy transaction.
    ///
    /// The transaction will only be broadcast if contract `name` is not already deployed.
    pub async fn deploy_tx<M, C>(
        &mut self,
        name: Contract,
        tx: ContractDeployer<M, C>,
    ) -> anyhow::Result<Address>
    where
        M: Middleware + 'static,
        C: Deref<Target = ethers::contract::Contract<M>>
            + From<ContractInstance<Arc<M>, M>>
            + Send
            + 'static,
    {
        self.deploy_fn(name, |_| {
            async {
                let contract = tx.send().await?;
                Ok(contract.address())
            }
            .boxed()
        })
        .await
    }

    /// Write a .env file.
    pub fn write(&self, mut w: impl Write) -> anyhow::Result<()> {
        for (contract, address) in &self.0 {
            writeln!(w, "{contract}={address:#x}")?;
        }
        Ok(())
    }
}

/// Default deployment function `LightClient.sol` in production
///
/// # NOTE:
/// currently, `LightClient.sol` follows upgradable contract, thus a follow-up
/// call to `.initialize()` with proper genesis block (and other constructor args)
/// are expected to be *delegatecall-ed through the proxy contract*.
pub async fn deploy_light_client_contract<M: Middleware + 'static>(
    l1: Arc<M>,
    contracts: &mut Contracts,
) -> anyhow::Result<Address> {
    // Deploy library contracts.
    let plonk_verifier = contracts
        .deploy_tx(
            Contract::PlonkVerifier,
            PlonkVerifier2::deploy(l1.clone(), ())?,
        )
        .await?;
    let vk = contracts
        .deploy_tx(
            Contract::StateUpdateVK,
            LightClientStateUpdateVK::deploy(l1.clone(), ())?,
        )
        .await?;

    // Link with LightClient's bytecode artifacts. We include the unlinked bytecode for the contract
    // in this binary so that the contract artifacts do not have to be distributed with the binary.
    // This should be fine because if the bindings we are importing are up to date, so should be the
    // contract artifacts: this is no different than foundry inlining bytecode objects in generated
    // bindings, except that foundry doesn't provide the bytecode for contracts that link with
    // libraries, so we have to do it ourselves.
    let mut bytecode: BytecodeObject = serde_json::from_str(include_str!(
        "../../contract-bindings/artifacts/LightClient_bytecode.json",
    ))?;
    bytecode
        .link_fully_qualified(
            "contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier",
            plonk_verifier,
        )
        .resolve()
        .context("error linking PlonkVerifier lib")?;
    bytecode
        .link_fully_qualified(
            "contracts/src/libraries/LightClientStateUpdateVK.sol:LightClientStateUpdateVK",
            vk,
        )
        .resolve()
        .context("error linking LightClientStateUpdateVK lib")?;
    ensure!(!bytecode.is_unlinked(), "failed to link LightClient.sol");

    // Deploy light client.
    let light_client_factory = ContractFactory::new(
        LIGHTCLIENT_ABI.clone(),
        bytecode
            .as_bytes()
            .context("error parsing bytecode for linked LightClient contract")?
            .clone(),
        l1,
    );
    let contract = light_client_factory.deploy(())?.send().await?;
    Ok(contract.address())
}

/// Default deployment function `LightClientMock.sol` for testing
///
/// # NOTE
/// unlike [`deploy_light_client_contract()`], the `LightClientMock` doesn't
/// use upgradable contract for simplicity, thus there's no follow-up `.initialize()`
/// necessary, as we have already call its un-disabled constructor.
pub async fn deploy_mock_light_client_contract<M: Middleware + 'static>(
    l1: Arc<M>,
    contracts: &mut Contracts,
    constructor_args: Option<LightClientConstructorArgs>,
) -> anyhow::Result<Address> {
    // Deploy library contracts.
    let plonk_verifier = contracts
        .deploy_tx(
            Contract::PlonkVerifier,
            PlonkVerifier2::deploy(l1.clone(), ())?,
        )
        .await?;
    let vk = contracts
        .deploy_tx(
            Contract::StateUpdateVK,
            LightClientStateUpdateVKMock::deploy(l1.clone(), ())?,
        )
        .await?;

    let mut bytecode: BytecodeObject = serde_json::from_str(include_str!(
        "../../contract-bindings/artifacts/LightClientMock_bytecode.json",
    ))?;
    bytecode
        .link_fully_qualified(
            "contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier",
            plonk_verifier,
        )
        .resolve()
        .context("error linking PlonkVerifier lib")?;
    bytecode
        .link_fully_qualified(
            "contracts/tests/mocks/LightClientStateUpdateVKMock.sol:LightClientStateUpdateVKMock",
            vk,
        )
        .resolve()
        .context("error linking LightClientStateUpdateVKMock lib")?;
    ensure!(
        !bytecode.is_unlinked(),
        "failed to link LightClientMock.sol"
    );

    // Deploy light client.
    let light_client_factory = ContractFactory::new(
        LIGHTCLIENTMOCK_ABI.clone(),
        bytecode
            .as_bytes()
            .context("error parsing bytecode for linked LightClientMock contract")?
            .clone(),
        l1,
    );
    let constructor_args: LightClientConstructorArgs = match constructor_args {
        Some(args) => args,
        None => LightClientConstructorArgs::dummy_genesis(),
    };
    let contract = light_client_factory
        .deploy(constructor_args)?
        .send()
        .await?;

    Ok(contract.address())
}

#[allow(clippy::too_many_arguments)]
pub async fn deploy(
    l1url: Url,
    mnemonic: String,
    account_index: u32,
    multisig_address: Option<H160>,
    use_mock_contract: bool,
    only: Option<Vec<ContractGroup>>,
    genesis: BoxFuture<'_, anyhow::Result<(ParsedLightClientState, ParsedStakeTableState)>>,
    mut contracts: Contracts,
) -> anyhow::Result<Contracts> {
    let provider = Provider::<Http>::try_from(l1url.to_string())?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .index(account_index)?
        .build()?
        .with_chain_id(chain_id);
    let deployer = wallet.address();
    let l1 = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    // As a sanity check, check that the deployer address has some balance of ETH it can use to pay
    // gas.
    let balance = l1.get_balance(deployer, None).await?;
    ensure!(
        balance > 0.into(),
        "deployer account {deployer:#x} is not funded!"
    );
    tracing::info!(%balance, "deploying from address {deployer:#x}");

    // `HotShot.sol`
    if should_deploy(ContractGroup::HotShot, &only) {
        contracts
            .deploy_tx(Contract::HotShot, HotShot::deploy(l1.clone(), ())?)
            .await?;
    }

    // `LightClient.sol`
    if should_deploy(ContractGroup::LightClient, &only) {
        // Deploy the upgradable light client contract first, then initialize it through a proxy contract
        let lc_address = if use_mock_contract {
            contracts
                .deploy_fn(Contract::LightClient, |contracts| {
                    deploy_mock_light_client_contract(l1.clone(), contracts, None).boxed()
                })
                .await?
        } else {
            contracts
                .deploy_fn(Contract::LightClient, |contracts| {
                    deploy_light_client_contract(l1.clone(), contracts).boxed()
                })
                .await?
        };
        let light_client = LightClient::new(lc_address, l1.clone());

        let (genesis_lc, genesis_stake) = genesis.await?.clone();

        let data = light_client
            .initialize(genesis_lc.into(), genesis_stake.into(), 864000, deployer)
            .calldata()
            .context("calldata for initialize transaction not available")?;
        let light_client_proxy_address = contracts
            .deploy_tx(
                Contract::LightClientProxy,
                ERC1967Proxy::deploy(l1.clone(), (lc_address, data))?,
            )
            .await?;

        // confirm that the implementation address is the address of the light client contract deployed above
        if !is_proxy_contract(provider.clone(), light_client_proxy_address)
            .await
            .expect("Failed to determine if light contract is a proxy")
        {
            panic!("Light Client contract's address is not a proxy");
        }

        // Instantiate a wrapper with the proxy address and light client ABI.
        let proxy = LightClient::new(light_client_proxy_address, l1.clone());

        // Transfer ownership to the multisig wallet if provided.
        if let Some(owner) = multisig_address {
            tracing::info!(
                %light_client_proxy_address,
                %owner,
                "transferring light client proxy ownership to multisig",
            );
            proxy.transfer_ownership(owner).send().await?;
        }
    }

    // `FeeContract.sol`
    if should_deploy(ContractGroup::FeeContract, &only) {
        let fee_contract_address = contracts
            .deploy_tx(Contract::FeeContract, FeeContract::deploy(l1.clone(), ())?)
            .await?;
        let fee_contract = FeeContract::new(fee_contract_address, l1.clone());
        let data = fee_contract
            .initialize(deployer)
            .calldata()
            .context("calldata for initialize transaction not available")?;
        let fee_contract_proxy_address = contracts
            .deploy_tx(
                Contract::FeeContractProxy,
                ERC1967Proxy::deploy(l1.clone(), (fee_contract_address, data))?,
            )
            .await?;

        // confirm that the implementation address is the address of the fee contract deployed above
        if !is_proxy_contract(provider.clone(), fee_contract_proxy_address)
            .await
            .expect("Failed to determine if fee contract is a proxy")
        {
            panic!("Fee contract's address is not a proxy");
        }

        // Instantiate a wrapper with the proxy address and fee contract ABI.
        let proxy = FeeContract::new(fee_contract_proxy_address, l1.clone());

        // Transfer ownership to the multisig wallet if provided.
        if let Some(owner) = multisig_address {
            tracing::info!(
                %fee_contract_proxy_address,
                %owner,
                "transferring fee contract proxy ownership to multisig",
            );
            proxy.transfer_ownership(owner).send().await?;
        }
    }

    Ok(contracts)
}

fn should_deploy(group: ContractGroup, only: &Option<Vec<ContractGroup>>) -> bool {
    match only {
        Some(groups) => groups.contains(&group),
        None => true,
    }
}

pub async fn is_proxy_contract(
    provider: Provider<Http>,
    proxy_address: H160,
) -> anyhow::Result<bool> {
    // confirm that the proxy_address is a proxy
    // using the implementation slot, 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc, which is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1
    let hex_bytes = hex::decode("360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")
        .expect("Failed to decode hex string");
    let implementation_slot = ethers::types::H256::from_slice(&hex_bytes);
    let storage = provider
        .get_storage_at(proxy_address, implementation_slot, None)
        .await?;

    let implementation_address = H160::from_slice(&storage[12..]);

    // when the implementation address is not equal to zero, it's a proxy
    Ok(implementation_address != H160::zero())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ContractGroup {
    #[clap(name = "hotshot")]
    HotShot, // TODO: confirm whether we keep HotShot in the contract group
    FeeContract,
    LightClient,
}
