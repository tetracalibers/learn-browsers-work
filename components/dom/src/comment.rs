use super::char_data::CharacterData;

pub struct Comment {
  characters: CharacterData,
}

impl Comment {
  pub fn new(data: String) -> Self {
    Self {
      characters: CharacterData::new(data),
    }
  }

  pub fn get_data(&self) -> String {
    self.characters.get_data()
  }
}
