// Go types?

use crate::utils::node_text;
use serde::{Deserialize, Serialize};
use std::fmt;
use tree_sitter::Node;

type Result<T> = std::result::Result<T, TypeError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeError {
    msg: String,
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl TypeError {
    /// Create a new error with msg.
    pub fn new(msg: String) -> TypeError {
        return TypeError { msg };
    }

    /// Wrap a String as a TypeError.
    pub fn wrap_string(msg: String) -> Result<GoType> {
        return Err(TypeError::new(msg));
    }
}

/// Represents a Go type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoType {
    SimpleType(SimpleType), // Lone (type_identifier)
    Slice(Slice),
    Pointer(Pointer),
    Array(Array),
    Map(Map),
    Channel(Channel),
    Qualified(Qualified),
}

/// A simple type which is just an identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleType {
    internal_type: String,
}

/// A slice. [](_type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slice {
    internal_type: Box<GoType>,
}

/// A pointer. *(_type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pointer {
    internal_type: Box<GoType>,
}

/// [length](_type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Array {
    internal_type: Box<GoType>,
    length: String, // Length is an (_expression), we will just store the text here.
}

/// map[(_type)](_type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    key: Box<GoType>,
    value: Box<GoType>,
}

/// E.g., chan <- (_type).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    internal_type: Box<GoType>,
}

/// package_name.(_type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qualified {
    internal_type: Box<GoType>,
    package: String, // Just an (identifier).
}

/// Convert a tree-sitter node that is a _simple_type to a GoType.
pub fn parse_go_type(n: Node, src: &str) -> Result<GoType> {
    // Check the type of the node.
    let n_text = node_text(n, src);
    let n_kind = n.kind();
    match n_kind {
        // If it's an (identifier), return a SimpleType.
        "type_identifier" => Ok(GoType::SimpleType(SimpleType {
            internal_type: n_text,
        })),

        // It's a slice. The `element` field has the type.
        "slice_type" => {
            // Get the element field.
            if let Some(element) = n.child_by_field_name("element") {
                parse_go_type(element, src).map_or_else(
                    |e| {
                        TypeError::wrap_string(format!(
                            "Couldn't parse the type of {}, text: {}, err: {}",
                            n_kind, n_text, e.msg
                        ))
                    },
                    // Return a slice with the parsed type.
                    |s_type| {
                        Ok(GoType::Slice(Slice {
                            internal_type: Box::new(s_type),
                        }))
                    },
                )
            } else {
                // Return an error if the element field doesn't exist.
                TypeError::wrap_string(format!(
                    "Got a {} without an element field, text: {}",
                    n_kind, n_text,
                ))
            }
        }

        // It's a pointer. It has a child with of type `_type`.
        "pointer_type" => {
            // The first child is `*`.
            // The second child contains the type of the pointer.
            if let Some(pointer_type) = n.child(1) {
                // Parse the type and return an error if it cannot be parsed.
                parse_go_type(pointer_type, src).map_or_else(
                    |e| {
                        TypeError::wrap_string(format!(
                            "Couldn't parse the type of {}, text: {}, err: {}",
                            n_kind, n_text, e.msg
                        ))
                    },
                    // If it was parsed correctly, return a Pointer.
                    |p_type| {
                        Ok(GoType::Pointer(Pointer {
                            internal_type: Box::new(p_type),
                        }))
                    },
                )
            } else {
                // Return an error if it doesn't have a child.
                TypeError::wrap_string(format!(
                    "Got a pointer_type without a child, kind: {}, text: {}",
                    n_kind, n_text,
                ))
            }
        }

        // It's an array.
        "array_type" => {
            // Assuming parsing was correct and array_type has both length and
            // element fields.
            let length = n.child_by_field_name("length").unwrap();
            let element = n.child_by_field_name("element").unwrap();

            parse_go_type(element, src).map_or_else(
                |e| {
                    TypeError::wrap_string(format!(
                        "Couldn't parse the type of {}, text: {}, err: {}",
                        n_kind, n_text, e.msg
                    ))
                },
                // Return an Array with the parsed type.
                |a_type| {
                    Ok(GoType::Array(Array {
                        internal_type: Box::new(a_type),
                        length: node_text(length, src),
                    }))
                },
            )
        }

        // It's a map.
        "map_type" => {
            // Assuming parsing was correct and map_type has two children, key
            // and value.
            let k = n.child_by_field_name("key").unwrap();
            let v = n.child_by_field_name("value").unwrap();

            parse_go_type(k, src).map_or_else(
                |e| {
                    TypeError::wrap_string(format!(
                        "Couldn't parse the key type of {}, text: {}, err: {}",
                        n_kind, n_text, e.msg
                    ))
                },
                // If key type was parsed correctly, parse the value type.
                |key_type| {
                    parse_go_type(v, src).map_or_else(
                        |e| {
                            TypeError::wrap_string(format!(
                                "Couldn't parse the value type of {}, text: {}, err: {}",
                                n_kind, n_text, e.msg
                            ))
                        },
                        // Both key and value types were parsed correctly. Return a Map.
                        |value_type| {
                            Ok(GoType::Map(Map {
                                key: Box::new(key_type),
                                value: Box::new(value_type),
                            }))
                        },
                    )
                },
            )
        }

        // It's a channel.
        "channel_type" => {
            // Assuming the channel_type has a value field.
            let val = n.child_by_field_name("value").unwrap();
            // Parse the type.
            parse_go_type(val, src).map_or_else(
                |e| {
                    TypeError::wrap_string(format!(
                        "Couldn't parse the value field of {}, text: {}, err: {}",
                        n_kind, n_text, e.msg
                    ))
                },
                // Return a Channel.
                |chan_type| {
                    Ok(GoType::Channel(Channel {
                        internal_type: Box::new(chan_type),
                    }))
                },
            )
        }

        // It's a qualified_type.
        "qualified_type" => {
            // Package name is just an identifier so we just store the text.
            let package_name = node_text(n.child_by_field_name("package").unwrap(), src);

            // The name field has the type.
            parse_go_type(n.child_by_field_name("name").unwrap(), src).map_or_else(
                |e| {
                    TypeError::wrap_string(format!(
                        "Couldn't parse the name field of {}, text: {}, err: {}",
                        n_kind, n_text, e.msg
                    ))
                },
                // If type was parsed, return a Qualified.
                |q_type| {
                    Ok(GoType::Qualified(Qualified {
                        internal_type: Box::new(q_type),
                        package: package_name,
                    }))
                },
            )
        }

        // Everything else, return an error.
        _ => TypeError::wrap_string(format!(
            "error parsing the type, kind: {}, text: {}",
            n_kind, n_text,
        )),
    }
}
