use crate::create_compat_2;

create_compat_2! {
  OperatorsThis,
  compat {
    name: "operators.this",
    description: "The `this` keyword refers to the object it belongs to. It has different values depending on where it is used.",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/this",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "1.0.0",
      firefox: "1.0.0",
      firefox_android: "1.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ThisExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsThis;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_this_keyword:{
      setup: OperatorsThis::default(),
      source_code: r#"
        console.log(this === window);
        const obj = {
          method() {
            return this;
          }
        };
      "#,
      eq: [
        r#"this"#,
        r#"this"#,
      ],
      ne: []
    }
  }
}
