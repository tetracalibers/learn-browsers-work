mod insert_mode;
mod stack_of_open_elements;

use std::env;

use self::stack_of_open_elements::StackOfOpenElements;

use super::tokenizer::token::Token;
use super::tokenizer::Tokenizing;

use dom::comment::Comment;
use dom::document::{Document, DocumentType};
use dom::node::{Node, NodeData, NodePtr};

use tree::{TreeNode, WeakTreeNode};

use insert_mode::InsertMode;

fn is_trace() -> bool {
  match env::var("TRACE_HTML_TREE_BUILDER") {
    Ok(s) => s == "true",
    _ => false,
  }
}

macro_rules! trace {
  ($err: expr) => {
    println!("[ParseError][HTML TreeBuilding] {}", $err);
  };
}

macro_rules! emit_error {
  ($err: expr) => {
    if is_trace() {
      trace!($err);
    }
  };
}

pub struct TreeBuilder<T: Tokenizing> {
  tokenizer: T,
  insert_mode: InsertMode,
  open_elements: StackOfOpenElements,
  document: NodePtr,
  head_pointer: Option<NodePtr>,
  should_stop: bool,
}

impl<T: Tokenizing> TreeBuilder<T> {
  pub fn new(tokenizer: T, document: NodePtr) -> Self {
    TreeBuilder {
      tokenizer,
      insert_mode: InsertMode::Initial,
      open_elements: StackOfOpenElements::new(),
      document,
      head_pointer: None,
      should_stop: false,
    }
  }

  pub fn default(tokenizer: T) -> Self {
    let document = NodePtr(TreeNode::new(Node::new(NodeData::Document(
      Document::new(),
    ))));
    Self::new(tokenizer, document)
  }

  pub fn run(mut self) {
    loop {
      let token = self.tokenizer.next_token();
      println!("{:?}", token);

      self.process(token);

      if self.should_stop {
        break;
      }
    }
  }

  pub fn process(&mut self, token: Token) {
    // TODO: 仮
    if token.is_eof() {
      self.stop_parsing();
      return;
    }

    match self.insert_mode {
      InsertMode::Initial => self.process_initial(token),
      InsertMode::BeforeHtml => self.process_before_html(token),
      InsertMode::BeforeHead => self.process_before_head(token),
      InsertMode::InHead => self.process_in_head(token),
    }
  }

  fn process_initial(&mut self, token: Token) {
    match token {
      Token::Character(c) if c.is_whitespace() => {
        return;
      }
      Token::Comment(_) => {
        todo!("process_initial: Token::Comment");
      }
      Token::DOCTYPE { ref name, .. } => {
        let name = name.clone().unwrap_or_default();

        if name.as_str() != "html" {
          self.unexpected(&token);
        }

        let doctype = DocumentType::new(name);

        if let Some(doc) = self.document.as_maybe_document() {
          doc.set_doctype(doctype);
        }

        self.switch_to(InsertMode::BeforeHtml);
      }
      _ => {
        self.unexpected(&token);
        self.switch_to(InsertMode::BeforeHtml);
        self.process(token);
      }
    }
  }

  fn process_before_html(&mut self, token: Token) {
    fn anything_else<T: Tokenizing>(this: &mut TreeBuilder<T>, token: Token) {
      let element = this.create_element_for_tag_name("html");
      this.document.append_child(element.0.clone());
      this.open_elements.push(element.clone());
      this.switch_to(InsertMode::BeforeHead);
      this.process(token.clone());
    }

    if let Token::DOCTYPE { .. } = token {
      self.unexpected(&token);
      return;
    }

    if let Token::Comment(text) = token {
      let data = NodeData::Comment(Comment::new(text));
      let comment = TreeNode::new(Node::new(data));
      self.document.append_child(comment);
      return;
    }

    if token.is_start_tag() && token.tag_name() == "html" {
      let element = self.create_element(token);
      self.document.append_child(element.0.clone());
      self.open_elements.push(element.clone());
      self.switch_to(InsertMode::BeforeHead);
      return;
    }

    if token.is_end_tag()
      && token.match_tag_name_in(&["head", "body", "html", "br"])
    {
      anything_else(self, token);
      return;
    }

    if token.is_end_tag() {
      self.unexpected(&token);
      anything_else(self, token);
      return;
    }

    anything_else(self, token);
  }

