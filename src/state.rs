use ndarray::{Array, IxDyn};

pub struct State<T>(Array<T, IxDyn>);

impl State<usize> {
    fn zeros(n_elements: usize) -> Self {
        State(Array::<usize, IxDyn>::zeros(vec![2; n_elements]))
    }
}

#[cfg(test)]
mod test {
    use super::State;

    #[test]
    fn test_zeros() {
        State::<usize>::zeros(10);
    }
}
