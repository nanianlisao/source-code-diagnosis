use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },
  compat {
    name: "operators_addition",
    description: "加法运算符 (<code>+</code>)",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Addition)
  }
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::operators::addition::setup};

  assert_ok_count! {
    "operators_addition",
    setup,

    should_ok_when_use_addition,
    r#"
      console.log(2 + 2);
    "#,
    1,
  }
}
