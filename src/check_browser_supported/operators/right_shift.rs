use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsRightShift,
  compat {
    name: "operators_right_shift",
    description: "按位右移运算符 (<code>a >> b</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Right_shift",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if matches!(expr.operator, BinaryOperator::ShiftRight))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsRightShift;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_right_shift:{
      setup: OperatorsRightShift::default(),
      source_code: r#"
        console.log(8 >> 1);
        let x = 16;
        x >>= 2;
      "#,
      eq: [
        r#"8 >> 1"#,
      ],
      ne: [
        r#"x >>= 2"#,
      ]
    }
  }
}
