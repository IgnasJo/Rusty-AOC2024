use std::collections::HashMap;

// generalized tree
pub struct TreeNode<T> {
  value: T,
  children: HashMap<T, TreeNode<T>>,
}

impl<T> TreeNode<T> {
  pub fn new(value: T) -> Self {
    TreeNode { value, children: HashMap::new()}
  }
}