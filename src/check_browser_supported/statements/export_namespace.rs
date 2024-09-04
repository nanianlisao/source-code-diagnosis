use crate::create_compat_2;

create_compat_2! {
  ExportNamespace,
  compat {
    name: "statements.export_namespace",
    description: "export * as namespace",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/export#namespace_export",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "72.0.0",
      chrome_android: "72.0.0",
      firefox: "80.0.0",
      firefox_android: "80.0.0",
      safari: "14.1.0",
      safari_ios: "14.1.0",
      edge: "72.0.0",
      node: "13.2.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ExportAllDeclaration(export_all) if export_all.exported.is_some())
  }
}

#[cfg(test)]
mod tests {
  use super::ExportNamespace;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_export_namespace_declaration:{
      setup: ExportNamespace::default(),
      source_code: r#"
        export * as ns from './other_module.js';
        export * from './another_module.js';
      "#,
      eq: [
        r#"export * as ns from './other_module.js';"#
      ],
      ne: [
        r#"export * from './another_module.js';"#
      ]
    }
  }
}
