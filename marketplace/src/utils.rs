use hotshot_types::traits::node_implementation::NodeType;

// TODO: Update commitment calculation with the new `commit`.
// <https://github.com/EspressoSystems/marketplace-builder-core/issues/143>
pub trait LegacyCommit<T: NodeType> {
    fn legacy_commit(&self) -> committable::Commitment<hotshot_types::data::Leaf<T>>;
}

impl<T: NodeType> LegacyCommit<T> for hotshot_types::data::Leaf<T> {
    fn legacy_commit(&self) -> committable::Commitment<hotshot_types::data::Leaf<T>> {
        <hotshot_types::data::Leaf<T> as committable::Committable>::commit(self)
    }
}
