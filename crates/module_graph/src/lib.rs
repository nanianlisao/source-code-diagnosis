use anyhow::Result;
use beans::{AstNode, Span};
use log::debug;
use napi_derive::napi;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use petgraph::algo::all_simple_paths;
use petgraph::algo::has_path_connecting;
use petgraph::algo::{kosaraju_scc, scc};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Bfs, Dfs, DfsPostOrder, EdgeRef};
use petgraph::Direction;
use serde::Serialize;

use std::{
  collections::{HashMap, HashSet},
  path::PathBuf,
  sync::{Arc, Mutex},
};
use utils::{glob, GlobOptions, SemanticBuilder};

#[napi[object]]
#[derive(Debug, Clone)]
pub struct Options {
  pub cwd: Option<String>,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Dependency {
  pub from: String,
  pub to: String,
  pub ast_node: AstNode,
}

pub fn get_node(options: Option<Options>) -> Result<Vec<Dependency>> {
  let used = Arc::new(Mutex::new(Vec::new()));

  let alias = match &options {
    Some(Options {
      alias: Some(alias), ..
    }) => alias.clone(),
    _ => HashMap::new(),
  };

  let modules = match &options {
    Some(Options {
      modules: Some(modules),
      ..
    }) => modules.clone(),
    _ => vec!["node_modules".into(), "web_modules".into()],
  };

  let resolver_alias = alias
    .into_iter()
    .map(|(key, values)| {
      (key, values.into_iter().map(AliasValue::Path).collect())
    })
    .collect();

  let resolver = Resolver::new(ResolveOptions {
    alias: resolver_alias,
    modules,
    extensions: vec![
      ".ts".into(),
      ".js".into(),
      ".jsx".into(),
      ".tsx".into(),
      ".json".into(),
    ],
    ..ResolveOptions::default()
  });

  let handler = {
    let used = Arc::clone(&used);

    move |path: PathBuf| {
      debug!("path: {}", path.display().to_string());

      let mut inline_usages: Vec<Dependency> = Vec::new();

      SemanticBuilder::file(path.clone())
        .build_handler()
        .each_node(|handler, node| {
          if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
            let value = import_declaration.source.value.to_string();
            debug!("value: {}", value);

            if let Some(parent) = path.parent() {
              debug!("parent: {}", parent.display().to_string());
              let resolved = resolver.resolve(&parent, &value);
              if let Ok(resolved_path) = resolved {
                debug!(
                  "resolved_path: {}",
                  resolved_path.full_path().display().to_string()
                );

                let (span, loc) = handler.get_node_box(node);

                inline_usages.push(Dependency {
                  from: path.display().to_string(),
                  to: resolved_path.full_path().display().to_string(),
                  ast_node: AstNode {
                    span: Span {
                      start: span.start,
                      end: span.end,
                    },
                    loc: loc,
                  },
                });
              } else {
                eprintln!(
                  "no resolved path  {} in {}",
                  value,
                  path.display().to_string()
                );
              }
            } else {
              eprintln!("no parent path {}", path.display().to_string());
            }
          }
        });

      let mut used = used.lock().unwrap();
      used.extend(inline_usages);
    }
  };

  let op = if let Some(options) = options {
    Some(GlobOptions {
      cwd: options.cwd,
      pattern: options.pattern,
      ignore: options.ignore,
    })
  } else {
    None
  };

  glob(handler, op).map_err(|err| {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  })?;

  let x = used.lock().unwrap().clone();

  Ok(x)
}

