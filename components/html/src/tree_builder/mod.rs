mod insert_mode;
mod stack_of_open_elements;

use std::env;

use self::stack_of_open_elements::StackOfOpenElements;

use super::tokenizer::token::Token;
use super::tokenizer::Tokenizing;

use dom::comment::Comment;
use dom::document::{Document, DocumentType};
use dom::node::{Node, NodeData, NodePtr};

use tree::TreeNode;

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
  should_stop: bool,
}

impl<T: Tokenizing> TreeBuilder<T> {
  pub fn new(tokenizer: T, document: NodePtr) -> Self {
    TreeBuilder {
      tokenizer,
      insert_mode: InsertMode::Initial,
      open_elements: StackOfOpenElements::new(),
      document,
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
    todo!("process_before_head: {:?}", token);
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

    todo!("create_element: {}", tag_name);
  }

  fn create_element_for_tag_name(&self, tag_name: &str) -> NodePtr {
    todo!("create_element_for: {}", tag_name);
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
