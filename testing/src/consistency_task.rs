// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

#![allow(clippy::unwrap_or_default)]
use std::collections::BTreeMap;

use anyhow::{bail, ensure, Context, Result};
use async_trait::async_trait;
use committable::Committable;
use hotshot_types::{
    data::Leaf,
    event::{Event, EventType},
    traits::node_implementation::NodeType,
};

use crate::{
    overall_safety_task::OverallSafetyPropertiesDescription,
    test_task::{TestResult, TestTaskState},
};

trait Validatable {
    fn valid(&self) -> Result<()>;
}

/// Map from views to leaves for a single node, allowing multiple leaves for each view (because the node may a priori send us multiple leaves for a given view).
pub type NodeMap<TYPES> = BTreeMap<<TYPES as NodeType>::Time, Vec<Leaf<TYPES>>>;

/// A sanitized map from views to leaves for a single node, with only a single leaf per view.
pub type NodeMapSanitized<TYPES> = BTreeMap<<TYPES as NodeType>::Time, Leaf<TYPES>>;

/// Validate that the `NodeMap` only has a single leaf per view.
pub fn sanitize_node_map<TYPES: NodeType>(
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

/// For a NodeLeafMap, we validate that each leaf extends the preceding leaf.
impl<TYPES: NodeType> Validatable for NodeMapSanitized<TYPES> {
    fn valid(&self) -> Result<()> {
        let leaf_pairs = self.values().zip(self.values().skip(1));

        // Check that the child leaf follows the parent, possibly with a gap.
        for (parent, child) in leaf_pairs {
            ensure!(
              child.justify_qc().view_number >= parent.view_number(),
              "The node has provided leaf:\n\n{child:?}\n\nbut its quorum certificate points to a view before the most recent leaf:\n\n{parent:?}"
            );

            if child.justify_qc().view_number == parent.view_number()
                && child.justify_qc().data.leaf_commit != parent.commit()
            {
                bail!("The node has provided leaf:\n\n{child:?}\n\nwhich points to:\n\n{parent:?}\n\nbut the commits do not match.");
            }
        }

        Ok(())
    }
}

/// A map from node ids to `NodeMap`s; note that the latter may have multiple leaves per view in principle.
pub type NetworkMap<TYPES> = BTreeMap<usize, NodeMap<TYPES>>;

/// A map from node ids to `NodeMapSanitized`s; the latter has been sanitized validated to have a single leaf per view.
pub type NetworkMapSanitized<TYPES> = BTreeMap<usize, NodeMapSanitized<TYPES>>;

/// Validate that each node has only produced one unique leaf per view, and produce a `NetworkMapSanitized`.
pub fn sanitize_network_map<TYPES: NodeType>(
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

impl<TYPES: NodeType> Validatable for NetworkMap<TYPES> {
    fn valid(&self) -> Result<()> {
        let sanitized = sanitize_network_map(self)?;

        sanitized.valid()
    }
}

/// For a NetworkLeafMap, we validate that no two nodes have submitted differing leaves for any given view, in addition to the individual NodeLeafMap checks.
impl<TYPES: NodeType> Validatable for NetworkMapSanitized<TYPES> {
    fn valid(&self) -> Result<()> {
        // Invert the map by interchanging the roles of the node_id and view number.
        let mut inverted_map = BTreeMap::new();
        for (node_id, node_map) in self.iter() {
            node_map
                .valid()
                .context(format!("Node {node_id} has an invalid leaf history"))?;

            // validate each node's leaf map
            for (view, leaf) in node_map.iter() {
                let view_map = inverted_map.entry(*view).or_insert(BTreeMap::new());
                view_map.insert(*node_id, leaf.clone());
            }
        }

        for (view, view_map) in inverted_map.iter() {
            let mut leaves: Vec<_> = view_map.iter().collect();

            leaves.dedup_by(|(_node_a, leaf_a), (_node_b, leaf_b)| leaf_a == leaf_b);

            ensure!(
                leaves.len() <= 1,
                view_map.iter().fold(
                    format!("The network does not agree on view {view:?}."),
                    |acc, (node, leaf)| {
                        format!("{acc}\n\nNode {node} sent us leaf:\n\n{leaf:?}")
                    }
                )
            );
        }

        Ok(())
    }
}

/// Data availability task state
pub struct ConsistencyTask<TYPES: NodeType> {
    /// A map from node ids to (leaves keyed on view number)
    pub consensus_leaves: NetworkMap<TYPES>,
    /// safety task requirements
    pub safety_properties: OverallSafetyPropertiesDescription<TYPES>,
}

#[async_trait]
impl<TYPES: NodeType> TestTaskState for ConsistencyTask<TYPES> {
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

    fn check(&self) -> TestResult {
        if let Err(e) = self.consensus_leaves.valid() {
            return TestResult::Fail(Box::new(e));
        }

        TestResult::Pass
    }
}
