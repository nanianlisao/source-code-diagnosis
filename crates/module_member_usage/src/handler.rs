use std::{collections::HashMap, path::PathBuf};

use beans::AstNode;
use oxc_ast::{ast::ImportDeclarationSpecifier, AstKind};

use utils::SemanticHandler;

use super::response::Response;

static ES_NAMESPACE: &str = "ES:NAMESPACE";
static ES_DEFAULT: &str = "ES:DEFAULT";
static SIDE_EFFECTS: &str = "ES:SIDE_EFFECTS";
static UNKNOWN: &str = "UNKNOWN";

pub struct ModuleMemberUsageHandler<'a> {
  npm_name_vec: Vec<String>,
  semantic_handler: SemanticHandler<'a>,
  path_str: String,
}

impl<'a> ModuleMemberUsageHandler<'a> {
  pub fn new(
    npm_name_vec: Vec<String>,
    path: PathBuf,
    semantic_handler: SemanticHandler<'a>,
  ) -> Self {
    Self {
      npm_name_vec,
      semantic_handler,
      path_str: path.display().to_string(),
    }
  }

  pub fn handle(&self) -> Vec<Response> {
    let mut mapper = HashMap::new();
    let mut inline_usages: Vec<Response> = Vec::new();

    let nodes = self.semantic_handler.semantic.nodes();

    for node in nodes.iter() {
      let kind = node.kind();
      let decl = match kind {
        AstKind::ImportDeclaration(decl) => decl,
        _ => continue,
      };

      let source_name = decl.source.value.as_str();

      if !self.npm_name_vec.contains(&source_name.to_string()) {
        continue;
      }

      let specifiers = match decl.specifiers {
        Some(ref specs) => specs,
        None => {
          let (span, loc) = self.semantic_handler.get_node_box(node);
          inline_usages.push(Response {
            lib_name: source_name.to_string(),
            member_name: SIDE_EFFECTS.to_string(),
            file_path: self.path_str.clone(),
            ast_node: AstNode::new((span.start, span.end), loc),
          });
          continue;
        }
      };

      if specifiers.is_empty() {
        continue;
      }

      for specifier in specifiers {
        let local_name = specifier.name();
        let imported_name = match specifier {
          ImportDeclarationSpecifier::ImportSpecifier(import_specifier) => {
            import_specifier.imported.name().as_str()
          }
          ImportDeclarationSpecifier::ImportDefaultSpecifier(_) => ES_DEFAULT,
          ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => {
            ES_NAMESPACE
          }
        };

        mapper.insert(local_name, imported_name);

        let references = self
          .semantic_handler
          .get_symbol_references(specifier.local());

        for reference in references {
          let (reference_node, span, loc) =
            self.semantic_handler.get_reference_node_box(reference);

          let is_in_jsx_closing_element = self
            .semantic_handler
            .is_parent_node_match_with_depth(reference_node, 6, |kind| {
              matches!(kind, AstKind::JSXClosingElement(_))
            });

          if is_in_jsx_closing_element {
            continue;
          }

          let is_in_member_expr = self
            .semantic_handler
            .is_parent_node_match_with_depth(reference_node, 3, |kind| {
              matches!(kind, AstKind::JSXMemberExpression(_))
            });

          let is_in_static_member_expr = self
            .semantic_handler
            .is_parent_node_match_with_depth(reference_node, 2, |kind| {
              matches!(kind, AstKind::MemberExpression(_))
            });

          if is_in_member_expr {
            // JSXMemberExpressionObject
            let parent_node =
              self.semantic_handler.get_parent_node(reference_node);

            // JSXMemberExpression
            let parent_node = parent_node
              .and_then(|item| self.semantic_handler.get_parent_node(item));

            if let Some(AstKind::JSXMemberExpression(member_expression)) =
              parent_node.map(|node| node.kind())
            {
              let property_name = member_expression.property.name.as_str();

              inline_usages.push(Response {
                lib_name: source_name.to_string(),
                member_name: property_name.to_string(),
                file_path: self.path_str.clone(),
                ast_node: AstNode::new((span.start, span.end), loc),
              });
            }
          } else if is_in_static_member_expr {
            let parent_node =
              self.semantic_handler.get_parent_node(reference_node);

            if let Some(AstKind::MemberExpression(member_expression)) =
              parent_node.map(|node| node.kind())
            {
              let property_name =
                member_expression.static_property_name().unwrap();

              inline_usages.push(Response {
                lib_name: source_name.to_string(),
                member_name: property_name.to_string(),
                file_path: self.path_str.clone(),
                ast_node: AstNode::new((span.start, span.end), loc),
              });
            }
          } else {
            inline_usages.push(Response {
              lib_name: source_name.to_string(),
              member_name: imported_name.to_string(),
              file_path: self.path_str.clone(),
              ast_node: AstNode::new((span.start, span.end), loc),
            });
          }
        }
      }
    }

    inline_usages.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::ModuleMemberUsageHandler;
  use std::path::PathBuf;
  use utils::SemanticBuilder;

  #[test]
  fn test_import_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
      import { useState } from 'react';
      function Component() {
          const [state, setState] = useState(0);
          return <div>{state}</div>;
      }
    "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "useState");
  }

  #[test]
  fn test_import_default_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import React from 'react';
        function Component() {
            return <React.Fragment>Hello</React.Fragment>;
        }
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "Fragment");
  }

  #[test]
  fn test_import_namespace_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import * as React from 'react';
        function Component() {
            return <React.Fragment>Hello</React.Fragment>;
        }
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "Fragment");
  }

  #[test]
  fn test_side_effects_import() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import 'react';
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "ES:SIDE_EFFECTS");
  }

  #[test]
  fn test_multiple_imports() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import React, { useState } from 'react';
        import * as ReactDOM from 'react-dom';
        function Component() {
            const [state, setState] = useState(0);
            return <React.Fragment>{state}</React.Fragment>;
        }
        ReactDOM.render(<Component />, document.getElementById('root'));
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string(), "react-dom".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 3);
    assert!(result
      .iter()
      .any(|r| r.lib_name == "react" && r.member_name == "useState"));
    assert!(result
      .iter()
      .any(|r| r.lib_name == "react" && r.member_name == "Fragment"));
    assert!(result
      .iter()
      .any(|r| r.lib_name == "react-dom" && r.member_name == "render"));
  }
}
