use oxc_ast::{
  ast::{FunctionType, ObjectPropertyKind},
  visit::walk,
  AstKind, Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ObjectInitializerSpreadPropertiesVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for ObjectInitializerSpreadPropertiesVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./object_initializer_spread_properties.json"))
        .unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for ObjectInitializerSpreadPropertiesVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ObjectInitializerSpreadPropertiesVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_expression(
    &mut self,
    it: &oxc_ast::ast::ObjectExpression<'a>,
  ) {
    for p in &it.properties {
      if matches!(p, ObjectPropertyKind::SpreadProperty(_)) {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }

    walk::walk_object_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| {
        item.name == "operators_object_initializer_spread_properties"
      })
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      ObjectInitializerSpreadPropertiesVisitor::default(),
    );
    let usage = tester.analyze(
      "
const obj1 = { a: 1, b: 2 };
const obj2 = { c: 3, ...obj1 };    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}