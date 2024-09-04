use crate::create_compat_2;

create_compat_2! {
  ClassesDeclarations,
  compat {
    name: "classes_declarations",
    description: "function declarations",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes#defining_classes",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49.0.0",
      chrome_android: "49.0.0",
      firefox: "45.0.0",
      firefox_android: "45.0.0",
      safari: "9.0.0",
      safari_ios: "9.0.0",
      edge: "13.0.0",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::Class(class) if class.is_declaration())
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesDeclarations;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: ClassesDeclarations::default(),
      source_code: r#"
        class A { constructor() { } }
        const b = class { }
      "#,
      eq: [
        r#"class A { constructor() { } }"#
      ],
      ne: [
        r#"class { }"#
      ]
    }
  }
}