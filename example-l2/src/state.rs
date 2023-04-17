use commit::{Commitment, Committable};
use ethers::abi::Address;
use hotshot_query_service::availability::BlockHash;
use sequencer::SeqTypes;
use std::collections::HashMap;

use crate::error::RollupError;
use crate::transaction::SignedTransaction;

pub type Amount = u64;
pub type Nonce = u64;

#[derive(Debug, Default, Clone)]
pub struct Account {
    balance: Amount,
    nonce: Nonce,
}

#[derive(Debug, Clone)]
pub struct State {
    accounts: HashMap<Address, Account>,
    block_hash: Option<BlockHash<SeqTypes>>, // Hash of most recent hotshot consensus block
    prev_state_commitment: Option<Commitment<State>>, // Previous state commitment, used to create a chain linking state committments
}

impl Committable for State {
    fn commit(&self) -> Commitment<State> {
        let mut builder = commit::RawCommitmentBuilder::new("State Commitment");

        if let Some(hash) = self.block_hash {
            builder = builder.var_size_field("block_hash", hash.as_ref());
        }

        if let Some(prev_comm) = self.prev_state_commitment {
            builder = builder.var_size_field("prev_state_commitment", prev_comm.as_ref());
        }

        builder.finalize()
    }
}

pub type StateCommitment = Commitment<(Commitment<BlockHash<SeqTypes>>, BlockHash<SeqTypes>)>;

impl State {
    /// Create new VM state seeded with some initial balances
    pub fn from_initial_balances(
        initial_balances: impl IntoIterator<Item = (Address, Amount)>,
    ) -> Self {
        let mut accounts = HashMap::new();
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

    pub fn set_block_hash(&mut self, hash: BlockHash<SeqTypes>) {
        self.block_hash = Some(hash);
    }

    pub fn set_prev_state_commitment(&mut self, commitment: Commitment<Self>) {
        self.prev_state_commitment = Some(commitment);
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
