extern crate clap;
extern crate csv;
#[macro_use] extern crate log;
extern crate pretty_env_logger;

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use clap::{App, Arg};

#[derive(Debug)]
struct GoogleLoadParseError {
    description: String
}

impl GoogleLoadParseError {
    fn new_boxed(description: String) -> Box<GoogleLoadParseError> {
        return Box::new(GoogleLoadParseError {
            description: description
        });
    }
}

impl fmt::Display for GoogleLoadParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser Error: {}", self.description)
    }
}

impl Error for GoogleLoadParseError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}

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

    match matches.value_of("action").unwrap() {
        "trim" => {
            let in_dir = matches.value_of("input directory").unwrap();
            let out_dir = matches.value_of("output directory").unwrap();

            if let Err(e) = trim(Path::new(in_dir), Path::new(out_dir)) {
                error!("{}", e);
            }
        },
        action => {
            error!("Unreconginzed action: {}", action);
        }
    }
}

fn trim(input_dir: &Path, output_dir: &Path) -> Result<(), Box<Error>> {
    println!("Trim the csv files in '{}' then put the results to '{}'", input_dir.display(), output_dir.display());

    // Check inputs
    if !input_dir.is_dir() {
        return Err(GoogleLoadParseError::new_boxed(format!("'{}' is not a directory.", input_dir.display())));
    }
    if !output_dir.is_dir() {
        return Err(GoogleLoadParseError::new_boxed(format!("'{}' is not a directory.", output_dir.display())));
    }

    // Read the input files
    for entry in fs::read_dir(input_dir)? {
        let file_path = entry?.path();

        // check the extension
        if let Some(exe) = file_path.extension() {
            if exe != "gz" {
                continue;
            }
        } else {
            continue;
        }

        // uncompress
        let csv_path = uncompress(&file_path)?;

        // set output file name
        let out_file_path = output_dir.join(csv_path.file_name().unwrap());
        info!("Triming file '{}' to '{}'", csv_path.display(), out_file_path.display());

        // read the csv file
        let mut reader = csv::Reader::from_path(csv_path)?;
        let mut writer = csv::Writer::from_path(out_file_path)?;
        for record in reader.records() {
            let mut record = record?;
            record.truncate(6);
            writer.write_record(record.iter())?;
        }
        writer.flush()?;
    }
    
    Ok(())
}

fn uncompress(gz_file: &Path) -> Result<PathBuf, Box<Error>> {
    let output_path = gz_file.with_file_name(gz_file.file_stem().unwrap());

    info!("Uncompressing file '{}' to '{}'", gz_file.display(), output_path.display());

    let result = Command::new("gzip").arg("-kd").arg(gz_file).output()?;
    
    if result.status.success() {
        info!("Uncompressing file '{}' successfully", gz_file.display());
    } else {
        let message = String::from_utf8(result.stderr)?;

        if message.contains("already exists") {
            warn!("File '{}' already exists", output_path.display());
        } else {
            error!("Uncompressing error: {}", message);
        }
    }

    Ok(output_path)
}