use css::structs::declaration_value::DeclarationValue;

use super::number::Number;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Percentage(pub Number);

impl Percentage {
  pub fn parse(values: &[DeclarationValue]) -> Option<Self> {
    todo!("Percentage::parse");
  }
}
