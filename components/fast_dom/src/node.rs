use std::cell::RefCell;
use std::ops::Deref;

use ecow::EcoString;

use super::document::Document;
use super::element::Element;
use super::text::Text;
use super::tree::{TreeNode, WeakTreeNode};

pub struct NodePtr(pub TreeNode<DOMNode>);

pub struct DOMNode {
  owner_document: RefCell<Option<WeakTreeNode<DOMNode>>>,
  data: Option<DOMNodeData>,
}

pub enum DOMNodeData {
  Element(Element),
  Text(Text),
  Document(Document),
  Comment(EcoString),
}

impl DOMNode {
  pub fn new(data: DOMNodeData) -> Self {
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

  pub fn data(&self) -> &Option<DOMNodeData> {
    &self.data
  }

  pub fn set_document(&self, document: WeakTreeNode<DOMNode>) {
    self.owner_document.replace(Some(document));
  }

  pub fn owner_document(&self) -> Option<TreeNode<DOMNode>> {
    match self.owner_document.borrow().deref() {
      Some(node) => node.upgrade(),
      _ => None,
    }
  }

  pub fn as_maybe_document(&self) -> Option<&Document> {
    match &self.data {
      Some(DOMNodeData::Document(doc)) => Some(doc),
      _ => None,
    }
  }

  pub fn as_maybe_element(&self) -> Option<&Element> {
    match &self.data {
      Some(DOMNodeData::Element(element)) => Some(element),
      _ => None,
    }
  }

  pub fn as_maybe_comment(&self) -> Option<&EcoString> {
    match &self.data {
      Some(DOMNodeData::Comment(comment)) => Some(comment),
      _ => None,
    }
  }

  pub fn as_maybe_text(&self) -> Option<&Text> {
    match &self.data {
      Some(DOMNodeData::Text(text)) => Some(text),
      _ => None,
    }
  }

  pub fn as_element(&self) -> &Element {
    self.as_maybe_element().expect("Node is not an Element")
  }

  pub fn as_text(&self) -> &Text {
    self.as_maybe_text().expect("Node is not a Text")
  }

  pub fn is_element(&self) -> bool {
    self.as_maybe_element().is_some()
  }
}

impl Clone for NodePtr {
  fn clone(&self) -> Self {
    NodePtr(self.0.clone())
  }
}

impl Deref for NodePtr {
  type Target = TreeNode<DOMNode>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl core::fmt::Debug for DOMNode {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let inner = match &self.data {
      Some(data) => format!("{:?}", data),
      None => "[Empty Node]".to_string(),
    };
    write!(f, "{}", inner)
  }
}

impl core::fmt::Debug for NodePtr {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{:?}", self.0)
  }
}

impl core::fmt::Debug for DOMNodeData {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      DOMNodeData::Text(text) => write!(f, "Text({:?})", text.value.borrow()),
      DOMNodeData::Comment(comment) => {
        write!(f, "Comment({:?})", comment)
      }
      DOMNodeData::Document(_) => write!(f, "Document"),
      DOMNodeData::Element(element) => write!(f, "{:?}", element),
    }
  }
}
