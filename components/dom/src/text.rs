pub struct Text {
  data: String,
}

impl Text {
  pub fn new(data: String) -> Text {
    Text { data }
  }

  pub fn get_data(&self) -> &String {
    &self.data
  }
}
