use oxc_ast::ast::BindingPatternKind;

use crate::create_compat_2;

create_compat_2! {
  DefaultParametersDestructuredParameterWithDefaultValueAssignment,
  compat {
    name: "default_parameters.destructured_parameter_with_default_value_assignment",
    description: "destructured parameter with default value assignment",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Default_parameters#destructured_parameter_with_default_value_assignment",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "49.0.0",
      chrome_android: "49.0.0",
      firefox: "41.0.0",
      firefox_android: "41.0.0",
      safari: "10.0.0",
      safari_ios: "10.0.0",
      edge: "14.0.0",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::FormalParameter(param) = node.kind() {
      if let BindingPatternKind::AssignmentPattern(_) = &param.pattern.kind {
        return true;
      }
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::DefaultParametersDestructuredParameterWithDefaultValueAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: DefaultParametersDestructuredParameterWithDefaultValueAssignment::default(),
      source_code: r#"
         function f({ a = 1 } = {}) {}
      "#,
      eq: [
        r#"{ a = 1 } = {}"#,
      ],
      ne: [
      ]
    },

    should_ok_when_use_class_declaration2:{
      setup: DefaultParametersDestructuredParameterWithDefaultValueAssignment::default(),
      source_code: r#"
         function f(bb,{ a = 1 } = {}) {}
      "#,
      eq: [
        r#"{ a = 1 } = {}"#,
      ],
      ne: [
      ]
    },




  }
}
