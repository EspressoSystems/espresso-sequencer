# Table of Contents

- [Deploying Upgradeable Smart Contracts](#deploying-upgradeable-smart-contracts)
  - [Prerequisites](#prerequisites)
  - [The Fee Contract](#deploying-the-fee-contract)
  - [The Light Client Contract](#deploying-the-light-client-contract)
- [Upgrading Upgradeable Smart Contracts](#upgrading-upgradeable-smart-contracts)
  - [The Fee Contract](#upgrading-the-fee-contract)
  - [The Light Client Contract](#2-upgrade-the-lightclient-contract)
- [Deploying Upgradeable Contracts without a Multisig Admin](#deploying-upgradable-contracts-without-a-safe-multisig-wallet-admin)
- [Deploying the Plonk Verifier](#deploy-the-plonk-verifier-library)
- [Solutions to Known Errors](#known-errors)

# Deploying Upgradeable Smart Contracts

## Prerequisites

1. **Create a Multisig Wallet**  
   Use [Safe](https://app.safe.global/welcome/accounts) to create a multisig wallet on the network you'd like to deploy
   to.
2. Install `npx` e.g. `brew install npx`
3. **Enter Nix Shell**  
   In the home folder of this repo, start a nix shell by entering `nix-shell` in the terminal.
4. **Compile Contracts (if necessary)** If the contracts have never been compiled, run `forge build`.
5. **Set Environment Variables**  
   Set the values for
   - `STATE_HISTORY_RETENTION_PERIOD`
   - `NUM_INIT_VALIDATORS`
   - `FEE_CONTRACT_ORIGINAL_NAME`
   - `LIGHT_CLIENT_ORIGINAL_CONTRACT_NAME` in the `.env.contracts` file.

### If Not Using OpenZeppelin Defender (current method)

1. **Set Deployment Values**  
   Set the values for `DEPLOYER_MNEMONIC` and `DEPLOYER_MNEMONIC_OFFSET` in the `.env.contracts` file.

### If Using OpenZeppelin Defender

1. **Create an Approval Process**  
   Create an Approval Process that requires the multisig wallet you created above by navigating to
   `Manage > Approval Processes` in OpenZeppelin Defender.

   - Enter a name for your approval process.
   - Enter the multisig address shown in the Safe UI.
   - Enter one of the multisig owner addresses.
   - Save the changes.

2. **Create a Deployment Environment**  
   In OpenZeppelin Defender, create a deployment environment by clicking on "Setup" in the
   [deploy tab](https://defender.openzeppelin.com/v2/#/deploy).
   - Use "Test Environment" for deploying to testnets (e.g., Sepolia) and "Production Environment" for mainnet.
   - Choose a network.
   - Select the approval process created in Step 1.
   - Save the `DEFENDER_SECRET` ("Team Secret key") and `DEFENDER_KEY` ("Team API Key") shown at the end of this step
     into the `.env.contracts` file. These keys won't be available later.

## Deployments

## Deploying the Fee Contract

### 1. Deploy

#### Without OpenZeppelin Defender (current method)

1. Run the following command in the home directory:

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

#### Without OpenZeppelin Defender and with a Ledger Hardware Wallet

- set the `DEPLOYER_HARDWARE_WALLET_ADDRESS` in `.env.contracts`

1. Run the following command in the home directory:

```bash
   source .env.contracts && \
   forge clean && \
   forge script contracts/script/FeeContract.s.sol:DeployFeeContractWithHDWalletScript $SAFE_MULTISIG_ADDRESS \
   --sig 'run(address)' \
   --ffi \
   --rpc-url https://ethereum-sepolia.publicnode.com  \
   --build-info true \
   --legacy \
   --ledger \
   --broadcast
```

#### With OpenZeppelin Defender

1. Run the following command in the home directory:

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/FeeContract.s.sol:DeployFeeContractWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
```

2. **View Submitted Transactions**: Go to the [deploy](https://defender.openzeppelin.com/v2/#/deploy) tab OpenZeppelin
   Defender's UI and click on the current environment to see the transaction. The transaction should be visible with
   status "SUBMITTED". The page may need to be refreshed a few times. It occasionally may take minutes for transactions
   to appear.
3. **Sign & Execute Transactions**: Click that transaction, then "Open in Safe App" which opens up the Safe UI where
   your signers for that Safe multi-sig wallet can confirm the transaction. The two transactions to be confirmed are:
   (i) deployment of implementation contract (ii) deployment of proxy contract.
   - If the transaction looks correct, each signer clicks "confirm".
   - once we have all signatures, any signer can submit the transactions to the blockchain by clicking, "Execute".
4. Refresh the OpenZeppelin Defender UI to see the transaction for the deployment of the proxy. For which you'll need to
   repeat steps 2 & 3. You may need to refresh the OpenZeppelin Defender "deploy" tab a few times until the second
   transaction appears.

### 2. Contract Verification

Verify the Implementation contract on Etherscan (Use another window as step would not have completed yet)

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

### 3. Inform Etherscan about your proxy

Inform Etherscan that it's a Proxy When the proxy is deployed, go to Etherscan. Go to Contract > Code > More Options and
select the `is this a proxy?` option. You should then be able to interact with the implementation contract via a proxy.

---

## Deploying the Light Client Contract

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

The Light Client contract is currently in permissioned mode so please follow through steps 4 to ensure that the
`permissionedProver` has been set.

### Prerequisites:

- Deploy the PlonkVerifier ([see steps below](#deploy-the-plonk-verifier-library-with-defender)
- Ensure the following are in the `.env.contracts` file.
  - `RPC_URL`
  - `SAFE_MULTISIG_ADDRESS`
  - `PLONK_VERIFIER_ADDRESS`
  - `DEPLOYER_MNEMONIC`
  - `DEPLOYER_MNEMONIC_OFFSET`

### 1. Deploy

#### Without OpenZeppelin Defender (current method)

1. Run the following command in the home directory:

```bash
   source .env.contracts && \
   forge clean && \
   forge script contracts/script/LightClient.s.sol:DeployLightClientScript $NUM_INIT_VALIDATORS $STATE_HISTORY_RETENTION_PERIOD $SAFE_MULTISIG_ADDRESS \
   --sig 'run(uint32, uint32, address)' \
   --ffi \
   --rpc-url https://ethereum-sepolia.publicnode.com  \
   --libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS \
   --build-info true \
   --legacy \
   --broadcast
```

#### Without OpenZeppelin Defender and with a Ledger Hardware Wallet

1. Run the following command in the home directory:

```bash
   source .env.contracts && \
   forge clean && \
   forge script contracts/script/LightClient.s.sol:DeployLightClientWithHDWalletScript $NUM_INIT_VALIDATORS $STATE_HISTORY_RETENTION_PERIOD $SAFE_MULTISIG_ADDRESS \
   --sig 'run(uint32, uint32, address)' \
   --ffi \
   --rpc-url https://ethereum-sepolia.publicnode.com  \
   --libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS \
   --build-info true \
   --legacy \
   --ledger \
   --broadcast
```

#### With OpenZeppelin Defender

1. Run the following command in the home directory:

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/LightClient.s.sol:DeployLightClientDefenderScript $STATE_HISTORY_RETENTION_PERIOD \
--sig 'run(uint32)' \
--ffi --rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS
```

### 2. Verify the Contract

```bash
forge verify-contract --chain-id 11155111 \
--watch --etherscan-api-key $ETHERSCAN_API_KEY \
--compiler-version $SOLC_VERSION \
$LIGHT_CLIENT_CONTRACT_ADDRESS \
contracts/src/LightClient.sol:LightClient \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS
```

### 3. Inform Etherscan about your proxy

Inform Etherscan that it's a Proxy When the proxy is deployed, go to Etherscan. Go to Contract > Code > More Options and
select the `is this a proxy?` option. You should then be able to interact with the implementation contract via a proxy.

### 4. Set Permissioned Prover

To enable the permissioned prover on the light client contract, ensure that the following environment variables are set
in the `.env.contracts` file:

- `SAFE_ORCHESTRATOR_PRIVATE_KEY`
- `APPROVED_PROVER_ADDRESS`
- `LIGHT_CLIENT_PROXY_CONTRACT_ADDRESS`

Assuming you're in the root folder, run the following command:

```bash
source .env.contracts && \
ts-node contracts/script/multisigTransactionProposals/safeSDK/modifyProverModeProposal.ts setProver
```

Open the URL shown in the console to sign the transaction in the Safe UI.

Once successful, all signers will see a transaction request on the SAFE UI e.g.
`https://app.safe.global/transactions/queue?safe=$SAFE_MULTISIG_ADDRESS`

Once the transaction has been signed by all signers and executed by one, you should be able to go to the light client
proxy and read the permissioned prover address on etherscan.

---

<br/>
<br/>

# Upgrading Upgradeable Smart Contracts

## Upgrading the Fee Contract

### Without OpenZeppelin Defender (current method)

1. Run the following command in the home directory:

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

### With OpenZeppelin Defender

1.  Ensure that the salt has been updated in the `.env.contracts` file. The upgrade script retrieves the proxyAddress
    from the previous deployment by reading a file in the following path:
    `script/output/defenderDeployments/$CONTRACT_NAME/$CHAIN_ID/$SALT.json`. It knows the salt from a previous
    deployment by reading the `saltHistory.json` file. Run the following command:

2.  Run the following command in the home directory:

```bash
source .env.contracts && \
forge script contracts/script/FeeContract.s.sol:UpgradeFeeContractWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true
```

3. **Sign and Execute the transaction**: This command requires you to go to OpenZeppelin Defender's UI to see the
   transaction. Click that transaction which opens up the Safe UI where your signers for that Safe multi-sig wallet can
   confirm the transaction.

The transactions being confirmed are: (i) the deployment of the new fee contract (ii) the execution of the
`upgradeToAndCall` method which updates the implementation contract that the proxy contract is referencing.

## Upgrading the Light Client Contract

Ensure that you update the version in the `getVersion()` method of the latest implementation contract.

Since the LightClient contract uses the PlonkVerifier library, the PlonkVerifier library has to be deployed and then
referenced at deployment time. Thus ensure you've deployed the PlonkVerifier
([see steps below](#deploy-the-plonk-verifier-library-with-defender)) and set the `$PLONK_VERIFIER_ADDRESS` variable in
the command below. Each time modifications are made to the Plonk Verifier, contracts that depend on it such as the Light
Client contract have to be upgraded and should use the new PlonkVerifier contract address as part of the deployment.

### Without OpenZeppelin Defender (current method)

1. Run the following command in the home directory:

```bash
source .env.contracts && \
forge clean && \
forge script contracts/script/LightClient.s.sol:LightClientContractUpgradeScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS \
--build-info true \
--legacy \
--broadcast
```

### With OpenZeppelin Defender

1.  Ensure that the salt has been updated in the `.env.contracts` file. The upgrade script retrieves the proxyAddress
    from the previous deployment by reading a file in the following path:
    `script/output/defenderDeployments/$CONTRACT_NAME/$CHAIN_ID/$SALT.json`. It knows the salt from a previous
    deployment by reading the `saltHistory.json` file. Run the following command:

2.  Run the following command in the home directory:

```bash
source .env.contracts && \
forge script contracts/script/LightClient.s.sol:UpgradeLightClientWithDefenderScript \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com  \
--build-info true \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS
```

3. **Sign and Execute the transaction**: This command requires you to go to OpenZeppelin Defender's UI to see the
   transaction. Click that transaction which opens up the Safe UI where your signers for that Safe multi-sig wallet can
   confirm the transaction.

The transactions being confirmed are: (i) the deployment of the new fee contract (ii) the execution of the
`upgradeToAndCall` method which updates the implementation contract that the proxy contract is referencing.

# Deploying Upgradable Contracts without a Safe Multisig Wallet Admin

Use these instructions for staging deployments only. Ensure that you have set the following variables in the `.env`
file.

- `MNEMONIC`
- `MNEMONIC_OFFSET`

## 1. Deploy the LightClient Contract

```bash
forge script contracts/script/LightClient.s.sol:DeployLightClientContractWithoutMultiSigScript $NUM_INIT_VALIDATORS $STATE_HISTORY_RETENTION_PERIOD \
--sig 'run(uint32, uint32)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com
```

## 2. Upgrade the LightClient Contract

```bash
forge script contracts/script/LightClient.s.sol:UpgradeLightClientWithoutMultisigAdminScript $LIGHT_CLIENT_CONTRACT_PROXY_ADDRESS \
--sig 'run(address)' \
--ffi \
--rpc-url https://ethereum-sepolia.publicnode.com
```

_Note_: the `$MNEMONIC_OFFSET` should be zero by default if address referenced by the `$MNEMONIC` in the `.env` is the
first address in that wallet. Otherwise, please specify the correct `$MNEMONIC_OFFSET`

## 3. Verify the Contract

```bash
forge verify-contract --chain-id 11155111 \
--watch --etherscan-api-key $ETHERSCAN_API_KEY \
--compiler-version $SOLC_VERSION \
$LIGHT_CLIENT_CONTRACT_ADDRESS \
contracts/src/LightClient.sol:LightClient \
--libraries contracts/src/libraries/PlonkVerifier.sol:PlonkVerifier:$PLONK_VERIFIER_ADDRESS
```

## 4. [Inform Etherscan about your proxy](#3-inform-etherscan-about-your-proxy)

## 5. [Set The Permissioned Prover](#4-set-permissioned-prover)

Inform Etherscan that it's a Proxy When the proxy is deployed, go to Etherscan. Go to Contract > Code > More Options and
select the `is this a proxy?` option. You should then be able to interact with the implementation contract via a proxy.

# Deploy the Plonk Verifier Library

## Without Defender (current method)

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

## With OpenZepplin Defender

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

# Known Errors

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
