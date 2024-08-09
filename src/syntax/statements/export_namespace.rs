use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ExportNamespaceVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ExportNamespaceVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./export_namespace.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ExportNamespaceVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ExportNamespaceVisitor {
  fn visit_export_named_declaration(
    &mut self,
    it: &oxc_ast::ast::ExportNamedDeclaration<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_export_named_declaration(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "export_namespace")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ExportNamespaceVisitor::default());
    let usage = tester
      .analyze("export { myFunction as renamed } from './other_module.js';");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}