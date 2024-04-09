use tree_sitter::{Query, QueryCursor};

use crate::utils::{node_text, parent_of_kind, parse_code};

#[derive(Debug)]
struct ChildFunction {
    name: String,
    package: String,
    parent: String,
}

pub(crate) fn run() {
    let query_text = "(call_expression) @callee";

    let src = include_str!("source4.go");

    // Parse the source.
    let parsed = parse_code(src).expect("couldn't parse the source code");

    // Compile the query.
    let query =
        Query::new(tree_sitter_go::language(), query_text).expect("couldn't parse the query");

    // // We can also use capture_names. Let's print the slice to see what's there.
    // let capture_names = query.capture_names();

    // Create a query cursor and create the matches.
    let mut cursor = QueryCursor::new();
    let query_matches = cursor.matches(&query, parsed.root_node(), src.as_bytes());

    // Create a vector to store functions.
    let mut funcs: Vec<ChildFunction> = Vec::new();

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

            let mut child_function: ChildFunction = ChildFunction {
                name: "".to_string(),
                package: "".to_string(),
                parent: "".to_string(),
            };

            // (call_expression) always has a "function" field so we can
            // simplify our code and just unwrap.
            let callee = current_node.child_by_field_name("function").unwrap();
            match callee.kind() {
                "identifier" => {
                    child_function.name = node_text(callee, src);
                }
                // Same with named fields of (selector_expression).
                "selector_expression" => {
                    child_function.package =
                        node_text(callee.child_by_field_name("operand").unwrap(), src);

                    child_function.name =
                        node_text(callee.child_by_field_name("field").unwrap(), src);
                }
                _ => {
                    println!(
                        "The 'function' field of node is of the unexpected kind, got: {}",
                        callee.kind()
                    );
                    continue;
                }
            };

            // Find the first parent of the callee of type `function_declaration`.
            if let Some(caller) = parent_of_kind(&callee, "function_declaration") {
                // Parent function name is the "name" field of the
                // (function_declaration) node.
                let caller_name = caller.child_by_field_name("name").unwrap();
                child_function.parent = node_text(caller_name, src);
                funcs.push(child_function);
            } else {
                // No parents found of type `function_declaration`. If this
                // happens the source code was wrong because everything in Go
                // is wrapped in a (function_declaration).
                panic!("Reached the top of the tree w/o reaching a (function_declaration)");
            }
        }
    }

    // Print the results in the funcs vector.
    for f in funcs {
        if f.package == "" {
            println!("{} -> {}", f.parent, f.name);
        } else {
            println!("{} -> {}.{}", f.parent, f.package, f.name);
        }
    }
}
