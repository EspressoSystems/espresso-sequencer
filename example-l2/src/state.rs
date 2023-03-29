use ethers::abi::Address;
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

#[derive(Debug, Default, Clone)]
pub struct State {
    accounts: HashMap<Address, Account>,
}

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
        State { accounts }
    }

    /// If the transaction is valid, transition the state and return the new state with updated balances.
    ///
    /// A transaction is valid iff
    /// 1) The signature on the transaction
    /// 2) The nonce of the transaction is greater than the sender nonce (this prevent replay attacks)
    /// 3) The sender has a high enough balance to cover the transfer amount
    pub fn apply_transaction(
        mut self,
        transaction: &SignedTransaction,
    ) -> Result<State, RollupError> {
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

        Ok(self)
    }

    /// Fetch the balance of an address
    pub fn get_balance(&self, address: &Address) -> Option<Amount> {
        self.accounts.get(address).map(|account| account.balance)
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
        state = state
            .apply_transaction(&signed_transaction)
            .expect("Valid transaction should transition state");
        let bob_balance = state
            .get_balance(&bob.address())
            .expect("Bob should have a balance");
        assert_eq!(bob_balance, 150);

        // Now try to replay the transaction
        let err = state
            .apply_transaction(&signed_transaction)
            .expect_err("Invalid transaction should throw error.");
        assert_eq!(err, RollupError::InvalidNonce);
    }
}
