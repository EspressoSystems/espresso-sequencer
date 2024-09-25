import dotenv from "dotenv";
import { ethers } from "ethers";
import { EthersAdapter, SafeAccountConfig, SafeFactory } from "@safe-global/protocol-kit";
import { getEnvVar, getSigner, getSignersListFromEnv } from "./utils";

/**
 * This script deploys a Safe Multisig Wallet using the Safe SDK
 * It's especially useful when you want to deploy to a network that's not supported by the Safe UI
 */
async function main() {
  dotenv.config();
  try {
    // Initialize web3 provider using the RPC URL from environment variables
    const rpcUrl = getEnvVar("RPC_URL");
    const web3Provider = new ethers.JsonRpcProvider(rpcUrl);

    // Get the signer, this signer must be one of the signers on the Safe Multisig Wallet
    const orchestratorSigner = getSigner(web3Provider);
    const orchestratorSignerAddress = await orchestratorSigner.getAddress();
    console.log(orchestratorSignerAddress);

    // Set up Eth Adapter with ethers and the signer
    const ethAdapter = new EthersAdapter({
      ethers,
      signerOrProvider: orchestratorSigner,
    });

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
