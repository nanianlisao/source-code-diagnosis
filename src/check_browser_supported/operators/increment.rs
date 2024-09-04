use oxc_syntax::operator::UpdateOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsIncrement,
  compat {
    name: "operators_increment",
    description: "递增运算符 (++)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Increment",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "2.0.0",
      chrome_android: "18.0.0",
      firefox: "1.0.0",
      firefox_android: "4.0.0",
      safari: "4.0.0",
      safari_ios: "4.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::UpdateExpression(expr) if matches!(expr.operator, UpdateOperator::Increment))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsIncrement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_increment_operator:{
      setup: OperatorsIncrement::default(),
      source_code: r#"
        let x = 3;
        const y = x++;
        ++x;
      "#,
      eq: [
        r#"x++"#,
        r#"++x"#,
      ],
      ne: []
    }
  }
}
