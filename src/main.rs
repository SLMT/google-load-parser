extern crate clap;
extern crate csv;
#[macro_use] extern crate log;
extern crate pretty_env_logger;

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
                    .arg(Arg::with_name("input file")
                        .required(true)
                        .index(2))
                    .get_matches();

    match matches.value_of("action").unwrap() {
        "trim" => {
            let filename = matches.value_of("input file").unwrap();
            
            println!("Trim file '{}'", filename);
        },
        action => {
            error!("Unreconginzed action: {}", action);
        }
    }
}
