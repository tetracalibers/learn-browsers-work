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

impl Clone for NodePtr {
  fn clone(&self) -> Self {
    NodePtr(self.0.clone())
  }
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

impl NodeHooks for NodeData {
  fn on_inserted(&self, context: InsertContext) {}

  fn on_children_updated(&self, context: ChildrenUpdateContext) {}
}

impl NodeData {
  pub fn handle_on_inserted(&self, context: InsertContext) {
    self.on_inserted(context);
  }

  pub fn handle_on_children_updated(&self, context: ChildrenUpdateContext) {
    self.on_children_updated(context);
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

  pub fn data(&self) -> &Option<NodeData> {
    &self.data
  }

  pub fn set_document(&self, document: WeakTreeNode<Node>) {
    self.owner_document.replace(Some(document));
  }

  pub fn owner_document(&self) -> Option<TreeNode<Node>> {
    match self.owner_document.borrow().deref() {
      Some(node) => node.upgrade(),
      _ => None,
    }
  }

  pub fn as_maybe_document(&self) -> Option<&Document> {
    match &self.data {
      Some(NodeData::Document(doc)) => Some(doc),
      _ => None,
    }
  }

  pub fn as_maybe_element(&self) -> Option<&Element> {
    match &self.data {
      Some(NodeData::Element(element)) => Some(element),
      _ => None,
    }
  }

  pub fn as_maybe_text(&self) -> Option<&Text> {
    match &self.data {
      Some(NodeData::Text(text)) => Some(text),
      _ => None,
    }
  }

  pub fn as_element(&self) -> &Element {
    self.as_maybe_element().expect("Node is not an Element")
  }

  pub fn as_text(&self) -> &Text {
    self.as_maybe_text().expect("Node is not a Text")
  }
}
