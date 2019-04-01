extern crate clap;
extern crate csv;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate libflate;
#[macro_use] extern crate serde;

mod error;
mod trim;
mod transfer;

use std::path::Path;
use clap::{App, Arg};

fn main() {
    // Initialize the logger
    pretty_env_logger::init();

    // Read arguments
    let matches = App::new("google-load-parser")
                    .version("v1.0.0")
                    .author("SLMT <sam123456777@gmail.com>")
                    .about("Parse the load trace of Google's testing cluster")
                    .arg(Arg::with_name("action")
                        .help("trim")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("input directory")
                        .required(true)
                        .index(2))
                    .arg(Arg::with_name("output directory")
                        .required(true)
                        .index(3))
                    .get_matches();
    
    let in_dir = matches.value_of("input directory").unwrap();
    let out_dir = matches.value_of("output directory").unwrap();

    match matches.value_of("action").unwrap() {
        "trim" => {
            if let Err(e) = trim::trim(Path::new(in_dir), Path::new(out_dir)) {
                error!("{}", e);
            }
        },
        "transfer" => {
            if let Err(e) = transfer::transfer(Path::new(in_dir), Path::new(out_dir)) {
                error!("{}", e);
            }
        },
        action => {
            error!("Unreconginzed action: {}", action);
        }
    }
}

