use std::{collections::HashMap, env::current_dir};

use camino::Utf8PathBuf;
use napi_derive::napi;
use serde::Serialize;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsArgs {
  pub cwd: Option<String>,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

impl JsArgs {
  pub fn get_cwd(&self) -> String {
    let default_cwd = current_dir().unwrap().display().to_string();
    Utf8PathBuf::from(self.cwd.clone().unwrap_or(default_cwd))
      .join("")
      .into_string()
  }

  pub fn get_pattern(&self) -> String {
    self
      .pattern
      .clone()
      .unwrap_or("**/*.{js,ts,jsx,tsx}".to_string())
  }

  pub fn get_ignore(&self) -> Vec<String> {
    self.ignore.clone().unwrap_or(vec![
      "**/node_modules/**".to_string(),
      "**/*.d.ts".to_string(),
    ])
  }

  pub fn get_alias(&self) -> HashMap<String, Vec<String>> {
    self.alias.clone().unwrap_or(HashMap::new())
  }

  pub fn get_modules(&self) -> Vec<String> {
    self
      .modules
      .clone()
      .unwrap_or(vec!["node_modules".to_string()])
  }
}

#[derive(Debug, Clone)]
pub struct Args<'a> {
  pub cwd: &'a str,
  pub pattern: &'a str,
  pub ignore: Vec<&'a str>,
  pub alias: HashMap<String, Vec<String>>,
  pub modules: Vec<String>,
}

#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
pub struct Edge {
  pub source: String,
  pub target: String,
  pub ast_node: beans::AstNode,
}

#[derive(Debug, Serialize)]
#[napi(object)]
pub struct GroupGraphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Vec<Edge>>,
}

#[napi(object)]
pub struct Graphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Edge>,
}
