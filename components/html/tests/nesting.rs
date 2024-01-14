extern crate html;

use html::debugger::*;

#[test]
fn valid_nest() {
  let html = r#"
  <h1>This is heading</h1>
  <p>This is paragraph</p>
  <p>This <mark>keyword</mark> is important</p>
  "#;

  // |-Document
  //   |-Element { data: Unknown(HTMLElement { tag_name: "html" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "head" }) }
  //       |-Element { data: Unknown(HTMLElement { tag_name: "body" }) }
  //           |-Element { data: Unknown(HTMLElement { tag_name: "h1" }) }
  //               |-Text("This is heading")
  //           |-Element { data: Unknown(HTMLElement { tag_name: "p" }) }
  //               |-Text("This is paragraph")
  //           |-Element { data: Unknown(HTMLElement { tag_name: "p" }) }
  //               |-Text("This ")
  //               |-Element { data: Unknown(HTMLElement { tag_name: "mark" }) }
  //                   |-Text("keyword")
  //               |-Text(" is important")

  let document = get_document_from_html(html);
  // Vec<(depth, TreeNode<Node>)>
  let dom = get_dom_list_with_depth_in_body(&document);

  assert_eq!(dom[1].0, 1);
  assert_eq!(dom[1].1.as_element().tag_name(), "h1");

  assert_eq!(dom[2].0, 2);
  assert_eq!(dom[2].1.as_text().characters.get_data(), "This is heading");

  assert_eq!(dom[3].0, 1);
  assert_eq!(dom[3].1.as_element().tag_name(), "p");

  assert_eq!(dom[4].0, 2);
  assert_eq!(
    dom[4].1.as_text().characters.get_data(),
    "This is paragraph"
  );

  assert_eq!(dom[5].0, 1);
  assert_eq!(dom[5].1.as_element().tag_name(), "p");

  assert_eq!(dom[6].0, 2);
  assert_eq!(dom[6].1.as_text().characters.get_data(), "This ");

  assert_eq!(dom[7].0, 2);
  assert_eq!(dom[7].1.as_element().tag_name(), "mark");

  assert_eq!(dom[8].0, 3);
  assert_eq!(dom[8].1.as_text().characters.get_data(), "keyword");

  assert_eq!(dom[9].0, 2);
  assert_eq!(dom[9].1.as_text().characters.get_data(), " is important");
}
