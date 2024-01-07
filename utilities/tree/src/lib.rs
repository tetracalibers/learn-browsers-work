use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

pub struct TreeNode<T: Debug>(Rc<Node<T>>);
pub struct WeekTreeNode<T: Debug>(Weak<Node<T>>);

pub type NullableNode<T> = Option<TreeNode<T>>;
pub type NullableWeekNode<T> = Option<WeekTreeNode<T>>;

pub struct Node<T: Debug> {
  pub data: T,
  pub parent: RefCell<NullableWeekNode<T>>,
  pub first_child: RefCell<NullableNode<T>>,
  pub last_child: RefCell<NullableNode<T>>,
  pub next_sibling: RefCell<NullableNode<T>>,
  pub prev_sibling: RefCell<NullableWeekNode<T>>,
}

impl<T: Debug> Node<T> {
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
