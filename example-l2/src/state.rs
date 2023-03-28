use ethers::abi::Address;
use std::collections::HashMap;

use crate::error::RollupError;
use crate::transaction::SignedTransaction;

pub type Amount = u64;
pub type Nonce = u64;

#[derive(Debug)]
pub struct State {
    balances: HashMap<Address, (Amount, Nonce)>,
}

impl State {
    // Create new VM state seeded with some initial balances
    pub fn from_initial_balances(initial_balances: &[(Address, Amount)]) -> Self {
        let mut balances = HashMap::new();
        for (addr, amount) in initial_balances.iter() {
            balances.insert(*addr, (*amount, 0));
        }
        State { balances }
    }

    // If the transaction is valid, transition the state and return the new state with updated balances.
    //
    // A transaction is valid iff
    // 1) The transaction on the signature is valid
    // 2) The nonce of the transaction is greater than the sender nonce (this prevent replay attacks)
    // 3) The sender has a high enough balance to cover the transfer amount
    pub fn apply_transaction(&self, transaction: &SignedTransaction) -> Result<State, RollupError> {
        // 1)
        let sender = transaction.recover()?;
        let destination = transaction.transaction.destination;
        let next_nonce = transaction.transaction.nonce;
        let transfer_amount = transaction.transaction.amount;
        let mut balances = self.balances.clone();
        let (sender_balance, prev_nonce) = balances
            .get_mut(&sender)
            .ok_or(RollupError(format!("No balance for sender: {}", sender)))?;

        // 2)
        if next_nonce <= *prev_nonce {
            return Err(RollupError("Invalid nonce".into()));
        }

        // 3)
        if transfer_amount > *sender_balance {
            return Err(RollupError("Insuffificent balance".into()));
        }

        // Transaction is valid, return the updated state
        *sender_balance -= transfer_amount;
        *prev_nonce = next_nonce;
        let (destination_balance, _) = balances.entry(destination).or_insert((0, 0));
        *destination_balance += transfer_amount;

        Ok(State { balances })
    }

    // Fetch the balance of an address
    pub fn get_balance(&self, address: &Address) -> Option<Amount> {
        self.balances.get(address).map(|(amt, _)| *amt)
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
        let mut state = State::from_initial_balances(&seed_data);
        let mut transaction = Transaction {
            amount: 110,
            destination: bob.address(),
            nonce: 1,
        };

        // Try to overspend
        let mut signed_transaction = SignedTransaction::new(transaction.clone(), &alice).await;
        let res = state.apply_transaction(&signed_transaction);
        if let Err(err) = res {
            assert!(err == RollupError("Insuffificent balance".into()));
        } else {
            panic!("Invalid transaction should have thrown error")
        }

        // Now spend an valid amount
        transaction.amount = 50;
        signed_transaction = SignedTransaction::new(transaction.clone(), &alice).await;
        state = state
            .apply_transaction(&signed_transaction)
            .expect("Valid transaction should transition state");
        let bob_balance = state
            .get_balance(&bob.address())
            .expect("Bob should have a balance");
        assert!(bob_balance == 150);

        // Now try to replay the transaction
        let res = state.apply_transaction(&signed_transaction);
        if let Err(err) = res {
            assert!(err == RollupError("Invalid nonce".into()));
        } else {
            panic!("Transaction with a stale nonce should have thrown error")
        }
    }
}
