# Deploying Upgradeable Smart Contracts

Upgradeable Smart contracts are deployed with Openzeppelin Defender to enable a deployment strategy that is more secure
and also uses a multi-sig Safe wallet. When deploying using openzeppelin the `defender` profile in the `foundry.toml`
file is used.

## Prerequisites

1. Create a multisig wallet using [Safe](https://app.safe.global/welcome/accounts) on the network you'd like to deploy
   to.
2. In the home folder of this repo, you're in a nix shell: Enter `nix-shell` in the terminal
3. If the contracts have never been compiled, run `forge build`
4. Set the values for `NUM_BLOCKS_PER_EPOCH` and `NUM_INIT_VALIDATORS` in the `.env.contracts` file.

If using [OpenZeppelin Defender](https://www.openzeppelin.com/defender), follow these steps:

1. Create an Approval Process that requires the multisig wallet you created above. `Manage > Approval Processes`.
   1. Enter a name for your approval process
   1. Enter the multisig address from shown in te Safe UI
   1. Enter one of multisig owner address addresses
   1. Save the changes
2. In OpenZeppelin Defender, create a deployment environment by clicking on "Setup" in the
   [deploy](https://defender.openzeppelin.com/v2/#/deploy) tab. Use "Test Environment" for deploying to testnets (e. g.
   Sepolia) and "Production Environment" for mainnet.
   1. Choose a network
   1. Select the approval process created in Step 2
   1. Be sure to save `DEFENDER_SECRET` ("Team Secret key") and `DEFENDER_KEY` ("Team API Key"), that is shown at the
      end of this step, into the `.env.contracts` file. The keys won't be available later at a later point.

If not using OpenZeppelin Defender, follow these steps:

1. Set the values for `DEPLOYER_MNEMONIC` and `DEPLOYER_MNEMONIC_OFFSET` in the `.env.contracts` file.

## Deployments

### Deploying the Fee Contract

Steps:

1. Run the Deployment command.

#### Without OpenZeppelin Defender (current method)

```bash
   source .env.contracts && \
   forge clean && \
   forge script contracts/script/FeeContract.s.sol:DeployFeeContractScript $SAFE_MULTISIG_ADDRESS \
   --sig 'run(address)' \
   --ffi \
   --rpc-url https://ethereum-sepolia.publicnode.com  \
   --build-info true \
   --legacy \
   --broadcast
```

#### With OpenZeppelin Defender

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/FeeContract.s.sol:DeployFeeContractWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
```

1.  Go to the [deploy](https://defender.openzeppelin.com/v2/#/deploy) tab OpenZeppelin Defender's UI and click on the
    current environment to see the transaction. The transaction should be visible with status "SUBMITTED". The page may
    need to be refreshed a few times. It occasionally may take minutes for transactions to appear.
2.  Click that transaction, then "Open in Safe App" which opens up the Safe UI where your signers for that Safe
    multi-sig wallet can confirm the transaction. The two transactions to be confirmed are: (i) deployment of
    implementation contract (ii) deployment of proxy contract
3.  If the transaction looks correct click "confirm".
4.  Click "Execute".
5.  Confirm the transaction with your wallet (e. g. metamask).
6.  Repeat steps 1 to 5 for the deployment of the proxy contract. You may need to refresh the OpenZeppelin Defender
    "deploy" tab a few times until the second transaction appears.

7.  Verify the Implementation contract on Etherscan (Use another window as step would not have completed yet)

```bash
forge verify-contract --chain-id 11155111 \
--watch --etherscan-api-key $ETHERSCAN_API_KEY \
--compiler-version $SOLC_VERSION \
$FEE_CONTRACT_ADDRESS \
contracts/src/FeeContract.sol:FeeContract
```

You can get the `$SOLC_VERSION` by running the command `solc --version`.

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

Read Deploying the Fee Contract for a more detailed version of this. Since the LightClient contract uses the
PlonkVerifier library, the PlonkVerifier library has to be deployed and then referenced at deployment time. Thus ensure
you've deployed the PlonkVerifier ([see steps below](#deploy-the-plonk-verifier-library-with-defender)) and set the
`$PLONK_VERIFIER_ADDRESS` variable in the command below. Each time modifications are made to the Plonk Verifier,
contracts that depend on it such as the Light Client contract have to be upgraded and should use the new PlonkVerifier
contract address as part of the deployment. Read Deploying the Fee Contract for a more detailed version of this. Since
the LightClient contract uses the PlonkVerifier library, the PlonkVerifier library has to be deployed and then
referenced at deployment time. Thus ensure you've deployed the PlonkVerifier
([see steps below](#deploy-the-plonk-verifier-library-with-defender)) and set the `$PLONK_VERIFIER_ADDRESS` variable in
the command below. Each time modifications are made to the Plonk Verifier, contracts that depend on it such as the Light
Client contract have to be upgraded and should use the new PlonkVerifier contract address as part of the deployment.

1. Run the deployment command

#### Without OpenZeppelin Defender (current method)

```bash
   source .env.contracts && \
   forge clean && \
   forge script contracts/script/LightClient.s.sol:DeployLightClientScript $NUM_BLOCKS_PER_EPOCH $NUM_INIT_VALIDATORS $SAFE_MULTISIG_ADDRESS \
   --sig 'run(uint32, uint32, address)' \
   --ffi \
   --rpc-url https://ethereum-sepolia.publicnode.com  \
   --build-info true \
   --legacy \
   --broadcast
```

#### With OpenZeppelin Defender

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/LightClient.s.sol:DeployLightClientDefenderScript \
--ffi --rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS
```

```
source .env.contracts && \
forge clean && \
forge script contracts/script/LightClient.s.sol:DeployLightClientDefenderScript \
--ffi --rpc-url https://ethereum-sepolia.publicnode.com \
--build-info true \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS
```

2. Verify the Contract

```bash
forge verify-contract --chain-id 11155111 \
--watch --etherscan-api-key $ETHERSCAN_API_KEY \
--compiler-version $SOLC_VERSION \
$LIGHT_CLIENT_CONTRACT_ADDRESS \
contracts/src/LightClient.sol:LightClient
```

3. Inform Etherscan that it's a Proxy When the proxy is deployed, go to Etherscan. Go to Contract > Code > More Options
   and select the 'is this a proxy?' option. You should then be able to interact with the implementation contract via a
   proxy.

## Upgrades

### Upgrading the Fee Contract

#### Without OpenZeppelin Defender

Steps:

1.  Ensure that the salt has been updated in the `.env.contracts` file. The upgrade script retrieves the proxyAddress
    from the previous deployment by reading a file in the following path:

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/FeeContract.s.sol:UpgradeFeeContractScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
--legacy \
--broadcast
```

#### With OpenZeppelin Defender

1.  Ensure that the salt has been updated in the `.env.contracts` file. The upgrade script retrieves the proxyAddress
    from the previous deployment by reading a file in the following path:
    `script/output/defenderDeployments/$CONTRACT_NAME/$CHAIN_ID/$SALT.json`. It knows the salt from a previous
    deployment by reading the `saltHistory.json` file. Run the following command:

```bash
source .env.contracts && \
forge script contracts/script/FeeContract.s.sol:UpgradeFeeContractWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true
```

2. This command requires you to go to OpenZeppelin Defender's UI to see the transaction. Click that transaction which
   opens up the Safe UI where your signers for that Safe multi-sig wallet can confirm the transaction.

The transactions being confirmed are: (i) the deployment of the new fee contract (ii) the execution of the
`upgradeToAndCall` method which updates the implementation contract that the proxy contract is referencing.

### Upgrading the Light Client Contract

Ensure that you update the version in the `getVersion()` method of the latest implementation contract.

Since the LightClient contract uses the PlonkVerifier library, the PlonkVerifier library has to be deployed and then
referenced at deployment time. Thus ensure you've deployed the PlonkVerifier
([see steps below](#deploy-the-plonk-verifier-library-with-defender)) and set the `$PLONK_VERIFIER_ADDRESS` variable in
the command below. Each time modifications are made to the Plonk Verifier, contracts that depend on it such as the Light
Client contract have to be upgraded and should use the new PlonkVerifier contract address as part of the deployment.

Since the LightClient contract uses the PlonkVerifier library, the PlonkVerifier library has to be deployed and then
referenced at deployment time. Thus ensure you've deployed the PlonkVerifier
([see steps below](#deploy-the-plonk-verifier-library-with-defender)) and set the `$PLONK_VERIFIER_ADDRESS` variable in
the command below. Each time modifications are made to the Plonk Verifier, contracts that depend on it such as the Light
Client contract have to be upgraded and should use the new PlonkVerifier contract address as part of the deployment.

#### Without OpenZeppelin Defender

Steps:

1.  Ensure that the salt has been updated in the `.env.contracts` file. The upgrade script retrieves the proxyAddress
    from the previous deployment by reading a file in the following path:

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/LightClient.s.sol:LightClientContractUpgradeScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
--legacy \
--broadcast
```

#### With OpenZeppelin Defender

Steps:

1.  Ensure that the salt has been updated in the `.env.contracts` file. The upgrade script retrieves the proxyAddress
    from the previous deployment by reading a file in the following path:
    `script/output/defenderDeployments/$CONTRACT_NAME/$CHAIN_ID/$SALT.json`. It knows the salt from a previous
    deployment by reading the `saltHistory.json` file. Run the following command:

```bash
source .env.contracts && \
forge script contracts/script/LightClient.s.sol:UpgradeLightClientWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS
```

2. This command requires you to go to OpenZeppelin Defender's UI to see the transaction. Click that transaction which
   opens up the Safe UI where your signers for that Safe multi-sig wallet can confirm the transaction.

The transactions being confirmed are: (i) the deployment of the new fee contract (ii) the execution of the
`upgradeToAndCall` method which updates the implementation contract that the proxy contract is referencing.

## Known Errors

1. Error Parsing ABI for contract Scenario: You ran `just gen-bindings` Example:

```bash
Error:
error parsing abi for contract '_70c760a3e059d83dbf77da7f6778fbc0': couldn't parse ABI string as either human readable (1) or JSON (2):
1. Illegal abi `{`, expected function
2. data did not match any variant of untagged enum JsonContract
error: Recipe `gen-bindings` failed on line 65 with exit code 1
```

This error occurs when build_info is set to true in the foundry.toml configuration. Ensure that this is false or the
foundry profile is set to default when running commands like `just gen-bindings`.

Solution: `export FOUNDRY_PROFILE=default`

# Deploying Upgradable Contracts without OpenZeppelin Defender and without a Safe Multisig Wallet

## LightClient Contract Deployment

```bash
forge script contracts/script/LightClient.s.sol:DeployLightClientContractWithoutMultiSigScript $numBlocksPerEpoch $numInitValidators \
--sig 'run(uint32, uint32)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com
```

## LightClient Contract Upgrade

```bash
forge script contracts/script/UpgradeLightClient.s.sol:UpgradeLightClientScript $admin $mostRecentlyDeployedProxy \
--sig 'run(address, address)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com
```

# Deploy and Upgrade without Defender

Change the $MNEMONIC in the .env file to the one of the admin

To Deploy

```bash
forge script contracts/script/LightClient.s.sol:DeployLightClientContractWithoutMultiSigScript $numBlocksPerEpoch $numInitValidators \
--sig 'run(uint32, uint32)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com\
--broadcast --legacy
```

To Upgrade (assuming it's the same LightClient.sol file being used (pre-mainnet))

```bash
forge script contracts/script/UpgradeSameLightClient.s.sol:UpgradeLightClientScript $mostRecentlyDeployedProxy \
--sig 'run(address)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com \
--broadcast --legacy
```

Note: the `$mnemonicOffset` should be zero by default if address referenced by the `$MNEMONIC` in the `.env` is the
first address in that wallet. Otherwise, please specify the correct `$mnemonicOffset`

# Deploy the Plonk Verifier Library with Defender

The Plonk Verifier contract is not upgradeable and deploying we deploy with defender as part of our workflow so that we
can also deploy it with a multisig wallet. Each time modifications are made to the Plonk Verifier, contracts that depend
on it such as the Light Client contract have to be upgraded and should use the new PlonkVerifier contract address as
part of the deployment.

Ensure that you update the salt, `PLONK_VERIFIER_SALT`, in the `.env.contracts` file before each deployment.

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/PlonkVerifier.s.sol:DeployPlonkVerifierWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com \
--build-info true
```

# Deploy the Plonk Verifier Library without Defender

The Plonk Verifier contract is not upgradeable and deploying we deploy with defender as part of our workflow so that we
can also deploy it with a multisig wallet. Each time modifications are made to the Plonk Verifier, contracts that depend
on it such as the Light Client contract have to be upgraded and should use the new PlonkVerifier contract address as
part of the deployment.

Ensure that you update the salt, `PLONK_VERIFIER_SALT`, in the `.env.contracts` file before each deployment.

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/PlonkVerifier.s.sol:DeployPlonkVerifierScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com \
--build-info true \
--legacy \
--broadcast
```

# Deploying Upgradable Contracts without OpenZeppelin Defender or a Safe Multisig Wallet

## LightClient Contract Deployment

```bash
forge script contracts/script/LightClient.s.sol:DeployLightClientContractWithoutMultiSigScript $numBlocksPerEpoch $numInitValidators \
--sig 'run(uint32, uint32)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com
```

## LightClient Contract Upgrade

```bash
forge script contracts/script/UpgradeLightClient.s.sol:UpgradeLightClientScript $admin $mostRecentlyDeployedProxy \
--sig 'run(address, address)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com
```

# Deploy and Upgrade without Defender

Change the $MNEMONIC in the .env file to the one of the admin

To Deploy

```bash
forge script contracts/script/LightClient.s.sol:DeployLightClientContractWithoutMultiSigScript $numBlocksPerEpoch $numInitValidators \
--sig 'run(uint32, uint32)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com\
--broadcast --legacy
```

To Upgrade (assuming it's the same LightClient.sol file being used (pre-mainnet))

```bash
forge script contracts/script/UpgradeSameLightClient.s.sol:UpgradeLightClientScript $mostRecentlyDeployedProxy \
--sig 'run(address)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com \
--broadcast --legacy
```

Note: the `$mnemonicOffset` should be zero by default if address referenced by the `$MNEMONIC` in the `.env` is the
first address in that wallet. Otherwise, please specify the correct `$mnemonicOffset`

# Deploy the Plonk Verifier Library with Defender

The Plonk Verifier contract is not upgradeable and deploying we deploy with defender as part of our workflow so that we
can also deploy it with a multisig wallet. Each time modifications are made to the Plonk Verifier, contracts that depend
on it such as the Light Client contract have to be upgraded and should use the new PlonkVerifier contract address as
part of the deployment.

Ensure that you update the salt, `PLONK_VERIFIER_SALT`, in the `.env.contracts` file before each deployment.

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/PlonkVerifierWithDefender.s.sol:DeployPlonkVerifierWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com \
--build-info true
```
