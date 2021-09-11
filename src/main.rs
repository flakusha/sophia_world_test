use std::io::BufReader;
use std::{fs::File, env, path::Path};
use clap::{App, Arg, ArgMatches};
use rect_utils::{detect_overlapping_pairs, display_rect_areas};
use serde::Deserialize;
mod rect_utils;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Deserialized data structure which contains some nested structures, see code
/// comments.
#[derive(Deserialize, Clone, Debug)]
struct TestInput {
    // draworder should be enum {topdown, downtop}, used string for simplicity
    draworder: String,
    id: usize,
    name: String,
    objects: Vec<RectObj>,
    // opacity should be float, although it's int in original .json file
    opacity: f32,
    visible: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RectObj {
    name: String,
    properties: Option<RectObjD>,
    width: f32, height: f32, x: f32, y: f32,
}

#[derive(Deserialize, Clone, Debug)]
struct RectObjD {
    flags: RectObjDT
}

#[derive(Deserialize, Clone, Debug)]
struct RectObjDT {
    name: String,
    r#type: String,
    value: String,
}

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

    // Evaluate that .json file exists
    if eval_input(&matches) {
        // Deserialize .json
        let content = process_json_file(&matches);
        let rectangles = content.objects;

        display_rect_areas(&rectangles);
        let overlapping = detect_overlapping_pairs(&rectangles);
        for rect in overlapping {
            println!("Rectangles: {} and {} intersection area: {}",
            rect.0.name, rect.1.name, rect.2);
        }
    }

}

/// Evaluate input path and exit application if *.json file is not provided.
fn eval_input(matches: &ArgMatches) -> bool {
    if let Some(i) = matches.value_of("input") {
        let p = Path::new(i);
        println!("{:?}", p);
        match p.exists() && p.is_file() {
            true => match p.extension().unwrap() == "json" { 
                true => return true,
                false => panic!("Provided file is not .json")
            },
            false => panic!("Provided file does not exist")
        }
    } else {
        panic!("No json file provided");
    }
}

/// Deserialize *.json file and print it's contents.
fn process_json_file(matches: &ArgMatches) -> TestInput {
    let file = Path::new(matches.value_of("input").unwrap());

    let content = read_test_input_from_file(file);
    // println!("Deserialized json: {:?}", content);
    content
}

/// Reads *.json file and parses it into TestInput. If there is error, panics
/// and displays error. Original serde_json example code refused to compile and
/// was replased with match statement.
fn read_test_input_from_file<P: AsRef<Path>>(path: P) -> TestInput {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    // let ti: = serde_json::from_reader(reader)?;
    let ti = serde_json::from_reader(reader);
    match ti {
        Ok(ti) => return ti,
        Err(error) => panic!("Could not deserialize .json: {}", error)
    }
}
