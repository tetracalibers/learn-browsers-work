extern crate fast_html;

use fast_html::debugger::*;

use assert_json_diff::*;
use serde_json::json;

#[test]
fn with_attributes() {
  let html = r#"<a href="https://example.com" target="_blank">sample link</a>"#;

  let excepted = json!(
    {
      "children": [
        {
          "attributes": {
            "href": "https://example.com",
            "target": "_blank"
          },
          "children": [
            {
              "type": "text",
              "value": "sample link"
            }
          ],
          "tag": "a",
          "type": "element"
        }
      ],
      "tag": "body",
      "type": "element"
    }
  );

  let document = get_document_from_html(html);
  let actual = dom_body_to_json(&document);

  assert_json_eq!(excepted, actual);
}
