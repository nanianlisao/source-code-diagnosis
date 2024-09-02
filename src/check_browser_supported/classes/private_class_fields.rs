use crate::create_compat_2;

create_compat_2! {
  ClassesPrivateClassFields,
  compat {
    name: "private_class_fields",
    description: "Private class fields",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Private_properties#browser_compatibility",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "74.0.0",
      chrome_android: "74.0.0",
      firefox: "90.0.0",
      firefox_android: "90.0.0",
      safari: "14.1.0",
      safari_ios: "14.1.0",
      edge: "74.0.0",
      node: "12.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::PropertyDefinition(property_definition) = node.kind() {
        if property_definition.key.is_private_identifier() {
            return true;
        }
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesPrivateClassFields;
  use crate::assert_source_seg;

  assert_source_seg! {
    test_private_class_fields:{
      setup: ClassesPrivateClassFields::default(),
      source_code: r#"
        class MyClass {
            #privateField = 42;
            #privateMethod() { return "Hello, world!"; }
            #privateMethod1 = () => {}
        }
      "#,
      eq: [
        r#"#privateField = 42;"#,
        r#"#privateMethod1 = () => {}"#,
      ],
      ne: [
        r#"#privateMethod() { return "Hello, world!"; }"#,
      ]
    }
  }
}
