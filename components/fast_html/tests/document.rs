extern crate fast_html;

use fast_html::debugger::*;

use assert_json_diff::*;
use serde_json::json;

#[test]
fn document_missing_head() {
  let html = r#"<!DOCTYPE html>
  <html>
  <body>
    <h1>heading</h1>
    <p>paragraph</p>
  </body>
  </html>"#;

  let expected = json!(
    {
      "children": [
        {
          "children": [
            {
              "tag": "head",
              "type": "element"
            },
            {
              "children": [
                {
                  "children": [
                    {
                      "type": "text",
                      "value": "heading"
                    }
                  ],
                  "tag": "h1",
                  "type": "element"
                },
                {
                  "children": [
                    {
                      "type": "text",
                      "value": "paragraph"
                    }
                  ],
                  "tag": "p",
                  "type": "element"
                }
              ],
              "tag": "body",
              "type": "element"
            }
          ],
          "tag": "html",
          "type": "element"
        }
      ],
      "type": "document"
    }
  );

  let document = get_document_from_html(html);
  let actual = dom_to_json(&document);

  assert_json_eq!(actual, expected);
}
