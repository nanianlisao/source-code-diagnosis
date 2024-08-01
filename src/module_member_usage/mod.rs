use std::{
  fs::read,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use module_member_usage_location::ModuleMemberUsageLocation;
use module_member_usage_visitor::ModuleMemberUsageVisitor;
use napi::{Error, Result};
use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;

use crate::oxc_visitor_processor::{oxc_visit_process, Options};

pub mod module_member_usage_location;
pub mod module_member_usage_visitor;

#[napi]
pub fn get_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<ModuleMemberUsageLocation>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let source_text = read(&path)
        .map_err(|err| {
          Error::new(
            napi::Status::GenericFailure,
            format!("Failed to read file: {}: {}", path.display(), err),
          )
        })
        .unwrap();
      let source_text = String::from_utf8(source_text).unwrap();
      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
        .unwrap();
      let allocator = Allocator::default();
      let ret = Parser::new(&allocator, &source_text, source_type).parse();

      let mut x = ModuleMemberUsageVisitor::new(
        path.to_path_buf(),
        npm_name_vec.to_vec(),
      );

      x.visit_program(&ret.program);
      let mut used = used.lock().unwrap();
      used.extend(x.used);
    }
  };
  oxc_visit_process(x, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  Ok(used)
}