extern crate html;

use html::debugger::*;

use assert_json_diff::*;
use serde_json::json;

#[test]
fn valid_nest() {
  let html = r#"
  <h1>This is heading</h1>
  <p>This is paragraph</p>
  <p>This <mark>keyword</mark> is important</p>
  "#;

  let expected = json!({
    "children": [
      {
        "children": [
          {
            "data": "This is heading",
            "type": "text"
          }
        ],
        "tag": "h1",
        "type": "element"
      },
      {
        "children": [
          {
            "data": "This is paragraph",
            "type": "text"
          }
        ],
        "tag": "p",
        "type": "element"
      },
      {
        "children": [
          {
            "data": "This ",
            "type": "text"
          },
          {
            "children": [
              {
                "data": "keyword",
                "type": "text"
              }
            ],
            "tag": "mark",
            "type": "element"
          },
          {
            "data": " is important",
            "type": "text"
          }
        ],
        "tag": "p",
        "type": "element"
      }
    ],
    "tag": "body",
    "type": "element"
  });

  let document = get_document_from_html(html);
  let actual = dom_in_body_to_json_obj(&document);

  assert_json_eq!(actual, expected);
}
