use anyhow::{bail, Result};
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot_types::{
    data::{DAProposal, VidDisperse},
    message::Proposal,
    traits::{node_implementation::NodeType, storage::Storage},
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TestStorageState<TYPES: NodeType> {
    vids: HashMap<TYPES::Time, Proposal<TYPES, VidDisperse<TYPES>>>,
    das: HashMap<TYPES::Time, Proposal<TYPES, DAProposal<TYPES>>>,
}

impl<TYPES: NodeType> Default for TestStorageState<TYPES> {
    fn default() -> Self {
        Self {
            vids: HashMap::new(),
            das: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TestStorage<TYPES: NodeType> {
    inner: Arc<RwLock<TestStorageState<TYPES>>>,
    /// `should_return_err` is a testing utility to validate negative cases.
    pub should_return_err: bool,
}

impl<TYPES: NodeType> Default for TestStorage<TYPES> {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TestStorageState::default())),
            should_return_err: false,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType> Storage<TYPES> for TestStorage<TYPES> {
    async fn append_vid(&self, proposal: &Proposal<TYPES, VidDisperse<TYPES>>) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        let mut inner = self.inner.write().await;
        inner
            .vids
            .insert(proposal.data.view_number, proposal.clone());
        Ok(())
    }

    async fn append_da(&self, proposal: &Proposal<TYPES, DAProposal<TYPES>>) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        let mut inner = self.inner.write().await;
        inner
            .das
            .insert(proposal.data.view_number, proposal.clone());
        Ok(())
    }

    async fn record_action(
        &self,
        _view: <TYPES as NodeType>::Time,
        _action: hotshot_types::event::HotShotAction,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append Action to storage");
        }
        Ok(())
    }

    async fn update_high_qc(
        &self,
        _qc: hotshot_types::simple_certificate::QuorumCertificate<TYPES>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to update high qc to storage");
        }
        Ok(())
    }
}
