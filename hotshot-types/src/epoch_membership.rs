use std::collections::BTreeSet;
use std::num::NonZeroU64;
use std::{collections::HashMap, sync::Arc};

use async_broadcast::{broadcast, InactiveReceiver};
use async_lock::{Mutex, RwLock};
use hotshot_utils::anytrace::Result;

use crate::traits::election::Membership;
use crate::traits::node_implementation::{ConsensusTime, NodeType};
use crate::traits::signature_key::SignatureKey;
use crate::utils::root_block_in_epoch;

#[derive(Clone, Debug)]
/// Struct to signal a catchup of a membership is complete
/// Used internally in `EpochMembershipCoordinator` only
struct CatchupSignal {}

/// Struct to Coordinate membership catchup
pub struct EpochMembershipCoordinator<TYPES: NodeType> {
    /// The underlying membhersip
    membership: Arc<RwLock<TYPES::Membership>>,

    /// Any in progress attempts at catching up are stored in this map
    /// Any new callers wantin an `EpochMembership` will await on the signal
    /// alerting them the membership is ready.  The first caller for an epoch will
    /// wait for the actual catchup and allert future callers when it's done
    catchup_map: Arc<Mutex<HashMap<TYPES::Epoch, InactiveReceiver<CatchupSignal>>>>,

    /// Number of blocks in an epoch
    pub epoch_height: u64,
}

impl<TYPES: NodeType> Clone for EpochMembershipCoordinator<TYPES> {
    fn clone(&self) -> Self {
        Self {
            membership: Arc::clone(&self.membership),
            catchup_map: Arc::clone(&self.catchup_map),
            epoch_height: self.epoch_height,
        }
    }
}
// async fn catchup_membership(coordinator: EpochMembershipCoordinator<TYPES>) {

// }

impl<TYPES: NodeType> EpochMembershipCoordinator<TYPES>
where
    Self: Send,
{
    /// Create an EpochMembershipCoordinator
    pub fn new(membership: Arc<RwLock<TYPES::Membership>>, epoch_height: u64) -> Self {
        Self {
            membership,
            catchup_map: Arc::default(),
            epoch_height,
        }
    }

    /// Get a reference to the membership
    #[must_use]
    pub fn membership(&self) -> &Arc<RwLock<TYPES::Membership>> {
        &self.membership
    }

    /// Get a Membership for a given Epoch, which is guarenteed to have a stake
    /// table for the given Epoch
    pub async fn membership_for_epoch(
        &self,
        maybe_epoch: Option<TYPES::Epoch>,
    ) -> EpochMembership<TYPES> {
        let ret_val = EpochMembership {
            epoch: maybe_epoch,
            coordinator: self.clone(),
        };
        let Some(epoch) = maybe_epoch else {
            return ret_val;
        };
        if self.membership.read().await.has_epoch(epoch) {
            return ret_val;
        }
        let tx = {
            let mut map = self.catchup_map.lock().await;
            if let Some(inactive_rx) = map.get(&epoch) {
                let mut rx = inactive_rx.activate_cloned();
                let _ = rx.recv().await;
                return ret_val;
            }
            let (tx, rx) = broadcast(1);
            map.insert(epoch, rx.deactivate());
            tx
        };
        // do catchup
        self.catchup(epoch).await;
        let _ = tx.broadcast_direct(CatchupSignal {}).await;
        ret_val
    }

    /// Catches the membership up to the epoch passed as an argument.  
    /// To do this try to get the stake table for the epoch containing this epoch's root
    /// if the root does not exist recusively catchup until you've found it
    ///
    /// If there is another catchup in progress this will not duplicate efforts
    /// e.g. if we start with only epoch 0 stake table and call catchup for epoch 10, then call catchup for epoch 20
    /// the first caller will actually do the work for to catchup to epoch 10 then the second caller will continue
    /// catching up to epoch 20
    async fn catchup(&self, epoch: TYPES::Epoch) {
        // recursively catchup until we have a stake table for the epoch containing our root
        assert!(
            *epoch != 0,
            "We are trying to catchup to epoch 0! This means the initial stake table is missing!"
        );
        let root_epoch = if *epoch == 1 {
            TYPES::Epoch::new(*epoch - 1)
        } else {
            TYPES::Epoch::new(*epoch - 2)
        };
        let root_membership = Box::pin(self.membership_for_epoch(Some(root_epoch))).await;

        // Get the epoch root headers and update our membership with them, finally sync them
        // Verification of the root is handled in get_epoch_root
        let Some((next_epoch, header)) = root_membership
            .get_epoch_root(root_block_in_epoch(*root_epoch, self.epoch_height))
            .await
        else {
            return;
        };
        let Some(updater) = self
            .membership
            .read()
            .await
            .add_epoch_root(next_epoch, header)
            .await
        else {
            return;
        };
        updater(&mut *(self.membership.write().await));

        let Some(sync) = self.membership.read().await.sync_l1().await else {
            return;
        };
        sync(&mut *(self.membership.write().await));
    }
}

