use crate::utils::run_query;

// Extract function names.
pub(crate) fn run_query0() {
    // Get the source.
    let src = include_str!("source.go");

    // Query to extract function names from Go code.
    let query_extract_func_names = r#"
    (function_declaration
      name: (identifier) @func.name)
    "#;

    run_query(query_extract_func_names, src);
}

// Extract function parameters.
pub(crate) fn run_query1() {
    // Get the source.
    let src = include_str!("source.go");

    // Query to extract function names from Go code.
    let query_extract_params = include_str!("query1");

    run_query(query_extract_params, src);
}

// Extract function return values.
pub(crate) fn run_query2() {
    // Get the source.
    let src = include_str!("source2.go");
    let query_return_values = include_str!("query2");

    run_query(query_return_values, src);
}
