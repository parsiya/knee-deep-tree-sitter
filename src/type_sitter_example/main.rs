use type_sitter_lib::{OptionNodeResultExt, TypedNode};
use yak_sitter;
mod type_sitter_go;

fn main() {
    // Create a parser.
    let mut yak_parser =
        yak_sitter::Parser::new(&tree_sitter_go::language()).expect("couldn't create a Go parser");

    // Parse the code.
    let src = include_str!("source5.go");
    // We need to create a path for some reason.
    let path = std::path::Path::new("./source5.go");
    let parsed = yak_parser
        .parse_string(src.to_string(), Some(path), None, ())
        .expect("couldn't parse the code");

    let yak_root = type_sitter_go::go::SourceFile::try_from(parsed.root_node())
        .expect("failed to wrap code root node");

    // Now we can go through the nodes and filter function return values.
    let func_returns = yak_root
        .children(&mut yak_root.walk())
        // Go through all the children and unwrap them.
        .filter_map(|child| child.unwrap().regular())
        // Filter (function_declaration) statements.
        .filter_map(|n| n.function_declaration())
        // Get the "result" field for each (function_declaration)
        .filter_map(|n| n.result().flatten())
        // The result could be a (parameter_list) or (simple_type).
        // We're gonna ignore parameter_list here and only select simple types.
        .filter_map(|n| n.simple_type())
        // Convert to text.
        .map(|n| n.text())
        // Collect in a vector.
        .collect::<Vec<_>>();

    func_returns.iter().for_each(|r| println!("{}", r));
}
