//! This module contains an optimizide implementation of a
//! [`BuilderStateId`] to [`BuilderState`] map.

use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::RangeBounds,
    sync::Arc,
};

use hotshot_types::{traits::node_implementation::NodeType, vid::VidCommitment};
use nonempty_collections::{nem, NEMap};

use crate::{block::BuilderStateId, state::BuilderState};

/// A map from [`BuilderStateId`] to [`BuilderState`], implemented as a tiered map
/// with the first tier being [`BTreeMap`] keyed by view number of [`BuilderStateId`]
/// and the second [`NEMap`] keyed by VID commitment of [`BuilderStateId`].
///
/// Usage of [`BTreeMap`] means that the map has an convenient property of always being
/// sorted by view number, which makes common operations such as pruning old views more efficient.
///
/// Second tier being non-empty by construction [`NEMap`] ensures that we can't accidentally
/// create phantom entries with empty maps in the first tier.
#[derive(Debug)]
pub struct BuilderStateMap<Types: NodeType>(
    BTreeMap<<Types as NodeType>::View, NEMap<VidCommitment, Arc<BuilderState<Types>>>>,
);

impl<Types: NodeType> BuilderStateMap<Types> {
    /// Create a new empty map
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Returns an iterator visiting all values in this map
    pub fn values(&self) -> impl Iterator<Item = &Arc<BuilderState<Types>>> {
        self.0
            .values()
            .flat_map(|bucket| bucket.values().into_iter())
    }

    /// Returns a nested iterator visiting all [`BuilderState`]s for view numbers in given range
    pub fn range<R>(
        &self,
        range: R,
    ) -> impl Iterator<Item = impl Iterator<Item = &Arc<BuilderState<Types>>>>
    where
        R: RangeBounds<Types::View>,
    {
        self.0
            .range(range)
            .map(|(_, bucket)| bucket.values().into_iter())
    }

    /// Returns an iterator visiting all [`BuilderState`]s for given view number
    pub fn bucket(
        &self,
        view_number: &Types::View,
    ) -> impl Iterator<Item = &Arc<BuilderState<Types>>> {
        self.0
            .get(view_number)
            .into_iter()
            .flat_map(|bucket| bucket.values().into_iter())
    }

    /// Returns the number of builder states stored
    pub fn len(&self) -> usize {
        self.0.values().map(|bucket| bucket.len().get()).sum()
    }

    /// Returns whether this map is empty
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Get builder state by ID
    pub fn get(&self, key: &BuilderStateId<Types>) -> Option<&Arc<BuilderState<Types>>> {
        self.0.get(&key.parent_view)?.get(&key.parent_commitment)
    }

    /// Get highest view builder state
    ///
    /// Returns `None` if the map is empty
    pub fn highest_view_builder(&self) -> Option<&Arc<BuilderState<Types>>> {
        Some(&self.0.last_key_value()?.1.head_val)
    }

    /// Insert a new builder state
    pub fn insert(&mut self, value: Arc<BuilderState<Types>>) {
        let key = value.id();
        match self.0.entry(key.parent_view) {
            Entry::Vacant(entry) => {
                entry.insert(nem![key.parent_commitment => value]);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(key.parent_commitment, value);
            }
        }
    }

    /// Returns highest view number for which we have a builder state
    pub fn highest_view(&self) -> Option<Types::View> {
        Some(*self.0.last_key_value()?.0)
    }

    /// Returns lowest view number for which we have a builder state
    pub fn lowest_view(&self) -> Option<Types::View> {
        Some(*self.0.first_key_value()?.0)
    }

    /// Removes every view lower than the `cutoff_view` (exclusive) from self and returns all removed views.
    pub fn prune(&mut self, cutoff_view: Types::View) -> Self {
        let high = self.0.split_off(&cutoff_view);
        let low = std::mem::replace(&mut self.0, high);
        Self(low)
    }
}

