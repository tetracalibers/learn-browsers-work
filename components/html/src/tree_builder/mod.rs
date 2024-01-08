mod insert_mode;

use std::env;

use super::tokenizer::token::Token;
use super::tokenizer::Tokenizing;

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
  document: NodePtr,
  should_stop: bool,
}

impl<T: Tokenizing> TreeBuilder<T> {
  pub fn new(tokenizer: T, document: NodePtr) -> Self {
    TreeBuilder {
      tokenizer,
      insert_mode: InsertMode::Initial,
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
    todo!("process_before_html");
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
