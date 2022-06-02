// use serde_json::Map;
use serde_json::{Value};

use crate::config::Config;
use crate::plugins::{Plugin, OS};
use crate::validator::validate_windows_path;

pub struct WindowsYara {}

impl Plugin for WindowsYara {
    fn name(&self) -> &str {
        &"windows_yara"
    }

    fn description(&self) -> &str {
        &"Yara rule scanning"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let scan_path =  match _config.get_string_setting(self.name(), &"scan_path") {
            Some(v) => { if validate_windows_path(&v) {
                    v
                } else {
                    return Err(format!("Not valid scan_path setting: {}", v));
                }}, 
            None => "c:\\".to_string(),
        };

        let command = format!("{0}\\{1} {0}\\{2} {3}", _binary_directory, "yara64.exe", ".\\yara.yml", scan_path);
        match self.windows_powershell_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
        // let mut data : Map::<String, Value> = Map::new();
        // let mut splitted_line: Vec<&str>;
        // for line in output.trim().split("\r\n") {
        //     splitted_line = line.split_whitespace().collect();
        //     data.insert("rule".to_string(), serde_json::Value::String(splitted_line[0].to_string()));
        //     data.insert("file".to_string(), serde_json::Value::String(splitted_line[1].to_string()));
        // }
        // Ok(serde_json::Value::Object(data))   
    }
}

impl WindowsYara {
    pub fn new() -> Self {
        WindowsYara {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::include_str;
    use super::*;

    #[test]
    fn test_windows_yara() {
        let data = json!([
            "ExampleRule .\\file1.txt",
            "ExampleRule .\\file2.txt",
            ]);
        let output = include_str!("output.txt");
        let windows_yara = WindowsYara::new();
        assert_eq!(data, windows_yara.process(output).unwrap());
    }
}