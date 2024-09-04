use crate::create_compat_2;

create_compat_2! {
  ContinueStatement,
  compat {
    name: "statements.continue",
    description: "continue 语句终止当前循环的当前迭代中的语句执行，并继续执行下一次迭代。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/continue",
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
    matches!(node.kind(), AstKind::ContinueStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::ContinueStatement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_continue_statement:{
      setup: ContinueStatement::default(),
      source_code: r#"
        for (let i = 0; i < 10; i++) {
          if (i === 3) {
            continue;
          }
          text = text + i;
        }
      "#,
      eq: [
        r#"continue;"#,
      ],
      ne: []
    }
  }
}
