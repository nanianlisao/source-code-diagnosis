use std::path::PathBuf;

use anyhow::{bail, Result};
use beans::{Location, Position};
use oxc_allocator::Allocator;
use oxc_ast::{ast::BindingIdentifier, AstKind};
use oxc_parser::Parser;
use oxc_semantic::{
  AstNode, Reference, Semantic, SemanticBuilder as OxcSemanticBuilder,
  SemanticBuilderReturn,
};
use oxc_span::{GetSpan, SourceType};
use ropey::Rope;

pub struct SemanticBuilder<'a> {
  pub source_code: &'a str,
  pub source_type: SourceType,
  pub allocator: Allocator,
  pub file_path: Option<PathBuf>,
}

impl<'a> SemanticBuilder<'a> {
  pub fn new(
    source_code: &'a str,
    source_type: SourceType,
    file_path: Option<PathBuf>,
  ) -> Self {
    let allocator = Allocator::default();
    Self {
      source_code,
      source_type,
      allocator,
      file_path,
    }
  }

  pub fn code(source_code: &'a str, source_type: SourceType) -> Self {
    Self::new(source_code, source_type, None)
  }

  pub fn ts(source_code: &'a str) -> Self {
    Self::new(
      source_code,
      SourceType::default()
        .with_module(true)
        .with_typescript(true)
        .with_jsx(true),
      None,
    )
  }

  pub fn js(source_text: &'a str) -> Self {
    Self::new(
      source_text,
      SourceType::default().with_module(true).with_jsx(true),
      None,
    )
  }

  pub fn build(&self) -> Semantic<'_> {
    let semantic_ret = self.build_with_errors();
    if !semantic_ret.errors.is_empty() {
      eprintln!(
        "Semantic analysis failed:\n\n{}",
        semantic_ret
          .errors
          .iter()
          .map(|e| format!("{e}"))
          .collect::<String>()
      );
    }
    semantic_ret.semantic
  }

  pub fn build_with_errors(&self) -> SemanticBuilderReturn<'_> {
    let parse = oxc_parser::Parser::new(
      &self.allocator,
      &self.source_code,
      self.source_type,
    )
    .parse();

    let program = self.allocator.alloc(parse.program);

    OxcSemanticBuilder::new()
      .with_check_syntax_error(true)
      // .with_trivias(parse.trivias)
      // .with_cfg(self.cfg)
      .build(program);

    OxcSemanticBuilder::new()
      .with_check_syntax_error(true)
      // .with_trivias(parse.trivias)
      // .with_cfg(self.cfg)
      .build(program)
  }

  pub fn build_handler(&self) -> Result<SemanticHandler> {
    let ret =
      Parser::new(&self.allocator, &self.source_code, self.source_type).parse();

    if ret.errors.len() > 0 {
      for err in ret.errors.iter() {
        eprintln!("parse error: {:?}", err);
      }
      bail!("parse error: ");
    }

    let program = self.allocator.alloc(ret.program);

    let semantic = OxcSemanticBuilder::new()
      .with_check_syntax_error(true)
      // .with_trivias(parse.trivias)
      // .with_cfg(self.cfg)
      .build(program)
      .semantic;
    if let Some(file_path) = &self.file_path {
      let file_path_str = file_path.to_string_lossy().to_string();
      Ok(SemanticHandler::new(file_path_str, semantic))
    } else {
      Ok(SemanticHandler::new(String::new(), semantic))
    }
  }
}

pub struct SemanticHandler<'a> {
  pub semantic: Semantic<'a>,
  pub file_path_str: Option<String>,
}

impl<'a> SemanticHandler<'a> {
  pub fn new(file_path_str: String, semantic: Semantic<'a>) -> Self {
    Self {
      file_path_str: Some(file_path_str),
      semantic,
    }
  }

  pub fn each_node<F>(&self, mut f: F)
  where
    F: FnMut(&SemanticHandler<'a>, &AstNode),
  {
    for node in self.semantic.nodes().iter() {
      f(&self, node);
    }
  }

  pub fn is_in<F>(
    &self,
    node: &AstNode,
    max_depth: usize,
    predicate: F,
  ) -> Option<&AstNode>
  where
    F: Fn(&AstKind) -> bool,
  {
    let mut depth = 0;
    let mut current_node_id = node.id();
    while let Some(pn) = self.semantic.nodes().parent_node(current_node_id) {
      if depth >= max_depth {
        return None;
      }

      if predicate(&pn.kind()) {
        return Some(pn);
      }

      current_node_id = pn.id();
      depth += 1;
    }
    None
  }

  pub fn offset_to_position(
    &self,
    offset: usize,
    source_text: &str,
  ) -> Position {
    // Unicode 换行符
    // \u{000A}    // LF (Line Feed)
    // \u{000B}    // VT (Vertical Tab)
    // \u{000C}    // FF (Form Feed)
    // \u{000D}    // CR (Carriage Return)
    // \u{0085}    // NEL (Next Line)
    // \u{2028}    // LS (Line Separator)
    // \u{2029}    // PS (Paragraph Separator)

    let normalized_text = source_text
      .replace('\u{85}', " ") // NEL
      .replace('\u{2028}', " ") // LS
      .replace('\u{2029}', " "); // PS

    let rope = Rope::from_str(&normalized_text);
    let line = rope.try_byte_to_line(offset).unwrap_or(0);

    let first_char_of_line = rope.try_line_to_char(line).unwrap_or(0);
    let offset = rope.try_byte_to_char(offset).unwrap_or(0);
    let col = offset - first_char_of_line;
    Position {
      line: (line + 1) as u32,
      col: (col + 1) as u32,
    }
  }

  pub fn offset_to_location(
    &self,
    source_text: &str,
    span: oxc_span::Span,
  ) -> Location {
    let start = self.offset_to_position(span.start as usize, source_text);
    let end = self.offset_to_position(span.end as usize, source_text);
    Location { start, end }
  }

  pub fn get_span(&self, ast_node: &AstNode) -> oxc_span::Span {
    GetSpan::span(&ast_node.kind())
  }

  pub fn get_reference_node_box(
    &self,
    reference: &Reference,
  ) -> (&AstNode, oxc_span::Span, Location) {
    let reference_node = self.parse_reference(reference);
    let span = GetSpan::span(&reference_node.kind());
    let loc = self.offset_to_location(self.semantic.source_text(), span);
    (reference_node, span, loc)
  }

  pub fn get_symbol_references(
    &self,
    binding: &BindingIdentifier,
  ) -> Vec<&Reference> {
    if let Some(symbol_id) = binding.symbol_id.get() {
      self
        .semantic
        .symbol_references(symbol_id)
        .into_iter()
        .collect()
    } else {
      vec![]
    }
  }

  // 解析 reference
  pub fn parse_reference(&self, reference: &Reference) -> &AstNode {
    let reference_node = self.semantic.nodes().get_node(reference.node_id());
    reference_node
  }

  pub fn get_parent_node(&self, node: &AstNode) -> Option<&AstNode> {
    self.semantic.nodes().parent_node(node.id())
  }
}
