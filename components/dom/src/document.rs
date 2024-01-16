use ecow::EcoString;

use std::cell::RefCell;

pub struct Document {
  title: RefCell<EcoString>,
  doctype: RefCell<Option<DocumentType>>,
}

pub struct DocumentType {
  pub name: EcoString,
}

impl Document {
  pub fn new() -> Self {
    Document {
      title: RefCell::new(EcoString::new()),
      doctype: RefCell::new(None),
    }
  }

  pub fn set_doctype(&self, doctype: DocumentType) {
    *self.doctype.borrow_mut() = Some(doctype);
  }
}

impl DocumentType {
  pub fn new(name: EcoString) -> Self {
    DocumentType { name }
  }
}
