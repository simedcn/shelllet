#[macro_use]
extern crate clap;
extern crate winreg;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate dyon;

extern crate winapi;
#[macro_use]
extern crate log;
extern crate env_logger;


extern crate xml;
use std::mem;
use std::sync::Arc;

mod engine;
mod parser;

use clap::{App, Arg, SubCommand};
use log::{info, trace, warn};
fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    
    env_logger::Builder::from_env(env).init();


    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }


    let input = matches.value_of("INPUT");
    let path = std::path::Path::new(input.expect("msg: &str"));
    if path.extension() == Some(std::ffi::OsStr::new("dyon")) {
        engine::context::load(path.to_str().expect("sss").to_owned());
    } else if path.extension() == Some(std::ffi::OsStr::new("xml")) {
        let code = parser::parser::parse(path.to_str().expect("sss").to_owned());
        info!("\n{}", code);
        engine::context::load_code(code);
    }
}
