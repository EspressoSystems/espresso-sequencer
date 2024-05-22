import { ethers } from "ethers";
import { EthersAdapter } from "@safe-global/protocol-kit";
import SafeApiKit from "@safe-global/api-kit";
import Safe from "@safe-global/protocol-kit";
import { getEnvVar, createSafeTransactionData, isValidEthereumAddress } from "./utils";

async function main() {
  try {
    /**TODO
     * change from SEPOLIA_RPC_URL to production URL when deploying to production
     */
    // Initialize web3 provider using the RPC URL from environment variables
    const web3Provider = new ethers.JsonRpcProvider(getEnvVar("SEPOLIA_RPC_URL"));
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
    isValidEthereumAddress(safeAddress);
    const safeSdk = await Safe.create({ ethAdapter, safeAddress });

    await proposeDisableProverTransaction(safeSdk, safeService, await orchestratorSigner.getAddress(), safeAddress);

    console.log(
      `The other owners of the Safe Multisig wallet need to sign the transaction via the Safe UI https://app.safe.global/transactions/queue?safe=sep:${safeAddress}`,
    );
  } catch (error) {
    throw new Error("An error occurred: " + error);
  }
}

/**
 * Function to propose the transaction data for disabling permissioned prover mode
 * @param {string} safeSDK - An instance of the Safe SDK
 * @param {string} safeService - An instance of the Safe Service
 * @param {string} signerAddress - The address of the address signing the transaction
 * @param {string} safeAddress - The address of the Safe multisig wallet
 */
export async function proposeDisableProverTransaction(
  safeSDK: Safe,
  safeService: SafeApiKit,
  signerAddress: string,
  safeAddress: string,
) {
  // Prepare the transaction data to disable permissioned prover mode
  let data = createDisablePermissionedProverTxData();

  const contractAddress = getEnvVar("LIGHT_CLIENT_CONTRACT_ADDRESS");
  isValidEthereumAddress(contractAddress);

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
 * Function to create the transaction data for disabling permissioned prover mode
 * @returns {string} - Encoded transaction data
 */
function createDisablePermissionedProverTxData(): string {
  // Define the ABI of the function to be called
  const abi = ["function disablePermissionedProverMode()"];

  // Encode the function call with the provided prover address
  const data = new ethers.Interface(abi).encodeFunctionData("disablePermissionedProverMode", []);
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
): Promise<any> {
  // Prepare the safe transaction data with the contract address, data, and value
  let safeTransactionData = createSafeTransactionData(contractAddress, data, value);

  // Create the safe transaction using the Safe SDK
  const safeTransaction = await safeSDK.createTransaction({ transactions: [safeTransactionData] });

  return safeTransaction;
}

main();
