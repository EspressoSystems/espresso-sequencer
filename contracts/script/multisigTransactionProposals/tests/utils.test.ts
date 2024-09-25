import { createSafeTransactionData, getEnvVar, getSignersListFromEnv, validateEthereumAddress } from "../safeSDK/utils";

// Mocking process.argv
const originalArgv = process.argv;
const validETHAddress = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

describe("environment tests", () => {
  afterEach(() => {
    process.argv = originalArgv; // Reset argv after each test
  });

  it("should throw an error if RPC_URL environment variable is not set", () => {
    process.env.RPC_URL = "";
    expect(() => getEnvVar("RPC_URL")).toThrow();
  });

  it("should return the rpc url if RPC_URL environment variable is set", () => {
    process.env.RPC_URL = "http://rpc";
    const result = getEnvVar("RPC_URL");
    expect(result).toEqual("http://rpc");
  });

  it("should return the signers list of one if there is only one signer in the SAFE_SIGNERS_LIST environment variable", () => {
    process.env.SAFE_SIGNERS_LIST = "address1";
    const result = getSignersListFromEnv("SAFE_SIGNERS_LIST");
    expect(result).toEqual(["address1"]);
  });

  it("should return the signers list of three if there are three signers in the SAFE_SIGNERS_LIST environment variable", () => {
    process.env.SAFE_SIGNERS_LIST = "address1,address2,address3";
    const result = getSignersListFromEnv("SAFE_SIGNERS_LIST");
    expect(result).toEqual(["address1", "address2", "address3"]);
  });

  it("should throw an error if SAFE_SIGNERS_LIST environment variable has duplicates", () => {
    process.env.SAFE_SIGNERS_LIST = "address1,address1";
    expect(() => getSignersListFromEnv("SAFE_SIGNERS_LIST")).toThrow();
  });
});

describe("createSafeTransactionData", () => {
  test("should throw an error if the destination address is not specified", () => {
    expect(() => {
      createSafeTransactionData("", "0x", "0");
    }).toThrow("must specify destination address");
  });

  test("should throw an error if the address is not valid", () => {
    expect(() => {
      createSafeTransactionData("0x123", "0x", "0");
    }).toThrow("Invalid Ethereum address format");
  });

  test("should throw an error if both data and value are empty", () => {
    expect(() => {
      createSafeTransactionData(validETHAddress, "", "");
    }).toThrow("Either the contract data or value to be sent must be specified");
  });

  test("should return safe transaction data when valid parameters are provided", () => {
    const result = createSafeTransactionData(validETHAddress, "0x", "100");
    expect(result).toEqual({
      to: validETHAddress,
      data: "0x",
      value: "100",
    });
  });

  test("should return safe transaction data when only data is provided", () => {
    const result = createSafeTransactionData(validETHAddress, "0x", "");
    expect(result).toEqual({
      to: validETHAddress,
      data: "0x",
      value: "",
    });
  });

  test("should return safe transaction data when only value is provided", () => {
    const result = createSafeTransactionData(validETHAddress, "", "100");
    expect(result).toEqual({
      to: validETHAddress,
      data: "",
      value: "100",
    });
  });
});

describe("validateEthereumAddress", () => {
  test("should return true for a valid Ethereum address", () => {
    expect(() => {
      validateEthereumAddress(validETHAddress);
    }).not.toThrow();
  });

  test("should throw an error for an invalid Ethereum address", () => {
    expect(() => {
      validateEthereumAddress("0xInvalidEthereumAddress");
    }).toThrow("Invalid Ethereum address format");
  });

  test("should throw an error for an empty string", () => {
    expect(() => {
      validateEthereumAddress("");
    }).toThrow("Invalid Ethereum address format");
  });
});
