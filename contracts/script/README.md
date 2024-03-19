# Deploying Upgradeable Smart Contracts

Upgradeable Smart contracts are deployed with Openzeppelin Defender to enable a deployment strategy that is more secure
and also uses a multi-sig Safe wallet. When deploying using openzeppelin the `defender` profile in the `foundry.toml`
file is used.

Note: One must set the environment variable `FOUNDRY_PROFILE` accordingly to use the appropriate profile in the
`foundry.toml` settings. E.g.

```bash
export FOUNDRY_PROFILE=defender
```

The profile, `profile.default` is used by default. Setting the `FOUNDRY_PROFILE` variable overrides the `foundry.toml`
settings.

## Prerequisites

1. Create a multisig wallet using [Safe](https://app.safe.global/welcome/accounts) on the network you'd like to deploy
   to.
2. In OpenZeppelin Defender, create an Approval Process that requires the multisig wallet you created above.
   `Manage > Approval Processes`
3. In OpenZeppelin Defender, create a deployment environment and use the approval process created in Step 2. Be sure to
   copy the Defender secret and key, that is shown at the end of this step, into the .env file.

## Deployments

### Deploying the Fee Contract

Steps:

1. Run the Deployment Command This command requires you to go to OpenZeppelin Defender's UI to see the transaction.
   Click that transaction which opens up the Safe UI where your signers for that Safe multi-sig wallet can confirm the
   transaction. The two transactions to be confirmed are: (i) deployment of implementation contract (ii) deployment of
   proxy contract

```bash
export FOUNDRY_PROFILE=defender && \
forge clean && \
forge build && \
forge script contracts/script/FeeContractWithDefender.s.sol:FeeContractDefenderDeployScript --ffi --rpc-url https://ethereum-sepolia.publicnode.com && \
export FOUNDRY_PROFILE=default && \
rm -rf out
```

2. Verify the Implementation contract on Etherscan (Use another window as step would not have completed yet)

```bash
forge verify-contract --etherscan-api-key $ETHERSCAN_API_KEY $FEE_CONTRACT_ADDRESS contracts/src/FeeContract.sol:FeeContract --chain 11155111
```

3. Inform Etherscan that it's a Proxy When the proxy is deployed, go to Etherscan. Go to Contract > Code > More Options
   and select the 'is this a proxy?' option. You should then be able to interact with the implementation contract via a
   proxy.

Example successful deployment response

```bash
[⠒] Compiling...
No files changed, compilation skipped
2024-03-11T16:41:29.919133Z  WARN foundry_evm_core::fork::cache: Failed to read cache file err=Os { code: 2, kind: NotFound, message: "No such file or directory" } path="/Users/alysiahuggins/.foundry/cache/rpc/sepolia/5464723"
Script ran successfully.

== Return ==
proxy: address payable 0x61B4C96475B99A6ce01AfF0da7910605D048c125
multisig: address 0xc56fA6505d10bF322e01327e22479DE78C3Bf1cE
```

### Deploying the Light Client Contract

Read Deploying the Fee Contract for a more detailed version of this.

1. Initiate the Deployment with OpenZeppelin Defender

```bash
export FOUNDRY_PROFILE=defender && \
forge clean && \
forge build && \
forge script contracts/script/LightClientWithDefender.s.sol:LightClientDefenderDeployScript --ffi --rpc-url https://ethereum-sepolia.publicnode.com && \
export FOUNDRY_PROFILE=default && \
rm -rf out
```

2. Verify the Contract

```bash
forge verify-contract --etherscan-api-key $ETHERSCAN_API_KEY $LIGHT_CLIENT_CONTRACT_ADDRESS contracts/src/LightClient.sol:LightClient --chain 11155111
```

3. Inform Etherscan that it's a Proxy When the proxy is deployed, go to Etherscan. Go to Contract > Code > More Options
   and select the 'is this a proxy?' option. You should then be able to interact with the implementation contract via a
   proxy.

## Upgrades

### Upgrading the Fee Contract

Steps:

1. In the `FeeContractWithDefender.s.sol` file, in the contract named, `FeeContractDefenderUpgradeScript`, replace the
   `proxyAddress` with the proxy address for the FeeContract. Ensure that the salt has been updated in the `.env` file
   and the run the following command.

```bash
export FOUNDRY_PROFILE=defender && \
forge clean && \
forge build && \
forge script contracts/script/FeeContractWithDefender.s.sol:FeeContractDefenderUpgradeScript --ffi --rpc-url https://ethereum-sepolia.publicnode.com && \
export FOUNDRY_PROFILE=default && \
rm -rf out
```

2. This command requires you to go to OpenZeppelin Defender's UI to see the transaction. Click that transaction which
   opens up the Safe UI where your signers for that Safe multi-sig wallet can confirm the transaction.

The transactions being confirmed are: (i) the deployment of the new fee contract (ii) the execution of the
`upgradeToAndCall` method which updates the implementation contract that the proxy contract is referencing.

### Upgrading the Light Client Contract

Ensure that you update the version in the `getVersion()` method of the latest implementation contract.

Steps:

1. In the `LightClientWithDefender.s.sol` file, in the contract named, `LightClientDefenderUpgradeScript`, replace the
   `proxyAddress` with the proxy address for the LightClient. Ensure that the salt has been updated in the `.env` file
   and the run the following command.

```bash
export FOUNDRY_PROFILE=defender && \
forge clean && \
forge build && \
forge script contracts/script/LightClientWithDefender.s.sol:LightClientDefenderUpgradeScript --ffi --rpc-url https://ethereum-sepolia.publicnode.com && \
export FOUNDRY_PROFILE=default && \
rm -rf out
```

2. This command requires you to go to OpenZeppelin Defender's UI to see the transaction. Click that transaction which
   opens up the Safe UI where your signers for that Safe multi-sig wallet can confirm the transaction.

The transactions being confirmed are: (i) the deployment of the new fee contract (ii) the execution of the
`upgradeToAndCall` method which updates the implementation contract that the proxy contract is referencing.
