use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators::common_trait::CommonTrait,
};

pub struct FunctionsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for FunctionsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> FunctionsVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./rest_parameters.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      compat: compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for FunctionsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_formal_parameters(
    &mut self,
    it: &oxc_ast::ast::FormalParameters<'a>,
  ) {
    let code_seq = self.get_source_code(it.span).to_string();
    if it.rest.is_some() {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seq.clone(),
        compat: self.compat.clone(),
      });
    }

    walk::walk_formal_parameters(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_rest_parameters() {
    let source_code = r##"
function sum(a,b,...theArgs) {
  let total = 0;
  for (const arg of theArgs) {
    total += arg;
  }
  return total;
}    
"##;
    let allocator = Allocator::default();
    t_any(
      "rest_parameters",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }

  #[test]
  fn should_exits_rest_parameters_1() {
    let source_code = r##"
const sum = function (a,b,...theArgs) {
  let total = 0;
  for (const arg of theArgs) {
    total += arg;
  }
  return total;
}    
"##;
    let allocator = Allocator::default();
    t_any(
      "rest_parameters",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }
}
