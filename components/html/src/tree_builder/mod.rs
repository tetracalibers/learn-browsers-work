mod element_types;
mod insert_mode;
mod list_of_active_formatting_elements;
mod stack_of_open_elements;

use std::env;
use std::rc::Rc;

use self::stack_of_open_elements::StackOfOpenElements;

use self::list_of_active_formatting_elements::Entry;
use self::list_of_active_formatting_elements::ListOfActiveFormattingElements;

use self::element_types::is_special_element;

use super::tokenizer::state::State;
use super::tokenizer::token::{Attribute, Token};
use super::tokenizer::Tokenizing;

use dom::comment::Comment;
use dom::document::DocumentType;
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

enum AdoptionAgencyAlgorithmOutcome {
  DoNothing,
  RunAnyOtherEndTagsSteps,
}

pub struct TreeBuilder<T: Tokenizing> {
  tokenizer: T,
  insert_mode: InsertMode,
  original_insert_mode: Option<InsertMode>,
  open_elements: StackOfOpenElements,
  active_formatting_elements: ListOfActiveFormattingElements,
  document: NodePtr,
  head_pointer: Option<NodePtr>,
  text_insertion_node: Option<NodePtr>,
  text_insertion_string_data: String,
  should_stop: bool,
  foster_parenting: bool,
  scripting: bool,
  frameset_ok: bool,
  pending_table_character_tokens: Vec<Token>,
}

impl<T: Tokenizing> TreeBuilder<T> {
  pub fn new(tokenizer: T, document: NodePtr) -> Self {
    Self {
      tokenizer,
      insert_mode: InsertMode::Initial,
      original_insert_mode: None,
      open_elements: StackOfOpenElements::new(),
      active_formatting_elements: ListOfActiveFormattingElements::new(),
      document,
      head_pointer: None,
      text_insertion_node: None,
      text_insertion_string_data: String::new(),
      should_stop: false,
      foster_parenting: false,
      scripting: false,
      frameset_ok: true,
      pending_table_character_tokens: Vec::new(),
    }
  }

  pub fn get_document(&self) -> NodePtr {
    self.document.clone()
  }

