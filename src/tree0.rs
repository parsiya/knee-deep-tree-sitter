use tree_sitter::{Query, QueryCursor};

use crate::utils::{node_string, node_text, parent_of_kind, parse_code};

pub(crate) fn run() {
    let query_text = "(call_expression) @callee";

    let src = include_str!("source3.go");

    // Parse the source.
    let parsed = match parse_code(src) {
        Some(p) => p,
        None => panic!("couldn't parse the source code"),
    };

    // Compile the query.
    let query =
        Query::new(tree_sitter_go::language(), query_text).expect("couldn't parse the query");

    // // We can also use capture_names. Let's print the slice to see what's there.
    // let capture_names = query.capture_names();

    // Create a query cursor and create the matches.
    let mut cursor = QueryCursor::new();
    let query_matches = cursor.matches(&query, parsed.root_node(), src.as_bytes());

    // Each match is a set of captures. In this case, each match has only one
    // capture.
    println!("captures:");
    for one_match in query_matches {
        for capture in one_match.captures.iter() {
            let current_node = capture.node;
            // We have one capture per match so this extra loop is not that
            // useful.

            // As an extra sanity check, we can check if the captured node is a
            // (call_expression).
            assert_eq!(
                current_node.kind(),
                "call_expression",
                "Expected (call_expression), got {}",
                current_node.kind()
            );

            // The current node is of type (call_expression). We can get the
            // function name from the "function" field.
            if let Some(callee_name) = current_node.child_by_field_name("function") {
                // Find the first (function_declaration) parent node.
                if let Some(caller) = parent_of_kind(&current_node, "function_declaration") {
                    // Parent function name is the "name" field of the
                    // (function_declaration) node.
                    if let Some(caller_name) = caller.child_by_field_name("name") {
                        println!(
                            "{} -> {}",
                            node_text(caller_name, src),
                            node_text(callee_name, src)
                        );
                        println!("-----");
                        // Done with this capture, we can break the loop early.
                        break;
                    }
                } else {
                    println!(
                        "Error: Reached the top of the tree w/o reaching a (function_declaration)."
                    );
                }
            } else {
                println!(
                    "Error: Captured a (call_expression) node w/o a 'function' field, got: {}",
                    node_string(current_node, src)
                );
            }
        }
    }
}
