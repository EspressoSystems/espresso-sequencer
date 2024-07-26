# Proposing Multisig Transactions via the Safe SDK

The [Safe SDK](https://github.com/safe-global/safe-core-sdk/blob/main/guides/integrating-the-safe-core-sdk.md) is being
used to propose transactions that only the Safe multisig admin wallet can perform. The proposer of these transactions is
also part of the multisig wallet but is used to orchestrate the process. E.g. If you require 3 of 5 trusted signers to
sign a transaction, then the multisig wallet should require 4 of 5 signers where the 4th signer is the orchestrator
wallet.

## Set Permissioned Prover

To enable the permissioned prover on the light client contract, ensure that the following environment variables are set
in the `.env` file:

- `SEPOLIA_RPC_URL`
- `SAFE_ORCHESTRATOR_PRIVATE_KEY`
- `SAFE_MULTISIG_ADDRESS`
- `APPROVED_PROVER_ADDRESS`
- `LIGHT_CLIENT_PROXY_CONTRACT_ADDRESS`

Assuming you're in the root folder, run the following command:

```bash
ts-node contracts/script/multisigTransactionProposals/safeSDK/modifyProverModeProposal.ts setProver
```

Open the the URL shown in the console to sign the transaction in the Safe UI.

Once successful, all signers will see a transaction request on the SAFE UI e.g.
`https://app.safe.global/transactions/queue?safe=$SAFE_MULTISIG_ADDRESS`

Once the transaction has been signed by all signers and executed by one, you should be able to go to the light client
proxy and read the permissioned prover address on etherscan.

## Disable Permissioned Prover

To disable the permissioned prover on the light client contract, ensure that the following environment variables are set
in the `.env` file:

- `SEPOLIA_RPC_URL`
- `SAFE_ORCHESTRATOR_PRIVATE_KEY`
- `SAFE_MULTISIG_ADDRESS`
- `LIGHT_CLIENT_PROXY_CONTRACT_ADDRESS`

Assuming you're in the root folder, run the following command:

```bash
ts-node contracts/script/multisigTransactionProposals/safeSDK/modifyProverModeProposal.ts disableProver
```

Once successful, all signers will see a transaction request on the SAFE UI
`https://app.safe.global/transactions/queue?safe=$SAFE_MULTISIG_ADDRESS`

Once the transaction has been signed by all signers and executed by one, you should be able to go to the light client
proxy and read the permissioned prover address. It will be equal to the 0 ETH address (address(0)).

## Demonstrating the setPermissionedProver workflow

1. Follow the steps in the deployment script [readme](../../contracts/script/README.md) to set up OpenZeppelin Defender,
   a Multisig Wallet and deploy the Light Client contract
2. Set the environment variables mentioned in the section, [Set Permissioned Prover](#set-permissioned-prover)
3. Run the `ts-node` command as mentioned in the section, [Set Permissioned Prover](#set-permissioned-prover)

## Demonstrating the disablePermissionedProver workflow

1. Follow the steps in the deployment script [readme](../../contracts/script/README.md) to set up OpenZeppelin Defender,
   a Multisig Wallet and deploy the Light Client contract
2. Set the environment variables mentioned in the section, [Disable Permissioned Prover](#disable-permissioned-prover)
3. Run the `ts-node` command as mentioned in the section, [Disable Permissioned Prover](#disable-permissioned-prover)

## Testing

### Testing Safe Multisig Wallets

The Safe Transaction Service requires a live network available for testing and the current service only supports mainnet
and testnets such as Sepolia. The Safe Wallet UI only works with public networks that they support and to customize it
for a private EVM network, read their [docs](https://help.safe.global/en/articles/40795-supported-networks) for more
info. It's non-trivial to set up Safe for private networks and therefore time has not been allocated to do so at this
stage.

### Testing the utils

Testing safeSDK/utils.ts

```bash
yarn jest contracts/script/multisigTransactionProposals/tests/utils.test.ts
```

OR

```bash
yarn jest
```
