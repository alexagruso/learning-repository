use std::{env, process};

use rustgrep::{run, QueryConfig};

fn main() {
    let query_config = QueryConfig::new(env::args().skip(1)).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = run(&query_config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
