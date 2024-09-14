use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  With,
  compat {
    name: "statements.with",
    description: "with 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/with",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::WithStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::With;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_with_statement:{
      setup: With::default(),
      source_code: r#"
        with ([1, 2, 3]) {
          console.log(toString()); // 1,2,3
        }    
      "#,
      eq: [
        r#"with ([1, 2, 3]) {
          console.log(toString()); // 1,2,3
        }"#
      ],
      ne: []
    }
  }
}