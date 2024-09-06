use serde::Serialize;

use crate::utils::Location;

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct ModuleMemberUsageLocation {
  pub lib_name: String,
  pub member_name: String,
  pub start: u32,
  pub end: u32,
  pub file_path: String,
  pub loc: Location,
}
