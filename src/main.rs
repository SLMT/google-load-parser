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
use clap::{App, Arg, SubCommand};

fn main() {
    // Initialize the logger
    pretty_env_logger::init();

    // Read arguments
    let matches = App::new("google-load-parser")
                    .version("v1.1.0")
                    .author("SLMT <sam123456777@gmail.com>")
                    .about("Parse the load trace of Google's testing cluster")
                    .subcommand(SubCommand::with_name("trim")
                                .about("Trims the files in the given directory to leave only necessary data")
                                .arg(Arg::with_name("INPUT DIR")
                                    .help("the directory containing input files")
                                    .required(true)
                                    .index(1))
                                .arg(Arg::with_name("OUTPUT DIR")
                                    .help("the directory for placing output files")
                                    .required(true)
                                    .index(2)))
                    .subcommand(SubCommand::with_name("transfer")
                                .about("Transfers the files in the given directory to daily timeline files")
                                .arg(Arg::with_name("INPUT DIR")
                                    .help("the directory containing input files")
                                    .required(true)
                                    .index(1))
                                .arg(Arg::with_name("OUTPUT DIR")
                                    .help("the directory for placing output files")
                                    .required(true)
                                    .index(2))
                                .arg(Arg::with_name("SLOT LENGTH")
                                    .help("the length of time slot (in seconds)")
                                    .default_value("60")
                                    .index(3)))
                    .get_matches();

    if let Some(matches) = matches.subcommand_matches("trim") {
        let in_dir = matches.value_of("INPUT DIR").unwrap();
        let out_dir = matches.value_of("OUTPUT DIR").unwrap();

        if let Err(e) = trim::trim(Path::new(in_dir), Path::new(out_dir)) {
            error!("{}", e);
        }
    } else if let Some(matches) = matches.subcommand_matches("transfer") {
        let in_dir = matches.value_of("INPUT DIR").unwrap();
        let out_dir = matches.value_of("OUTPUT DIR").unwrap();
        let slot_len: u64 = matches.value_of("SLOT LENGTH").unwrap().parse()
                .expect("cannot parse the number to u64");

        if let Err(e) = transfer::transfer(Path::new(in_dir), Path::new(out_dir), slot_len) {
            error!("{}", e);
        }
    }
}

