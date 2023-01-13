#[cfg(test)]
mod tests {
    use crate::bindings::{matic::Matic, proof_of_efficiency::ProofOfEfficiency};
    use ethers::{
        prelude::{
            coins_bip39::English, Address, Http, Middleware, MnemonicBuilder, Provider, Signer,
            SignerMiddleware,
        },
        types::Bytes,
    };
    use std::sync::Arc;

    /// To run this test the zkevm-node needs to be running
    /// ```bash
    /// cd zkevm-node/test
    /// make run
    /// docker-compose logs zkevm-mock-l1-network -f # to see some logs
    /// ```
    #[ignore]
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
        let signer = Arc::new(SignerMiddleware::new(provider.clone(), deployer_wallet));

        // See config file
        // https://github.com/EspressoSystems/zkevm-node/blob/add-nix-shell/test/config/test.node.config.toml#L30
        // because the documentation may be out of date.
        let rollup_address = "0x2279B7A0a67DB372996a5FaB50D91eAA73d2eBe6"
            .parse::<Address>()
            .unwrap();
        let rollup = ProofOfEfficiency::new(rollup_address, signer.clone());

        let matic_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
            .parse::<Address>()
            .unwrap();
        let matic = Matic::new(matic_address, signer.clone());

        let fee = rollup.get_current_batch_fee().await.unwrap();

        matic
            .approve(rollup.address(), fee)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        rollup
            .set_force_batch_allowed(true)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        rollup.force_batch(Bytes::default(), fee).await.unwrap();

        Ok(())
    }
}
