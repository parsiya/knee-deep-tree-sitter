use tree_sitter::{Parser, Query, QueryCursor, Tree};

/// Extract the text of tree-sitter captured node from source.
fn node_text(node: tree_sitter::Node, src: &str) -> String {
    return src[node.start_byte()..node.end_byte()].to_string();
}

/// Return the node information as a pretty string.
pub(crate) fn node_string(node: tree_sitter::Node, src: &str) -> String {
    return format!(
        "Text: {} - Kind: {} - sexp: {}",
        node_text(node, src),
        node.kind(),
        node.to_sexp() // convert the node to S-expression
    );
}

/// Parse the source code and return the tree.
pub(crate) fn parse_code(src: &str) -> Option<Tree> {
    // Get the language from tree_sitter_go.
    // It's Go Lang, har har!
    let go_lang = tree_sitter_go::language();

    let mut parser = Parser::new();

    parser
        .set_language(go_lang)
        .expect("Error loading Go grammar");

    return parser.parse(src, None);
}

/// Return the contents of a &[T] as text.
pub(crate) fn print_slice<T>(slc: &[T]) -> String
where
    T: std::fmt::Debug,
{
    let mut output = String::new();

    for i in 0..slc.len() {
        output.push_str(format!("({}, {:#?})\n", i, slc[i]).as_str());
    }
    return output;
}

/// Runs query on the file and prints the results.
pub(crate) fn run_query(query_text: &str, src: &str) {
    // Parse the source.
    let parsed = match parse_code(src) {
        Some(p) => p,
        None => panic!("couldn't parse the source code"),
    };

    // Compile the query.
    let query =
        Query::new(tree_sitter_go::language(), query_text).expect("couldn't parse the query");

    // We can also use capture_names. Let's print the slice to see what's there.
    let capture_names = query.capture_names();

    println!("capture_names slice:");
    println!("{}", print_slice(capture_names));
    println!("-----");

    // Create a query cursor and create the matches.
    let mut cursor = QueryCursor::new();
    let query_matches = cursor.matches(&query, parsed.root_node(), src.as_bytes());

    // Each match is a set of captures. In this case, each match has only one
    // capture.
    println!("captures:");
    for one_match in query_matches {
        for capture in one_match.captures.iter() {
            println!(
                "capture: {} - {}",
                capture_names[capture.index as usize],
                node_string(capture.node, src)
            );
        }
        println!("-----");
    }
}
