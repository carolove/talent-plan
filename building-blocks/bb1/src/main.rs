#[macro_use]
extern crate clap;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use clap::{Arg, App, SubCommand};
use bb1::Move;
use serde_json;
use std::str;
use ron;
use bson;


fn main() {
    serde();
}

fn matches(){
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let yamlMatches = App::from_yaml(yaml).get_matches();

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();
}

fn serde(){
    let a :Move = Move{ x:12,y:23};
    let a_vec = serde_json::to_vec(&a).unwrap();
    let a_string = str::from_utf8(&a_vec).unwrap();
    println!("{}", a_string);
    let b: Move = serde_json::from_slice(&a_vec).unwrap();
    println!("{:?}", b);
    let c = ron::ser::to_string(&b).unwrap();
    println!("{}", c);
    let d : Move = ron::de::from_str(&c).unwrap();
    println!("{:?}", d);
    let e = bson::to_bson(&d).unwrap();
    println!("{}", e);
    let f:Move = bson::from_bson(e).unwrap();
    println!("{:?}", f);
    let path = Path::new("hello.txt");
    let display = path.display();
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true).open(path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };
    let g = serde_json::to_vec(&f).unwrap();
    match file.write_all(&g) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => print!("{} size :\n{}", display,  g.len()),
    }
}