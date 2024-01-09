mod insert_mode;
mod stack_of_open_elements;

use std::env;
use std::rc::Rc;

use self::stack_of_open_elements::StackOfOpenElements;

use super::tokenizer::state::State;
use super::tokenizer::token::Token;
use super::tokenizer::Tokenizing;

use dom::comment::Comment;
use dom::document::{Document, DocumentType};
use dom::node::{ChildrenUpdateContext, Node, NodeData, NodePtr};
use dom::text::Text;

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

pub enum AdjustedInsertionLocation {
  LastChild(NodePtr),
  BeforeSibling(NodePtr, NodePtr),
}

impl AdjustedInsertionLocation {
  pub fn parent(&self) -> &NodePtr {
    match self {
      AdjustedInsertionLocation::LastChild(parent) => parent,
      AdjustedInsertionLocation::BeforeSibling(parent, _) => parent,
    }
  }
}

enum TextOnlyElementParsingAlgorithm {
  GenericRawText,
  GenericRCDataElement,
}

pub struct TreeBuilder<T: Tokenizing> {
  tokenizer: T,
  insert_mode: InsertMode,
  original_insert_mode: Option<InsertMode>,
  open_elements: StackOfOpenElements,
  document: NodePtr,
  head_pointer: Option<NodePtr>,
  text_insertion_node: Option<NodePtr>,
  text_insertion_string_data: String,
  should_stop: bool,
  foster_parenting: bool,
  scripting: bool,
  frameset_ok: bool,
}

impl<T: Tokenizing> TreeBuilder<T> {
  pub fn new(tokenizer: T, document: NodePtr) -> Self {
    Self {
      tokenizer,
      insert_mode: InsertMode::Initial,
      original_insert_mode: None,
      open_elements: StackOfOpenElements::new(),
      document,
      head_pointer: None,
      text_insertion_node: None,
      text_insertion_string_data: String::new(),
      should_stop: false,
      foster_parenting: false,
      scripting: false,
      frameset_ok: true,
    }
  }

  pub fn get_document(&self) -> NodePtr {
    self.document.clone()
  }

  pub fn default(tokenizer: T) -> Self {
    let document = NodePtr(TreeNode::new(Node::new(NodeData::Document(
      Document::new(),
    ))));
    Self::new(tokenizer, document)
  }

