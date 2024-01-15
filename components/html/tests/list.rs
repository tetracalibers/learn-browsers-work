extern crate html;

use html::debugger::*;

use assert_json_diff::*;
use serde_json::json;

#[test]
fn unordered_list() {
  let html = r#"
  <ul>
    <li>item1</li>
    <li>item2</li>
    <li>item3</li>
  </ul>
  "#;

  let excepted = json!(
    {
      "children": [
        {
          "children": [
            {
              "children": [
                {
                  "data": "item1",
                  "type": "text"
                }
              ],
              "tag": "li",
              "type": "element"
            },
            {
              "children": [
                {
                  "data": "item2",
                  "type": "text"
                }
              ],
              "tag": "li",
              "type": "element"
            },
            {
              "children": [
                {
                  "data": "item3",
                  "type": "text"
                }
              ],
              "tag": "li",
              "type": "element"
            }
          ],
          "tag": "ul",
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

#[test]
fn ordered_list() {
  let html = r#"
  <ol>
    <li>item1</li>
    <li>item2</li>
    <li>item3</li>
  </ol>
  "#;

  let excepted = json!(
    {
      "children": [
        {
          "children": [
            {
              "children": [
                {
                  "data": "item1",
                  "type": "text"
                }
              ],
              "tag": "li",
              "type": "element"
            },
            {
              "children": [
                {
                  "data": "item2",
                  "type": "text"
                }
              ],
              "tag": "li",
              "type": "element"
            },
            {
              "children": [
                {
                  "data": "item3",
                  "type": "text"
                }
              ],
              "tag": "li",
              "type": "element"
            }
          ],
          "tag": "ol",
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

#[test]
fn description_list() {
  let html = r#"
  <dl>
    <dt>name1</dt>
    <dd>desc1</dd>
    <dt>name2</dt>
    <dd>desc2</dd>
  </dl>
  "#;

  let excepted = json!(
    {
      "children": [
        {
          "children": [
            {
              "children": [
                {
                  "data": "name1",
                  "type": "text"
                }
              ],
              "tag": "dt",
              "type": "element"
            },
            {
              "children": [
                {
                  "data": "desc1",
                  "type": "text"
                }
              ],
              "tag": "dd",
              "type": "element"
            },
            {
              "children": [
                {
                  "data": "name2",
                  "type": "text"
                }
              ],
              "tag": "dt",
              "type": "element"
            },
            {
              "children": [
                {
                  "data": "desc2",
                  "type": "text"
                }
              ],
              "tag": "dd",
              "type": "element"
            }
          ],
          "tag": "dl",
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
