use std::fmt::Debug;

use super::TreeNode;

pub struct ChildrenIterator<T: Debug> {
  parent: TreeNode<T>,
  current_node: Option<TreeNode<T>>,
  first_iter: bool,
}

impl<T: Debug> ChildrenIterator<T> {
  pub fn new(parent: TreeNode<T>) -> Self {
    Self {
      parent,
      current_node: None,
      first_iter: true,
    }
  }
}

impl<T: Debug> Iterator for ChildrenIterator<T> {
  type Item = TreeNode<T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.first_iter {
      self.current_node = self.parent.first_child();
      self.first_iter = false;
    }

    let current = self.current_node.clone();
    let next = current.clone().and_then(|node| node.next_sibling());

    self.current_node = next;

    current
  }
}