  pub fn run(mut self) -> NodePtr {
    loop {
      let token = self.tokenizer.next_token();

      if is_trace() {
        println!("{:?}", token);
      }

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
      InsertMode::AfterBody => self.process_after_body(token),
      InsertMode::AfterAfterBody => self.process_after_after_body(token),
      InsertMode::InTable => self.process_in_table(token),
      InsertMode::InTableText => self.process_in_table_text(token),
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

    // TODO: framesetタグの扱いを決める

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

  fn process_in_body(&mut self, mut token: Token) {
    fn any_other_end_tags<T: Tokenizing>(
      this: &mut TreeBuilder<T>,
      token: Token,
    ) {
      let mut match_index = None;

      for (index, node) in this.open_elements.iter().enumerate().rev() {
        let node_tag_name = node.as_element().tag_name();

        if node_tag_name == *token.tag_name() {
          if !Rc::ptr_eq(&node, &this.current_node()) {
            this.unexpected(&token);
          }

          match_index = Some(index);
          break;
        }

        if is_special_element(&node_tag_name) {
          this.unexpected(&token);
          return;
        }
      }

      if let Some(index) = match_index {
        this.generate_implied_end_tags(token.tag_name());
        this.open_elements.pop_before_index(index);
      }
    }

    if let Token::Character(c) = token {
      if c == '\0' {
        emit_error!("Unexpected null character");
        return;
      }

      if c.is_whitespace() {
        self.reconstruct_active_formatting_elements();
        self.insert_char(c);
        return;
      }

      self.reconstruct_active_formatting_elements();
      self.insert_char(c);
      self.frameset_ok = false;
      return;
    }

    if let Token::Comment(text) = token {
      self.insert_comment(text);
      return;
    }

    if let Token::DOCTYPE { .. } = token {
      emit_error!("Unexpected DOCTYPE");
      return;
    }

    if token.is_start_tag() && token.tag_name() == "html" {
      emit_error!("Unexpected HTML tag");

      if self.open_elements.contains("template") {
        return;
      }

      let current_node = self.open_elements.current_node().unwrap();
      self.add_missing_attributes_to_node(&current_node, &token);

      return;
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&[
        "base", "basefont", "bgsound", "link", "meta", "noframes", "script",
        "style", "template", "title",
      ])
    {
      return self.process_in_head(token);
    }

    if token.is_end_tag() && token.tag_name() == "template" {
      return self.process_in_head(token);
    }

    if token.is_start_tag() && token.tag_name() == "body" {
      self.unexpected(&token);

      // 開いている要素が1つしか存在しない場合
      if self.open_elements.len() == 1 {
        return;
      }

      let body = self.open_elements.get(1);

      // 開いている要素の2番目の要素がbody要素でない場合
      if let Some(element) = body.as_maybe_element() {
        if element.tag_name() != "body" {
          return;
        }
      }

      // 開いている要素にtemplate要素が存在する場合
      if self.open_elements.contains("template") {
        return;
      }

      self.frameset_ok = false;
      self.add_missing_attributes_to_node(&body, &token);
    }

    // TODO: framesetタグの扱いを決める

    if token.is_eof() {
      // TODO: template挿入モードを扱うかどうか決める

      if self.open_elements.contains_in(&[
        "dd", "dt", "li", "optgroup", "option", "p", "rb", "rp", "rt", "rtc",
        "tbody", "td", "tfoot", "th", "thead", "tr", "body", "html",
      ]) {
        self.unexpected(&token);
      }

      self.stop_parsing();
    }

    if token.is_end_tag() && token.tag_name() == "body" {
      if !self.open_elements.has_element_name_in_scope("body") {
        self.unexpected(&token);
        return;
      }

      if self.open_elements.contains_not_in(&[
        "dd", "dt", "li", "optgroup", "option", "p", "rb", "rp", "rt", "rtc",
        "tbody", "td", "tfoot", "th", "thead", "tr", "body", "html",
      ]) {
        self.unexpected(&token);
      }

      self.switch_to(InsertMode::AfterBody);
      return;
    }

    if token.is_end_tag() && token.tag_name() == "html" {
      if self.open_elements.has_element_name_in_scope("body") {
        self.unexpected(&token);
        return;
      }

      if self.open_elements.contains_not_in(&[
        "dd", "dt", "li", "optgroup", "option", "p", "rb", "rp", "rt", "rtc",
        "tbody", "td", "tfoot", "th", "thead", "tr", "body", "html",
      ]) {
        self.unexpected(&token);
      }

      self.switch_to(InsertMode::AfterBody);
      return self.process(token);
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&[
        "address",
        "article",
        "aside",
        "blockquote",
        "center",
        "details",
        "dialog",
        "dir",
        "div",
        "dl",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "header",
        "hgroup",
        "main",
        "menu",
        "nav",
        "ol",
        "p",
        "section",
        "summary",
        "ul",
      ])
    {
      if self.open_elements.has_element_name_in_button_scope("p") {
        self.close_p_element();
      }

      self.insert_html_element(token);
      return;
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&["h1", "h2", "h3", "h4", "h5", "h6"])
    {
      if self.open_elements.has_element_name_in_button_scope("p") {
        self.close_p_element();
      }

      let current_node = self.open_elements.current_node().unwrap();
      let current_element = current_node.as_element();

      if current_element
        .match_tag_name_in(&["h1", "h2", "h3", "h4", "h5", "h6"])
      {
        self.unexpected(&token);
        self.open_elements.pop();
      }

      self.insert_html_element(token);
      return;
    }

    if token.is_start_tag() && token.match_tag_name_in(&["pre", "listing"]) {
      if self.open_elements.has_element_name_in_button_scope("p") {
        self.close_p_element();
      }

      self.insert_html_element(token);
      self.frameset_ok = false;

      let next_token = self.tokenizer.next_token();

      if let Token::Character(c) = next_token {
        if c == '\n' {
          return;
        }
        self.process(next_token);
      }

      return;
    }

    if token.is_start_tag() && token.tag_name() == "form" {
      todo!("process_in_body: form start tag");
    }

    if token.is_start_tag() && token.tag_name() == "li" {
      self.frameset_ok = false;

      for node in self.open_elements.iter().rev() {
        let element = node.as_element();
        let tag_name = element.tag_name();

        if tag_name == "li" {
          self.generate_implied_end_tags("li");

          if self.current_node().as_element().tag_name() != "li" {
            emit_error!("Expected 'li' tag");
          }

          self.open_elements.pop_until("li");
          break;
        }

        if is_special_element(&tag_name)
          && !element.match_tag_name_in(&["address", "div", "p"])
        {
          break;
        }
      }

      if self.open_elements.has_element_name_in_button_scope("p") {
        self.close_p_element();
      }

      self.insert_html_element(token);

      return;
    }

    if token.is_start_tag() && token.match_tag_name_in(&["dd", "dt"]) {
      self.frameset_ok = false;

      for node in self.open_elements.iter().rev() {
        let element = node.as_element();
        let tag_name = element.tag_name();

        if tag_name == "dd" {
          self.generate_implied_end_tags("dd");

          if self.current_node().as_element().tag_name() != "dd" {
            emit_error!("Expected 'dd' tag");
          }

          self.open_elements.pop_until("dd");

          break;
        }

        if tag_name == "dt" {
          self.generate_implied_end_tags("dt");

          if self.current_node().as_element().tag_name() != "dt" {
            emit_error!("Expected 'dt' tag");
          }

          self.open_elements.pop_until("dt");

          break;
        }

        if is_special_element(&tag_name)
          && !element.match_tag_name_in(&["address", "div", "p"])
        {
          break;
        }
      }

      if self.open_elements.has_element_name_in_button_scope("p") {
        self.close_p_element();
      }

      self.insert_html_element(token);

      return;
    }

    if token.is_start_tag() && token.tag_name() == "plaintext" {
      todo!("process_in_body: plaintext start tag");
    }

    if token.is_start_tag() && token.tag_name() == "button" {
      if self.open_elements.has_element_name_in_scope("button") {
        self.unexpected(&token);
        self.generate_implied_end_tags("");
        self.open_elements.pop_until("button");
      }

      self.reconstruct_active_formatting_elements();
      self.insert_html_element(token);
      self.frameset_ok = false;

      return;
    }

    if token.is_end_tag()
      && token.match_tag_name_in(&[
        "address",
        "article",
        "aside",
        "blockquote",
        "button",
        "center",
        "details",
        "dialog",
        "dir",
        "div",
        "dl",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "header",
        "hgroup",
        "listing",
        "main",
        "menu",
        "nav",
        "ol",
        "pre",
        "section",
        "summary",
        "ul",
      ])
    {
      if !self.open_elements.has_element_name_in_scope(&token.tag_name()) {
        self.unexpected(&token);
        return;
      }

      self.generate_implied_end_tags("");

      let current_node = self.current_node();
      let current_element = current_node.as_element();

      if current_element.tag_name() != *token.tag_name() {
        self.unexpected(&token);
        return;
      }

      self.open_elements.pop_until(&token.tag_name());
      return;
    }

    if token.is_end_tag() && token.tag_name() == "form" {
      todo!("process_in_body: form end tag");
    }

    if token.is_end_tag() && token.tag_name() == "p" {
      if !self.open_elements.has_element_name_in_button_scope("p") {
        self.unexpected(&token);
        self.insert_html_element(Token::new_start_tag_of("p"));
      }
      self.close_p_element();
      return;
    }

    if token.is_end_tag() && token.tag_name() == "li" {
      if !self.open_elements.has_element_name_in_list_item_scope("li") {
        self.unexpected(&token);
        return;
      }

      self.generate_implied_end_tags("li");

      if self.current_node().as_element().tag_name() != "li" {
        self.unexpected(&token);
      }

      self.open_elements.pop_until("li");

      return;
    }

    if token.is_end_tag() && token.match_tag_name_in(&["dd", "dt"]) {
      let tag_name = token.tag_name();

      if !self.open_elements.has_element_name_in_scope(&tag_name) {
        self.unexpected(&token);
        return;
      }

      self.generate_implied_end_tags(&tag_name);

      if self.current_node().as_element().tag_name() != *tag_name {
        self.unexpected(&token);
      }

      self.open_elements.pop_until(&tag_name);

      return;
    }

    if token.is_end_tag()
      && token.match_tag_name_in(&["h1", "h2", "h3", "h4", "h5", "h6"])
    {
      if self.open_elements.has_not_all_element_names_in_scope(&[
        "h1", "h2", "h3", "h4", "h5", "h6",
      ]) {
        self.unexpected(&token);
        return;
      }

      self.generate_implied_end_tags("");

      let current_node = self.open_elements.current_node().unwrap();
      let current_element = current_node.as_element();

      if current_element.tag_name() != *token.tag_name() {
        self.unexpected(&token);
      }

      self
        .open_elements
        .pop_until_some_in(&["h1", "h2", "h3", "h4", "h5", "h6"]);

      return;
    }

    if token.is_start_tag() && token.tag_name() == "a" {
      if let Some(element) =
        self.active_formatting_elements.get_element_after_last_marker("a")
      {
        self.unexpected(&token);
        match self.adoption_agency_algorithm(&token) {
          AdoptionAgencyAlgorithmOutcome::DoNothing => {}
          AdoptionAgencyAlgorithmOutcome::RunAnyOtherEndTagsSteps => {
            return any_other_end_tags(self, token);
          }
        }
        self.active_formatting_elements.remove_element(&element);
        self
          .open_elements
          .remove_first_matching_node(|node| Rc::ptr_eq(node, &element));
      }

      self.reconstruct_active_formatting_elements();
      let element = self.insert_html_element(token);
      self.active_formatting_elements.push(Entry::Element(element));
      return;
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&[
        "b", "big", "code", "em", "font", "i", "s", "small", "strike",
        "strong", "tt", "u",
      ])
    {
      todo!("process_in_body: formatting start tag");
    }

    if token.is_start_tag() && token.tag_name() == "nobr" {
      todo!("process_in_body: nobr start tag");
    }

    if token.is_end_tag()
      && token.match_tag_name_in(&[
        "a", "b", "big", "code", "em", "font", "i", "nobr", "s", "small",
        "strike", "strong", "tt", "u",
      ])
    {
      match self.adoption_agency_algorithm(&token) {
        AdoptionAgencyAlgorithmOutcome::RunAnyOtherEndTagsSteps => {
          any_other_end_tags(self, token)
        }
        AdoptionAgencyAlgorithmOutcome::DoNothing => {}
      }

      return;
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&["applet", "marquee", "object"])
    {
      todo!("process_in_body: applet/marquee/object start tag");
    }

    if token.is_end_tag()
      && token.match_tag_name_in(&["applet", "marquee", "object"])
    {
      todo!("process_in_body: applet/marquee/object end tag");
    }

    if token.is_start_tag() && token.tag_name() == "table" {
      // TODO: quicksモードの場合、この処理は行わない
      if self.open_elements.has_element_name_in_button_scope("p") {
        self.close_p_element();
      }

      self.insert_html_element(token);
      self.frameset_ok = false;
      self.switch_to(InsertMode::InTable);

      return;
    }

    if token.is_end_tag() && token.tag_name() == "br" {
      todo!("process_in_body: br end tag");
    }

    if token.is_start_tag()
      && token
        .match_tag_name_in(&["area", "br", "embed", "img", "keygen", "wbr"])
    {
      self.reconstruct_active_formatting_elements();
      token.acknowledge_self_closing_if_set();
      self.insert_html_element(token);
      self.open_elements.pop();
      self.frameset_ok = false;
      return;
    }

    if token.is_start_tag() && token.tag_name() == "input" {
      todo!("process_in_body: input start tag");
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&["param", "source", "track"])
    {
      todo!("process_in_body: param/source/track start tag");
    }

    if token.is_start_tag() && token.tag_name() == "hr" {
      todo!("process_in_body: hr start tag");
    }

    if token.is_start_tag() && token.tag_name() == "image" {
      todo!("process_in_body: image start tag");
    }

    if token.is_start_tag() && token.tag_name() == "textarea" {
      todo!("process_in_body: textarea start tag");
    }

    if token.is_start_tag() && token.tag_name() == "xmp" {
      todo!("process_in_body: xmp start tag");
    }

    if token.is_start_tag() && token.tag_name() == "iframe" {
      todo!("process_in_body: iframe start tag");
    }

    if token.is_start_tag() && token.tag_name() == "noembed" {
      todo!("process_in_body: noembed start tag");
    }

    if token.is_start_tag() && token.tag_name() == "noscript" && self.scripting
    {
      todo!("process_in_body: noscript start tag");
    }

    if token.is_start_tag() && token.tag_name() == "select" {
      todo!("process_in_body: select start tag");
    }

    if token.is_start_tag() && token.match_tag_name_in(&["optgroup", "option"])
    {
      todo!("process_in_body: optgroup/option start tag");
    }

    if token.is_start_tag() && token.match_tag_name_in(&["rb", "rtc"]) {
      todo!("process_in_body: rb/rtc start tag");
    }

    if token.is_start_tag() && token.match_tag_name_in(&["rp", "rt"]) {
      todo!("process_in_body: rp/rt start tag");
    }

    if token.is_start_tag() && token.tag_name() == "math" {
      todo!("process_in_body: math start tag");
    }

    if token.is_start_tag() && token.tag_name() == "svg" {
      todo!("process_in_body: svg start tag");
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&[
        "caption", "col", "colgroup", "frame", "head", "tbody", "td", "tfoot",
        "th", "thead", "tr",
      ])
    {
      todo!("process_in_body: table related start tag");
    }

    if token.is_start_tag() {
      self.reconstruct_active_formatting_elements();
      self.insert_html_element(token);
      return;
    }

    if token.is_end_tag() {
      return any_other_end_tags(self, token);
    }
  }

  fn process_after_body(&mut self, token: Token) {
    if let Token::Character(c) = token {
      if c.is_whitespace() {
        return self.process_in_body(token);
      }
    }

    if let Token::Comment(text) = token {
      let data = NodeData::Comment(Comment::new(text));
      let comment = TreeNode::new(Node::new(data));
      let first_open_element = self.open_elements.get(0);
      first_open_element.append_child(comment);
      return;
    }

    if let Token::DOCTYPE { .. } = token {
      self.unexpected(&token);
      return;
    }

    if token.is_start_tag() && token.tag_name() == "html" {
      return self.process_in_body(token);
    }

    if token.is_end_tag() && token.tag_name() == "html" {
      // TODO: フラグメント解析アルゴリズムをサポートするか決める

      self.switch_to(InsertMode::AfterAfterBody);
      return;
    }

    if token.is_eof() {
      self.stop_parsing();
      return;
    }

    self.unexpected(&token);
    self.switch_to(InsertMode::InBody);
    return self.process(token);
  }

  fn process_after_after_body(&mut self, token: Token) {
    if let Token::Comment(text) = token {
      let data = NodeData::Comment(Comment::new(text));
      let comment = TreeNode::new(Node::new(data));
      self.document.append_child(comment);
      return;
    }

    if let Token::DOCTYPE { .. } = token {
      return self.process_in_body(token);
    }

    if let Token::Character(c) = token {
      if c.is_whitespace() {
        return self.process_in_body(token);
      }
    }

    if token.is_start_tag() && token.tag_name() == "html" {
      return self.process_in_body(token);
    }

    if token.is_eof() {
      self.stop_parsing();
      return;
    }

    self.unexpected(&token);
    self.switch_to(InsertMode::InBody);
    return self.process(token);
  }

  fn process_in_table(&mut self, token: Token) {
    if let Token::Character(_) = token {
      if self
        .current_node()
        .as_element()
        .match_tag_name_in(&["table", "tbody", "tfoot", "thead", "tr"])
      {
        self.pending_table_character_tokens.clear();
        self.original_insert_mode = Some(self.insert_mode.clone());
        self.switch_to(InsertMode::InTableText);
        return self.process(token);
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

    if token.is_start_tag() && token.tag_name() == "caption" {
      todo!("process_in_table: caption start tag");
    }

    if token.is_start_tag() && token.tag_name() == "colgroup" {
      todo!("process_in_table: colgroup start tag");
    }

    if token.is_start_tag() && token.tag_name() == "col" {
      todo!("process_in_table: col start tag");
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&["tbody", "tfoot", "thead"])
    {
      todo!("process_in_table: tbody/tfoot/thead start tag");
    }

    if token.is_start_tag() && token.match_tag_name_in(&["td", "th", "tr"]) {
      todo!("process_in_table: td/th/tr start tag");
    }

    if token.is_start_tag() && token.tag_name() == "table" {
      todo!("process_in_table: table start tag");
    }

    if token.is_end_tag() && token.tag_name() == "table" {
      todo!("process_in_table: table end tag");
    }

    if token.is_end_tag()
      && token.match_tag_name_in(&[
        "body", "caption", "col", "colgroup", "html", "tbody", "td", "tfoot",
        "th", "thead", "tr",
      ])
    {
      todo!("process_in_table: body/caption/col/colgroup/html/tbody/td/tfoot/th/thead/tr end tag");
    }

    if token.is_start_tag()
      && token.match_tag_name_in(&["style", "script", "template"])
    {
      todo!("process_in_table: style/script/template start tag");
    }

    if token.is_end_tag() && token.tag_name() == "template" {
      todo!("process_in_table: template end tag");
    }

    if token.is_start_tag() && token.tag_name() == "input" {
      todo!("process_in_table: input start tag");
    }

    if token.is_start_tag() && token.tag_name() == "form" {
      todo!("process_in_table: form start tag");
    }

    if let Token::EOF = token {
      return self.process_in_body(token);
    }

    self.unexpected(&token);
    self.foster_parenting = true;
    self.process_in_body(token);
    self.foster_parenting = false;
  }

  fn process_in_table_text(&mut self, token: Token) {
    todo!("process_in_table_text");
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

  /* parse flow --------------------------------- */

  fn stop_parsing(&mut self) {
    self.should_stop = true;
  }

  fn switch_to(&mut self, mode: InsertMode) {
    if is_trace() {
      println!("-- Builder State: switch to {:#?}", mode);
    }
    self.insert_mode = mode;
  }

  /* create element ----------------------------- */

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

    for attr in attributes {
      element_ref.as_element().set_attribute(&attr.name, &attr.value);
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

  fn create_tag_token_cloned_from(&self, element: &NodePtr) -> Token {
    let element = element.as_element();
    let tag_name = element.tag_name();
    let attributes = element
      .attributes()
      .borrow()
      .iter()
      .map(|(name, value)| Attribute {
        name: name.clone(),
        value: value.clone(),
      })
      .collect();

    Token::Tag {
      tag_name,
      attributes,
      is_end_tag: false,
      self_closing: false,
      self_closing_acknowledged: false,
    }
  }

  fn create_element_cloned_from(&self, element: &NodePtr) -> NodePtr {
    let tag_token = self.create_tag_token_cloned_from(element);
    self.create_element(tag_token)
  }

  /* structure ---------------------------------- */

  fn current_node(&self) -> NodePtr {
    self.open_elements.current_node().unwrap()
  }

  fn generate_implied_end_tags(&mut self, exclude: &str) {
    while let Some(node) = self.open_elements.current_node() {
      let element = node.as_element();

      if element.tag_name() == exclude {
        break;
      }

      if element.match_tag_name_in(&[
        "dd", "dt", "li", "optgroup", "option", "p", "rb", "rt", "rtc", "rp",
      ]) {
        self.open_elements.pop();
      } else {
        break;
      }
    }
  }

  fn close_p_element(&mut self) {
    self.generate_implied_end_tags("p");

    let current_node = self.open_elements.current_node().unwrap();
    let current_element = current_node.as_element();

    if current_element.tag_name() != "p" {
      emit_error!("Expected p element");
    }

    self.open_elements.pop_until("p");
  }

  fn is_marker_or_open_element(&self, entry: &Entry) -> bool {
    match entry {
      Entry::Marker => true,
      Entry::Element(node) if self.open_elements.contains_node(node) => true,
      _ => false,
    }
  }

  fn reconstruct_active_formatting_elements(&mut self) {
    if self.active_formatting_elements.is_empty() {
      return;
    }

    let last_active_element = self.active_formatting_elements.last().unwrap();

    if self.is_marker_or_open_element(last_active_element) {
      return;
    }

    // リストの最後のエントリから始める
    let last_index = self.active_formatting_elements.len() - 1;
    let mut index = last_index;

    // Rewind step
    loop {
      // リストに前のエントリがなければ Create step へ
      if index == 0 {
        break;
      }

      // エントリを1つ前に進める
      index -= 1;
      let entry = &self.active_formatting_elements[index];

      // マーカーでも開いている要素でもなければ Rewind step へ（繰り返し）
      // マーカーか開いている要素ならば、Advance step へ

      // Advance step
      if self.is_marker_or_open_element(entry) {
        // エントリを1つ後に進める
        index += 1;
        // Create step へ
        break;
      }
    }

    // Create step
    loop {
      let element = match &self.active_formatting_elements[index] {
        Entry::Element(node) => node.clone(),
        Entry::Marker => panic!("Unexpected marker while building DOM tree!"),
      };
      let new_element =
        self.insert_html_element(self.create_tag_token_cloned_from(&element));

      // 新しい要素に置き換え
      self.active_formatting_elements[last_index] = Entry::Element(new_element);

      // 最後のエントリまで来たら終了
      if index == last_index {
        break;
      }

      // Advance step
      index += 1;
    }
  }

  /* adoption agency algorithm ------------------ */

  // formatting_elementよりもスタックの下位にあり、特別なカテゴリの要素である、オープン要素のスタックの最上位ノード
  fn find_furthest_block_and_index(
    &self,
    formatting_element: &NodePtr,
  ) -> (Option<NodePtr>, Option<usize>) {
    let mut furthest_block = None;
    let mut furthest_block_index = None;

    for (index, element) in self.open_elements.iter().rev().enumerate() {
      if Rc::ptr_eq(&element, formatting_element) {
        break;
      }

      if is_special_element(&element.as_element().tag_name()) {
        furthest_block = Some(element.clone());
        furthest_block_index = Some(index);
      }
    }

    (furthest_block, furthest_block_index)
  }

  // 開いている要素のスタックでformattingElementのすぐ上にある要素
  fn find_common_ancestor(
    &self,
    formatting_element: &NodePtr,
  ) -> Option<NodePtr> {
    let mut found_element = None;

    for (index, el) in self.open_elements.iter().rev().enumerate() {
      if Rc::ptr_eq(&el, &formatting_element) {
        if index < self.open_elements.len() - 1 {
          found_element = Some(self.open_elements.get(index - 1));
        }
        break;
      }
    }

    found_element
  }

  fn adoption_agency_algorithm(
    &mut self,
    token: &Token,
  ) -> AdoptionAgencyAlgorithmOutcome {
    let subject = token.tag_name();
    let current_node = self.current_node();

    if current_node.as_element().tag_name() == *subject {
      if !self.active_formatting_elements.contains_node(&current_node) {
        self.open_elements.pop();
        return AdoptionAgencyAlgorithmOutcome::DoNothing;
      }
    }

    let mut external_loop_counter = 0;

    loop {
      if external_loop_counter >= 8 {
        break;
      }
      external_loop_counter += 1;

      let formatting_element =
        self.active_formatting_elements.get_element_after_last_marker(subject);

      if formatting_element.is_none() {
        return AdoptionAgencyAlgorithmOutcome::RunAnyOtherEndTagsSteps;
      }

      let formatting_element = formatting_element.unwrap();

      if !self.open_elements.contains_node(&formatting_element) {
        self.unexpected(&token);
        self.active_formatting_elements.remove_element(&formatting_element);
        return AdoptionAgencyAlgorithmOutcome::DoNothing;
      }

      if !self.open_elements.has_element_in_scope(&formatting_element) {
        self.unexpected(&token);
        return AdoptionAgencyAlgorithmOutcome::DoNothing;
      }

      if !Rc::ptr_eq(&formatting_element, &current_node) {
        self.unexpected(&token);
      }

      let (mut furthest_block, mut furthest_block_index) =
        self.find_furthest_block_and_index(&formatting_element);

      if furthest_block.is_none() {
        while !Rc::ptr_eq(&current_node, &formatting_element) {
          self.open_elements.pop();
        }
        self.open_elements.pop();

        self.active_formatting_elements.remove_element(&formatting_element);
        return AdoptionAgencyAlgorithmOutcome::DoNothing;
      }

      let common_ancestor = self
        .find_common_ancestor(&formatting_element)
        .take()
        .expect("Common ancestor doesn't exists in agency adoption algorithm");

      let furthest_block = furthest_block.take().unwrap();
      let furthest_block_index = furthest_block_index.take().unwrap();

      // アクティブな書式設定要素のリスト内のformattingElementの位置を、リスト内のその両側の要素を基準にして記録
      let mut bookmark = self
        .active_formatting_elements
        .iter()
        .rposition(|el| match el {
          Entry::Element(node) => Rc::ptr_eq(node, &formatting_element),
          Entry::Marker => false,
        })
        .unwrap();

      let mut node;
      let mut node_index = furthest_block_index;
      let mut last_node = furthest_block.clone();

      let mut inner_loop_counter = 0;

      loop {
        inner_loop_counter += 1;

        node_index -= 1;
        node = self.open_elements.get(node_index);

        if Rc::ptr_eq(&node, &formatting_element) {
          break;
        }

        if inner_loop_counter > 3
          && self.active_formatting_elements.contains_node(&node)
        {
          self.active_formatting_elements.remove_element(&node);
          // continue; // TODO: これが仕様通りか調べる
        }

        if !self.active_formatting_elements.contains_node(&node) {
          self
            .open_elements
            .remove_first_matching_node(|n| Rc::ptr_eq(n, &node));
          continue;
        }

        let new_element = self.create_element_cloned_from(&node);

        // nodeが含まれていない場合はそもそもここに到達していないので、unwrapしても問題ない
        let node_index_in_formatting =
          self.active_formatting_elements.get_index_of_node(&node).unwrap();

        self.open_elements[node_index] = new_element.clone();
        self.active_formatting_elements[node_index_in_formatting] =
          Entry::Element(new_element.clone());

        node = new_element;

        if Rc::ptr_eq(&last_node, &furthest_block) {
          bookmark = node_index_in_formatting + 1;
        }

        node.append_child(last_node.0);
        last_node = node;
      }

      let insert_place =
        self.get_appropriate_insert_position(Some(common_ancestor));
      self.insert_at(insert_place, last_node);

      let new_element = self.create_element_cloned_from(&node);

      furthest_block.transfer_children_to(new_element.0.clone());
      furthest_block.append_child(new_element.0.clone());

      self.active_formatting_elements.remove_element(&formatting_element);
      self.active_formatting_elements[bookmark] =
        Entry::Element(new_element.clone());

      self
        .open_elements
        .remove_first_matching_node(|n| Rc::ptr_eq(n, &formatting_element));
      self.open_elements.insert(furthest_block_index + 1, new_element);
    }

    AdoptionAgencyAlgorithmOutcome::DoNothing
  }

  /* attribute ---------------------------------- */

  // tokenが持つ属性をelementが持っていなければ、elementに追加する
  fn add_missing_attributes_to_node(&self, node: &NodePtr, token: &Token) {
    if let Token::Tag { attributes, .. } = token {
      let element = node.as_element();
      for attr in attributes {
        if element.has_attribute(&attr.name) {
          continue;
        }
        element.set_attribute(&attr.name, &attr.value);
      }
    }
  }

  /* insert ------------------------------------- */

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
