use super::char_data::CharacterData;

pub struct Text {
  pub characters: CharacterData,
}

impl Text {
  pub fn new(data: String) -> Self {
    Self {
      characters: CharacterData::new(data),
    }
  }

  pub fn get_data(&self) -> String {
    self.characters.get_data()
  }
}
