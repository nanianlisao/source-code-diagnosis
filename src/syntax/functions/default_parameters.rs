use oxc_ast::{
  ast::{BindingPatternKind, FunctionType},
  visit::walk,
  Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct DefaultParametersVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for DefaultParametersVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./default_parameters.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for DefaultParametersVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for DefaultParametersVisitor {
  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_semantic::ScopeFlags,
  ) {
    for item in &it.params.items {
      if matches!(item.pattern.kind, BindingPatternKind::AssignmentPattern(_)) {
        self
          .usage
          .push(CompatBox::new(item.span.clone(), self.compat.clone()));
      }
    }

    walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "functions_default_parameters")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(DefaultParametersVisitor::default());
    let usage = tester.analyze(
      "
function multiply(a, b = 1) {
  return a * b;
}
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}