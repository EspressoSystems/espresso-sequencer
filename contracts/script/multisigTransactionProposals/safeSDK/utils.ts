import { ethers } from "ethers"; // Import ethers from the ethers library
import { LedgerSigner } from "@ethers-ext/signer-ledger";
import HIDTransport from "@ledgerhq/hw-transport-node-hid";

/**
 * Function to check if a given string is a valid Ethereum address
 * @param {string} address - The Ethereum address to validate
 * @throws {Error} - Throws an error if the address is invalid and doesn't follow Ethereum address standards
 */
export function validateEthereumAddress(address: string) {
  if (!ethers.isAddress(address)) {
    throw new Error(`Invalid Ethereum address format: ${address}`); // Throw an error if the address is invalid
  }
}

/**
 * Function to get the value of an environment variable from the .env file
 * @param {string} name - The name of the environment variable to retrieve
 * @returns {string} - Returns the value of the environment variable
 */
export function getEnvVar(name: string): string {
  const value = process.env[name]; // Retrieve the environment variable value
  // Check if the environment variable is undefined or empty
  if (value === undefined || value === "") {
    throw new Error(`Environment variable ${name} is not set`); // Throw an error if the environment variable is not set
  }
  return value; // Return the value of the environment variable
}

/**
 * Function to create safe transaction data to be used with the safe SDK
 * @param {string} to - The destination address for the transaction
 * @param {string} data - The contract data to be sent
 * @param {string} value - The value to be sent
 * @returns {object} - Returns the safe transaction data object
 */
export function createSafeTransactionData(to: string, data: string, value: string) {
  // Check if the destination address is specified
  if (to == "") {
    throw new Error("must specify destination address"); // Throw an error if the destination address is not specified
  }
  // Check if both data and value are empty
  if (data == "" && value == "") {
    throw new Error("Either the contract data or value to be sent must be specified"); // Throw an error if both data and value are empty
  }
  validateEthereumAddress(to); // Validate the destination address
  // Create the safe transaction data object
  const safeTransactionData = {
    to: to,
    data: data,
    value: value,
  };
  return safeTransactionData; // Return the safe transaction data object
}

/**
 * Function to check if a given string is a valid Ethereum address
 * @param {string} address - The Ethereum address to validate
 * @throws {Error} - Throws an error if the address is invalid and doesn't follow Ethereum address standards
 */
export function getSigner(web3Provider: ethers.Provider): ethers.Signer {
  let orchestratorSigner;
  const use_hardware_wallet = getEnvVar("USE_HARDWARE_WALLET");
  if (use_hardware_wallet == "true") {
    // Create a signer using the ledger
    orchestratorSigner = new LedgerSigner(HIDTransport, web3Provider);
  } else {
    // Create a signer using the orchestrator's private key and the web3 provider
    orchestratorSigner = new ethers.Wallet(getEnvVar("SAFE_ORCHESTRATOR_PRIVATE_KEY"), web3Provider);
  }

  return orchestratorSigner;
}
