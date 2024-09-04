use crate::create_compat_2;

create_compat_2! {
    Import,
    compat {
        name: "import",
        description: "import 语句",
        mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/import",
        tags: ["web-features:js-modules"],
        support: {
            chrome: "61.0.0",
            chrome_android: "61.0.0",
            firefox: "60.0.0",
            firefox_android: "60.0.0",
            safari: "10.1.0",
            safari_ios: "10.1.0",
            edge: "16.0.0",
            node: "13.2.0",
            deno: "1.0.0",
        }
    },
    fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
        matches!(node.kind(), AstKind::ImportDeclaration(_))
    }
}

#[cfg(test)]
mod tests {
  use super::Import;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_import_default_specifier: {
          setup: Import::default(),
          source_code: r#"
                import defaultExport from 'module-name';
            "#,
          eq: [
              r#"import defaultExport from 'module-name';"#,
          ],
          ne: []
      },

      should_ok_when_import_namespace_specifier: {
          setup: Import::default(),
          source_code: r#"
                import * as name from 'module-name';
            "#,
          eq: [
              r#"import * as name from 'module-name';"#,
          ],
          ne: []
      },

      should_ok_when_import_specifier: {
          setup: Import::default(),
          source_code: r#"
                import { export1 } from 'module-name';
            "#,
          eq: [
              r#"import { export1 } from 'module-name';"#,
          ],
          ne: []
      },

      should_ok_when_import_alias_specifier: {
          setup: Import::default(),
          source_code: r#"
                import { export1 as alias1 } from 'module-name';
            "#,
          eq: [
              r#"import { export1 as alias1 } from 'module-name';"#,
          ],
          ne: []
      },

      should_ok_when_import_declaration: {
          setup: Import::default(),
          source_code: r#"
                import 'module-name';
            "#,
          eq: [
              r#"import 'module-name';"#,
          ],
          ne: []
      }
  }
}
