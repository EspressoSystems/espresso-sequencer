use committable::Committable;
use hotshot_example_types::{node_types::TestTypes, state_types::TestValidatedState};
use hotshot_types::{
    data::Leaf,
    utils::{View, ViewInner},
};

/// This function permutes the provided input vector `inputs`, given some order provided within the
/// `order` vector.
///
/// # Examples
/// let output = permute_input_with_index_order(vec![1, 2, 3], vec![2, 1, 0]);
/// // Output is [3, 2, 1] now
pub fn permute_input_with_index_order<T>(inputs: Vec<T>, order: Vec<usize>) -> Vec<T>
where
    T: Clone,
{
    let mut ordered_inputs = Vec::with_capacity(inputs.len());
    for &index in &order {
        ordered_inputs.push(inputs[index].clone());
    }
    ordered_inputs
}

/// This function will create a fake [`View`] from a provided [`Leaf`].
pub fn create_fake_view_with_leaf(leaf: Leaf<TestTypes>) -> View<TestTypes> {
    create_fake_view_with_leaf_and_state(leaf, TestValidatedState::default())
}

/// This function will create a fake [`View`] from a provided [`Leaf`] and `state`.
pub fn create_fake_view_with_leaf_and_state(
    leaf: Leaf<TestTypes>,
    state: TestValidatedState,
) -> View<TestTypes> {
    View {
        view_inner: ViewInner::Leaf {
            leaf: leaf.commit(),
            state: state.into(),
            delta: None,
        },
    }
}
