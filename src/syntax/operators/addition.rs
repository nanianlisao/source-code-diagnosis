use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use oxc_syntax::operator::BinaryOperator;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct AdditionVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for AdditionVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./addition.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for AdditionVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for AdditionVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_binary_expression(
    &mut self,
    it: &oxc_ast::ast::BinaryExpression<'a>,
  ) {
    if matches!(it.operator, BinaryOperator::Addition) {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "operators_addition")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(AdditionVisitor::default());
    let usage = tester.analyze(
      "
console.log(2 + 2);    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
