use std::{env, process};

mod query;
mod utils;

fn main() {
    // Process command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided.
    if args.len() < 2 {
        println!("Need at least one argument. Valid options are 00, 01, and 02");
        process::exit(1);
    }

    // We will only use the first argument.
    let cmd = &args[1];

    match cmd.as_str() {
        "00" => query::run_query0(),
        "01" => query::run_query1(),
        "02" => query::run_query2(),
        &_ => println!("invalid option, use 00, 01, or 02 "),
    }
}
