use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat_2;

create_compat_2! {
  RightShiftAssignment,
  compat {
    name: "operators.right_shift_assignment",
    description: "右移赋值运算符 (<code>x >>= y</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Right_shift_assignment",
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
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::ShiftRight))
  }
}

#[cfg(test)]
mod tests {
  use super::RightShiftAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_right_shift_assignment:{
      setup: RightShiftAssignment::default(),
      source_code: r#"
        let num = 16;
        num >>= 2;
        console.log(num);
      "#,
      eq: [
        r#"num >>= 2"#,
      ],
      ne: []
    }
  }
}
