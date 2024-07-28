mod classes;
mod compat;
mod functions;
mod grammar;
mod regular_expressions;
mod statements;
mod support;
mod syntax_record_visitor;
use std::{
  fs::read,
  ops::Deref,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use compat::Compat;
use napi::{Error, Result};
use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use support::Support;
use syntax_record_visitor::SyntaxRecordVisitor;

use crate::oxc_visitor_processor::{oxc_visit_process, Options};

#[napi]
pub fn demo(danger_strings: Vec<String>, options: Option<Options>) -> Result<()> {
  let used: Arc<Mutex<Vec<Compat>>> = Arc::new(Mutex::new(Vec::new()));
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

      let mut x = SyntaxRecordVisitor::new(path.to_path_buf(), danger_strings.to_vec());

      x.visit_program(&ret.program);

      // used.extend(x.cache);
      // let mut used = used.lock().unwrap();
      // x.cache.iter().for_each(|item| {
      //   let x = item.clone();
      //   used.push(Compat {
      //     name: x.name,
      //     description: x.description,
      //     mdn_url: x.mdn_url,
      //     spec_url: x.spec_url,
      //     support: Support {
      //       chrome: x.support.chrome,
      //       edge: x.support.edge,
      //       firefox: x.support.firefox,
      //       ie: x.support.ie,
      //       node: x.support.node,
      //       safari: x.support.safari,
      //     },
      //   });
      // });

      // println!("file: {:?}", x.cache.len());

      x.cache.iter().for_each(|item| {
        println!("syntax: {} ", item.name,);
        println!(
          "compat: chrome:{:<10} safari:{:<10} edge:{:<10}",
          item.support.chrome, item.support.safari, item.support.edge
        );
        println!("file: {}", path.display().to_string());
        println!("-----------------------------------");
      })
    }
  };
  oxc_visit_process(x, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  println!("used: {:?}", used);

  // Ok(used)
  Ok(())
}
