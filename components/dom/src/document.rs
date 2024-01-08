use std::cell::RefCell;

pub struct Document {
  title: RefCell<String>,
  doctype: RefCell<Option<DocumentType>>,
}

pub struct DocumentType {
  pub name: String,
}

impl Document {
  pub fn new() -> Self {
    Document {
      title: RefCell::new(String::new()),
      doctype: RefCell::new(None),
    }
  }

  pub fn set_doctype(&self, doctype: DocumentType) {
    *self.doctype.borrow_mut() = Some(doctype);
  }
}

impl DocumentType {
  pub fn new(name: String) -> Self {
    DocumentType { name }
  }
}