/// Wrapper around a membership that guarentees that the epoch
/// has a stake table
pub struct EpochMembership<TYPES: NodeType> {
    /// Epoch the `membership` is guarenteed to have a stake table for
    pub epoch: Option<TYPES::Epoch>,
    /// Underlying membership
    pub coordinator: EpochMembershipCoordinator<TYPES>,
}

impl<TYPES: NodeType> Clone for EpochMembership<TYPES> {
    fn clone(&self) -> Self {
        Self {
            coordinator: self.coordinator.clone(),
            epoch: self.epoch,
        }
    }
}

impl<TYPES: NodeType> EpochMembership<TYPES> {
    /// Get the epoch this membership is good for
    pub fn epoch(&self) -> Option<TYPES::Epoch> {
        self.epoch
    }

    /// Get a membership for the next epoch
    pub async fn next_epoch(&self) -> Self {
        if self.epoch.is_none() {
            self.clone()
        } else {
            self.coordinator
                .membership_for_epoch(self.epoch.map(|e| e + 1))
                .await
        }
    }

    /// Get the prior epoch
    pub async fn prev_epoch(&self) -> Self {
        let Some(epoch) = self.epoch else {
            return self.clone();
        };
        if *epoch == 0 {
            return self.clone();
        }
        self.coordinator.membership_for_epoch(Some(epoch - 1)).await
    }

    /// Wraps the same named Membership trait fn
    async fn get_epoch_root(
        &self,
        block_height: u64,
    ) -> Option<(TYPES::Epoch, TYPES::BlockHeader)> {
        self.coordinator
            .membership
            .read()
            .await
            .get_epoch_root(block_height)
            .await
    }

