use std::{env, process};

mod go_types;
mod query;
mod tree0;
mod tree1;
mod tree2;
mod tree3;
mod utils;

fn main() {
    // Process command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided.
    if args.len() < 2 {
        println!("Need at least one argument. Valid options are 00 to 06");
        process::exit(1);
    }

    // We will only use the first argument.
    let cmd = &args[1];

    match cmd.as_str() {
        "00" => query::run_query0(),
        "01" => query::run_query1(),
        "02" => query::run_query2(),
        "03" => tree0::run(),
        "04" => tree1::run(),
        "05" => tree2::run(),
        "06" => tree3::run(),
        &_ => println!("invalid option, use 00 to 06"),
    }
}
