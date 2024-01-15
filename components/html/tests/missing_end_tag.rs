extern crate html;

use html::debugger::*;

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
              "data": "paragraph1",
              "type": "text"
            }
          ],
          "tag": "p",
          "type": "element"
        },
        {
          "children": [
            {
              "data": "paragraph2",
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
  let actual = dom_in_body_to_json_obj(&document);

  assert_json_eq!(excepted, actual);
}
