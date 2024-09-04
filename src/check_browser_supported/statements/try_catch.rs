use crate::create_compat_2;

create_compat_2! {
  TryCatch,
  compat {
    name: "statements.try_catch",
    description: "try...catch 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/try...catch",
    tags: ["web-features:snapshot:ecmascript-3"],
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
    matches!(node.kind(), AstKind::TryStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::TryCatch;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_try_catch:{
      setup: TryCatch::default(),
      source_code: r#"
        try {
          nonExistentFunction();
        } catch (error) {
          console.error(error);
        }
      "#,
      eq: [
        r#"try {
          nonExistentFunction();
        } catch (error) {
          console.error(error);
        }"#,
      ],
      ne: []
    }
  }
}
