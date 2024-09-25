import dotenv from "dotenv";
import { SafeAccountConfig, SafeFactory } from "@safe-global/protocol-kit";
import { getEnvVar, getSignersListFromEnv, initializeClient } from "./utils";

/**
 * This script deploys a Safe Multisig Wallet using the Safe SDK
 * It's especially useful when you want to deploy to a network that's not supported by the Safe UI
 */
async function main() {
  dotenv.config();
  try {
    const [orchestratorSigner, ethAdapter] = await initializeClient();

    const orchestratorSignerAddress = await orchestratorSigner.getAddress();
    console.log(`You're deploying with wallet: ${orchestratorSignerAddress}`);

    // Get list of signers
    const signers = getSignersListFromEnv("SAFE_SIGNERS_LIST");
    const signerThreshold = parseInt(getEnvVar("SAFE_SIGNER_THRESHOLD"));

    if (signerThreshold == 0 || signerThreshold > signers.length) {
      throw new Error("Signer threshold should be greater than zero and less than the total number of signers");
    }

    // Set up Safe
    const safeAccountConfig: SafeAccountConfig = {
      owners: signers,
      threshold: signerThreshold,
    };

    const safeFactory = await SafeFactory.create({
      ethAdapter: ethAdapter,
    });

    // Deploy Safe
    console.log("Safe is being deployed.");
    if (getEnvVar("USE_HARDWARE_WALLET") === "true") {
      console.log("Please sign transaction in Hardware Wallet");
    }
    const protocolKitOwner1 = await safeFactory.deploySafe({ safeAccountConfig });
    const safeAddress = await protocolKitOwner1.getAddress();
    console.log(`Your Safe has been deployed: https://sepolia.etherscan.io/address/${safeAddress}`);
    console.log(
      `If the Safe UI supports the network, you can view the wallet here: https://app.safe.global/sep:${safeAddress} `,
    );
  } catch (e) {
    console.log(e);
  }
}

main();
