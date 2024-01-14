extern crate html;

use html::debugger::*;

#[test]
fn unordered_list() {
  let html = r#"
  <ul>
    <li>item1</li>
    <li>item2</li>
    <li>item3</li>
  </ul>
  "#;

  // |-Document
  //   |-Element { data: Unknown(HTMLElement { tag_name: "html" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "head" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "body" }) }
  //           |-Element { data: Unknown(HTMLElement { tag_name: "ul" }) }
  //               |-Element { data: Unknown(HTMLElement { tag_name: "li" }) }
  //                   |-Text("item1")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "li" }) }
  //                   |-Text("item2")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "li" }) }
  //                   |-Text("item3")

  let document = get_document_from_html(html);
  // Vec<(depth, TreeNode<Node>)>
  let dom = get_dom_list_with_depth_in_body(&document);

  assert_eq!(dom[1].0, 1);
  assert_eq!(dom[1].1.as_element().tag_name(), "ul");

  assert_eq!(dom[2].0, 2);
  assert_eq!(dom[2].1.as_element().tag_name(), "li");

  assert_eq!(dom[3].0, 3);
  assert_eq!(dom[3].1.as_text().characters.get_data(), "item1");

  assert_eq!(dom[4].0, 2);
  assert_eq!(dom[4].1.as_element().tag_name(), "li");

  assert_eq!(dom[5].0, 3);
  assert_eq!(dom[5].1.as_text().characters.get_data(), "item2");

  assert_eq!(dom[6].0, 2);
  assert_eq!(dom[6].1.as_element().tag_name(), "li");

  assert_eq!(dom[7].0, 3);
  assert_eq!(dom[7].1.as_text().characters.get_data(), "item3");
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

  // |-Document
  //   |-Element { data: Unknown(HTMLElement { tag_name: "html" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "head" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "body" }) }
  //           |-Element { data: Unknown(HTMLElement { tag_name: "ol" }) }
  //               |-Element { data: Unknown(HTMLElement { tag_name: "li" }) }
  //                   |-Text("item1")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "li" }) }
  //                   |-Text("item2")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "li" }) }
  //                   |-Text("item3")

  let document = get_document_from_html(html);
  // Vec<(depth, TreeNode<Node>)>
  let dom = get_dom_list_with_depth_in_body(&document);

  assert_eq!(dom[1].0, 1);
  assert_eq!(dom[1].1.as_element().tag_name(), "ol");

  assert_eq!(dom[2].0, 2);
  assert_eq!(dom[2].1.as_element().tag_name(), "li");

  assert_eq!(dom[3].0, 3);
  assert_eq!(dom[3].1.as_text().characters.get_data(), "item1");

  assert_eq!(dom[4].0, 2);
  assert_eq!(dom[4].1.as_element().tag_name(), "li");

  assert_eq!(dom[5].0, 3);
  assert_eq!(dom[5].1.as_text().characters.get_data(), "item2");

  assert_eq!(dom[6].0, 2);
  assert_eq!(dom[6].1.as_element().tag_name(), "li");

  assert_eq!(dom[7].0, 3);
  assert_eq!(dom[7].1.as_text().characters.get_data(), "item3");
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

  // |-Document
  //   |-Element { data: Unknown(HTMLElement { tag_name: "html" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "head" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "body" }) }
  //           |-Element { data: Unknown(HTMLElement { tag_name: "dl" }) }
  //               |-Element { data: Unknown(HTMLElement { tag_name: "dt" }) }
  //                   |-Text("name1")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "dd" }) }
  //                   |-Text("desc1")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "dt" }) }
  //                   |-Text("name2")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "dd" }) }
  //                   |-Text("desc2")

  let document = get_document_from_html(html);
  // Vec<(depth, TreeNode<Node>)>
  let dom = get_dom_list_with_depth_in_body(&document);

  assert_eq!(dom[1].0, 1);
  assert_eq!(dom[1].1.as_element().tag_name(), "dl");

  assert_eq!(dom[2].0, 2);
  assert_eq!(dom[2].1.as_element().tag_name(), "dt");

  assert_eq!(dom[3].0, 3);
  assert_eq!(dom[3].1.as_text().characters.get_data(), "name1");

  assert_eq!(dom[4].0, 2);
  assert_eq!(dom[4].1.as_element().tag_name(), "dd");

  assert_eq!(dom[5].0, 3);
  assert_eq!(dom[5].1.as_text().characters.get_data(), "desc1");

  assert_eq!(dom[6].0, 2);
  assert_eq!(dom[6].1.as_element().tag_name(), "dt");

  assert_eq!(dom[7].0, 3);
  assert_eq!(dom[7].1.as_text().characters.get_data(), "name2");

  assert_eq!(dom[8].0, 2);
  assert_eq!(dom[8].1.as_element().tag_name(), "dd");

  assert_eq!(dom[9].0, 3);
  assert_eq!(dom[9].1.as_text().characters.get_data(), "desc2");
}
