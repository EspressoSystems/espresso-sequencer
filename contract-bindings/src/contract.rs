#[cfg(test)]
mod tests {
    use crate::bindings::proof_of_efficiency::ProofOfEfficiency;
    use ethers::{
        prelude::{
            coins_bip39::English, Address, Http, Middleware, MnemonicBuilder, Provider, Signer,
            SignerMiddleware, U256,
        },
        types::Bytes,
    };
    use std::sync::Arc;

    #[async_std::test]
    async fn test_bindings() -> Result<(), ()> {
        let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
        let chain_id = provider.get_chainid().await.unwrap().as_u64();

        const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";
        let deployer_wallet = MnemonicBuilder::<English>::default()
            .phrase(TEST_MNEMONIC)
            .build()
            .unwrap()
            .with_chain_id(chain_id);
        let signer = Arc::new(SignerMiddleware::new(provider, deployer_wallet));

        // See https://github.com/0xPolygonHermez/zkevm-node/blob/develop/docs/running_local.md#l1-addresses
        let rollup_address = "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9"
            .parse::<Address>()
            .unwrap();

        let contract = ProofOfEfficiency::new(rollup_address, signer);

        contract
            .force_batch(Bytes::default(), U256::from("1000000000000000000"))
            .await
            .unwrap();

        Ok(())
    }
}
