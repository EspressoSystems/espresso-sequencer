import dotenv from "dotenv";
import { ethers } from "ethers";
import { EthersAdapter } from "@safe-global/protocol-kit";
import SafeApiKit from "@safe-global/api-kit";
import Safe from "@safe-global/protocol-kit";
import { getEnvVar, createSafeTransactionData, validateEthereumAddress } from "./utils";

// declaring the type returned by the createTransaction method in the safe package locally (since the return type isn't exposed) so that if it's updated, it's reflected here too
type LocalSafeTransaction = Awaited<ReturnType<Safe["createTransaction"]>>;

async function main() {
  dotenv.config();

  try {
    // Initialize web3 provider using the RPC URL from environment variables
    const web3Provider = new ethers.JsonRpcProvider(getEnvVar("RPC_URL"));
    // Create a signer using the orchestrator's private key and the web3 provider
    const orchestratorSigner = new ethers.Wallet(getEnvVar("SAFE_ORCHESTRATOR_PRIVATE_KEY"), web3Provider);

    // Set up Eth Adapter with ethers and the signer
    const ethAdapter = new EthersAdapter({
      ethers,
      signerOrProvider: orchestratorSigner,
    });

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
  // Prepare the transaction data to set the stateHistoryRetentionPeriod (seconds)
  let data = createStateHistoryRetentionPeriodTxData(stateHistoryRetentionPeriod);

  const contractAddress = getEnvVar("LIGHT_CLIENT_CONTRACT_PROXY_ADDRESS");
  validateEthereumAddress(contractAddress);

  // Create the Safe Transaction Object
  const safeTransaction = await createSafeTransaction(safeSDK, contractAddress, data, "0");

  // Get the transaction hash and sign the transaction
  const safeTxHash = await safeSDK.getTransactionHash(safeTransaction);

  // Sign the transaction with orchestrator signer that was specified when we created the safeSDK
  const senderSignature = await safeSDK.signHash(safeTxHash);

  // Propose the transaction which can be signed by other owners via the Safe UI
  await safeService.proposeTransaction({
    safeAddress: safeAddress,
    safeTransactionData: safeTransaction.data,
    safeTxHash: safeTxHash,
    senderAddress: signerAddress,
    senderSignature: senderSignature.data,
  });
}

/**
 * Function to create the transaction data for setting the stateHistoryRetentionPeriod
 * @param {number} retention_period - The state history retention period in seconds
 * @returns {string} - Encoded transaction data
 */
function createStateHistoryRetentionPeriodTxData(retention_period: number): string {
  // Define the ABI of the function to be called
  const abi = ["function setstateHistoryRetentionPeriod(uint32)"];

  // Encode the function call with the provided stateHistoryRetentionPeriod
  const data = new ethers.Interface(abi).encodeFunctionData("setstateHistoryRetentionPeriod", [retention_period]);
  return data; // Return the encoded transaction data
}

/**
 * Creates a Safe transaction object
 *
 * @param {Safe} safeSDK - An instance of the Safe SDK
 * @param {string} contractAddress - The address of the contract to interact with
 * @param {string} data - The data payload for the transaction
 * @param {string} value - The value to be sent with the transaction
 * @returns {Promise<any>} - A promise that resolves to the Safe transaction object
 */
async function createSafeTransaction(
  safeSDK: Safe,
  contractAddress: string,
  data: string,
  value: string,
): Promise<LocalSafeTransaction> {
  // Prepare the safe transaction data with the contract address, data, and value
  let safeTransactionData = createSafeTransactionData(contractAddress, data, value);
  console.log("data hex: ", data);
  // Create the safe transaction using the Safe SDK
  const safeTransaction = await safeSDK.createTransaction({ transactions: [safeTransactionData] });

  return safeTransaction;
}

main();
