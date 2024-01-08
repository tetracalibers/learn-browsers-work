pub struct Comment {
  data: String,
}

impl Comment {
  pub fn new(data: String) -> Comment {
    Comment { data }
  }

  pub fn get_data(&self) -> &String {
    &self.data
  }
}