    /// Get all participants in the committee (including their stake) for a specific epoch
    pub async fn stake_table(&self) -> Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        self.coordinator
            .membership
            .read()
            .await
            .stake_table(self.epoch)
    }

    /// Get all participants in the committee (including their stake) for a specific epoch
    pub async fn da_stake_table(
        &self,
    ) -> Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        self.coordinator
            .membership
            .read()
            .await
            .da_stake_table(self.epoch)
    }

    /// Get all participants in the committee for a specific view for a specific epoch
    pub async fn committee_members(
        &self,
        view_number: TYPES::View,
    ) -> BTreeSet<TYPES::SignatureKey> {
        self.coordinator
            .membership
            .read()
            .await
            .committee_members(view_number, self.epoch)
    }

    /// Get all participants in the committee for a specific view for a specific epoch
    pub async fn da_committee_members(
        &self,
        view_number: TYPES::View,
    ) -> BTreeSet<TYPES::SignatureKey> {
        self.coordinator
            .membership
            .read()
            .await
            .da_committee_members(view_number, self.epoch)
    }

    /// Get all leaders in the committee for a specific view for a specific epoch
    pub async fn committee_leaders(
        &self,
        view_number: TYPES::View,
    ) -> BTreeSet<TYPES::SignatureKey> {
        self.coordinator
            .membership
            .read()
            .await
            .committee_leaders(view_number, self.epoch)
    }

    /// Get the stake table entry for a public key, returns `None` if the
    /// key is not in the table for a specific epoch
    pub async fn stake(
        &self,
        pub_key: &TYPES::SignatureKey,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        self.coordinator
            .membership
            .read()
            .await
            .stake(pub_key, self.epoch)
    }

    /// Get the DA stake table entry for a public key, returns `None` if the
    /// key is not in the table for a specific epoch
    pub async fn da_stake(
        &self,
        pub_key: &TYPES::SignatureKey,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        self.coordinator
            .membership
            .read()
            .await
            .da_stake(pub_key, self.epoch)
    }

    /// See if a node has stake in the committee in a specific epoch
    pub async fn has_stake(&self, pub_key: &TYPES::SignatureKey) -> bool {
        self.coordinator
            .membership
            .read()
            .await
            .has_stake(pub_key, self.epoch)
    }

    /// See if a node has stake in the committee in a specific epoch
    pub async fn has_da_stake(&self, pub_key: &TYPES::SignatureKey) -> bool {
        self.coordinator
            .membership
            .read()
            .await
            .has_da_stake(pub_key, self.epoch)
    }

    /// The leader of the committee for view `view_number` in `epoch`.
    ///
    /// Note: this function uses a HotShot-internal error type.
    /// You should implement `lookup_leader`, rather than implementing this function directly.
    ///
    /// # Errors
    /// Returns an error if the leader cannot be calculated.
    pub async fn leader(&self, view: TYPES::View) -> Result<TYPES::SignatureKey> {
        self.coordinator
            .membership
            .read()
            .await
            .leader(view, self.epoch)
    }

    /// The leader of the committee for view `view_number` in `epoch`.
    ///
    /// Note: There is no such thing as a DA leader, so any consumer
    /// requiring a leader should call this.
    ///
    /// # Errors
    /// Returns an error if the leader cannot be calculated
    pub async fn lookup_leader(
        &self,
        view: TYPES::View,
    ) -> std::result::Result<
        TYPES::SignatureKey,
        <<TYPES as NodeType>::Membership as Membership<TYPES>>::Error,
    > {
        self.coordinator
            .membership
            .read()
            .await
            .lookup_leader(view, self.epoch)
    }

    /// Returns the number of total nodes in the committee in an epoch `epoch`
    pub async fn total_nodes(&self) -> usize {
        self.coordinator
            .membership
            .read()
            .await
            .total_nodes(self.epoch)
    }

    /// Returns the number of total DA nodes in the committee in an epoch `epoch`
    pub async fn da_total_nodes(&self) -> usize {
        self.coordinator
            .membership
            .read()
            .await
            .da_total_nodes(self.epoch)
    }

    /// Returns the threshold for a specific `Membership` implementation
    pub async fn success_threshold(&self) -> NonZeroU64 {
        self.coordinator
            .membership
            .read()
            .await
            .success_threshold(self.epoch)
    }

    /// Returns the DA threshold for a specific `Membership` implementation
    pub async fn da_success_threshold(&self) -> NonZeroU64 {
        self.coordinator
            .membership
            .read()
            .await
            .da_success_threshold(self.epoch)
    }

    /// Returns the threshold for a specific `Membership` implementation
    pub async fn failure_threshold(&self) -> NonZeroU64 {
        self.coordinator
            .membership
            .read()
            .await
            .failure_threshold(self.epoch)
    }

    /// Returns the threshold required to upgrade the network protocol
    pub async fn upgrade_threshold(&self) -> NonZeroU64 {
        self.coordinator
            .membership
            .read()
            .await
            .upgrade_threshold(self.epoch)
    }
}
