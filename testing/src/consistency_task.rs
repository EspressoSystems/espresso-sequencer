// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

#![allow(clippy::unwrap_or_default)]
use std::{collections::BTreeMap, marker::PhantomData};

use anyhow::{bail, ensure, Context, Result};
use async_trait::async_trait;
use committable::Committable;
use hotshot_example_types::block_types::TestBlockHeader;
use hotshot_types::{
    data::Leaf2,
    event::{Event, EventType},
    message::UpgradeLock,
    traits::node_implementation::{ConsensusTime, NodeType, Versions},
};

use crate::{
    overall_safety_task::OverallSafetyPropertiesDescription,
    test_builder::TransactionValidator,
    test_task::{TestResult, TestTaskState},
};

/// Map from views to leaves for a single node, allowing multiple leaves for each view (because the node may a priori send us multiple leaves for a given view).
pub type NodeMap<TYPES> = BTreeMap<<TYPES as NodeType>::View, Vec<Leaf2<TYPES>>>;

/// A sanitized map from views to leaves for a single node, with only a single leaf per view.
pub type NodeMapSanitized<TYPES> = BTreeMap<<TYPES as NodeType>::View, Leaf2<TYPES>>;

/// Validate that the `NodeMap` only has a single leaf per view.
fn sanitize_node_map<TYPES: NodeType>(
    node_map: &NodeMap<TYPES>,
) -> Result<NodeMapSanitized<TYPES>> {
    let mut result = BTreeMap::new();

    for (view, leaves) in node_map.iter() {
        let mut reduced = leaves.clone();

        reduced.dedup();

        match reduced.len() {
            0 => {}
            1 => {
                result.insert(*view, reduced[0].clone());
            }
            _ => {
                bail!(
                    "We have received inconsistent leaves for view {view:?}. Leaves:\n\n{leaves:?}"
                );
            }
        }
    }

    Ok(result)
}

/// For a NodeMapSanitized, we validate that each leaf extends the preceding leaf.
async fn validate_node_map<TYPES: NodeType, V: Versions>(
    node_map: &NodeMapSanitized<TYPES>,
) -> Result<()> {
    // We first scan 3-chains to find an upgrade certificate that has reached a decide.
    let leaf_triples = node_map
        .values()
        .zip(node_map.values().skip(1))
        .zip(node_map.values().skip(2))
        .map(|((a, b), c)| (a, b, c));

    let mut decided_upgrade_certificate = None;
    let mut view_decided = TYPES::View::new(0);

    for (grandparent, _parent, child) in leaf_triples {
        if let Some(cert) = grandparent.upgrade_certificate() {
            if cert.data.decide_by <= child.view_number() {
                decided_upgrade_certificate = Some(cert);
                view_decided = child.view_number();

                break;
            }
        }
    }

    // To mimic consensus to use e.g. the `extends_upgrade` method,
    // we cannot immediately put the upgrade certificate in the lock.
    //
    // Instead, we initialize an empty lock and add the certificate in the appropriate view.
    let upgrade_lock = UpgradeLock::<TYPES, V>::new();

    let leaf_pairs = node_map.values().zip(node_map.values().skip(1));

    // Check that the child leaf follows the parent, possibly with a gap.
    for (parent, child) in leaf_pairs {
        ensure!(
              child.justify_qc().view_number >= parent.view_number(),
              "The node has provided leaf:\n\n{child:?}\n\nbut its quorum certificate points to a view before the most recent leaf:\n\n{parent:?}"
        );

        child
            .extends_upgrade(parent, &upgrade_lock.decided_upgrade_certificate)
            .await
            .context("Leaf {child} does not extend its parent {parent}")?;

        // We want to make sure the commitment matches,
        // but allow for the possibility that we may have skipped views in between.
        if child.justify_qc().data.leaf_commit != parent.commit() {
            bail!("The node has provided leaf:\n\n{child:?}\n\nwhich points to:\n\n{parent:?}\n\nbut the commits do not match.");
        }

        if child.view_number() == view_decided {
            upgrade_lock
                .decided_upgrade_certificate
                .write()
                .await
                .clone_from(&decided_upgrade_certificate);
        }
    }

    Ok(())
}

/// A map from node ids to `NodeMap`s; note that the latter may have multiple leaves per view in principle.
pub type NetworkMap<TYPES> = BTreeMap<usize, NodeMap<TYPES>>;

/// A map from node ids to `NodeMapSanitized`s; the latter has been sanitized validated to have a single leaf per view.
pub type NetworkMapSanitized<TYPES> = BTreeMap<usize, NodeMapSanitized<TYPES>>;

/// Validate that each node has only produced one unique leaf per view, and produce a `NetworkMapSanitized`.
fn sanitize_network_map<TYPES: NodeType>(
    network_map: &NetworkMap<TYPES>,
) -> Result<NetworkMapSanitized<TYPES>> {
    let mut result = BTreeMap::new();

    for (node, node_map) in network_map {
        result.insert(
            *node,
            sanitize_node_map(node_map)
                .context(format!("Node {node} produced inconsistent leaves."))?,
        );
    }

    Ok(result)
}

