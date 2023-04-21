use sequencer::{Vm, VmId};
use transaction::SignedTransaction;

pub mod api;
pub mod error;
pub mod executor;
mod prover;
pub mod state;
pub mod transaction;

// The VmID helps Rollups find their transactions in the sequenced block.
pub const VM_ID: u64 = 1;

#[derive(Clone, Copy, Debug, Default)]
pub struct RollupVM;

impl Vm for RollupVM {
    type Transaction = SignedTransaction;

    fn id(&self) -> VmId {
        VM_ID.into()
    }
}
