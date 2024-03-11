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
