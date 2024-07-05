#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod danger_string_usage;
mod module_member_usage;
mod oxc_visitor_processor;

pub use danger_string_usage::get_usage_of_danger_strings;
pub use module_member_usage::get_module_member_usage;
