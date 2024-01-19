extern crate fast_html;

use fast_html::debugger::*;

use assert_json_diff::*;
use serde_json::json;

#[test]
fn valid_nest() {
  let html = r#"
  <h1>This is heading</h1>
  <p>This is paragraph</p>
  <p>This <mark>keyword</mark> is important</p>
  "#;

  let expected = json!(
    {
      "children": [
        {
          "children": [
            {
              "type": "text",
              "value": "This is heading"
            }
          ],
          "tag": "h1",
          "type": "element"
        },
        {
          "children": [
            {
              "type": "text",
              "value": "This is paragraph"
            }
          ],
          "tag": "p",
          "type": "element"
        },
        {
          "children": [
            {
              "type": "text",
              "value": "This "
            },
            {
              "children": [
                {
                  "type": "text",
                  "value": "keyword"
                }
              ],
              "tag": "mark",
              "type": "element"
            },
            {
              "type": "text",
              "value": " is important"
            }
          ],
          "tag": "p",
          "type": "element"
        }
      ],
      "tag": "body",
      "type": "element"
    }
  );

  let document = get_document_from_html(html);
  let actual = dom_body_to_json(&document);

  assert_json_eq!(actual, expected);
}
