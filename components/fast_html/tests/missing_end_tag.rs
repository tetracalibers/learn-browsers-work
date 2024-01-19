extern crate fast_html;

use fast_html::debugger::*;

use assert_json_diff::*;
use serde_json::json;

#[test]
fn missing_p_end_tag() {
  let html = r#"<p>paragraph1<p>paragraph2"#;

  let excepted = json!(
    {
      "children": [
        {
          "children": [
            {
              "value": "paragraph1",
              "type": "text"
            }
          ],
          "tag": "p",
          "type": "element"
        },
        {
          "children": [
            {
              "value": "paragraph2",
              "type": "text"
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

  assert_json_eq!(excepted, actual);
}
