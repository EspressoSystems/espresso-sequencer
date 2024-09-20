import dotenv from "dotenv";
import { ethers } from "ethers";
import { EthersAdapter } from "@safe-global/protocol-kit";
import SafeApiKit from "@safe-global/api-kit";
import Safe from "@safe-global/protocol-kit";
import { getEnvVar, createSafeTransactionData, validateEthereumAddress, getSigner } from "./utils";
const UPGRADE_PROXY_CMD = "upgradeProxy" as const;

// declaring the type returned by the createTransaction method in the safe package locally (since the return type isn't exposed) so that if it's updated, it's reflected here too
type LocalSafeTransaction = Awaited<ReturnType<Safe["createTransaction"]>>;

interface UpgradeData {
  proxyAddress: string;
  implementationAddress: string;
  initData: string;
}

async function main() {
  dotenv.config();

  try {
    const upgradeData: UpgradeData = processCommandLineArguments();

    // Initialize web3 provider using the RPC URL from environment variables
    const web3Provider = new ethers.JsonRpcProvider(getEnvVar("RPC_URL"));

    // Get the signer, this signer must be one of the signers on the Safe Multisig Wallet
    const orchestratorSigner = getSigner(web3Provider);

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

    await proposeUpgradeTransaction(safeSdk, safeService, orchestratorSignerAddress, safeAddress, upgradeData);

    console.log(
      `The other owners of the Safe Multisig wallet need to sign the transaction via the Safe UI https://app.safe.global/transactions/queue?safe=sep:${safeAddress}`,
    );
  } catch (error) {
    throw new Error("An error occurred: " + error);
  }
}

function processCommandLineArguments(): UpgradeData {
  const args = process.argv.slice(2); // Remove the first two args (node command and script name)
  if (args.length === 0) {
    console.log("No commands provided.");
    throw new Error(
      `No commands provided, enter ${UPGRADE_PROXY_CMD} followed by the new implementation contract address and its init data`,
    );
  } else if (args.length < 3) {
    throw new Error(
      `Incorrect number of arguments, enter ${UPGRADE_PROXY_CMD} followed by the new implementation contract address and its init data`,
    );
  }

  const command = args[0];
  if (command !== UPGRADE_PROXY_CMD) {
    throw new Error(`Only ${UPGRADE_PROXY_CMD} command is supported.`);
  }
  const proxyAddress = args[1];
  const implementationAddress = args[2];
  validateEthereumAddress(implementationAddress);
  const initData = args[3];

  return { proxyAddress: proxyAddress, implementationAddress: implementationAddress, initData: initData };
}

/**
 * Function to propose the transaction data for upgrading the new implementation
 * @param {string} safeSDK - An instance of the Safe SDK
 * @param {string} safeService - An instance of the Safe Service
 * @param {string} signerAddress - The address of the address signing the transaction
 * @param {string} safeAddress - The address of the Safe multisig wallet
 * @param {string} newContractAddress - The address of the new implementation
 * @param {string} initData - The initialization data for the new implementation, may be '0x' if there is not initialization required
 */
export async function proposeUpgradeTransaction(
  safeSDK: Safe,
  safeService: SafeApiKit,
  signerAddress: string,
  safeAddress: string,
  upgradeData: UpgradeData,
) {
  // Prepare the transaction data to upgrade the proxy
  let data = createUpgradeTxData(upgradeData.implementationAddress, upgradeData.initData);

  // Create the Safe Transaction Object
  const safeTransaction = await createSafeTransaction(safeSDK, upgradeData.proxyAddress, data, "0");

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
 * Function to create the transaction data for setting the new implementation
 * @param {string} newContractAddress - The address of the new implementation
 * @param {string} initData - The initialization data for the new implementation
 * @returns {string} - Encoded transaction data
 */
function createUpgradeTxData(newContractAddress: string, initData: string): string {
  // Define the ABI of the function to be called
  const abi = ["function upgradeToAndCall(address,bytes)"];

  // Encode the function call with the new implementation address and its init data
  const data = new ethers.Interface(abi).encodeFunctionData("upgradeToAndCall", [newContractAddress, initData]);
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

  if (getEnvVar("USE_HARDWARE_WALLET")) {
    console.log(`Please sign the message on your connected Ledger device`);
  }

  // Create the safe transaction using the Safe SDK
  const safeTransaction = await safeSDK.createTransaction({ transactions: [safeTransactionData] });

  return safeTransaction;
}

main();