pub fn get_dependents(
  file: String,
  options: Option<Options>,
) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;

  let mut graph = DiGraph::new();
  let mut module_map = HashMap::new();
  let mut node_indices: HashMap<&str, NodeIndex> = HashMap::new();

  for value in used.iter() {
    let from = value.from.as_str();
    let to = value.to.as_str();

    let from_node = *node_indices
      .entry(from)
      .or_insert_with(|| graph.add_node(from));

    let to_node = *node_indices.entry(to).or_insert_with(|| graph.add_node(to));
    module_map.insert((to, from), value);
    graph.add_edge(to_node, from_node, ());
  }

  let target_index = *node_indices.get(file.as_str()).unwrap();
  let mut result = Vec::new();

  // 使用 Dfs 遍历反向图
  let mut dfs = Dfs::new(&graph, target_index);
  while let Some(nx) = dfs.next(&graph) {
    if nx != target_index {
      // 使用 all_simple_paths 找到所有路径

      all_simple_paths::<Vec<_>, _>(&graph, target_index, nx, 0, None)
        .for_each(|path| {
          let mut inline_path = Vec::with_capacity(path.len() - 1);
          for window in path
            .into_iter()
            .rev()
            .collect::<Vec<NodeIndex>>()
            .windows(2)
          {
            let source = graph[window[0]];
            let target = graph[window[1]];

            inline_path.push(Cycle {
              source: source.to_string(),
              target: target.to_string(),
              ast_node: module_map[&(target, source)].ast_node.clone(),
            });
          }

          result.push(inline_path);
        });
    }
  }

  Ok(result)
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct DependencyNode {
  pub name: String,
  pub children: Vec<DependencyNode>,
  pub ast_node: Option<AstNode>,
}

pub fn get_dependencies(
  file: String,
  options: Option<Options>,
) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;

  let mut graph = DiGraph::new();
  let mut module_map = HashMap::new();
  let mut node_indices: HashMap<&str, NodeIndex> = HashMap::new();

  for value in used.iter() {
    let from = value.from.as_str();
    let to = value.to.as_str();

    let from_node = *node_indices
      .entry(from)
      .or_insert_with(|| graph.add_node(from));

    let to_node = *node_indices.entry(to).or_insert_with(|| graph.add_node(to));
    module_map.insert((from, to), value);
    graph.add_edge(from_node, to_node, ());
  }

  let target_index = node_indices.get(file.as_str()).unwrap();
  let mut result = Vec::new();

  // 使用 Dfs 遍历图
  let mut dfs = Dfs::new(&graph, *target_index);
  while let Some(nx) = dfs.next(&graph) {
    if nx != *target_index {
      petgraph::algo::all_simple_paths::<Vec<_>, _>(
        &graph,
        *target_index,
        nx,
        0,
        None,
      )
      .for_each(|path| {
        let mut inline_path = Vec::with_capacity(path.len() - 1);
        for window in path.windows(2) {
          let source = graph[window[0]];
          let target = graph[window[1]];

          inline_path.push(Cycle {
            source: source.to_string(),
            target: target.to_string(),
            ast_node: module_map[&(source, target)].ast_node.clone(),
          });
        }
        result.push(inline_path);
      });

      // // 检查是否存在从 nx 到 target_index 的路径（循环依赖）
      if has_path_connecting(&graph, nx, *target_index, None) {
        // let mut cycle_path = paths[0].clone();
        // cycle_path.push(file.clone());
        // result.push(cycle_path);
        // todo!("cycle");
        println!("TODO cycle: {}", graph[nx].to_string());
      }
    }
  }

  Ok(result)
}

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct Cycle {
  pub source: String,
  pub target: String,
  pub ast_node: AstNode,
}

