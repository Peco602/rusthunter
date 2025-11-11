use chrono::prelude::Local;
use glob::glob;
use serde_json::{Map, Value};
use std::fs;

use crate::constants::SNAPSHOT_EXTENSION;
use crate::utils::{output_json, print_info, print_success};

pub fn merge(merging_directory: &String, snapshot_tag: &str, verbose: &bool) -> Result<(), String> {
    //let merging_glob = merging_directory.clone() + &"/*.json".to_string();
    let merging_glob = format!(
        "{}/{}*.{}",
        merging_directory, snapshot_tag, SNAPSHOT_EXTENSION
    );

    let files = match glob(merging_glob.as_str()) {
        Ok(files) => files,
        Err(e) => {
            return Err(format!(
                "Error during file reading from merging directory: {}",
                e
            ))
        }
    };

    let mut merged_data: Map<String, Value> = Map::new();
    for entry in files {
        match entry {
            Ok(path) => {
                print_info(&format!("Reading file: {:?}", path.display()));
                match fs::read_to_string(path) {
                    Ok(read_data) => match serde_json::from_str(&read_data) {
                        Ok(data) => {
                            if let Value::Object(m) = data {
                                for (k, v) in m {
                                    merged_data.insert(k.clone(), v.clone());
                                }
                            }
                        }
                        Err(e) => return Err(format!("Error during data merging: {}", e)),
                    },
                    Err(e) => return Err(format!("Error during data reading: {}", e)),
                };
            }
            Err(e) => return Err(format!("Error during data reading: {}", e)),
        }
    }

    let local_time = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let merged_snapshots_filename: String =
        format!("{}_{}.{}", snapshot_tag, local_time, SNAPSHOT_EXTENSION);
    print_success(&format!(
        "Merged snapshots file: {}",
        merged_snapshots_filename
    ));
    output_json(&merged_data, merged_snapshots_filename, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_snapshot(dir: &std::path::Path, filename: &str, content: &str) {
        let file_path = dir.join(filename);
        let mut file = File::create(file_path).unwrap();
        write!(file, "{}", content).unwrap();
    }

    #[test]
    fn test_merge_valid_files() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path();

        create_test_snapshot(
            test_dir,
            "snapshot-HOST1.json",
            r#"{"HOST1": {"plugin1": [{"key": "value1"}]}}"#,
        );
        create_test_snapshot(
            test_dir,
            "snapshot-HOST2.json",
            r#"{"HOST2": {"plugin2": [{"key": "value2"}]}}"#,
        );

        let result = merge(&test_dir.to_str().unwrap().to_string(), "snapshot", &false);
        assert!(result.is_ok());

        // Check merged file was created in current directory
        let files: Vec<_> = fs::read_dir(".")
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_str().unwrap().starts_with("snapshot_"))
            .collect();
        assert!(!files.is_empty(), "Merged file should be created");

        // Cleanup
        for file in files {
            let _ = fs::remove_file(file.path());
        }
    }

    #[test]
    fn test_merge_no_matching_files() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path();

        create_test_snapshot(
            test_dir,
            "other-HOST1.json",
            r#"{"HOST1": {"plugin1": [{"key": "value1"}]}}"#,
        );

        let result = merge(&test_dir.to_str().unwrap().to_string(), "snapshot", &false);
        // Should succeed but create empty merged file
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path();

        create_test_snapshot(
            test_dir,
            "snapshot-HOST1.json",
            r#"{"HOST1": invalid json}"#,
        );

        let result = merge(&test_dir.to_str().unwrap().to_string(), "snapshot", &false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Error during data merging"));
    }

    #[test]
    fn test_merge_nonexistent_directory() {
        let result = merge(&"/nonexistent/directory".to_string(), "snapshot", &false);
        // glob doesn't fail on nonexistent directory, it just returns no matches
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path();

        let result = merge(&test_dir.to_str().unwrap().to_string(), "snapshot", &false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_with_tag() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path();

        create_test_snapshot(
            test_dir,
            "custom_tag-HOST1.json",
            r#"{"HOST1": {"plugin1": [{"key": "value1"}]}}"#,
        );
        create_test_snapshot(
            test_dir,
            "other_tag-HOST2.json",
            r#"{"HOST2": {"plugin2": [{"key": "value2"}]}}"#,
        );

        let result = merge(
            &test_dir.to_str().unwrap().to_string(),
            "custom_tag",
            &false,
        );
        assert!(result.is_ok());

        // Should only merge files with custom_tag in current directory
        let merged_files: Vec<_> = fs::read_dir(".")
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_str().unwrap().starts_with("custom_tag_"))
            .collect();
        assert!(!merged_files.is_empty(), "Merged file should be created");

        // Cleanup
        for file in merged_files {
            let _ = fs::remove_file(file.path());
        }
    }
}