impl<Types: NodeType> Default for BuilderStateMap<Types> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, ops::Bound};

    use crate::testing::mock;

    use super::*;
    use hotshot_example_types::node_types::TestTypes;
    use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
    use rand::{distributions::Standard, thread_rng, Rng};

    type View = ViewNumber;
    type BuilderStateMap = super::BuilderStateMap<TestTypes>;

    #[test]
    fn test_new_map() {
        let new_map = BuilderStateMap::new();
        assert!(new_map.is_empty());
        assert_eq!(new_map.len(), 0);

        let default_map = BuilderStateMap::default();
        assert!(default_map.is_empty());
        assert_eq!(default_map.len(), 0);
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = BuilderStateMap::new();

        let builder_state = mock::builder_state(1);
        let state_id = builder_state.id();

        map.insert(builder_state.clone());

        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
        assert!(Arc::ptr_eq(map.get(&state_id).unwrap(), &builder_state));
    }

    #[test]
    fn test_range_iteration() {
        let mut map = BuilderStateMap::new();

        for i in 0..5 {
            map.insert(mock::builder_state(i));
        }

        let start = View::new(1);
        let end = View::new(3);

        let collected: Vec<_> = map
            .range((Bound::Included(start), Bound::Excluded(end)))
            .flatten()
            .collect();

        assert_eq!(collected.len(), 2);
        assert_eq!(*collected[0].parent_block_references.view_number, 1);
        assert_eq!(*collected[1].parent_block_references.view_number, 2);
    }

    #[test]
    fn test_pruning() {
        let view_count = 11;
        let states_per_view = 13;
        let cutoff = 7;

        let mut map = BuilderStateMap::new();

        for view in 0..view_count {
            for _ in 0..states_per_view {
                map.insert(mock::builder_state(view));
            }
        }

        let pruned_map = map.prune(View::new(cutoff));
        assert_eq!(pruned_map.len() as u64, cutoff * states_per_view);
        assert_eq!(map.len() as u64, (view_count - cutoff) * states_per_view);

        assert!(pruned_map.bucket(&View::new(cutoff - 1)).next().is_some());
        assert!(map.bucket(&View::new(cutoff)).next().is_some());

        assert!(pruned_map.bucket(&View::new(cutoff)).next().is_none());
        assert!(map.bucket(&View::new(cutoff - 1)).next().is_none());
    }

    #[test]
    fn test_highest_and_lowest_view() {
        let mut map = BuilderStateMap::new();
        assert_eq!(map.highest_view(), None);
        assert_eq!(map.lowest_view(), None);

        for i in 3..13 {
            map.insert(mock::builder_state(i));
        }

        assert_eq!(*map.highest_view().unwrap(), 12);
        assert_eq!(*map.lowest_view().unwrap(), 3);
    }

    #[test]
    fn test_highest_view_builder() {
        let mut map = BuilderStateMap::new();
        assert!(map.highest_view_builder().is_none());

        let mut states: Vec<_> = thread_rng()
            .sample_iter(Standard)
            .take(100)
            .map(mock::builder_state)
            .collect();

        for state in states.iter() {
            map.insert(state.clone());
        }

        states.sort_by(|a, b| {
            a.parent_block_references
                .view_number
                .cmp(&b.parent_block_references.view_number)
        });

        assert_eq!(
            map.highest_view_builder()
                .unwrap()
                .parent_block_references
                .view_number,
            states.last().unwrap().parent_block_references.view_number
        );
    }

    #[test]
    fn test_iterator() {
        let mut map = BuilderStateMap::new();

        let mut states: Vec<_> = thread_rng()
            .sample_iter(Standard)
            .take(100)
            .map(mock::builder_state)
            .collect();
        assert_eq!(states.len(), 100);

        for state in states.iter() {
            map.insert(state.clone());
        }

        states.sort_by(compare_builders);

        let mut map_states = map.values().cloned().collect::<Vec<_>>();
        map_states.sort_by(compare_builders);

        for (original_state, map_state) in states.into_iter().zip(map_states) {
            assert!(Arc::ptr_eq(&original_state, &map_state));
        }
    }

    fn compare_builders(
        a: &Arc<BuilderState<TestTypes>>,
        b: &Arc<BuilderState<TestTypes>>,
    ) -> Ordering {
        a.parent_block_references
            .view_number
            .cmp(&b.parent_block_references.view_number)
            .then(
                a.parent_block_references
                    .vid_commitment
                    .cmp(&b.parent_block_references.vid_commitment),
            )
    }
}
