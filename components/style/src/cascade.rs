// ref:
// - https://developer.mozilla.org/ja/docs/Learn/CSS/Building_blocks/Cascade_and_inheritance
// - https://developer.mozilla.org/ja/docs/Web/CSS/Cascade

use std::cmp::Ordering;

use rustc_hash::FxHashMap;

use css::structs::selector::Specificity;
use css_defs::{
  context::{CSSLocation, CascadeOrigin},
  property::Property,
  value::Value,
};

pub type Properties = FxHashMap<Property, Value>;

#[derive(Debug, Eq, PartialEq)]
struct PropertyDeclaration {
  pub value: Value,
  pub important: bool,
  pub origin: CascadeOrigin,
  pub location: CSSLocation,
  pub specificity: Specificity,
}

/// sort and get the wining value
fn cascade(declared_values: &mut Vec<PropertyDeclaration>) -> Value {
  declared_values.sort();
  declared_values.last().unwrap().value.clone()
}

impl Ord for PropertyDeclaration {
  fn cmp(&self, other: &Self) -> Ordering {
    // location > origin > specificity の順に比較
    match cmp_location(self, other) {
      Ordering::Equal => match cmp_cascade_origin(self, other) {
        Ordering::Equal => self.specificity.cmp(&other.specificity),
        other => other,
      },
      other => other,
    }
  }
}

impl PartialOrd for PropertyDeclaration {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn cmp_location(a: &PropertyDeclaration, b: &PropertyDeclaration) -> Ordering {
  match (&a.location, &b.location) {
    // style属性が最優先
    (CSSLocation::Inline, CSSLocation::Embedded) => return Ordering::Greater,
    (CSSLocation::Inline, CSSLocation::External) => return Ordering::Greater,
    // styleタグが次に優先
    (CSSLocation::Embedded, CSSLocation::External) => return Ordering::Greater,
    (CSSLocation::Embedded, CSSLocation::Inline) => return Ordering::Less,
    // 外部CSSが最後に優先
    (CSSLocation::External, CSSLocation::Inline) => return Ordering::Less,
    (CSSLocation::External, CSSLocation::Embedded) => return Ordering::Less,
    // 同じ場所の場合はEqual
    _ => return Ordering::Equal,
  }
}

// ref: https://www.w3.org/TR/css3-cascade/#cascade-origin
//
// The precedence of the various origins is, in descending order:
//
// 1. Transition declarations [css-transitions-1]
// 2. Important user agent declarations
// 3. Important user declarations
// 4. Important author declarations
// 5. Animation declarations [css-animations-1]
// 6. Normal author declarations
// 7. Normal user declarations
// 8. Normal user agent declarations
//
// Declarations from origins earlier in this list win over declarations from later origins.
fn cmp_cascade_origin(
  a: &PropertyDeclaration,
  b: &PropertyDeclaration,
) -> Ordering {
  match (a.important, b.important) {
    // importantが片方にある場合、それを優先する
    (true, false) => return Ordering::Greater,
    (false, true) => return Ordering::Less,
    // どちらもimportantの場合、step.2~4の順番で優先する
    (true, true) => match (&a.origin, &b.origin) {
      // UserAgentが最優先
      (CascadeOrigin::UserAgent, CascadeOrigin::User) => {
        return Ordering::Greater
      }
      (CascadeOrigin::UserAgent, CascadeOrigin::Author) => {
        return Ordering::Greater
      }
      // User > Author
      (CascadeOrigin::User, CascadeOrigin::Author) => return Ordering::Greater,
      // originが同じ場合はEqual
      (CascadeOrigin::UserAgent, CascadeOrigin::UserAgent) => {
        return Ordering::Equal
      }
      (CascadeOrigin::User, CascadeOrigin::User) => return Ordering::Equal,
      (CascadeOrigin::Author, CascadeOrigin::Author) => return Ordering::Equal,
      _ => return Ordering::Less,
    },
    // どちらもimportantでない場合、step.6~8の順番で優先する
    (false, false) => match (&a.origin, &b.origin) {
      // Authorが最優先
      (CascadeOrigin::Author, CascadeOrigin::User) => return Ordering::Greater,
      (CascadeOrigin::Author, CascadeOrigin::UserAgent) => {
        return Ordering::Greater
      }
      // User > UserAgent
      (CascadeOrigin::User, CascadeOrigin::UserAgent) => {
        return Ordering::Greater
      }
      // originが同じ場合はEqual
      (CascadeOrigin::Author, CascadeOrigin::Author) => return Ordering::Equal,
      (CascadeOrigin::User, CascadeOrigin::User) => return Ordering::Equal,
      (CascadeOrigin::UserAgent, CascadeOrigin::UserAgent) => {
        return Ordering::Equal
      }
      _ => return Ordering::Less,
    },
  }
}