  pub fn run(mut self) -> NodePtr {
    loop {
      let token = self.tokenizer.next_token();
      println!("{:?}", token);

      self.process(token);

      if self.should_stop {
        break;
      }
    }
    self.flush_text_insertion();

    self.document
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
      InsertMode::InHeadNoScript => self.process_in_head_no_script(token),
      InsertMode::AfterHead => self.process_after_head(token),
      InsertMode::InBody => self.process_in_body(token),
      InsertMode::Text => self.process_text(token),
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

    if let Token::Character(c) = token {
      if c.is_whitespace() {
        return;
      }
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
        self_closing_acknowledged: false,
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

  fn process_in_head(&mut self, mut token: Token) {
    if let Token::Character(c) = token {
      if c.is_whitespace() {
        self.insert_char(c);
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

    if token.is_start_tag()
      && token.match_tag_name_in(&["base", "basefont", "bgsound", "link"])
    {
      self.insert_html_element(token.clone());
      self.open_elements.pop();
      token.acknowledge_self_closing_if_set();
      return;
    }

    if token.is_start_tag() && token.tag_name() == "meta" {
      self.insert_html_element(token.clone());
      self.open_elements.pop();
      token.acknowledge_self_closing_if_set();
      return;
    }

    if token.is_start_tag() && token.tag_name() == "title" {
      self.handle_text_only_element(
        token,
        TextOnlyElementParsingAlgorithm::GenericRCDataElement,
      );
      return;
    }

    if token.is_start_tag() && token.tag_name() == "noscript" && !self.scripting
    {
      self.handle_text_only_element(
        token,
        TextOnlyElementParsingAlgorithm::GenericRawText,
      );
      return;
    }

    if token.is_start_tag() && token.match_tag_name_in(&["noframes", "style"]) {
      self.handle_text_only_element(
        token,
        TextOnlyElementParsingAlgorithm::GenericRawText,
      );
      return;
    }

    if token.is_start_tag() && token.tag_name() == "noscript" && self.scripting
    {
      self.insert_html_element(token.clone());
      self.switch_to(InsertMode::InHeadNoScript);
      return;
    }

    if token.is_start_tag() && token.tag_name() == "script" {
      todo!("process_in_head: script");
    }

    if token.is_end_tag() && token.tag_name() == "head" {
      self.open_elements.pop();
      self.switch_to(InsertMode::AfterHead);
      return;
    }

    if token.is_start_tag() && token.tag_name() == "template" {
      todo!("process_in_head: template start");
    }

    if token.is_end_tag() && token.tag_name() == "template" {
      todo!("process_in_head: template end");
    }

    if token.is_start_tag() && token.tag_name() == "head" {
      self.unexpected(&token);
      return;
    }

    if token.is_end_tag() {
      self.unexpected(&token);
      return;
    }

    self.open_elements.pop();
    self.switch_to(InsertMode::AfterHead);
    self.process(token);
  }

  fn process_in_head_no_script(&mut self, token: Token) {
    todo!("process_in_head_no_script");
  }

  fn process_after_head(&mut self, token: Token) {
    fn anything_else<T: Tokenizing>(this: &mut TreeBuilder<T>, token: Token) {
      this.insert_html_element(Token::Tag {
        tag_name: "body".to_owned(),
        attributes: Vec::new(),
        is_end_tag: false,
        self_closing: false,
        self_closing_acknowledged: false,
      });
      this.switch_to(InsertMode::InBody);
      this.process(token);
    }

    if let Token::Character(c) = token {
      if c.is_whitespace() {
        self.insert_char(c);
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

    if token.is_start_tag() && token.tag_name() == "body" {
      self.insert_html_element(token);
      self.frameset_ok = false;
      self.switch_to(InsertMode::InBody);
      return;
    }

    // 本来ならここでframesetタグを処理するが、framesetタグ自体非推奨なので対応しない

    if token.is_start_tag()
      && token.match_tag_name_in(&[
        "base", "basefont", "bgsound", "link", "meta", "noframes", "script",
        "style", "template", "title",
      ])
    {
      self.unexpected(&token);

      let head = self.head_pointer.clone().unwrap();
      self.open_elements.push(head.clone());
      self.process_in_head(token);
      self
        .open_elements
        .remove_first_matching_node(|node| Rc::ptr_eq(node, &head));

      return;
    }

    if token.is_end_tag() && token.tag_name() == "template" {
      return self.process_in_head(token);
    }

    if token.is_end_tag() && token.match_tag_name_in(&["body", "html", "br"]) {
      return anything_else(self, token);
    }

    if token.is_start_tag() && token.tag_name() == "head" {
      self.unexpected(&token);
      return;
    }

    if token.is_end_tag() {
      self.unexpected(&token);
      return;
    }

    anything_else(self, token);
  }

  fn process_in_body(&mut self, token: Token) {
    todo!("process_in_body");
  }

  fn process_text(&mut self, token: Token) {
    if let Token::Character(c) = token {
      self.insert_char(c);
      return;
    }

    if let Token::EOF = token {
      self.unexpected(&token);
      self.open_elements.pop();
      self.switch_to(self.original_insert_mode.clone().unwrap());
      return self.process(token);
    }

    if token.is_end_tag() && token.tag_name() == "script" {
      todo!("process_text: script");
    }

    if token.is_end_tag() {
      self.open_elements.pop();
      self.switch_to(self.original_insert_mode.clone().unwrap());
      return;
    }
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
      self_closing_acknowledged: false,
      is_end_tag: false,
      attributes: vec![],
    })
  }

  /* insert ------------------------------------- */

  fn current_node(&self) -> NodePtr {
    self.open_elements.current_node().unwrap()
  }

  fn get_appropriate_insert_position(
    &self,
    target: Option<NodePtr>,
  ) -> AdjustedInsertionLocation {
    let target = target.unwrap_or(self.current_node());

    let adjust_location = if self.foster_parenting
      && target
        .as_element()
        .match_tag_name_in(&["table", "tbody", "tfoot", "thead", "tr"])
    {
      todo!("get_appropriate_insert_position: foster parenting");
    } else {
      AdjustedInsertionLocation::LastChild(target)
    };

    adjust_location
  }

  fn get_node_for_text_insertion(
    &mut self,
    insert_position: AdjustedInsertionLocation,
  ) -> NodePtr {
    match &insert_position {
      AdjustedInsertionLocation::LastChild(parent) => {
        if let Some(last_child) = parent.last_child() {
          if last_child.as_maybe_text().is_some() {
            return NodePtr(last_child);
          }
        }
      }
      AdjustedInsertionLocation::BeforeSibling(_, sibling) => {
        if let Some(prev_sibling) = sibling.prev_sibling() {
          if prev_sibling.as_maybe_text().is_some() {
            return NodePtr(prev_sibling);
          }
        }
      }
    }

    let text_node = Node::new(NodeData::Text(Text::new(String::new())));
    let text = NodePtr(TreeNode::new(text_node));

    text.set_document(WeakTreeNode::from(&self.document.0));
    self.insert_at(insert_position, text.clone());

    text
  }

  fn insert_at(&mut self, location: AdjustedInsertionLocation, child: NodePtr) {
    match location {
      AdjustedInsertionLocation::LastChild(parent) => {
        parent.append_child(child.0);
      }
      AdjustedInsertionLocation::BeforeSibling(parent, sibling) => {
        parent.insert_before(child.0, Some(sibling.0));
      }
    }
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

  fn insert_char(&mut self, ch: char) {
    let insert_position = self.get_appropriate_insert_position(None);

    if insert_position.parent().as_maybe_document().is_some() {
      return;
    }

    let text_insertion_node = self.get_node_for_text_insertion(insert_position);

    match &self.text_insertion_node {
      Some(node) if Rc::ptr_eq(&node, &text_insertion_node) => {
        self.text_insertion_string_data.push(ch);
      }
      None => {
        self.text_insertion_node = Some(text_insertion_node);
        self.text_insertion_string_data.push(ch);
      }
      _ => {
        self.flush_text_insertion();
        self.text_insertion_node = Some(text_insertion_node);
        self.text_insertion_string_data.push(ch);
      }
    }
  }

  fn flush_text_insertion(&mut self) {
    if self.text_insertion_string_data.is_empty() {
      return;
    }

    if let Some(node) = &self.text_insertion_node {
      let text_node = node.as_text();
      text_node.characters.set_data(&self.text_insertion_string_data);

      let parent = node.parent().unwrap();
      let context = ChildrenUpdateContext {
        document: NodePtr(parent.owner_document().unwrap()),
        current_node: node.clone(),
      };

      // TODO: 目的がはっきりしてから
      // parent.data().as_ref().unwrap().handle_on_children_updated(context);

      self.text_insertion_string_data.clear();
    }
  }

  fn handle_text_only_element(
    &mut self,
    token: Token,
    algorithm: TextOnlyElementParsingAlgorithm,
  ) {
    self.insert_html_element(token);

    match algorithm {
      TextOnlyElementParsingAlgorithm::GenericRawText => {
        self.tokenizer.switch_to(State::RAWTEXT);
      }
      TextOnlyElementParsingAlgorithm::GenericRCDataElement => {
        self.tokenizer.switch_to(State::RCDATA);
      }
    }

    // テキストの解析が終わったら、元の挿入モードに戻る
    self.original_insert_mode = Some(self.insert_mode.clone());
    self.switch_to(InsertMode::Text);
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