pub type ViewMap<TYPES> = BTreeMap<<TYPES as NodeType>::View, BTreeMap<usize, Leaf2<TYPES>>>;

// Invert the network map by interchanging the roles of the node_id and view number.
//
// # Errors
//
// Returns an error if any node map is invalid.
async fn invert_network_map<TYPES: NodeType, V: Versions>(
    network_map: &NetworkMapSanitized<TYPES>,
) -> Result<ViewMap<TYPES>> {
    let mut inverted_map = BTreeMap::new();
    for (node_id, node_map) in network_map.iter() {
        validate_node_map::<TYPES, V>(node_map)
            .await
            .context(format!("Node {node_id} has an invalid leaf history"))?;

        // validate each node's leaf map
        for (view, leaf) in node_map.iter() {
            let view_map = inverted_map.entry(*view).or_insert(BTreeMap::new());
            view_map.insert(*node_id, leaf.clone());
        }
    }

    Ok(inverted_map)
}

/// A view map, sanitized to have exactly one leaf per view.
pub type ViewMapSanitized<TYPES> = BTreeMap<<TYPES as NodeType>::View, Leaf2<TYPES>>;

fn sanitize_view_map<TYPES: NodeType>(
    view_map: &ViewMap<TYPES>,
) -> Result<ViewMapSanitized<TYPES>> {
    let mut result = BTreeMap::new();

    for (view, leaf_map) in view_map.iter() {
        let mut node_leaves: Vec<_> = leaf_map.iter().collect();

        node_leaves.dedup_by(|(_node_a, leaf_a), (_node_b, leaf_b)| leaf_a == leaf_b);

        ensure!(
            node_leaves.len() <= 1,
            leaf_map.iter().fold(
                format!("The network does not agree on view {view:?}."),
                |acc, (node, leaf)| { format!("{acc}\n\nNode {node} sent us leaf:\n\n{leaf:?}") }
            )
        );

        if let Some(leaf) = node_leaves.first() {
            result.insert(*view, leaf.1.clone());
        }
    }

    Ok(result)
}

/// Data availability task state
pub struct ConsistencyTask<TYPES: NodeType, V: Versions> {
    /// A map from node ids to (leaves keyed on view number)
    pub consensus_leaves: NetworkMap<TYPES>,
    /// safety task requirements
    pub safety_properties: OverallSafetyPropertiesDescription<TYPES>,
    /// whether we should have seen an upgrade certificate or not
    pub ensure_upgrade: bool,
    /// phantom marker
    pub _pd: PhantomData<V>,
    /// function used to validate the number of transactions committed in each block
    pub validate_transactions: TransactionValidator,
}

impl<TYPES: NodeType<BlockHeader = TestBlockHeader>, V: Versions> ConsistencyTask<TYPES, V> {
    pub async fn validate(&self) -> Result<()> {
        let sanitized_network_map = sanitize_network_map(&self.consensus_leaves)?;

        let inverted_map = invert_network_map::<TYPES, V>(&sanitized_network_map).await?;

        let sanitized_view_map = sanitize_view_map(&inverted_map)?;

        let expected_upgrade = self.ensure_upgrade;
        let actual_upgrade = sanitized_view_map.iter().fold(false, |acc, (_view, leaf)| {
            acc || leaf.upgrade_certificate().is_some()
        });

        let mut transactions = Vec::new();

        transactions = sanitized_view_map
            .iter()
            .fold(transactions, |mut acc, (view, leaf)| {
                acc.push((**view, leaf.block_header().metadata.num_transactions));

                acc
            });

        (self.validate_transactions)(&transactions)?;

        ensure!(
          expected_upgrade == actual_upgrade,
          "Mismatch between expected and actual upgrade. Expected upgrade: {expected_upgrade}. Actual upgrade: {actual_upgrade}"
        );

        Ok(())
    }
}

#[async_trait]
impl<TYPES: NodeType<BlockHeader = TestBlockHeader>, V: Versions> TestTaskState
    for ConsistencyTask<TYPES, V>
{
    type Event = Event<TYPES>;

    /// Handles an event from one of multiple receivers.
    async fn handle_event(&mut self, (message, id): (Self::Event, usize)) -> Result<()> {
        if let Event {
            event: EventType::Decide { leaf_chain, .. },
            ..
        } = message
        {
            let map = &mut self.consensus_leaves.entry(id).or_insert(BTreeMap::new());

            leaf_chain.iter().for_each(|leaf_info| {
                map.entry(leaf_info.leaf.view_number())
                    .and_modify(|vec| vec.push(leaf_info.leaf.clone()))
                    .or_insert(vec![leaf_info.leaf.clone()]);
            });
        }

        Ok(())
    }

    async fn check(&self) -> TestResult {
        if let Err(e) = self.validate().await {
            return TestResult::Fail(Box::new(e));
        }

        TestResult::Pass
    }
}
