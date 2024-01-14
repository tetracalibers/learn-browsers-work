mod children_iterator;

use self::children_iterator::ChildrenIterator;

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

impl<T: TreeNodeHooks<T> + Debug> WeakTreeNode<T> {
  pub fn upgrade(&self) -> Option<TreeNode<T>> {
    self.0.upgrade().map(|rc| TreeNode::from(rc))
  }
}

impl<T: TreeNodeHooks<T> + Debug> Debug for TreeNode<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.data)
  }
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

impl<T: TreeNodeHooks<T> + Debug> Clone for TreeNode<T> {
  fn clone(&self) -> Self {
    TreeNode(self.0.clone())
  }
}

impl<T: TreeNodeHooks<T> + Debug> Clone for WeakTreeNode<T> {
  fn clone(&self) -> Self {
    WeakTreeNode(self.0.clone())
  }
}

impl<T: TreeNodeHooks<T> + Debug> From<Rc<Node<T>>> for TreeNode<T> {
  fn from(rc: Rc<Node<T>>) -> Self {
    TreeNode(rc)
  }
}

impl<T: TreeNodeHooks<T> + Debug> From<TreeNode<T>> for WeakTreeNode<T> {
  fn from(rc: TreeNode<T>) -> Self {
    WeakTreeNode(Rc::downgrade(&rc))
  }
}

impl<T: TreeNodeHooks<T> + Debug> From<&TreeNode<T>> for WeakTreeNode<T> {
  fn from(rc: &TreeNode<T>) -> Self {
    WeakTreeNode(Rc::downgrade(rc))
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

  pub fn first_child(&self) -> NullableNode<T> {
    self.first_child.borrow().clone()
  }

  pub fn last_child(&self) -> NullableNode<T> {
    self.last_child.borrow().clone()
  }

  pub fn next_sibling(&self) -> NullableNode<T> {
    self.next_sibling.borrow().clone()
  }

  pub fn prev_sibling(&self) -> NullableNode<T> {
    match self.prev_sibling.borrow().deref() {
      Some(node) => node.upgrade().map(|rc| TreeNode::from(rc)),
      _ => None,
    }
  }

  pub fn parent(&self) -> NullableNode<T> {
    match self.parent.borrow().deref() {
      Some(node) => node.upgrade().map(|rc| TreeNode::from(rc)),
      _ => None,
    }
  }

  pub fn detach(&self) {
    if let Some(prev_sibling) = self.prev_sibling() {
      prev_sibling.next_sibling.replace(self.next_sibling());
    }
    if let Some(next_sibling) = self.next_sibling() {
      next_sibling.prev_sibling.replace(self.prev_sibling.borrow().clone());
    }
    if let Some(parent) = self.parent() {
      let first_child = self.first_child().unwrap();
      let last_child = self.last_child().unwrap();

      if Rc::ptr_eq(&self, &first_child) {
        parent.first_child.replace(self.next_sibling());
      } else if Rc::ptr_eq(&self, &last_child) {
        parent.last_child.replace(self.prev_sibling());
      }
    }

    self.parent.replace(None);
    self.next_sibling.replace(None);
    self.prev_sibling.replace(None);
  }

  pub fn transfer_children_to(&self, new_parent: TreeNode<T>) {
    new_parent.first_child.replace(self.first_child());
    new_parent.last_child.replace(self.last_child());
    self.first_child.replace(None);
    self.last_child.replace(None);
  }

  pub fn find_first_anecestor<F>(&self, predicate: F) -> Option<TreeNode<T>>
  where
    F: Fn(TreeNode<T>) -> bool,
  {
    let mut parent = self.parent();
    while let Some(node) = parent {
      if predicate(node.clone()) {
        return Some(node);
      }
      parent = node.parent();
    }
    None
  }

  pub fn append_child(&self, child: TreeNode<T>) {
    if self.find_first_anecestor(|parent| Rc::ptr_eq(&child, &parent)).is_some()
    {
      panic!("Cannot append parent: {:?}", child);
    }

    // 親から切り離す
    child.detach();

    if let Some(last_child) = self.last_child() {
      last_child.next_sibling.replace(Some(child.clone()));
      child.prev_sibling.replace(Some(WeakTreeNode::from(last_child)));
    }

    child.parent.replace(Some(WeakTreeNode::from(self)));

    if self.first_child().is_none() {
      self.first_child.replace(Some(child.clone()));
    }

    self.last_child.replace(Some(child.clone()));

    // TODO: 目的がはっきりしてから
    // self.data.on_inserted(child.clone(), self.clone());
    // self.data.on_children_updated(self.clone());
  }

  pub fn insert_before(
    &self,
    child: TreeNode<T>,
    ref_child: Option<TreeNode<T>>,
  ) {
    if self.find_first_anecestor(|parent| Rc::ptr_eq(&child, &parent)).is_some()
    {
      panic!("Cannot append parent: {:?}", child);
    }

    child.detach();

    if let Some(ref_child) = ref_child {
      child.parent.replace(Some(WeakTreeNode::from(self)));

      if let Some(prev_sibling) = ref_child.prev_sibling() {
        prev_sibling.next_sibling.replace(Some(child.clone()));
        child.prev_sibling.replace(Some(WeakTreeNode::from(prev_sibling)));
      } else {
        self.first_child.replace(Some(child.clone()));
      }

      ref_child.prev_sibling.replace(Some(WeakTreeNode::from(child.clone())));
    } else {
      self.append_child(child);
    }
  }

  pub fn iterate_children(&self) -> ChildrenIterator<T> {
    ChildrenIterator::new(self.clone())
  }
}
