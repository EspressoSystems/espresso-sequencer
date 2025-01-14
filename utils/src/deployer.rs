use alloy::{
    hex::FromHex,
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::{Address, B256, U256},
    providers::{
        fillers::{FillProvider, JoinFill, WalletFiller},
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::TransactionRequest,
    signers::local::{coins_bip39::English, MnemonicBuilder},
    transports::http::{Client, Http},
};
use anyhow::{ensure, Context};
use clap::{builder::OsStr, Parser, ValueEnum};
use contract_bindings::{
    erc1967proxy::ERC1967Proxy,
    feecontract::FeeContract,
    lightclient::LightClient::LightClientInstance,
    lightclientmock::{self, LightClientMock},
    lightclientstateupdatevk::LightClientStateUpdateVK,
    permissionedstaketable::PermissionedStakeTable::{self, NodeInfo},
    plonkverifier::PlonkVerifier,
};
use derive_more::Display;
use ethers::solc::artifacts::BytecodeObject;
use ethers_conv::{ToAlloy, ToEthers};
use futures::{
    future::{BoxFuture, FutureExt},
    TryFutureExt,
};
use hotshot_contract_adapter::light_client::{
    LightClientConstructorArgs, ParsedLightClientState, ParsedStakeTableState,
};
use std::{collections::HashMap, future::Future, io::Write, pin::Pin};
use url::Url;

/// A type alias for a provider that has signing capabilities (i.e. can deploy contracts)
type DeployerProvider = FillProvider<
    JoinFill<Identity, WalletFiller<EthereumWallet>>,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

/// Set of predeployed contracts.
#[derive(Clone, Debug, Parser)]
pub struct DeployedContracts {
    /// Use an already-deployed PlonkVerifier.sol instead of deploying a new one.
    #[clap(long, env = Contract::PlonkVerifier)]
    plonk_verifier: Option<Address>,

    /// Use an already-deployed LightClientStateUpdateVK.sol instead of deploying a new one.
    #[clap(long, env = Contract::LightClientStateUpdateVK)]
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

    /// Use an already-deployed PermissionedStakeTable.sol proxy instead of deploying a new one.
    #[clap(long, env = Contract::PermissionedStakeTable)]
    permissioned_stake_table: Option<Address>,
}

/// An identifier for a particular contract.
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub enum Contract {
    #[display("ESPRESSO_SEQUENCER_PLONK_VERIFIER_ADDRESS")]
    PlonkVerifier,
    #[display("ESPRESSO_SEQUENCER_LIGHT_CLIENT_STATE_UPDATE_VK_ADDRESS")]
    LightClientStateUpdateVK,
    #[display("ESPRESSO_SEQUENCER_LIGHT_CLIENT_ADDRESS")]
    LightClient,
    #[display("ESPRESSO_SEQUENCER_LIGHT_CLIENT_PROXY_ADDRESS")]
    LightClientProxy,
    #[display("ESPRESSO_SEQUENCER_FEE_CONTRACT_ADDRESS")]
    FeeContract,
    #[display("ESPRESSO_SEQUENCER_FEE_CONTRACT_PROXY_ADDRESS")]
    FeeContractProxy,
    #[display("ESPRESSO_SEQUENCER_PERMISSIONED_STAKE_TABLE_ADDRESS")]
    PermissionedStakeTable,
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
        if let Some(addr) = deployed.plonk_verifier {
            m.insert(Contract::PlonkVerifier, addr);
        }
        if let Some(addr) = deployed.light_client_state_update_vk {
            m.insert(Contract::LightClientStateUpdateVK, addr);
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
        if let Some(addr) = deployed.permissioned_stake_table {
            m.insert(Contract::PermissionedStakeTable, addr);
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
        deploy: Pin<Box<dyn Future<Output = Result<Address, anyhow::Error>> + Send>>,
    ) -> anyhow::Result<Address> {
        if let Some(addr) = self.0.get(&name) {
            tracing::info!("skipping deployment of {name}, already deployed at {addr:#x}");
            return Ok(*addr);
        }
        tracing::info!("deploying {name}");
        let addr = deploy.await?;
        tracing::info!("deployed {name} at {addr:#x}");

        self.0.insert(name, addr);
        Ok(addr)
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
pub async fn deploy_light_client_contract(
    l1: &mut DeployerProvider,
    contracts: &mut Contracts,
) -> anyhow::Result<Address> {
    // Deploy library contracts.
    let plonk_verifier = contracts
        .deploy_fn(
            Contract::PlonkVerifier,
            PlonkVerifier::deploy(l1.clone())
                .map_ok(|instance| instance.address().clone())
                .map_err(|e| anyhow::anyhow!(e))
                .boxed(),
        )
        .await?;
    let vk = contracts
        .deploy_fn(
            Contract::LightClientStateUpdateVK,
            LightClientStateUpdateVK::deploy(l1.clone())
                .map_ok(|instance| instance.address().clone())
                .map_err(|e| anyhow::anyhow!(e))
                .boxed(),
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
            plonk_verifier.to_ethers(),
        )
        .resolve()
        .context("error linking PlonkVerifier lib")?;
    bytecode
        .link_fully_qualified(
            "contracts/src/libraries/LightClientStateUpdateVK.sol:LightClientStateUpdateVK",
            vk.to_ethers(),
        )
        .resolve()
        .context("error linking LightClientStateUpdateVK lib")?;
    ensure!(!bytecode.is_unlinked(), "failed to link LightClient.sol");

    // Create the deploy transaction
    let deploy_tx = TransactionRequest::default().with_deploy_code(
        bytecode
            .as_bytes()
            .ok_or_else(|| {
                anyhow::anyhow!("error parsing bytecode for linked LightClient contract")
            })?
            .clone()
            .to_alloy(),
    );

    // Send the transaction
    let receipt = l1
        .send_transaction(deploy_tx)
        .await
        .with_context(|| "error building deploy transaction")
        .with_context(|| "error sending deploy transaction")?
        .get_receipt()
        .await
        .with_context(|| "error getting deploy receipt")?;

    // Get the address of the deployed contract
    let contract_address = receipt
        .contract_address
        .context("no contract address in receipt")?;

    Ok(contract_address)
}

/// Default deployment function `LightClientMock.sol` for testing
///
/// # NOTE
/// unlike [`deploy_light_client_contract()`], the `LightClientMock` doesn't
/// use upgradable contract for simplicity, thus there's no follow-up `.initialize()`
/// necessary, as we have already call its un-disabled constructor.
pub async fn deploy_mock_light_client_contract(
    l1: &mut DeployerProvider,
    contracts: &mut Contracts,
    constructor_args: Option<LightClientConstructorArgs>,
) -> anyhow::Result<Address> {
    // Deploy library contracts.
    let plonk_verifier = contracts
        .deploy_fn(
            Contract::PlonkVerifier,
            PlonkVerifier::deploy(l1.clone())
                .map_ok(|instance| instance.address().clone())
                .map_err(|e| anyhow::anyhow!(e))
                .boxed(),
        )
        .await?;
    let vk = contracts
        .deploy_fn(
            Contract::LightClientStateUpdateVK,
            LightClientStateUpdateVK::deploy(l1.clone())
                .map_ok(|instance| instance.address().clone())
                .map_err(|e| anyhow::anyhow!(e))
                .boxed(),
        )
        .await?;

    let mut bytecode: BytecodeObject = serde_json::from_str(include_str!(
        "../../contract-bindings/artifacts/LightClientMock_bytecode.json",
    ))?;
    bytecode
        .link_fully_qualified(
            "contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier",
            plonk_verifier.to_ethers(),
        )
        .resolve()
        .context("error linking PlonkVerifier lib")?;
    bytecode
        .link_fully_qualified(
            "contracts/tests/mocks/LightClientStateUpdateVKMock.sol:LightClientStateUpdateVKMock",
            vk.to_ethers(),
        )
        .resolve()
        .context("error linking LightClientStateUpdateVKMock lib")?;
    ensure!(
        !bytecode.is_unlinked(),
        "failed to link LightClientMock.sol"
    );

    let constructor_args: LightClientConstructorArgs = match constructor_args {
        Some(args) => args,
        None => LightClientConstructorArgs::dummy_genesis(),
    };

    // Create the deploy transaction
    let deploy_tx = LightClientMock::deploy_builder(
        l1.clone(),
        lightclientmock::LightClient::LightClientState::from(constructor_args.light_client_state),
        lightclientmock::LightClient::StakeTableState::from(constructor_args.stake_table_state),
        constructor_args.max_history_seconds,
    )
    .into_transaction_request()
    .with_deploy_code(
        bytecode
            .as_bytes()
            .ok_or_else(|| {
                anyhow::anyhow!("error parsing bytecode for linked LightClientMock contract")
            })?
            .clone()
            .to_alloy(),
    );

    // Send the transaction
    let receipt = l1
        .send_transaction(deploy_tx)
        .await
        .with_context(|| "error building deploy transaction")
        .with_context(|| "error sending deploy transaction")?
        .get_receipt()
        .await
        .with_context(|| "error getting deploy receipt")?;

    // Get the address of the deployed contract
    let contract_address = receipt
        .contract_address
        .context("no contract address in receipt")?;

    Ok(contract_address)
}

#[allow(clippy::too_many_arguments)]
pub async fn deploy(
    l1url: Url,
    mnemonic: String,
    account_index: u32,
    multisig_address: Option<Address>,
    use_mock_contract: bool,
    only: Option<Vec<ContractGroup>>,
    genesis: BoxFuture<'_, anyhow::Result<(ParsedLightClientState, ParsedStakeTableState)>>,
    permissioned_prover: Option<Address>,
    mut contracts: Contracts,
    initial_stake_table: Option<Vec<NodeInfo>>,
) -> anyhow::Result<Contracts> {
    // Create a deployer signer from the mnemonic and account index specified (from the args)
    let deployer_signer = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .index(account_index)?
        .build()?;

    // Get the deployer's public key
    let deployer_public_key = deployer_signer.address();

    // Create an Ethereum wallet from the signer
    let deployer_wallet = EthereumWallet::from(deployer_signer);

    // Create a new L1 provider with the deployer wallet
    let mut l1_provider = ProviderBuilder::new()
        .wallet(deployer_wallet)
        .on_http(l1url);

    // As a sanity check, check that the deployer address has some balance of ETH it can use to pay
    // gas.
    let balance = l1_provider.get_balance(deployer_public_key).await?;
    ensure!(
        balance > U256::ZERO,
        "deployer account {deployer_public_key:#x} is not funded!"
    );
    tracing::info!(%balance, "Deploying from address {deployer_public_key:#x}");

    // `LightClient.sol`
    if should_deploy(ContractGroup::LightClient, &only) {
        // Deploy the upgradable light client contract first, then initialize it through a proxy contract
        let lc_address = if use_mock_contract {
            deploy_mock_light_client_contract(&mut l1_provider, &mut contracts, None).await?
        } else {
            deploy_light_client_contract(&mut l1_provider, &mut contracts).await?
        };

        let light_client = LightClientInstance::new(lc_address, l1_provider.clone());

        let (genesis_lc, genesis_stake) = genesis.await?.clone();

        let data = light_client
            .initialize(
                genesis_lc.into(),
                genesis_stake.into(),
                864000,
                deployer_public_key,
            )
            .calldata()
            .clone();
        let light_client_proxy_address = contracts
            .deploy_fn(
                Contract::LightClientProxy,
                ERC1967Proxy::deploy(l1_provider.clone(), lc_address, data)
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e: alloy::contract::Error| anyhow::anyhow!(e))
                    .boxed(),
            )
            .await?;

        // confirm that the implementation address is the address of the light client contract deployed above
        if !is_proxy_contract(&l1_provider.root(), light_client_proxy_address)
            .await
            .expect("Failed to determine if light contract is a proxy")
        {
            panic!("Light Client contract's address is not a proxy");
        }

        // Instantiate a wrapper with the proxy address and light client ABI.
        let proxy = LightClientInstance::new(light_client_proxy_address, l1_provider.clone());

        // Perission the light client prover.
        if let Some(prover) = permissioned_prover {
            tracing::info!(%light_client_proxy_address, %prover, "setting permissioned prover");
            proxy
                .setPermissionedProver(prover)
                .send()
                .await?
                .watch()
                .await?;
        }

        // Transfer ownership to the multisig wallet if provided.
        if let Some(owner) = multisig_address {
            tracing::info!(
                ?light_client_proxy_address,
                ?owner,
                "transferring light client proxy ownership to multisig",
            );
            proxy.transferOwnership(owner).send().await?.watch().await?;
        }
    }

    // `FeeContract.sol`
    if should_deploy(ContractGroup::FeeContract, &only) {
        let fee_contract_address = contracts
            .deploy_fn(
                Contract::FeeContract,
                FeeContract::deploy(l1_provider.clone())
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e: alloy::contract::Error| anyhow::anyhow!(e))
                    .boxed(),
            )
            .await?;
        let fee_contract = FeeContract::new(fee_contract_address, l1_provider.clone());
        let data = fee_contract
            .initialize(deployer_public_key)
            .calldata()
            .clone();
        let fee_contract_proxy_address = contracts
            .deploy_fn(
                Contract::FeeContractProxy,
                ERC1967Proxy::deploy(l1_provider.clone(), fee_contract_address, data)
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e: alloy::contract::Error| anyhow::anyhow!(e))
                    .boxed(),
            )
            .await?;

        // confirm that the implementation address is the address of the fee contract deployed above
        if !is_proxy_contract(&mut l1_provider.root(), fee_contract_proxy_address)
            .await
            .expect("Failed to determine if fee contract is a proxy")
        {
            panic!("Fee contract's address is not a proxy");
        }

        // Instantiate a wrapper with the proxy address and fee contract ABI.
        let proxy = FeeContract::new(fee_contract_proxy_address, l1_provider.clone());

        // Transfer ownership to the multisig wallet if provided.
        if let Some(owner) = multisig_address {
            tracing::info!(
                ?fee_contract_proxy_address,
                ?owner,
                "transferring fee contract proxy ownership to multisig",
            );
            proxy.transferOwnership(owner).send().await?.watch().await?;
        }
    }

    // `PermissionedStakeTable.sol`
    if should_deploy(ContractGroup::PermissionedStakeTable, &only) {
        let initial_stake_table: Vec<_> = initial_stake_table.unwrap_or_default();
        let stake_table_address = contracts
            .deploy_fn(
                Contract::PermissionedStakeTable,
                PermissionedStakeTable::deploy(l1_provider.clone(), initial_stake_table)
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e| anyhow::anyhow!(e))
                    .boxed(),
            )
            .await?;

        let stake_table = PermissionedStakeTable::new(stake_table_address, l1_provider.clone());

        // Transfer ownership to the multisig wallet if provided.
        if let Some(owner) = multisig_address {
            tracing::info!(
                ?stake_table_address,
                ?owner,
                "transferring PermissionedStakeTable ownership to multisig",
            );
            stake_table
                .transferOwnership(owner)
                .send()
                .await?
                .watch()
                .await?;
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
    provider: &RootProvider<Http<Client>>,
    proxy_address: Address,
) -> anyhow::Result<bool> {
    // confirm that the proxy_address is a proxy
    // using the implementation slot, 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc, which is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1
    let implementation_slot =
        B256::from_hex("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")
            .with_context(|| {
                format!("Failed to decode implementation slot for proxy contract {proxy_address}")
            })?;
    let storage = provider
        .get_storage_at(proxy_address, implementation_slot.into())
        .await?;

    let implementation_address = Address::from_slice(&storage.to_be_bytes::<32>()[12..]);

    // when the implementation address is not equal to zero, it's a proxy
    Ok(implementation_address != Address::ZERO)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ContractGroup {
    FeeContract,
    LightClient,
    PermissionedStakeTable,
}

#[cfg(any(test, feature = "testing"))]
pub mod test_helpers {

    use alloy::{
        network::TransactionBuilder, primitives::Address, providers::Provider,
        rpc::types::TransactionRequest,
    };
    use anyhow::{ensure, Context};
    use contract_bindings::{
        erc1967proxy::ERC1967Proxy, feecontract::FeeContract,
        lightclient::LightClient::LightClientInstance,
        lightclientstateupdatevk::LightClientStateUpdateVK, plonkverifier::PlonkVerifier,
    };
    use ethers::solc::artifacts::BytecodeObject;
    use ethers_conv::{ToAlloy, ToEthers};
    use futures::{FutureExt, TryFutureExt};
    use hotshot_contract_adapter::light_client::LightClientConstructorArgs;

    use super::{Contract, Contracts, DeployerProvider};

    /// Deployment `LightClientMock.sol` as proxy for testing
    pub async fn deploy_light_client_contract_as_proxy_for_test(
        l1: &mut DeployerProvider,
        contracts: &mut Contracts,
        constructor_args: Option<LightClientConstructorArgs>,
    ) -> anyhow::Result<Address> {
        // Deploy library contracts.
        let plonk_verifier = contracts
            .deploy_fn(
                Contract::PlonkVerifier,
                PlonkVerifier::deploy(l1.clone())
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e| anyhow::anyhow!(e))
                    .boxed(),
            )
            .await?;
        let vk = contracts
            .deploy_fn(
                Contract::LightClientStateUpdateVK,
                LightClientStateUpdateVK::deploy(l1.clone())
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e| anyhow::anyhow!(e))
                    .boxed(),
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
                plonk_verifier.to_ethers(),
            )
            .resolve()
            .context("error linking PlonkVerifier lib")?;
        bytecode
            .link_fully_qualified(
                "contracts/src/libraries/LightClientStateUpdateVK.sol:LightClientStateUpdateVK",
                vk.to_ethers(),
            )
            .resolve()
            .context("error linking LightClientStateUpdateVK lib")?;
        ensure!(!bytecode.is_unlinked(), "failed to link LightClient.sol");

        // Create the deploy transaction
        let deploy_tx = TransactionRequest::default().with_deploy_code(
            bytecode
                .as_bytes()
                .ok_or_else(|| {
                    anyhow::anyhow!("error parsing bytecode for linked LightClient contract")
                })?
                .clone()
                .to_alloy(),
        );

        // Send the transaction
        let receipt = l1
            .send_transaction(deploy_tx)
            .await
            .with_context(|| "error building deploy transaction")
            .with_context(|| "error sending deploy transaction")?
            .get_receipt()
            .await
            .with_context(|| "error getting deploy receipt")?;

        // Get the address of the deployed contract
        let light_client_address = receipt
            .contract_address
            .context("no contract address in receipt")?;

        let light_client = LightClientInstance::new(light_client_address, l1.clone());

        let constructor_args: LightClientConstructorArgs = match constructor_args {
            Some(args) => args,
            None => LightClientConstructorArgs::dummy_genesis(),
        };

        let deployer = *(l1.clone().get_accounts().await?.first()).expect("Address not found");

        let data = light_client
            .initialize(
                constructor_args.light_client_state.into(),
                constructor_args.stake_table_state.into(),
                constructor_args.max_history_seconds,
                deployer,
            )
            .calldata()
            .clone();
        let light_client_proxy_address = contracts
            .deploy_fn(
                Contract::LightClientProxy,
                ERC1967Proxy::deploy(l1.clone(), light_client_address, data)
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e| anyhow::anyhow!(e))
                    .boxed(),
            )
            .await?;

        Ok(light_client_proxy_address)
    }

    /// Default deployment function `FeeContract.sol` for testing
    ///
    pub async fn deploy_fee_contract(l1: &mut DeployerProvider) -> anyhow::Result<Address> {
        // Deploy fee contract
        let fee_contract_instance = FeeContract::deploy(l1.clone())
            .await
            .with_context(|| "error deploying fee contract")?;

        // Return the address
        Ok(fee_contract_instance.address().clone())
    }

    /// Default deployment function `FeeContract.sol` (as proxy) for testing
    ///
    pub async fn deploy_fee_contract_as_proxy(
        l1: &mut DeployerProvider,
        contracts: &mut Contracts,
    ) -> anyhow::Result<Address> {
        // Deploy fee contract
        let fee_contract_address = deploy_fee_contract(l1).await?;

        let fee_contract = FeeContract::new(fee_contract_address, l1.clone());

        let deployer = *(l1.clone().get_accounts().await?.first()).expect("Address not found");

        let data = fee_contract.initialize(deployer).calldata().clone();

        let fee_contract_proxy_address = contracts
            .deploy_fn(
                Contract::FeeContractProxy,
                ERC1967Proxy::deploy(l1.clone(), fee_contract_address, data)
                    .map_ok(|instance| instance.address().clone())
                    .map_err(|e| anyhow::anyhow!(e))
                    .boxed(),
            )
            .await?;

        Ok(fee_contract_proxy_address)
    }
}
