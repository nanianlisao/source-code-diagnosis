use oxc_syntax::operator::{BinaryOperator, LogicalOperator};

use crate::create_compat_2;

create_compat_2! {
  OperatorsLogicalOr,
  compat {
    name: "operators_logical_or",
    description: "逻辑或运算符 (||)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_OR",
    tags: ["web-features:snapshot:ecmascript-1"],
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
    matches!(node.kind(), AstKind::LogicalExpression(expr) if expr.operator == LogicalOperator::Or)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsLogicalOr;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_logical_or:{
      setup: OperatorsLogicalOr::default(),
      source_code: r#"
        console.log(true || false);
        let x = a || b;
      "#,
      eq: [
        r#"true || false"#,
        r#"a || b"#,
      ],
      ne: []
    }
  }
}
