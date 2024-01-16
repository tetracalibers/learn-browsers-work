use ecow::EcoString;

use super::char_data::CharacterData;

pub struct Text {
  pub characters: CharacterData,
}

impl Text {
  pub fn new(data: EcoString) -> Self {
    Self {
      characters: CharacterData::new(data),
    }
  }

  pub fn get_data(&self) -> EcoString {
    self.characters.get_data()
  }
}
