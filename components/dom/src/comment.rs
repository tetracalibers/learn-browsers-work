use ecow::EcoString;

use super::char_data::CharacterData;

pub struct Comment {
  characters: CharacterData,
}

impl Comment {
  pub fn new(data: EcoString) -> Self {
    Self {
      characters: CharacterData::new(data),
    }
  }

  pub fn get_data(&self) -> EcoString {
    self.characters.get_data()
  }
}