  fn process_before_head(&mut self, token: Token) {
    fn anything_else<T: Tokenizing>(this: &mut TreeBuilder<T>, token: Token) {
      let head_element = this.insert_html_element(Token::Tag {
        tag_name: "head".to_owned(),
        attributes: vec![],
        is_end_tag: false,
        self_closing: false,
      });
      this.head_pointer = Some(head_element.clone());
      this.switch_to(InsertMode::InHead);
      this.process(token);
    }

    if let Token::Character(c) = token {
      if c.is_whitespace() {
        return;
      }
    }

    if let Token::Comment(text) = token {
      self.insert_comment(text);
      return;
    }

    if let Token::DOCTYPE { .. } = token {
      self.unexpected(&token);
      return;
    }

    if token.is_start_tag() && token.tag_name() == "html" {
      return self.process_in_body(token);
    }

    if token.is_start_tag() && token.tag_name() == "head" {
      let head_element = self.insert_html_element(token);
      self.head_pointer = Some(head_element);
      self.switch_to(InsertMode::InHead);
      return;
    }

    if token.is_end_tag()
      && token.match_tag_name_in(&["head", "body", "html", "br"])
    {
      anything_else(self, token);
      return;
    }

    if token.is_end_tag() {
      self.unexpected(&token);
      return;
    }

    anything_else(self, token);
  }

  fn process_in_head(&mut self, token: Token) {
    todo!("process_in_head");
  }

  fn process_in_body(&mut self, token: Token) {
    todo!("process_in_body");
  }

  /* -------------------------------------------- */

  fn stop_parsing(&mut self) {
    self.should_stop = true;
  }

  fn switch_to(&mut self, mode: InsertMode) {
    if is_trace() {
      println!("-- Builder State: switch to {:#?}", mode);
    }
    self.insert_mode = mode;
  }

  /* element ------------------------------------ */

  fn create_element(&self, tag_token: Token) -> NodePtr {
    let (tag_name, attributes) = if let Token::Tag {
      tag_name,
      attributes,
      ..
    } = tag_token
    {
      (tag_name, attributes)
    } else {
      ("".to_string(), vec![])
    };

    let element_ref =
      dom::create_element(WeakTreeNode::from(&self.document.0), &tag_name);
    let element = element_ref.as_element();

    for attr in attributes {
      element.set_attribute(&attr.name, &attr.value);
    }

    element_ref
  }

  fn create_element_for_tag_name(&self, tag_name: &str) -> NodePtr {
    self.create_element(Token::Tag {
      tag_name: tag_name.to_owned(),
      self_closing: false,
      is_end_tag: false,
      attributes: vec![],
    })
  }

  /* insert ------------------------------------- */

  fn get_appropriate_insert_position(&self, target: Option<NodePtr>) {
    todo!("get_appropriate_insert_position");
  }

  fn insert_at(&mut self, location: (), child: NodePtr) {
    todo!("insert_at");
  }

  fn insert_html_element(&mut self, token: Token) -> NodePtr {
    let insert_position = self.get_appropriate_insert_position(None);
    let element = self.create_element(token);
    let return_ref = element.clone();

    self.open_elements.push(element.clone());
    self.insert_at(insert_position, element);

    return_ref
  }

  fn insert_comment(&mut self, data: String) {
    let insert_position = self.get_appropriate_insert_position(None);
    let comment =
      TreeNode::new(Node::new(NodeData::Comment(Comment::new(data))));
    comment.set_document(WeakTreeNode::from(&self.document.0));
    self.insert_at(insert_position, NodePtr(comment));
  }

  /* -------------------------------------------- */

  fn unexpected(&self, token: &Token) {
    match token {
      Token::Tag {
        tag_name,
        is_end_tag,
        ..
      } => {
        if *is_end_tag {
          emit_error!(format!("Unexpected end tag: {}", tag_name))
        } else {
          emit_error!(format!("Unexpected start tag: {}", tag_name))
        }
      }
      Token::DOCTYPE { .. } => {
        todo!("unexpected: Token::DOCTYPE");
      }
      Token::Comment(data) => {
        emit_error!(format!("Unexpected comment: {}", data))
      }
      Token::Character(ch) => {
        emit_error!(format!("Unexpected character: {}", ch))
      }
      Token::EOF => {
        todo!("unexpected: Token::EOF");
      }
    }
  }
}
