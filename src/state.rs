use ndarray::{Array, IxDyn};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State<T>(pub Array<T, IxDyn>);
