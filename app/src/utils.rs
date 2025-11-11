use colored::{ColoredString, Colorize};

//
pub fn info(message: &str) -> ColoredString {
    format!("[*] {}", message).white()
}

pub fn print_info(message: &str) {
    eprintln!("{}", info(message))
}

pub fn success(message: &str) -> ColoredString {
    format!("[+] {}", message).green().bold()
}

pub fn print_success(message: &str) {
    eprintln!("{}", success(message))
}

pub fn warning(message: &str) -> ColoredString {
    format!("[!] {}", message).yellow().bold()
}

pub fn print_warning(message: &str) {
    eprintln!("{}", warning(message))
}

pub fn error(message: &str) -> ColoredString {
    format!("[-] {}", message).red().bold()
}

pub fn print_error(message: &str) {
    eprintln!("{}", error(message))
}

use serde_json::{Map, Value};
use std::{fs::File, io::Write};
pub fn output_json(
    data: &Map<String, Value>,
    filename: String,
    verbose: &bool,
) -> Result<(), String> {
    let output_data = match serde_json::to_string_pretty(&data) {
        Ok(data) => data,
        Err(e) => return Err(format!("Error during data jsonify: {}", e)),
    };

    let mut output_file;
    match File::create(filename) {
        Ok(file) => output_file = file,
        Err(e) => return Err(format!("Error during output file creation: {}", e)),
    }

    if *verbose {
        println!("{}", output_data);
    }

    match output_file.write_all(output_data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error during output file writing: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_info_format() {
        let result = info("test message");
        assert!(result.to_string().contains("[*]"));
        assert!(result.to_string().contains("test message"));
    }

    #[test]
    fn test_success_format() {
        let result = success("test message");
        assert!(result.to_string().contains("[+]"));
        assert!(result.to_string().contains("test message"));
    }

    #[test]
    fn test_warning_format() {
        let result = warning("test message");
        assert!(result.to_string().contains("[!]"));
        assert!(result.to_string().contains("test message"));
    }

    #[test]
    fn test_error_format() {
        let result = error("test message");
        assert!(result.to_string().contains("[-]"));
        assert!(result.to_string().contains("test message"));
    }

    #[test]
    fn test_output_json_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_output.json");
        let file_path_str = file_path.to_str().unwrap().to_string();

        let mut data: Map<String, Value> = Map::new();
        data.insert("test_key".to_string(), json!({"value": "test_value"}));

        let result = output_json(&data, file_path_str.clone(), &false);
        assert!(result.is_ok());

        // Verify file was created and contains correct data
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("test_key"));
        assert!(content.contains("test_value"));
    }

    #[test]
    fn test_output_json_verbose() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_output_verbose.json");
        let file_path_str = file_path.to_str().unwrap().to_string();

        let mut data: Map<String, Value> = Map::new();
        data.insert("test_key".to_string(), json!({"value": "test_value"}));

        // With verbose=true, should still succeed
        let result = output_json(&data, file_path_str, &true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_json_invalid_path() {
        let mut data: Map<String, Value> = Map::new();
        data.insert("test_key".to_string(), json!({"value": "test_value"}));

        let result = output_json(&data, "/invalid/path/file.json".to_string(), &false);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Error during output file creation"));
    }

    #[test]
    fn test_output_json_empty_data() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_empty.json");
        let file_path_str = file_path.to_str().unwrap().to_string();

        let data: Map<String, Value> = Map::new();

        let result = output_json(&data, file_path_str.clone(), &false);
        assert!(result.is_ok());

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content.trim(), "{}");
    }
}
