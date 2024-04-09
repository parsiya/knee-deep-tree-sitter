use std::collections::HashMap;

use crate::utils::{node_text, parse_code};

use tree_sitter::{Query, QueryCursor};

pub(crate) fn run() {
    let src = include_str!("source4.go");

    let parsed = parse_code(src).expect("couldn't parse the source code");

    let mut c = parsed.walk();

    // Create a map of nodes where the node is the key and the text is the value.
    let mut node_map = HashMap::new();

    'outer: loop {
        // 0. Add the current node to the map.
        node_map.insert(c.node(), node_text(c.node(), src));

        // 1. Go to its child and continue.
        if c.goto_first_child() {
            continue 'outer;
        }

        // 2. We've reached a leaf (node without a child). We will go to a sibling.
        if c.goto_next_sibling() {
            continue 'outer;
        }

        // 3. If there are no more siblings, we need to go back up.
        'inner: loop {
            // 4. Check if we've reached the root node. If so, we're done.
            if !c.goto_parent() {
                break 'outer;
            }
            // 5. Go to the previous node's sibling.
            if c.goto_next_sibling() {
                // And break out of the inner loop.
                break 'inner;
            }
        }
    }

    // See if we can use this map somewhere?

    // Run a query that extracts all function calls.
    let query_text = "(call_expression) @callee";

    let query =
        Query::new(tree_sitter_go::language(), query_text).expect("couldn't parse the query");
    let mut cursor = QueryCursor::new();
    let query_matches = cursor.matches(&query, parsed.root_node(), src.as_bytes());

    for one_match in query_matches {
        for capture in one_match.captures.iter() {
            // Get the current node's text from the map.
            let text_from_map = node_map.get(&capture.node).unwrap();
            // Get the current node's text from the source.
            let text_from_src = node_text(capture.node, src);

            println!("Text from map: {}", text_from_map);
            println!("Text from source: {}", text_from_src);
            println!("-----");

            assert_eq!(
                text_from_map, &text_from_src,
                "Different text from map: {} and text from source: {}",
                text_from_map, text_from_src
            );
        }
    }

    println!("Done");
}
