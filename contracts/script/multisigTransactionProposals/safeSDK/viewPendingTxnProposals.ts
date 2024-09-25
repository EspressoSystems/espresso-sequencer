import dotenv from "dotenv";
import { getEnvVar, initializeClient } from "./utils";
import SafeApiKit from "@safe-global/api-kit";

/**
 * This script allows you to view the pending transaction proposals
 */
async function main() {
  dotenv.config();
  try {
    // Initialize the client
    const [_, ethAdapter] = await initializeClient();

    // Set up the Safe Service
    const chainId = await ethAdapter.getChainId();
    const safeService = new SafeApiKit({ chainId });
    const safeAddress = getEnvVar("SAFE_MULTISIG_ADDRESS");

    // Get and Print Pending Transaction List
    console.log(`Getting transactions for ${safeAddress}`);
    await new Promise((resolve) => setTimeout(resolve, 1000)); // wait one second so that user can read the above console log
    const pendingTransactions = (await safeService.getPendingTransactions(safeAddress)).results;
    if (pendingTransactions.length > 0) {
      console.log(`Here are the pending transaction(s):`);
      console.log(JSON.stringify(pendingTransactions, null, 2));
    }
    console.log(`There are ${pendingTransactions.length} pending transaction(s).`);
  } catch (e) {
    console.error("An error occurred while fetching pending transactions:", e);
  }
}

main();
