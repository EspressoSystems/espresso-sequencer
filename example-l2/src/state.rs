use commit::{Commitment, Committable};
use ethers::abi::Address;
use hotshot_query_service::availability::{BlockHash, BlockQueryData};
use sequencer::SeqTypes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::error::RollupError;
use crate::prover::Proof;
use crate::transaction::SignedTransaction;
use crate::RollupVM;

pub type Amount = u64;
pub type Nonce = u64;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Account {
    balance: Amount,
    nonce: Nonce,
}

#[derive(Debug, Clone)]
pub struct State {
    // Account state, represented as a BTreeMap so that we can obtain a canonical serialization of the data structure for the state commitment
    // A live rollup would likely represent accounts as a Sparse Merkle Tree instead of a BTreeMap.
    // Rollup clients would then be able to use merkle proofs to authenticate a subset of user balances
    // without knowledge of the entire account state. Such "light clients" are less constrained by bandwidth
    // because they do not need to constantly sync up with a full node.
    accounts: BTreeMap<Address, Account>,
    block_hash: Option<BlockHash<SeqTypes>>, // Hash of most recent hotshot consensus block
    prev_state_commitment: Option<Commitment<State>>, // Previous state commitment, used to create a chain linking state committments
}

impl Committable for State {
    fn commit(&self) -> Commitment<State> {
        let serialized_accounts =
            serde_json::to_string(&self.accounts).expect("Serialization should not fail");

        commit::RawCommitmentBuilder::new("State Commitment")
            .array_field(
                "block_hash",
                &self
                    .block_hash
                    .iter()
                    .cloned()
                    .map(BlockHash::<SeqTypes>::from)
                    .collect::<Vec<_>>(),
            )
            .array_field(
                "prev_state_commitment",
                &self
                    .prev_state_commitment
                    .iter()
                    .cloned()
                    .map(Commitment::<State>::from)
                    .collect::<Vec<_>>(),
            )
            .var_size_field("accounts", serialized_accounts.as_bytes())
            .finalize()
    }
}

impl State {
    /// Create new VM state seeded with some initial balances
    pub fn from_initial_balances(
        initial_balances: impl IntoIterator<Item = (Address, Amount)>,
    ) -> Self {
        let mut accounts = BTreeMap::new();
        for (addr, amount) in initial_balances.into_iter() {
            accounts.insert(
                addr,
                Account {
                    balance: amount,
                    nonce: 0,
                },
            );
        }
        State {
            accounts,
            block_hash: None,
            prev_state_commitment: None,
        }
    }

    /// If the transaction is valid, transition the state and return the new state with updated balances.
    ///
    /// A transaction is valid iff
    /// 1) The signature on the transaction
    /// 2) The nonce of the transaction is greater than the sender nonce (this prevent replay attacks)
    /// 3) The sender has a high enough balance to cover the transfer amount
    pub fn apply_transaction(
        &mut self,
        transaction: &SignedTransaction,
    ) -> Result<(), RollupError> {
        // 1)
        let sender = transaction.recover()?;
        let destination = transaction.transaction.destination;
        let next_nonce = transaction.transaction.nonce;
        let transfer_amount = transaction.transaction.amount;
        let Account {
            nonce: prev_nonce,
            balance: sender_balance,
        } = self
            .accounts
            .get_mut(&sender)
            .ok_or(RollupError::InsufficientBalance { address: sender })?;

        // 2)
        if next_nonce != *prev_nonce + 1 {
            return Err(RollupError::InvalidNonce);
        }

        // 3)
        if transfer_amount > *sender_balance {
            return Err(RollupError::InsufficientBalance { address: sender });
        }

        // Transaction is valid, return the updated state
        *sender_balance -= transfer_amount;
        *prev_nonce = next_nonce;
        let Account {
            balance: destination_balance,
            ..
        } = self
            .accounts
            .entry(destination)
            .or_insert(Account::default());
        *destination_balance += transfer_amount;

        Ok(())
    }

    /// Fetch the balance of an address
    pub fn get_balance(&self, address: &Address) -> Amount {
        self.accounts
            .get(address)
            .map(|account| account.balance)
            .unwrap_or(0)
    }

    pub(crate) async fn execute_block(&mut self, block: &BlockQueryData<SeqTypes>) -> Proof {
        let state_commitment = self.commit();
        for txn in block.block().vm_transactions(&RollupVM) {
            let res = self.apply_transaction(&txn);
            if let Err(err) = res {
                // TODO: more informative logging
                tracing::error!("Transaction invalid: {}", err)
            } else {
                tracing::info!("Transaction applied")
            }
        }
        self.block_hash = Some(block.hash());
        self.prev_state_commitment = Some(state_commitment);

        Proof::generate(block, self.commit(), self.prev_state_commitment.unwrap())
    }
}
#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;

    use ethers::signers::{LocalWallet, Signer};

    use super::*;
    #[async_std::test]
    async fn smoke_test() {
        let mut rng = rand::thread_rng();
        let alice = LocalWallet::new(&mut rng);
        let bob = LocalWallet::new(&mut rng);
        let seed_data = [(alice.address(), 100), (bob.address(), 100)];
        let mut state = State::from_initial_balances(seed_data);
        let mut transaction = Transaction {
            amount: 110,
            destination: bob.address(),
            nonce: 1,
        };

        // Try to overspend
        let mut signed_transaction = SignedTransaction::new(transaction.clone(), &alice).await;
        let err = state
            .clone()
            .apply_transaction(&signed_transaction)
            .expect_err("Invalid transaction should throw error.");
        assert_eq!(
            err,
            RollupError::InsufficientBalance {
                address: alice.address()
            }
        );

        // Now spend an valid amount
        transaction.amount = 50;
        signed_transaction = SignedTransaction::new(transaction, &alice).await;
        state
            .apply_transaction(&signed_transaction)
            .expect("Valid transaction should transition state");
        let bob_balance = state.get_balance(&bob.address());
        assert_eq!(bob_balance, 150);

        // Now try to replay the transaction
        let err = state
            .apply_transaction(&signed_transaction)
            .expect_err("Invalid transaction should throw error.");
        assert_eq!(err, RollupError::InvalidNonce);
    }
}
