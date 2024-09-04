use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsInstanceof,
  compat {
    name: "operators_instanceof",
    description: "instanceof 运算符",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/instanceof",
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
    matches!(node.kind(), AstKind::BinaryExpression(binary_expr) if binary_expr.operator == BinaryOperator::Instanceof)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsInstanceof;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_instanceof_operator:{
      setup: OperatorsInstanceof::default(),
      source_code: r#"
        console.log(auto instanceof Car);
      "#,
      eq: [
        r#"auto instanceof Car"#,
      ],
      ne: []
    }
  }
}
