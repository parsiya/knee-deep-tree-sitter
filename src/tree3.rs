use tree_sitter::{Query, QueryCursor};

use crate::go_types::parse_go_type;
use crate::utils::{node_string, parse_code};

pub(crate) fn run() {
    // Parse the code.
    let src = include_str!("source5.go");
    let parsed = parse_code(src).expect("couldn't parse the source code");

    // Compile the query.
    let query = Query::new(
        tree_sitter_go::language(),
        "(function_declaration  result: (_) @res)",
    )
    .expect("couldn't parse the query");

    // Create a query cursor and create the matches.
    let mut cursor = QueryCursor::new();
    let query_matches = cursor.matches(&query, parsed.root_node(), src.as_bytes());

    for one_match in query_matches {
        for capture in one_match.captures.iter() {
            let current_node = capture.node;
            let n_text = node_string(current_node, src);

            parse_go_type(current_node, src).map_or_else(
                |e| {
                    println!(
                        "Error converting the parsed type to string: {} - node: {}",
                        e, n_text
                    )
                },
                |parsed_type| match serde_json::to_string(&parsed_type) {
                    Ok(s) => {
                        println!("{}", s)
                    }
                    Err(e) => {
                        println!(
                            "Error converting the parsed type to string: {} - node: {}",
                            e, n_text
                        )
                    }
                },
            );
        }
    }
}
