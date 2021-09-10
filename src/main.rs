use serde_json;
use std::env;
use std::io::prelude::*;
use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// Collects .json data
fn main() {
    // Basic application information
    let matches = App::new("Sophia World json test app")
        .version(VERSION)
        .author("Constantine F. <zenflak@gmail.com>")
        .about("Parses provided json file")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("json file path to read")
                .takes_value(true),
        )
        .get_matches();

    if let Some(i) = matches.value_of("input") {
        println!("Processing {}", i);
    } else {
        panic!("No json file provided");
    }

}
