import dotenv from "dotenv";
import { ethers } from "ethers";
import SafeApiKit from "@safe-global/api-kit";
import Safe from "@safe-global/protocol-kit";
import { getEnvVar, validateEthereumAddress, createAndSignSafeTransaction, initializeClient } from "./utils";

async function main() {
  dotenv.config();

  try {
    const [orchestratorSigner, ethAdapter] = await initializeClient();

    const chainId = await ethAdapter.getChainId();
    const safeService = new SafeApiKit({ chainId });
    const safeAddress = getEnvVar("SAFE_MULTISIG_ADDRESS");
    validateEthereumAddress(safeAddress);
    const safeSdk = await Safe.create({ ethAdapter, safeAddress });
    const orchestratorSignerAddress = await orchestratorSigner.getAddress();

    const stateHistoryRetentionPeriod = parseInt(getEnvVar("STATE_HISTORY_RETENTION_PERIOD"));
    console.log("Modify the stateHistoryRetentionPeriod to ", stateHistoryRetentionPeriod);

    await proposeSetStateHistoryRetentionTransaction(
      safeSdk,
      safeService,
      orchestratorSignerAddress,
      safeAddress,
      stateHistoryRetentionPeriod,
    );

    console.log(
      `The other owners of the Safe Multisig wallet need to sign the transaction via the Safe UI https://app.safe.global/transactions/queue?safe=sep:${safeAddress}`,
    );
  } catch (error) {
    throw new Error("An error occurred: " + error);
  }
}

/**
 * Function to propose the transaction data for setting the state history retention period
 * @param {string} safeSDK - An instance of the Safe SDK
 * @param {string} safeService - An instance of the Safe Service
 * @param {string} signerAddress - The address of the address signing the transaction
 * @param {string} safeAddress - The address of the Safe multisig wallet
 * @param {number} stateHistoryRetentionPeriod - The state history retention period in seconds
 */
export async function proposeSetStateHistoryRetentionTransaction(
  safeSDK: Safe,
  safeService: SafeApiKit,
  signerAddress: string,
  safeAddress: string,
  stateHistoryRetentionPeriod: number,
) {
  // Define the ABI of the function to be called
  const abi = ["function setstateHistoryRetentionPeriod(uint32)"];

  // Encode the function call with the provided stateHistoryRetentionPeriod
  const data = new ethers.Interface(abi).encodeFunctionData("setstateHistoryRetentionPeriod", [
    stateHistoryRetentionPeriod,
  ]);

  const contractAddress = getEnvVar("LIGHT_CLIENT_CONTRACT_PROXY_ADDRESS");
  validateEthereumAddress(contractAddress);

  // Create & Sign the Safe Transaction Object
  const { safeTransaction, safeTxHash, senderSignature } = await createAndSignSafeTransaction(
    safeSDK,
    contractAddress,
    data,
  );

  // Propose the transaction which can be signed by other owners via the Safe UI
  await safeService.proposeTransaction({
    safeAddress: safeAddress,
    safeTransactionData: safeTransaction.data,
    safeTxHash: safeTxHash,
    senderAddress: signerAddress,
    senderSignature: senderSignature.data,
  });
}

main();
