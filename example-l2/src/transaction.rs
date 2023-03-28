use crate::error::RollupError;
use crate::state::{Amount, Nonce};
use ethers::{
    abi::Address,
    signers::{Signer, Wallet},
    types::Signature,
};
use ethers_core::k256::ecdsa::SigningKey;

#[derive(Clone)]
pub struct Transaction {
    pub amount: Amount,
    pub destination: Address,
    pub nonce: Nonce,
}

impl Transaction {
    pub fn as_bytes(&self) -> Vec<u8> {
        // TODO: serialization
        vec![]
    }
}
pub struct SignedTransaction {
    pub transaction: Transaction,
    signature: Signature,
}

impl SignedTransaction {
    pub fn recover(&self) -> Result<Address, RollupError> {
        let bytes = self.transaction.as_bytes();
        self.signature
            .recover(bytes)
            .map_err(|_| RollupError("signature error".into()))
    }

    pub async fn new(transaction: Transaction, wallet: &Wallet<SigningKey>) -> Self {
        let bytes = transaction.as_bytes();
        let signature = wallet.sign_message(&bytes).await.unwrap();
        Self {
            signature,
            transaction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;
    use ethers::signers::{LocalWallet, Signer};

    use super::*;
    #[async_std::test]
    async fn test_transaction_signature() {
        let mut rng = rand::thread_rng();
        let alice = LocalWallet::new(&mut rng);
        let transaction = Transaction {
            amount: 100,
            destination: alice.address(),
            nonce: 1,
        };
        let signed_transaction = SignedTransaction::new(transaction.clone(), &alice).await;
        let recovered_address = signed_transaction
            .recover()
            .expect("Should recover address");
        assert!(recovered_address == alice.address());
    }
}
