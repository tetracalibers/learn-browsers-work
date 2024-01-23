const SPECIAL_ELEMENTS: [&[u8]; 82] = [
  b"address",
  b"applet",
  b"area",
  b"article",
  b"aside",
  b"base",
  b"basefont",
  b"bgsound",
  b"blockquote",
  b"body",
  b"br",
  b"button",
  b"caption",
  b"center",
  b"col",
  b"colgroup",
  b"dd",
  b"details",
  b"dir",
  b"div",
  b"dl",
  b"dt",
  b"embed",
  b"fieldset",
  b"figcaption",
  b"figure",
  b"footer",
  b"form",
  b"frame",
  b"frameset",
  b"h1",
  b"h2",
  b"h3",
  b"h4",
  b"h5",
  b"h6",
  b"head",
  b"header",
  b"hgroup",
  b"hr",
  b"html",
  b"iframe",
  b"img",
  b"input",
  b"keygen",
  b"li",
  b"link",
  b"listing",
  b"main",
  b"marquee",
  b"menu",
  b"meta",
  b"nav",
  b"noembed",
  b"noframes",
  b"noscript",
  b"object",
  b"ol",
  b"p",
  b"param",
  b"plaintext",
  b"pre",
  b"script",
  b"section",
  b"select",
  b"source",
  b"style",
  b"summary",
  b"table",
  b"tbody",
  b"td",
  b"template",
  b"textarea",
  b"tfoot",
  b"th",
  b"thead",
  b"title",
  b"tr",
  b"track",
  b"ul",
  b"wbr",
  b"xmp",
];

pub fn is_special_element(name: &[u8]) -> bool {
  SPECIAL_ELEMENTS.contains(&name)
}
