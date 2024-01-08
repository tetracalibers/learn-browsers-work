use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct TreeNode<T: TreeNodeHooks<T> + Debug>(Rc<Node<T>>);
pub struct WeakTreeNode<T: TreeNodeHooks<T> + Debug>(Weak<Node<T>>);

pub type NullableNode<T> = Option<TreeNode<T>>;
pub type NullableWeakNode<T> = Option<WeakTreeNode<T>>;

pub struct Node<T: TreeNodeHooks<T> + Debug> {
  pub data: T,
  pub parent: RefCell<NullableWeakNode<T>>,
  pub first_child: RefCell<NullableNode<T>>,
  pub last_child: RefCell<NullableNode<T>>,
  pub next_sibling: RefCell<NullableNode<T>>,
  pub prev_sibling: RefCell<NullableWeakNode<T>>,
}

#[allow(unused_variables)]
pub trait TreeNodeHooks<T: TreeNodeHooks<T> + Debug> {
  fn on_inserted(&self, current: TreeNode<T>, parent: TreeNode<T>) {}
  fn on_children_updated(&self, current: TreeNode<T>) {}
}

impl<T: TreeNodeHooks<T> + Debug> Deref for Node<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<T: TreeNodeHooks<T> + Debug> Deref for TreeNode<T> {
  type Target = Rc<Node<T>>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: TreeNodeHooks<T> + Debug> Deref for WeakTreeNode<T> {
  type Target = Weak<Node<T>>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: TreeNodeHooks<T> + Debug> Node<T> {
  pub fn new(data: T) -> Self {
    Self {
      data,
      parent: RefCell::new(None),
      first_child: RefCell::new(None),
      last_child: RefCell::new(None),
      next_sibling: RefCell::new(None),
      prev_sibling: RefCell::new(None),
    }
  }
}

impl<T: TreeNodeHooks<T> + Debug> TreeNode<T> {
  pub fn new(data: T) -> Self {
    Self(Rc::new(Node::new(data)))
  }
}
