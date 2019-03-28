
use std::error::Error;
use std::path::Path;
use std::fs::{self, File};
use std::process::Command;
use std::io::BufReader;

use libflate::gzip;

use crate::error::GoogleLoadParseError;

pub fn trim(input_dir: &Path, output_dir: &Path) -> Result<(), Box<Error>> {
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
        
        if let Err(e) = uncompress_and_trim(&file_path, output_dir) {
            error!("Detected error while parsing file {}: {}", file_path.display(), e);
        }
    }
    
    Ok(())
}

fn uncompress_and_trim(gz_file: &Path, out_dir: &Path) -> Result<(), Box<Error>> {
    // Open the file
    let reader = BufReader::new(File::open(gz_file)?);

    // Uncompress the gz file
    let reader = gzip::Decoder::new(reader)?;

    // set output file name
    let out_file_path = out_dir.join(gz_file.file_stem().unwrap());

    info!("Uncompressing and triming file '{}' to '{}'", gz_file.display(), out_file_path.display());

    // read the csv file
    let mut reader = csv::Reader::from_reader(reader);
    let mut writer = csv::Writer::from_path(out_file_path)?;
    let target_fields: [usize; 4] = [0, 1, 4, 5]; // [0: "start time", 1: "end time", 4: "node id", 5: "avg CPU usage"]
    for record in reader.records() {
        let record = record?;
        
        for field in target_fields.iter() {
            if let Some(v) = record.get(*field) {
                writer.write_field(v)?;
            } else {
                return Err(GoogleLoadParseError::new_boxed(
                    format!("there is no value at row {}", record.position().unwrap().line())
                ));
            }
        }

        writer.write_record(None::<&[u8]>)?;
    }
    writer.flush()?;

    Ok(())
}

// Left for benchmarking
#[allow(dead_code)]
fn uncompress_with_external(gz_file: &Path, out_dir: &Path) -> Result<(), Box<Error>> {
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

    // set output file name
    let out_file_path = out_dir.join(output_path.file_name().unwrap());
    info!("Triming file '{}' to '{}'", output_path.display(), out_file_path.display());

    // read the csv file
    let mut reader = csv::Reader::from_path(output_path)?;
    let mut writer = csv::Writer::from_path(out_file_path)?;
    for record in reader.records() {
        let mut record = record?;
        record.truncate(6);
        writer.write_record(record.iter())?;
    }
    writer.flush()?;

    Ok(())
}