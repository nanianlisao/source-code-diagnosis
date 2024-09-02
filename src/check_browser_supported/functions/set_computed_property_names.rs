use oxc_ast::ast::{MethodDefinitionKind, PropertyKind};

use crate::create_compat_2;

create_compat_2! {
  SetComputedPropertyNames,
  compat {
    name: "get_computed_property_names",
    description: "Get computed property names",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "46.0.0",
      chrome_android: "46.0.0",
      firefox: "34.0.0",
      firefox_android: "34.0.0",
      safari: "9.1.0",
      safari_ios: "9.1.0",
      edge: "12.0.0",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::ObjectProperty(object_property) = node.kind() {
      return matches!(object_property.kind, PropertyKind::Set) && object_property.computed;
    }

    if let AstKind::MethodDefinition(method_definition) = node.kind() {
      return matches!(method_definition.kind, MethodDefinitionKind::Set) && method_definition.computed;
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::SetComputedPropertyNames;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: SetComputedPropertyNames::default(),
      source_code: r#"
        const obj = {
          set [expr]() { return "bar"; },
      };
      "#,
      eq: [
        r#"set [expr]() { return "bar"; }"#,
      ],
      ne: []
    },

    should_ng_when_not_use_get_computed_property_names: {
      setup: SetComputedPropertyNames::default(),
      source_code: r#"
        const obj = {
          set foo(value) { return "bar"; },
        };
      "#,
      eq: [],
      ne: [
        r#"set foo(value) { return "bar"; }"#,
      ]
    },

    should_ok_when_use_set_computed_property_names_with_class_declaration: {
      setup: SetComputedPropertyNames::default(),
      source_code: r#"
        class A {
          set [expr]() { return "bar"; }
        }
      "#,
      eq: [
        r#"set [expr]() { return "bar"; }"#,
      ],
      ne: []
    }

  }
}
