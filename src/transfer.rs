
use std::ops::Add;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;
use std::fs;

use crate::error::GoogleLoadParseError;

const TIME_MIN: u64 = 600_000_000; // in us
const TIME_MAX: u64 = 2_506_200_000_000; // in us (29 days later)
const TIME_SLOT_LEN: u64 = 60_000_000; // a minute

struct NodeTimeMatrix<T>
        where T: Add<Output=T> + Default + Copy {
    matrix: Vec<Vec<T>>,
    time_count: usize
}

impl<T> NodeTimeMatrix<T>
        where T: Add<Output=T> + Default + Copy {
    pub fn new() -> NodeTimeMatrix<T> {
        NodeTimeMatrix {
            matrix: Vec::new(),
            time_count: 0
        }
    }

    pub fn node_count(&self) -> usize {
        self.matrix.len()
    }

    pub fn time_count(&self) -> usize {
        self.time_count
    }

    pub fn add(&mut self, node_id: usize, time_id: usize, val: T) {
        let node_count = self.matrix.len();

        // Ensure there is enough nodes
        if node_count <= node_id {
            for _ in 0 .. node_id - node_count + 1 {
                self.matrix.push(Vec::new());
            }
        }

        // Ensure there is enough time slots
        if self.matrix[node_id].len() <= time_id {
            for _ in 0 .. time_id - self.matrix[node_id].len() + 1 {
                self.matrix[node_id].push(T::default());
            }
        }

        // Update time_count
        if self.time_count < time_id {
            self.time_count = time_id;
        }

        self.matrix[node_id][time_id] = self.matrix[node_id][time_id] + val;
    }

    // NOTE: We expect no one give a node_id > node_count because it checks node_count.
    pub fn get(&self, node_id: usize, time_id: usize) -> T {
        if self.matrix[node_id].len() <= time_id {
            T::default()
        } else {
            self.matrix[node_id][time_id]
        }
    }
}

#[derive(Deserialize)]
struct Row {
    start_time: u64,
    end_time: u64,
    machine_id: u64,
    cpu_usage: f64
}

pub fn transfer(input_dir: &Path, output_dir: &Path) -> Result<(), Box<Error>> {
    // Check inputs
    if !input_dir.is_dir() {
        return Err(GoogleLoadParseError::new_boxed(format!("'{}' is not a directory.", input_dir.display())));
    }
    if !output_dir.is_dir() {
        return Err(GoogleLoadParseError::new_boxed(format!("'{}' is not a directory.", output_dir.display())));
    }

    println!("Reading the csv files in {}", input_dir.display());

    // Create a mapping table for machine id
    let mut machine_ids: HashMap<u64, usize> = HashMap::new();
    let mut next_machine_id: usize = 0;

    // Create a matrix
    let mut statistics: NodeTimeMatrix<f64> = NodeTimeMatrix::new();

    // Read the input files
    for entry in fs::read_dir(input_dir)? {
        let file_path = entry?.path();

        // check the extension
        if let Some(exe) = file_path.extension() {
            if exe != "csv" {
                continue;
            }
        } else {
            continue;
        }

        debug!("Reading file {}", file_path.display());

        // Read the file
        let mut reader = csv::Reader::from_path(file_path)?;
        for record in reader.records() {
            let record = record?;
            let row: Row = record.deserialize(None)?;

            // Translate the machine id
            let new_machine_id = match machine_ids.get(&row.machine_id) {
                Some(id) => *id,
                None => {
                    let new_id = next_machine_id;
                    next_machine_id += 1;
                    machine_ids.insert(row.machine_id, new_id);
                    new_id
                }
            };

            // Add the cpu usage
            let start = to_time_slot_index(row.start_time);
            let end = to_time_slot_index(row.end_time);
            for index in start .. end {
                statistics.add(new_machine_id, index, row.cpu_usage);
            }
        }
    }

    // Set the path to the output csv file
    println!("Get statistics spans {} nodes and {} minutes", statistics.node_count(), statistics.time_count());
    println!("Writing the result to '{}'", output_dir.display());

    // Write the result
    let day_limit = statistics.time_count() / 1440 + 1;
    for day_id in 0 .. day_limit {
        let start_time = day_id * 1440;
        let mut end_time = start_time + 1440; // not included

        if end_time > statistics.time_count() {
            end_time = statistics.time_count();
        }

        // Each day has a directory
        let day_dir = output_dir.join(format!("day-{}", day_id + 1));
        fs::create_dir_all(&day_dir)?;
        info!("Writing the result to {}", day_dir.display());

        // Every 1000 nodes use a csv file
        let range_limit = statistics.node_count() / 1000 + 1;
        for node_range_id in 0 .. range_limit {
            let start_node = node_range_id * 1000;
            let mut end_node = start_node + 1000; // not included

            if end_node > statistics.node_count() {
                end_node = statistics.node_count();
            }

            // Open a csv file
            let csv_file = day_dir.join(format!("node-{}-{}.csv", start_node + 1, end_node));
            debug!("Writing the result to {}", csv_file.display());
            let mut writer = csv::Writer::from_path(csv_file)?;
            for node_id in start_node .. end_node {
                for time_id in start_time .. end_time {
                    writer.write_field(statistics.get(node_id, time_id).to_string())?;
                }
                writer.write_record(None::<&[u8]>)?;
            }
            writer.flush()?;
        }
    }
    
    
    Ok(())
}

fn to_time_slot_index(timestamp: u64) -> usize {
    ((timestamp - TIME_MIN) / TIME_SLOT_LEN) as usize
}