pub fn check_cycle(options: Option<Options>) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;

  let mut module_map = HashMap::new();
  let mut graph = DiGraph::new();
  let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();

  // 构建图的代码保持不变
  for value in used.iter() {
    let from = value.from.as_str();
    let to = value.to.as_str();

    let from_node =
      *node_map.entry(from).or_insert_with(|| graph.add_node(from));

    let to_node = *node_map.entry(to).or_insert_with(|| graph.add_node(to));

    module_map.insert((from, to), value);
    graph.add_edge(from_node, to_node, ());
  }

  // 使用 kosaraju_scc 算法找出强连通分量
  let sccs = kosaraju_scc(&graph);

  let mut result = Vec::new();

  for scc in sccs.into_iter().filter(|scc| scc.len() > 1) {
    let mut cycles = Vec::new();
    let mut visited = HashSet::new();

    for &start_node in scc.iter() {
      if !visited.contains(&start_node) {
        let mut cycle = Vec::new();
        let mut stack = vec![(start_node, Vec::new())];

        while let Some((node, path)) = stack.pop() {
          if path.contains(&node) {
            let cycle_start = path.iter().position(|&n| n == node).unwrap();
            let cycle_path = &path[cycle_start..];

            for (&from, &to) in cycle_path
              .iter()
              .zip(cycle_path.iter().skip(1).chain(std::iter::once(&node)))
            {
              if let Some(dependency) =
                module_map.get(&(graph[from], graph[to]))
              {
                cycle.push(Cycle {
                  source: graph[from].to_string(),
                  target: graph[to].to_string(),
                  ast_node: dependency.ast_node.clone(),
                });
              }
            }

            if !cycle.is_empty() {
              cycles.push(cycle);
            }
            cycle = Vec::new();
          } else if !visited.contains(&node) {
            visited.insert(node);
            let mut new_path = path.clone();
            new_path.push(node);

            for edge in graph.edges(node) {
              let target = edge.target();
              if scc.contains(&target) {
                stack.push((target, new_path.clone()));
              }
            }
          }
        }
      }
    }

    if !cycles.is_empty() {
      result.extend(cycles);
    }
  }

  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_danger_strings() {
    let mut alias = HashMap::new();

    // let x = &[
    //   ("@src", "src"),
    //   ("@public-component", "src/component/public-component"),
    //   ("@search-queries", "src/component/search-queries"),
    //   (
    //     "@shein-components/dateRangePicker2",
    //     "src/component/public-component/dateRangePicker2Wrapper",
    //   ),
    //   ("@", "src"),
    // ];

    let x = &[
      ("apis", "web_modules/apis"),
      ("common", "web_modules/common"),
      ("shein-lib", "web_modules/shein-lib"),
      ("hooks", "web_modules/hooks"),
      ("publicComponent", "web_modules/public/spmb"),
      ("@", "src"),
    ];

    for (key, value) in x.iter() {
      alias.insert(
        key.to_string(),
        vec![format!(
          "{}/{}",
          "/Users/ityuany/GitRepository/bmas",
          value.to_string()
        )],
      );
    }

    // alias.insert(
    //   "@src".to_string(),
    //   vec!["/Users/ityuany/GitRepository/wms/src".to_string()],
    // );
    // alias.insert(
    //   "@public-component".to_string(),
    //   vec![
    //     "/Users/ityuany/GitRepository/wms/src/component/public-component"
    //       .to_string(),
    //   ],
    // );
    // alias.insert(
    //   "@search-queries".to_string(),
    //   vec![
    //     "/Users/ityuany/GitRepository/wms/src/component/search-queries"
    //       .to_string(),
    //   ],
    // );
    // alias.insert(
    //   "@shein-components/dateRangePicker2".to_string(),
    //   vec![
    //     "/Users/ityuany/GitRepository/wms/src/component/public-component/dateRangePicker2Wrapper"
    //       .to_string(),
    //   ],
    // );
    // alias.insert(
    //   "@".to_string(),
    //   vec!["/Users/ityuany/GitRepository/wms/src".to_string()],
    // );

    let op = Options {
      cwd: Some("/Users/ityuany/GitRepository/bmas/src".to_string()),
      modules: Some(vec!["node_modules".into(), "web_modules".into()]),
      pattern: None,
      ignore: None,
      alias: Some(alias),
    };

    // let result = get_dependents(
    //   "/Users/ityuany/GitRepository/wms/src/lib/dealFunc.js".to_string(),
    //   Some(op.clone()),
    // );
    // if let Ok(result) = result {
    //   for x in result {
    //     println!("{}", x);
    //   }
    // }

    let result1 = check_cycle(Some(op.clone())).unwrap();

    for x in result1 {
      println!("\n跨越{}个文件的循环依赖", x.len());
      for y in x.iter() {
        // println!("{}  {} ", y.from, y.to);
        println!(
          "{}:{}..{}",
          y.source, y.ast_node.loc.start.line, y.ast_node.loc.end.line
        );
      }
    }
  }
}
