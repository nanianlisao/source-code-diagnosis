use oxc_ast::ast::ImportAttributeKey;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
  },
  compat {
    name: "import_import_assertions_type_css",
    description: "assert {type: 'css'} 语法",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "93",
      chrome_android: "93",
      firefox: "-1",
      firefox_android: "-1",
      opera: "-1",
      opera_android: "-1",
      safari: "-1",
      safari_ios: "-1",
      edge: "93",
      oculus: "93",
      node: "-1",
      deno: "-1",
    }
  },
  walk_with_clause,
  |ctx: &mut Context, it: &oxc_ast::ast::WithClause| {
    let mut result = false;
    if it.attributes_keyword.name == "assert" {
      for item in &it.with_entries {
        if let ImportAttributeKey::Identifier(key) = &item.key {
          if key.name == "type" && item.value.value == "css" {
            result = true;
          }
        }
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count!(
    "import_import_assertions_type_css",
    setup,
    should_ok_when_import_assertions_type_css,
    r#"
    import styles from './styles.css' assert { type: 'css' };
    "#,
    1,
    should_fail_when_import_assertions_type_json,
    r#"
    import styles from './styles.css' assert { type: 'json' };
    "#,
    0,
    should_fail_when_import_attributes_type_css,
    r#"
    import styles from './styles.css' with { type: 'css' };
    "#,
    0,
  );
}