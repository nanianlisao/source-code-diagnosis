use std::{
  collections::{HashMap, HashSet},
  marker::PhantomData,
  path::PathBuf,
};

use oxc_ast::Visit;
use oxc_syntax::scope::ScopeFlags;

use super::{classes::CLASSES, compat::Compat, functions::FUNCTIONS, statements::STATEMENTS};

#[derive(Debug)]
pub struct SyntaxRecordVisitor<'a> {
  pub cache: HashSet<Compat<'a>>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> SyntaxRecordVisitor<'a> {
  pub fn new(file_path: PathBuf, danger_strings: Vec<String>) -> Self {
    Self {
      cache: HashSet::new(),
      _phantom: PhantomData {},
    }
  }
}

impl<'a> Visit<'a> for SyntaxRecordVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {}

  fn leave_node(&mut self, kind: oxc_ast::AstKind<'a>) {}

  fn enter_scope(&mut self, flags: ScopeFlags) {}

  fn leave_scope(&mut self) {}

  fn alloc<T>(&self, t: &T) -> &'a T {
    // SAFETY:
    // This should be safe as long as `src` is an reference from the allocator.
    // But honestly, I'm not really sure if this is safe.
    #[allow(unsafe_code)]
    unsafe {
      std::mem::transmute(t)
    }
  }

  // fn visit_program(&mut self, program: &oxc_ast::ast::Program<'a>) {
  //   oxc_ast::visit::walk::walk_program(self, program);
  // }

  // fn visit_statements(&mut self, stmts: &oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>) {
  //   oxc_ast::visit::walk::walk_statements(self, stmts);
  // }

  // fn visit_statement(&mut self, stmt: &oxc_ast::ast::Statement<'a>) {
  //   oxc_ast::visit::walk::walk_statement(self, stmt);
  // }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/block
  fn visit_block_statement(&mut self, stmt: &oxc_ast::ast::BlockStatement<'a>) {
    self.cache.insert(STATEMENTS.block);
    oxc_ast::visit::walk::walk_block_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/break
  fn visit_break_statement(&mut self, stmt: &oxc_ast::ast::BreakStatement<'a>) {
    self.cache.insert(STATEMENTS.r#break);
    oxc_ast::visit::walk::walk_break_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/continue
  fn visit_continue_statement(&mut self, stmt: &oxc_ast::ast::ContinueStatement<'a>) {
    self.cache.insert(STATEMENTS.r#continue);
    oxc_ast::visit::walk::walk_continue_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/debugger
  fn visit_debugger_statement(&mut self, stmt: &oxc_ast::ast::DebuggerStatement) {
    self.cache.insert(STATEMENTS.r#debugger);
    oxc_ast::visit::walk::walk_debugger_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/do...while
  fn visit_do_while_statement(&mut self, stmt: &oxc_ast::ast::DoWhileStatement<'a>) {
    self.cache.insert(STATEMENTS.do_while);
    oxc_ast::visit::walk::walk_do_while_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/Empty
  fn visit_empty_statement(&mut self, stmt: &oxc_ast::ast::EmptyStatement) {
    self.cache.insert(STATEMENTS.empty);
    oxc_ast::visit::walk::walk_empty_statement(self, stmt);
  }

  // fn visit_expression_statement(&mut self, stmt: &oxc_ast::ast::ExpressionStatement<'a>) {
  //   oxc_ast::visit::walk::walk_expression_statement(self, stmt);
  // }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for
  fn visit_for_statement(&mut self, stmt: &oxc_ast::ast::ForStatement<'a>) {
    self.cache.insert(STATEMENTS.r#for);
    oxc_ast::visit::walk::walk_for_statement(self, stmt);
  }

  // fn visit_for_statement_init(&mut self, init: &oxc_ast::ast::ForStatementInit<'a>) {
  //   self.cache.insert(STATEMENTS.r#for);
  //   oxc_ast::visit::walk::walk_for_statement_init(self, init);
  // }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for...in
  fn visit_for_in_statement(&mut self, stmt: &oxc_ast::ast::ForInStatement<'a>) {
    self.cache.insert(STATEMENTS.for_in);
    oxc_ast::visit::walk::walk_for_in_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for...of
  fn visit_for_of_statement(&mut self, stmt: &oxc_ast::ast::ForOfStatement<'a>) {
    self.cache.insert(STATEMENTS.for_of);
    if stmt.r#await {
      self.cache.insert(STATEMENTS.for_await_of);
    }
    oxc_ast::visit::walk::walk_for_of_statement(self, stmt);
  }

  // fn visit_for_statement_left(&mut self, left: &oxc_ast::ast::ForStatementLeft<'a>) {
  //   oxc_ast::visit::walk::walk_for_statement_left(self, left);
  // }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/if...else
  fn visit_if_statement(&mut self, stmt: &oxc_ast::ast::IfStatement<'a>) {
    self.cache.insert(STATEMENTS.if_else);
    oxc_ast::visit::walk::walk_if_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/label
  fn visit_labeled_statement(&mut self, stmt: &oxc_ast::ast::LabeledStatement<'a>) {
    self.cache.insert(STATEMENTS.r#label);
    oxc_ast::visit::walk::walk_labeled_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/return
  fn visit_return_statement(&mut self, stmt: &oxc_ast::ast::ReturnStatement<'a>) {
    self.cache.insert(STATEMENTS.r#return);
    oxc_ast::visit::walk::walk_return_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/switch
  fn visit_switch_statement(&mut self, stmt: &oxc_ast::ast::SwitchStatement<'a>) {
    self.cache.insert(STATEMENTS.r#switch);
    oxc_ast::visit::walk::walk_switch_statement(self, stmt);
  }

  // fn visit_switch_case(&mut self, case: &oxc_ast::ast::SwitchCase<'a>) {
  //   self.cache.insert(STATEMENTS.switch);
  //   oxc_ast::visit::walk::walk_switch_case(self, case);
  // }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/throw
  fn visit_throw_statement(&mut self, stmt: &oxc_ast::ast::ThrowStatement<'a>) {
    self.cache.insert(STATEMENTS.r#throw);
    oxc_ast::visit::walk::walk_throw_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/try...catch
  fn visit_try_statement(&mut self, stmt: &oxc_ast::ast::TryStatement<'a>) {
    self.cache.insert(STATEMENTS.try_catch);
    oxc_ast::visit::walk::walk_try_statement(self, stmt);
  }

  // fn visit_catch_clause(&mut self, clause: &oxc_ast::ast::CatchClause<'a>) {
  //   self.cache.insert(STATEMENTS.try_catch);
  //   oxc_ast::visit::walk::walk_catch_clause(self, clause);
  // }

  // fn visit_catch_parameter(&mut self, param: &oxc_ast::ast::CatchParameter<'a>) {
  //   self.cache.insert(STATEMENTS.try_catch);
  //   oxc_ast::visit::walk::walk_catch_parameter(self, param);
  // }

  // fn visit_finally_clause(&mut self, clause: &oxc_ast::ast::BlockStatement<'a>) {
  //   self.cache.insert(STATEMENTS.block);
  //   oxc_ast::visit::walk::walk_finally_clause(self, clause);
  // }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/while
  fn visit_while_statement(&mut self, stmt: &oxc_ast::ast::WhileStatement<'a>) {
    self.cache.insert(STATEMENTS.r#while);
    oxc_ast::visit::walk::walk_while_statement(self, stmt);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/with
  fn visit_with_statement(&mut self, stmt: &oxc_ast::ast::WithStatement<'a>) {
    self.cache.insert(STATEMENTS.r#with);
    oxc_ast::visit::walk::walk_with_statement(self, stmt);
  }

  // fn visit_directive(&mut self, directive: &oxc_ast::ast::Directive<'a>) {
  //   oxc_ast::visit::walk::walk_directive(self, directive);
  // }

  fn visit_variable_declaration(&mut self, decl: &oxc_ast::ast::VariableDeclaration<'a>) {
    if decl.kind == oxc_ast::ast::VariableDeclarationKind::Const {
      // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/const
      self.cache.insert(STATEMENTS.r#const);
    }
    if decl.kind == oxc_ast::ast::VariableDeclarationKind::Let {
      // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/let
      self.cache.insert(STATEMENTS.r#let);
    }
    if decl.kind == oxc_ast::ast::VariableDeclarationKind::Var {
      // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/var
      self.cache.insert(STATEMENTS.r#var);
    }

    oxc_ast::visit::walk::walk_variable_declaration(self, decl);
  }

  // fn visit_variable_declarator(&mut self, declarator: &oxc_ast::ast::VariableDeclarator<'a>) {
  //   if declarator.kind == oxc_ast::ast::VariableDeclarationKind::Const {
  //     self.cache.insert(STATEMENTS.r#const);
  //   }
  //   if declarator.kind == oxc_ast::ast::VariableDeclarationKind::Let {
  //     self.cache.insert(STATEMENTS.r#let);
  //   }
  //   if declarator.kind == oxc_ast::ast::VariableDeclarationKind::Var {
  //     self.cache.insert(STATEMENTS.r#var);
  //   }

  //   oxc_ast::visit::walk::walk_variable_declarator(self, declarator);
  // }

  fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, flags: Option<ScopeFlags>) {
    // self.cache.insert(STATEMENTS.function);

    // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions
    self.cache.insert(FUNCTIONS.functions);
    match (func.r#async, func.generator) {
      (true, true) => {
        // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/async_function*
        self.cache.insert(STATEMENTS.async_generator_function);
      }
      (true, false) => {
        // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/async_function
        self.cache.insert(STATEMENTS.async_function);
      }
      (false, true) => {
        // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/function*
        self.cache.insert(STATEMENTS.generator_function);
      }
      _ => {}
    };

    oxc_ast::visit::walk::walk_function(self, func, flags);
  }

  // fn visit_function_body(&mut self, body: &oxc_ast::ast::FunctionBody<'a>) {
  //   self.cache.insert(STATEMENTS.function);
  //   oxc_ast::visit::walk::walk_function_body(self, body);
  // }

  // fn visit_formal_parameters(&mut self, params: &oxc_ast::ast::FormalParameters<'a>) {
  //   oxc_ast::visit::walk::walk_formal_parameters(self, params);
  // }

  // fn visit_formal_parameter(&mut self, param: &oxc_ast::ast::FormalParameter<'a>) {
  //   oxc_ast::visit::walk::walk_formal_parameter(self, param);
  // }

  fn visit_decorator(&mut self, decorator: &oxc_ast::ast::Decorator<'a>) {
    // 🤔  mdn none ???
    oxc_ast::visit::walk::walk_decorator(self, decorator);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/class
  fn visit_class(&mut self, class: &oxc_ast::ast::Class<'a>) {
    self.cache.insert(STATEMENTS.class);
    oxc_ast::visit::walk::walk_class(self, class);
  }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/extends
  fn visit_class_heritage(&mut self, expr: &oxc_ast::ast::Expression<'a>) {
    self.cache.insert(CLASSES.extends);
    oxc_ast::visit::walk::walk_class_heritage(self, expr);
  }

  // fn visit_ts_class_implements(&mut self, expr: &oxc_ast::ast::TSClassImplements<'a>) {
  //   oxc_ast::visit::walk::walk_ts_class_implements(self, expr);
  // }

  // fn visit_class_body(&mut self, body: &oxc_ast::ast::ClassBody<'a>) {
  //   self.cache.insert(STATEMENTS.class);
  //   oxc_ast::visit::walk::walk_class_body(self, body);
  // }

  // fn visit_class_element(&mut self, elem: &oxc_ast::ast::ClassElement<'a>) {
  //   oxc_ast::visit::walk::walk_class_element(self, elem);
  // }

  // https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/static
  fn visit_static_block(&mut self, block: &oxc_ast::ast::StaticBlock<'a>) {
    self.cache.insert(CLASSES.static_initialization_blocks);
    oxc_ast::visit::walk::walk_static_block(self, block);
  }

  fn visit_method_definition(&mut self, def: &oxc_ast::ast::MethodDefinition<'a>) {
    if def.r#static {
      self.cache.insert(CLASSES.r#static);
    }
    // if def.kind == oxc_ast::ast::MethodDefinitionKind::Method {
    //   self.cache.insert(FUNCTIONS.method);
    // }
    oxc_ast::visit::walk::walk_method_definition(self, def);
  }

  fn visit_property_definition(&mut self, def: &oxc_ast::ast::PropertyDefinition<'a>) {
    println!("property_definition");
    oxc_ast::visit::walk::walk_property_definition(self, def);
  }

  fn visit_using_declaration(&mut self, decl: &oxc_ast::ast::UsingDeclaration<'a>) {
    oxc_ast::visit::walk::walk_using_declaration(self, decl);
  }

  // fn visit_expression(&mut self, expr: &oxc_ast::ast::Expression<'a>) {
  //   oxc_ast::visit::walk::walk_expression(self, expr);
  // }

  fn visit_meta_property(&mut self, meta: &oxc_ast::ast::MetaProperty<'a>) {
    oxc_ast::visit::walk::walk_meta_property(self, meta);
  }

  fn visit_array_expression(&mut self, expr: &oxc_ast::ast::ArrayExpression<'a>) {
    oxc_ast::visit::walk::walk_array_expression(self, expr);
  }

  fn visit_array_expression_element(&mut self, arg: &oxc_ast::ast::ArrayExpressionElement<'a>) {
    oxc_ast::visit::walk::walk_array_expression_element(self, arg);
  }

  fn visit_argument(&mut self, arg: &oxc_ast::ast::Argument<'a>) {
    oxc_ast::visit::walk::walk_argument(self, arg);
  }

  fn visit_spread_element(&mut self, elem: &oxc_ast::ast::SpreadElement<'a>) {
    oxc_ast::visit::walk::walk_spread_element(self, elem);
  }

  fn visit_expression_array_element(&mut self, expr: &oxc_ast::ast::Expression<'a>) {
    oxc_ast::visit::walk::walk_expression_array_element(self, expr);
  }

  fn visit_elision(&mut self, elision: &oxc_ast::ast::Elision) {
    oxc_ast::visit::walk::walk_elision(self, elision);
  }

  fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
    oxc_ast::visit::walk::walk_assignment_expression(self, expr);
  }

  fn visit_arrow_expression(&mut self, expr: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
    oxc_ast::visit::walk::walk_arrow_expression(self, expr);
  }

  fn visit_await_expression(&mut self, expr: &oxc_ast::ast::AwaitExpression<'a>) {
    oxc_ast::visit::walk::walk_await_expression(self, expr);
  }

  fn visit_binary_expression(&mut self, expr: &oxc_ast::ast::BinaryExpression<'a>) {
    oxc_ast::visit::walk::walk_binary_expression(self, expr);
  }

  fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
    oxc_ast::visit::walk::walk_call_expression(self, expr);
  }

  fn visit_chain_expression(&mut self, expr: &oxc_ast::ast::ChainExpression<'a>) {
    oxc_ast::visit::walk::walk_chain_expression(self, expr);
  }

  fn visit_chain_element(&mut self, elem: &oxc_ast::ast::ChainElement<'a>) {
    oxc_ast::visit::walk::walk_chain_element(self, elem);
  }

  fn visit_conditional_expression(&mut self, expr: &oxc_ast::ast::ConditionalExpression<'a>) {
    oxc_ast::visit::walk::walk_conditional_expression(self, expr);
  }

  fn visit_import_expression(&mut self, expr: &oxc_ast::ast::ImportExpression<'a>) {
    oxc_ast::visit::walk::walk_import_expression(self, expr);
  }

  fn visit_logical_expression(&mut self, expr: &oxc_ast::ast::LogicalExpression<'a>) {
    oxc_ast::visit::walk::walk_logical_expression(self, expr);
  }

  fn visit_member_expression(&mut self, expr: &oxc_ast::ast::MemberExpression<'a>) {
    oxc_ast::visit::walk::walk_member_expression(self, expr);
  }

  fn visit_computed_member_expression(
    &mut self,
    expr: &oxc_ast::ast::ComputedMemberExpression<'a>,
  ) {
    oxc_ast::visit::walk::walk_computed_member_expression(self, expr);
  }

  fn visit_static_member_expression(&mut self, expr: &oxc_ast::ast::StaticMemberExpression<'a>) {
    oxc_ast::visit::walk::walk_static_member_expression(self, expr);
  }

  fn visit_private_field_expression(&mut self, expr: &oxc_ast::ast::PrivateFieldExpression<'a>) {
    oxc_ast::visit::walk::walk_private_field_expression(self, expr);
  }

  fn visit_new_expression(&mut self, expr: &oxc_ast::ast::NewExpression<'a>) {
    oxc_ast::visit::walk::walk_new_expression(self, expr);
  }

  fn visit_object_expression(&mut self, expr: &oxc_ast::ast::ObjectExpression<'a>) {
    oxc_ast::visit::walk::walk_object_expression(self, expr);
  }

  fn visit_object_property_kind(&mut self, prop: &oxc_ast::ast::ObjectPropertyKind<'a>) {
    oxc_ast::visit::walk::walk_object_property_kind(self, prop);
  }

  fn visit_object_property(&mut self, prop: &oxc_ast::ast::ObjectProperty<'a>) {
    oxc_ast::visit::walk::walk_object_property(self, prop);
  }

  fn visit_property_key(&mut self, key: &oxc_ast::ast::PropertyKey<'a>) {
    oxc_ast::visit::walk::walk_property_key(self, key);
  }

  fn visit_parenthesized_expression(&mut self, expr: &oxc_ast::ast::ParenthesizedExpression<'a>) {
    oxc_ast::visit::walk::walk_parenthesized_expression(self, expr);
  }

  fn visit_private_in_expression(&mut self, expr: &oxc_ast::ast::PrivateInExpression<'a>) {
    oxc_ast::visit::walk::walk_private_in_expression(self, expr);
  }

  fn visit_sequence_expression(&mut self, expr: &oxc_ast::ast::SequenceExpression<'a>) {
    oxc_ast::visit::walk::walk_sequence_expression(self, expr);
  }

  fn visit_tagged_template_expression(
    &mut self,
    expr: &oxc_ast::ast::TaggedTemplateExpression<'a>,
  ) {
    oxc_ast::visit::walk::walk_tagged_template_expression(self, expr);
  }

  fn visit_this_expression(&mut self, expr: &oxc_ast::ast::ThisExpression) {
    oxc_ast::visit::walk::walk_this_expression(self, expr);
  }

  fn visit_unary_expression(&mut self, expr: &oxc_ast::ast::UnaryExpression<'a>) {
    oxc_ast::visit::walk::walk_unary_expression(self, expr);
  }

  fn visit_update_expression(&mut self, expr: &oxc_ast::ast::UpdateExpression<'a>) {
    oxc_ast::visit::walk::walk_update_expression(self, expr);
  }

  fn visit_yield_expression(&mut self, expr: &oxc_ast::ast::YieldExpression<'a>) {
    oxc_ast::visit::walk::walk_yield_expression(self, expr);
  }

  fn visit_super(&mut self, expr: &oxc_ast::ast::Super) {
    oxc_ast::visit::walk::walk_super(self, expr);
  }

  fn visit_assignment_target(&mut self, target: &oxc_ast::ast::AssignmentTarget<'a>) {
    oxc_ast::visit::walk::walk_assignment_target(self, target);
  }

  fn visit_simple_assignment_target(&mut self, target: &oxc_ast::ast::SimpleAssignmentTarget<'a>) {
    oxc_ast::visit::walk::walk_simple_assignment_target(self, target);
  }

  fn visit_assignment_target_pattern(&mut self, pat: &oxc_ast::ast::AssignmentTargetPattern<'a>) {
    oxc_ast::visit::walk::walk_assignment_target_pattern(self, pat);
  }

  fn visit_array_assignment_target(&mut self, target: &oxc_ast::ast::ArrayAssignmentTarget<'a>) {
    oxc_ast::visit::walk::walk_array_assignment_target(self, target);
  }

  fn visit_assignment_target_maybe_default(
    &mut self,
    target: &oxc_ast::ast::AssignmentTargetMaybeDefault<'a>,
  ) {
    oxc_ast::visit::walk::walk_assignment_target_maybe_default(self, target);
  }

  fn visit_assignment_target_with_default(
    &mut self,
    target: &oxc_ast::ast::AssignmentTargetWithDefault<'a>,
  ) {
    oxc_ast::visit::walk::walk_assignment_target_with_default(self, target);
  }

  fn visit_object_assignment_target(&mut self, target: &oxc_ast::ast::ObjectAssignmentTarget<'a>) {
    oxc_ast::visit::walk::walk_object_assignment_target(self, target);
  }

  fn visit_assignment_target_property(
    &mut self,
    property: &oxc_ast::ast::AssignmentTargetProperty<'a>,
  ) {
    oxc_ast::visit::walk::walk_assignment_target_property(self, property);
  }

  fn visit_assignment_target_property_identifier(
    &mut self,
    ident: &oxc_ast::ast::AssignmentTargetPropertyIdentifier<'a>,
  ) {
    oxc_ast::visit::walk::walk_assignment_target_property_identifier(self, ident);
  }

  fn visit_assignment_target_property_property(
    &mut self,
    property: &oxc_ast::ast::AssignmentTargetPropertyProperty<'a>,
  ) {
    oxc_ast::visit::walk::walk_assignment_target_property_property(self, property);
  }

  fn visit_assignment_target_rest(&mut self, rest: &oxc_ast::ast::AssignmentTargetRest<'a>) {
    oxc_ast::visit::walk::walk_assignment_target_rest(self, rest);
  }

  fn visit_jsx_element(&mut self, elem: &oxc_ast::ast::JSXElement<'a>) {
    oxc_ast::visit::walk::walk_jsx_element(self, elem);
  }

  fn visit_jsx_opening_element(&mut self, elem: &oxc_ast::ast::JSXOpeningElement<'a>) {
    oxc_ast::visit::walk::walk_jsx_opening_element(self, elem);
  }

  fn visit_jsx_closing_element(&mut self, elem: &oxc_ast::ast::JSXClosingElement<'a>) {
    oxc_ast::visit::walk::walk_jsx_closing_element(self, elem);
  }

  fn visit_jsx_element_name(&mut self, name: &oxc_ast::ast::JSXElementName<'a>) {
    oxc_ast::visit::walk::walk_jsx_element_name(self, name);
  }

  fn visit_jsx_identifier(&mut self, ident: &oxc_ast::ast::JSXIdentifier<'a>) {
    oxc_ast::visit::walk::walk_jsx_identifier(self, ident);
  }

  fn visit_jsx_member_expression(&mut self, expr: &oxc_ast::ast::JSXMemberExpression<'a>) {
    oxc_ast::visit::walk::walk_jsx_member_expression(self, expr);
  }

  fn visit_jsx_member_expression_object(
    &mut self,
    expr: &oxc_ast::ast::JSXMemberExpressionObject<'a>,
  ) {
    oxc_ast::visit::walk::walk_jsx_member_expression_object(self, expr);
  }

  fn visit_jsx_namespaced_name(&mut self, name: &oxc_ast::ast::JSXNamespacedName<'a>) {
    oxc_ast::visit::walk::walk_jsx_namespaced_name(self, name);
  }

  fn visit_jsx_attribute_item(&mut self, item: &oxc_ast::ast::JSXAttributeItem<'a>) {
    oxc_ast::visit::walk::walk_jsx_attribute_item(self, item);
  }

  fn visit_jsx_attribute(&mut self, attribute: &oxc_ast::ast::JSXAttribute<'a>) {
    oxc_ast::visit::walk::walk_jsx_attribute(self, attribute);
  }

  fn visit_jsx_spread_attribute(&mut self, attribute: &oxc_ast::ast::JSXSpreadAttribute<'a>) {
    oxc_ast::visit::walk::walk_jsx_spread_attribute(self, attribute);
  }

  fn visit_jsx_attribute_value(&mut self, value: &oxc_ast::ast::JSXAttributeValue<'a>) {
    oxc_ast::visit::walk::walk_jsx_attribute_value(self, value);
  }

  fn visit_jsx_expression_container(&mut self, expr: &oxc_ast::ast::JSXExpressionContainer<'a>) {
    oxc_ast::visit::walk::walk_jsx_expression_container(self, expr);
  }

  fn visit_jsx_expression(&mut self, expr: &oxc_ast::ast::JSXExpression<'a>) {
    oxc_ast::visit::walk::walk_jsx_expression(self, expr);
  }

  fn visit_jsx_fragment(&mut self, elem: &oxc_ast::ast::JSXFragment<'a>) {
    oxc_ast::visit::walk::walk_jsx_fragment(self, elem);
  }

  fn visit_jsx_child(&mut self, child: &oxc_ast::ast::JSXChild<'a>) {
    oxc_ast::visit::walk::walk_jsx_child(self, child);
  }

  fn visit_jsx_spread_child(&mut self, child: &oxc_ast::ast::JSXSpreadChild<'a>) {
    oxc_ast::visit::walk::walk_jsx_spread_child(self, child);
  }

  fn visit_jsx_text(&mut self, child: &oxc_ast::ast::JSXText<'a>) {
    oxc_ast::visit::walk::walk_jsx_text(self, child);
  }

  fn visit_binding_pattern(&mut self, pat: &oxc_ast::ast::BindingPattern<'a>) {
    oxc_ast::visit::walk::walk_binding_pattern(self, pat);
  }

  fn visit_binding_identifier(&mut self, ident: &oxc_ast::ast::BindingIdentifier<'a>) {
    oxc_ast::visit::walk::walk_binding_identifier(self, ident);
  }

  fn visit_object_pattern(&mut self, pat: &oxc_ast::ast::ObjectPattern<'a>) {
    oxc_ast::visit::walk::walk_object_pattern(self, pat);
  }

  fn visit_binding_property(&mut self, prop: &oxc_ast::ast::BindingProperty<'a>) {
    oxc_ast::visit::walk::walk_binding_property(self, prop);
  }

  fn visit_array_pattern(&mut self, pat: &oxc_ast::ast::ArrayPattern<'a>) {
    oxc_ast::visit::walk::walk_array_pattern(self, pat);
  }

  fn visit_rest_element(&mut self, pat: &oxc_ast::ast::BindingRestElement<'a>) {
    oxc_ast::visit::walk::walk_rest_element(self, pat);
  }

  fn visit_assignment_pattern(&mut self, pat: &oxc_ast::ast::AssignmentPattern<'a>) {
    self
      .cache
      .insert(FUNCTIONS.default_parameters_parameters_without_defaults_after_default_parameters);
    oxc_ast::visit::walk::walk_assignment_pattern(self, pat);
  }

  fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
    oxc_ast::visit::walk::walk_identifier_reference(self, ident);
  }

  fn visit_private_identifier(&mut self, ident: &oxc_ast::ast::PrivateIdentifier<'a>) {
    oxc_ast::visit::walk::walk_private_identifier(self, ident);
  }

  fn visit_label_identifier(&mut self, ident: &oxc_ast::ast::LabelIdentifier<'a>) {
    oxc_ast::visit::walk::walk_label_identifier(self, ident);
  }

  fn visit_identifier_name(&mut self, ident: &oxc_ast::ast::IdentifierName<'a>) {
    oxc_ast::visit::walk::walk_identifier_name(self, ident);
  }

  fn visit_number_literal(&mut self, lit: &oxc_ast::ast::NumericLiteral<'a>) {
    oxc_ast::visit::walk::walk_number_literal(self, lit);
  }

  fn visit_boolean_literal(&mut self, lit: &oxc_ast::ast::BooleanLiteral) {
    oxc_ast::visit::walk::walk_boolean_literal(self, lit);
  }

  fn visit_null_literal(&mut self, lit: &oxc_ast::ast::NullLiteral) {
    oxc_ast::visit::walk::walk_null_literal(self, lit);
  }

  fn visit_bigint_literal(&mut self, lit: &oxc_ast::ast::BigIntLiteral<'a>) {
    oxc_ast::visit::walk::walk_bigint_literal(self, lit);
  }

  fn visit_string_literal(&mut self, lit: &oxc_ast::ast::StringLiteral<'a>) {
    oxc_ast::visit::walk::walk_string_literal(self, lit);
  }

  fn visit_template_literal(&mut self, lit: &oxc_ast::ast::TemplateLiteral<'a>) {
    oxc_ast::visit::walk::walk_template_literal(self, lit);
  }

  fn visit_reg_expr_literal(&mut self, lit: &oxc_ast::ast::RegExpLiteral<'a>) {
    oxc_ast::visit::walk::walk_reg_expr_literal(self, lit);
  }

  fn visit_template_element(&mut self, elem: &oxc_ast::ast::TemplateElement) {
    oxc_ast::visit::walk::walk_template_element(self, elem);
  }

  fn visit_module_declaration(&mut self, decl: &oxc_ast::ast::ModuleDeclaration<'a>) {
    oxc_ast::visit::walk::walk_module_declaration(self, decl);
  }

  fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration<'a>) {
    oxc_ast::visit::walk::walk_import_declaration(self, decl);
  }

  fn visit_with_clause(&mut self, with_clause: &oxc_ast::ast::WithClause<'a>) {
    oxc_ast::visit::walk::walk_with_clause(self, with_clause);
  }

  fn visit_import_attribute(&mut self, attribute: &oxc_ast::ast::ImportAttribute<'a>) {
    oxc_ast::visit::walk::walk_import_attribute(self, attribute);
  }

  fn visit_import_attribute_key(&mut self, key: &oxc_ast::ast::ImportAttributeKey<'a>) {
    oxc_ast::visit::walk::walk_import_attribute_key(self, key);
  }

  fn visit_import_declaration_specifier(
    &mut self,
    specifier: &oxc_ast::ast::ImportDeclarationSpecifier<'a>,
  ) {
    oxc_ast::visit::walk::walk_import_declaration_specifier(self, specifier);
  }

  fn visit_import_specifier(&mut self, specifier: &oxc_ast::ast::ImportSpecifier<'a>) {
    oxc_ast::visit::walk::walk_import_specifier(self, specifier);
  }

  fn visit_import_default_specifier(
    &mut self,
    specifier: &oxc_ast::ast::ImportDefaultSpecifier<'a>,
  ) {
    oxc_ast::visit::walk::walk_import_default_specifier(self, specifier);
  }

  fn visit_import_name_specifier(
    &mut self,
    specifier: &oxc_ast::ast::ImportNamespaceSpecifier<'a>,
  ) {
    oxc_ast::visit::walk::walk_import_name_specifier(self, specifier);
  }

  fn visit_export_all_declaration(&mut self, decl: &oxc_ast::ast::ExportAllDeclaration<'a>) {
    oxc_ast::visit::walk::walk_export_all_declaration(self, decl);
  }

  fn visit_export_default_declaration(
    &mut self,
    decl: &oxc_ast::ast::ExportDefaultDeclaration<'a>,
  ) {
    oxc_ast::visit::walk::walk_export_default_declaration(self, decl);
  }

  fn visit_export_named_declaration(&mut self, decl: &oxc_ast::ast::ExportNamedDeclaration<'a>) {
    oxc_ast::visit::walk::walk_export_named_declaration(self, decl);
  }

  fn visit_export_specifier(&mut self, specifier: &oxc_ast::ast::ExportSpecifier<'a>) {
    oxc_ast::visit::walk::walk_export_specifier(self, specifier);
  }

  fn visit_module_export_name(&mut self, name: &oxc_ast::ast::ModuleExportName<'a>) {
    oxc_ast::visit::walk::walk_module_export_name(self, name);
  }

  fn visit_enum_member(&mut self, member: &oxc_ast::ast::TSEnumMember<'a>) {
    oxc_ast::visit::walk::walk_enum_member(self, member);
  }

  fn visit_enum(&mut self, decl: &oxc_ast::ast::TSEnumDeclaration<'a>) {
    oxc_ast::visit::walk::walk_enum(self, decl);
  }

  fn visit_declaration(&mut self, decl: &oxc_ast::ast::Declaration<'a>) {
    oxc_ast::visit::walk::walk_declaration(self, decl);
  }

  // fn visit_ts_import_equals_declaration(
  //   &mut self,
  //   decl: &oxc_ast::ast::TSImportEqualsDeclaration<'a>,
  // ) {
  //   oxc_ast::visit::walk::walk_ts_import_equals_declaration(self, decl);
  // }

  // fn visit_ts_module_reference(&mut self, reference: &oxc_ast::ast::TSModuleReference<'a>) {
  //   oxc_ast::visit::walk::walk_ts_module_reference(self, reference);
  // }

  // fn visit_ts_type_name(&mut self, name: &oxc_ast::ast::TSTypeName<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_name(self, name);
  // }

  // fn visit_ts_external_module_reference(
  //   &mut self,
  //   reference: &oxc_ast::ast::TSExternalModuleReference<'a>,
  // ) {
  //   oxc_ast::visit::walk::walk_ts_external_module_reference(self, reference);
  // }

  // fn visit_ts_module_declaration(&mut self, decl: &oxc_ast::ast::TSModuleDeclaration<'a>) {
  //   oxc_ast::visit::walk::walk_ts_module_declaration(self, decl);
  // }

  // fn visit_ts_module_block(&mut self, block: &oxc_ast::ast::TSModuleBlock<'a>) {
  //   oxc_ast::visit::walk::walk_ts_module_block(self, block);
  // }

  // fn visit_ts_type_alias_declaration(&mut self, decl: &oxc_ast::ast::TSTypeAliasDeclaration<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_alias_declaration(self, decl);
  // }

  // fn visit_ts_interface_declaration(&mut self, decl: &oxc_ast::ast::TSInterfaceDeclaration<'a>) {
  //   oxc_ast::visit::walk::walk_ts_interface_declaration(self, decl);
  // }

  // fn visit_ts_interface_heritage(&mut self, heritage: &oxc_ast::ast::TSInterfaceHeritage<'a>) {
  //   oxc_ast::visit::walk::walk_ts_interface_heritage(self, heritage);
  // }

  // fn visit_ts_as_expression(&mut self, expr: &oxc_ast::ast::TSAsExpression<'a>) {
  //   oxc_ast::visit::walk::walk_ts_as_expression(self, expr);
  // }

  // fn visit_ts_satisfies_expression(&mut self, expr: &oxc_ast::ast::TSSatisfiesExpression<'a>) {
  //   oxc_ast::visit::walk::walk_ts_satisfies_expression(self, expr);
  // }

  // fn visit_ts_non_null_expression(&mut self, expr: &oxc_ast::ast::TSNonNullExpression<'a>) {
  //   oxc_ast::visit::walk::walk_ts_non_null_expression(self, expr);
  // }

  // fn visit_ts_type_assertion(&mut self, expr: &oxc_ast::ast::TSTypeAssertion<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_assertion(self, expr);
  // }

  // fn visit_ts_instantiation_expression(
  //   &mut self,
  //   expr: &oxc_ast::ast::TSInstantiationExpression<'a>,
  // ) {
  //   oxc_ast::visit::walk::walk_ts_instantiation_expression(self, expr);
  // }

  // fn visit_ts_type_annotation(&mut self, annotation: &oxc_ast::ast::TSTypeAnnotation<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_annotation(self, annotation);
  // }

  // fn visit_ts_type(&mut self, ty: &oxc_ast::ast::TSType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type(self, ty);
  // }

  // fn visit_ts_tuple_element(&mut self, ty: &oxc_ast::ast::TSTupleElement<'a>) {
  //   oxc_ast::visit::walk::walk_ts_tuple_element(self, ty);
  // }

  // fn visit_ts_this_parameter(&mut self, param: &oxc_ast::ast::TSThisParameter<'a>) {
  //   oxc_ast::visit::walk::walk_ts_this_parameter(self, param);
  // }

  // fn visit_ts_type_parameter(&mut self, ty: &oxc_ast::ast::TSTypeParameter<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_parameter(self, ty);
  // }

  // fn visit_ts_type_parameter_instantiation(
  //   &mut self,
  //   ty: &oxc_ast::ast::TSTypeParameterInstantiation<'a>,
  // ) {
  //   oxc_ast::visit::walk::walk_ts_type_parameter_instantiation(self, ty);
  // }

  // fn visit_ts_type_parameter_declaration(
  //   &mut self,
  //   ty: &oxc_ast::ast::TSTypeParameterDeclaration<'a>,
  // ) {
  //   oxc_ast::visit::walk::walk_ts_type_parameter_declaration(self, ty);
  // }

  // fn visit_ts_any_keyword(&mut self, ty: &oxc_ast::ast::TSAnyKeyword) {
  //   oxc_ast::visit::walk::walk_ts_any_keyword(self, ty);
  // }

  // fn visit_ts_big_int_keyword(&mut self, ty: &oxc_ast::ast::TSBigIntKeyword) {
  //   oxc_ast::visit::walk::walk_ts_big_int_keyword(self, ty);
  // }

  // fn visit_ts_boolean_keyword(&mut self, ty: &oxc_ast::ast::TSBooleanKeyword) {
  //   oxc_ast::visit::walk::walk_ts_boolean_keyword(self, ty);
  // }

  // fn visit_ts_intrinsic_keyword(&mut self, ty: &oxc_ast::ast::TSIntrinsicKeyword) {
  //   oxc_ast::visit::walk::walk_ts_intrinsic_keyword(self, ty);
  // }

  // fn visit_ts_never_keyword(&mut self, ty: &oxc_ast::ast::TSNeverKeyword) {
  //   oxc_ast::visit::walk::walk_ts_never_keyword(self, ty);
  // }

  // fn visit_ts_null_keyword(&mut self, ty: &oxc_ast::ast::TSNullKeyword) {
  //   oxc_ast::visit::walk::walk_ts_null_keyword(self, ty);
  // }

  // fn visit_ts_number_keyword(&mut self, ty: &oxc_ast::ast::TSNumberKeyword) {
  //   oxc_ast::visit::walk::walk_ts_number_keyword(self, ty);
  // }

  // fn visit_ts_object_keyword(&mut self, ty: &oxc_ast::ast::TSObjectKeyword) {
  //   oxc_ast::visit::walk::walk_ts_object_keyword(self, ty);
  // }

  // fn visit_ts_string_keyword(&mut self, ty: &oxc_ast::ast::TSStringKeyword) {
  //   oxc_ast::visit::walk::walk_ts_string_keyword(self, ty);
  // }

  // fn visit_ts_symbol_keyword(&mut self, ty: &oxc_ast::ast::TSSymbolKeyword) {
  //   oxc_ast::visit::walk::walk_ts_symbol_keyword(self, ty);
  // }

  // fn visit_ts_undefined_keyword(&mut self, ty: &oxc_ast::ast::TSUndefinedKeyword) {
  //   oxc_ast::visit::walk::walk_ts_undefined_keyword(self, ty);
  // }

  // fn visit_ts_unknown_keyword(&mut self, ty: &oxc_ast::ast::TSUnknownKeyword) {
  //   oxc_ast::visit::walk::walk_ts_unknown_keyword(self, ty);
  // }

  // fn visit_ts_void_keyword(&mut self, ty: &oxc_ast::ast::TSVoidKeyword) {
  //   oxc_ast::visit::walk::walk_ts_void_keyword(self, ty);
  // }

  // fn visit_ts_array_type(&mut self, ty: &oxc_ast::ast::TSArrayType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_array_type(self, ty);
  // }

  // fn visit_ts_conditional_type(&mut self, ty: &oxc_ast::ast::TSConditionalType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_conditional_type(self, ty);
  // }

  // fn visit_ts_constructor_type(&mut self, ty: &oxc_ast::ast::TSConstructorType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_constructor_type(self, ty);
  // }

  // fn visit_ts_function_type(&mut self, ty: &oxc_ast::ast::TSFunctionType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_function_type(self, ty);
  // }

  // fn visit_ts_import_type(&mut self, ty: &oxc_ast::ast::TSImportType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_import_type(self, ty);
  // }

  // fn visit_ts_indexed_access_type(&mut self, ty: &oxc_ast::ast::TSIndexedAccessType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_indexed_access_type(self, ty);
  // }

  // fn visit_ts_infer_type(&mut self, ty: &oxc_ast::ast::TSInferType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_infer_type(self, ty);
  // }

  // fn visit_ts_intersection_type(&mut self, ty: &oxc_ast::ast::TSIntersectionType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_intersection_type(self, ty);
  // }

  // fn visit_ts_literal_type(&mut self, ty: &oxc_ast::ast::TSLiteralType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_literal_type(self, ty);
  // }

  // fn visit_ts_mapped_type(&mut self, ty: &oxc_ast::ast::TSMappedType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_mapped_type(self, ty);
  // }

  // fn visit_ts_named_tuple_member(&mut self, ty: &oxc_ast::ast::TSNamedTupleMember<'a>) {
  //   oxc_ast::visit::walk::walk_ts_named_tuple_member(self, ty);
  // }

  // fn visit_ts_qualified_name(&mut self, name: &oxc_ast::ast::TSQualifiedName<'a>) {
  //   oxc_ast::visit::walk::walk_ts_qualified_name(self, name);
  // }

  // fn visit_ts_template_literal_type(&mut self, ty: &oxc_ast::ast::TSTemplateLiteralType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_template_literal_type(self, ty);
  // }

  // fn visit_ts_this_type(&mut self, ty: &oxc_ast::ast::TSThisType) {
  //   oxc_ast::visit::walk::walk_ts_this_type(self, ty);
  // }

  // fn visit_ts_tuple_type(&mut self, ty: &oxc_ast::ast::TSTupleType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_tuple_type(self, ty);
  // }

  // fn visit_ts_type_literal(&mut self, ty: &oxc_ast::ast::TSTypeLiteral<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_literal(self, ty);
  // }

  // fn visit_ts_type_operator_type(&mut self, ty: &oxc_ast::ast::TSTypeOperator<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_operator_type(self, ty);
  // }

  // fn visit_ts_type_predicate(&mut self, ty: &oxc_ast::ast::TSTypePredicate<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_predicate(self, ty);
  // }

  // fn visit_ts_type_query(&mut self, ty: &oxc_ast::ast::TSTypeQuery<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_query(self, ty);
  // }

  // fn visit_ts_type_reference(&mut self, ty: &oxc_ast::ast::TSTypeReference<'a>) {
  //   oxc_ast::visit::walk::walk_ts_type_reference(self, ty);
  // }

  // fn visit_ts_union_type(&mut self, ty: &oxc_ast::ast::TSUnionType<'a>) {
  //   oxc_ast::visit::walk::walk_ts_union_type(self, ty);
  // }

  // fn visit_ts_signature(&mut self, signature: &oxc_ast::ast::TSSignature<'a>) {
  //   oxc_ast::visit::walk::walk_ts_signature(self, signature);
  // }

  // fn visit_ts_construct_signature_declaration(
  //   &mut self,
  //   signature: &oxc_ast::ast::TSConstructSignatureDeclaration<'a>,
  // ) {
  //   oxc_ast::visit::walk::walk_ts_construct_signature_declaration(self, signature);
  // }

  // fn visit_ts_method_signature(&mut self, signature: &oxc_ast::ast::TSMethodSignature<'a>) {
  //   oxc_ast::visit::walk::walk_ts_method_signature(self, signature);
  // }

  // fn visit_ts_index_signature_name(&mut self, name: &oxc_ast::ast::TSIndexSignatureName<'a>) {
  //   oxc_ast::visit::walk::walk_ts_index_signature_name(self, name);
  // }

  // fn visit_ts_index_signature(&mut self, signature: &oxc_ast::ast::TSIndexSignature<'a>) {
  //   oxc_ast::visit::walk::walk_ts_index_signature(self, signature);
  // }

  // fn visit_ts_property_signature(&mut self, signature: &oxc_ast::ast::TSPropertySignature<'a>) {
  //   oxc_ast::visit::walk::walk_ts_property_signature(self, signature);
  // }

  // fn visit_ts_call_signature_declaration(
  //   &mut self,
  //   signature: &oxc_ast::ast::TSCallSignatureDeclaration<'a>,
  // ) {
  //   oxc_ast::visit::walk::walk_ts_call_signature_declaration(self, signature);
  // }

  // fn visit_ts_import_attributes(&mut self, attributes: &oxc_ast::ast::TSImportAttributes<'a>) {
  //   oxc_ast::visit::walk::walk_ts_import_attributes(self, attributes);
  // }

  // fn visit_ts_import_attribute(&mut self, attribute: &oxc_ast::ast::TSImportAttribute<'a>) {
  //   oxc_ast::visit::walk::walk_ts_import_attribute(self, attribute);
  // }

  // fn visit_ts_import_attribute_name(&mut self, name: &oxc_ast::ast::TSImportAttributeName<'a>) {
  //   oxc_ast::visit::walk::walk_ts_import_attribute_name(self, name);
  // }
}
