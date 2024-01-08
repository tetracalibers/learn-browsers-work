use std::cell::RefCell;
use std::ops::Deref;

use tree::{TreeNode, TreeNodeHooks, WeakTreeNode};

use super::comment::Comment;
use super::document::Document;
use super::element::Element;
use super::text::Text;

pub struct NodePtr(pub TreeNode<Node>);

pub struct Node {
  owner_document: RefCell<Option<WeakTreeNode<Node>>>,
  data: Option<NodeData>,
}

pub enum NodeData {
  Element(Element),
  Text(Text),
  Document(Document),
  Comment(Comment),
}

pub struct InsertContext {
  pub document: NodePtr,
  pub current_node: NodePtr,
  pub parent_node: NodePtr,
}

pub struct ChildrenUpdateContext {
  pub document: NodePtr,
  pub current_node: NodePtr,
}

pub trait NodeHooks {
  #[allow(unused_variables)]
  fn on_inserted(&self, context: InsertContext) {}
  #[allow(unused_variables)]
  fn on_children_updated(&self, context: ChildrenUpdateContext) {}
}

impl Deref for NodePtr {
  type Target = TreeNode<Node>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl core::fmt::Debug for Node {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let inner = match &self.data {
      Some(data) => format!("{:?}", data),
      None => "[Empty Node]".to_string(),
    };
    write!(f, "{}", inner)
  }
}

impl core::fmt::Debug for NodeData {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      NodeData::Text(text) => write!(f, "Text({:?})", text.get_data()),
      NodeData::Comment(comment) => {
        write!(f, "Comment({:?})", comment.get_data())
      }
      NodeData::Document(_) => write!(f, "Document"),
      NodeData::Element(element) => write!(f, "{:?}", element),
    }
  }
}

impl TreeNodeHooks<Node> for Node {
  fn on_inserted(&self, current: TreeNode<Node>, parent: TreeNode<Node>) {
    todo!("Node::on_inserted");
  }

  fn on_children_updated(&self, current: TreeNode<Node>) {
    todo!("Node::on_children_updated");
  }
}

impl Node {
  pub fn new(data: NodeData) -> Self {
    let mut node = Self::empty();
    node.data = Some(data);
    node
  }

  pub fn empty() -> Self {
    Self {
      owner_document: RefCell::new(None),
      data: None,
    }
  }

  pub fn as_maybe_document(&self) -> Option<&Document> {
    match &self.data {
      Some(NodeData::Document(doc)) => Some(doc),
      _ => None,
    }
  }
}
