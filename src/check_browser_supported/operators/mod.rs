use super::visitor::SyntaxVisitor;

pub mod addition;
pub mod addition_assignment;
pub mod assignment;
pub mod async_function;
pub mod async_generator_function;
pub mod r#await;
pub mod await_top_level;
pub mod bitwise_and;
pub mod bitwise_and_assignment;
pub mod bitwise_not;
pub mod bitwise_or;
pub mod bitwise_or_assignment;
pub mod bitwise_xor;
pub mod bitwise_xor_assignment;
pub mod class;
pub mod comma;
pub mod conditional;
pub mod decrement;
pub mod delete;
pub mod destructuring;
pub mod destructuring_computed_property_names;
pub mod destructuring_rest_in_arrays;
pub mod destructuring_rest_in_objects;
pub mod division;
pub mod division_assignment;
pub mod equality;
pub mod exponentiation;
pub mod exponentiation_assignment;
pub mod function;
pub mod function_trailing_comma;
pub mod generator_function;
pub mod generator_function_trailing_comma;
pub mod greater_than;
pub mod greater_than_or_equal;
pub mod grouping;
pub mod import;
pub mod import_meta;
pub mod import_meta_resolve;
pub mod import_options_parameter;
pub mod r#in;
pub mod increment;
pub mod inequality;
pub mod instanceof;
pub mod left_shift;
pub mod left_shift_assignment;
pub mod less_than;
pub mod less_than_or_equal;
pub mod logical_and;
pub mod logical_and_assignment;
pub mod logical_not;
pub mod logical_or;
pub mod logical_or_assignment;
pub mod multiplication;
pub mod multiplication_assignment;
pub mod new;
pub mod new_target;
pub mod null;
pub mod nullish_coalescing;
pub mod nullish_coalescing_assignment;
pub mod object_initializer;
pub mod object_initializer_computed_property_names;
pub mod object_initializer_shorthand_method_names;
pub mod object_initializer_shorthand_property_names;
pub mod object_initializer_spread_properties;
pub mod optional_chaining;
pub mod property_accessors;
pub mod r#r_super;
pub mod remainder;
pub mod remainder_assignment;
pub mod right_shift;
pub mod right_shift_assignment;
pub mod spread;
pub mod spread_spread_in_arrays;
pub mod spread_spread_in_function_calls;
pub mod spread_spread_in_object_literals;
pub mod strict_equality;
pub mod strict_inequality;
pub mod subtraction;
pub mod subtraction_assignment;
pub mod this;
pub mod r#typeof;
pub mod unary_negation;
pub mod unary_plus;
pub mod unsigned_right_shift;
pub mod unsigned_right_shift_assignment;
pub mod void;
pub mod r#yield;
pub mod yield_star;

pub fn setup_operators(v: &mut SyntaxVisitor) {
  // addition_assignment::setup(v);
  // addition::setup(v);
  // assignment::setup(v);
  async_function::setup(v);
  async_generator_function::setup(v);
  await_top_level::setup(v);
  r#await::setup(v);
  // bitwise_and_assignment::setup(v);
  // bitwise_and::setup(v);
  // bitwise_not::setup(v);
  // bitwise_or_assignment::setup(v);
  // bitwise_or::setup(v);
  // bitwise_xor_assignment::setup(v);
  // bitwise_xor::setup(v);
  class::setup(v);
  // comma::setup(v);
  // conditional::setup(v);
  // decrement::setup(v);
  // delete::setup(v);
  destructuring_computed_property_names::setup(v);
  destructuring_rest_in_arrays::setup(v);
  destructuring_rest_in_objects::setup(v);
  destructuring::setup(v);
  // division_assignment::setup(v);
  // division::setup(v);
  // equality::setup(v);
  exponentiation_assignment::setup(v);
  exponentiation::setup(v);
  function_trailing_comma::setup(v);
  // function::setup(v);
  generator_function_trailing_comma::setup(v);
  generator_function::setup(v);
  // greater_than_or_equal::setup(v);
  // greater_than::setup(v);
  // grouping::setup(v);
  import_meta_resolve::setup(v);
  import_meta::setup(v);
  import_options_parameter::setup(v);
  import::setup(v);
  // r#in::setup(v);
  // increment::setup(v);
  // inequality::setup(v);
  // instanceof::setup(v);
  // left_shift_assignment::setup(v);
  // left_shift::setup(v);
  // less_than_or_equal::setup(v);
  // less_than::setup(v);
  logical_and_assignment::setup(v);
  // logical_and::setup(v);
  // logical_not::setup(v);
  logical_or_assignment::setup(v);
  // logical_or::setup(v);
  // multiplication_assignment::setup(v);
  // multiplication::setup(v);
  new_target::setup(v);
  // new::setup(v);
  // null::setup(v);
  nullish_coalescing_assignment::setup(v);
  nullish_coalescing::setup(v);
  object_initializer_computed_property_names::setup(v);
  object_initializer_shorthand_method_names::setup(v);
  object_initializer_shorthand_property_names::setup(v);
  object_initializer_spread_properties::setup(v);
  // object_initializer::setup(v);
  optional_chaining::setup(v);
  // property_accessors::setup(v);
  r#r_super::setup(v);
  // remainder_assignment::setup(v);
  // remainder::setup(v);
  // right_shift_assignment::setup(v);
  // right_shift::setup(v);
  spread_spread_in_arrays::setup(v);
  spread_spread_in_function_calls::setup(v);
  spread_spread_in_object_literals::setup(v);
  spread::setup(v);
  // strict_equality::setup(v);
  // strict_inequality::setup(v);
  // subtraction_assignment::setup(v);
  // subtraction::setup(v);
  // this::setup(v);
  // r#typeof::setup(v);
  // unary_negation::setup(v);
  // unary_plus::setup(v);
  // unsigned_right_shift_assignment::setup(v);
  // unsigned_right_shift::setup(v);
  // void::setup(v);
  yield_star::setup(v);
  r#yield::setup(v);
}
