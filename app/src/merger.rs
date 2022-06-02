use std::fs;
use glob::glob;
use chrono::prelude::*;
use serde_json::{Value, Map};

use crate::constants::{SNAPSHOT_FILENAME, SNAPSHOT_EXTENSION};
use crate::utils::{info, success, output_json};

pub fn merge(merging_directory: &String, verbose: &bool) -> Result<(), String> {
    //let merging_glob = merging_directory.clone() + &"/*.json".to_string(); 
    let merging_glob = format!("{}/*.{}", merging_directory, SNAPSHOT_EXTENSION); 
    
    let files = match glob(merging_glob.as_str()) {
        Ok(files) => files,
        Err(e) => return Err(format!("Error during file reading from merging directory: {}", e)), 
    };

    let mut merged_data : Map::<String, Value> = Map::new();
    for entry in files {
        match entry {
            Ok(path) => {
                info(&format!("Reading file: {:?}", path.display()));
                match fs::read_to_string(path) {
                    Ok(read_data) => {
                        match serde_json::from_str(&read_data) {
                           Ok(data) => {
                            match data {
                                Value::Object(m) => {
                                    for (k, v) in m  {
                                        merged_data.insert(k.clone(), v.clone());
                                    }
                                }
                                _ => {},
                            }
                           },
                           Err(e) => return Err(format!("Error during data merging: {}", e))
                        }
                    },
                    Err(e) => return Err(format!("Error during data reading: {}", e)),
                };
            },
            Err(e) => return Err(format!("Error during data reading: {}", e)),
        }
    }

    let local_time = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let merged_snapshots_filename: String = format!("{}-{}.{}", SNAPSHOT_FILENAME, local_time, SNAPSHOT_EXTENSION);
    success(&format!("Merged snapshots file: {}", merged_snapshots_filename));
    output_json(&merged_data, merged_snapshots_filename, verbose)

}

