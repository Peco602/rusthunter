// https://stackoverflow.com/questions/39146584/how-do-i-create-a-rust-hashmap-where-the-value-can-be-one-of-multiple-types
// https://docs.serde.rs/serde_json/value/enum.Value.html
// https://docs.serde.rs/serde_json/

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

pub mod test;

use serde_json::Map;

#[cfg(target_os = "windows")]
use powershell_script;

use std::process::Command;
use std::{env};
use serde_json::Value;

use crate::config::Config;

pub trait Plugin {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn os(&self) -> OS;
    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String>;
    fn process(&self, output: &str) -> Result<Value, String>;
    
    fn show(&self) {
        println!("{: <32} {: <50} {:?}", self.name(), self.description(), self.os());
    }

    #[cfg(target_os = "linux")]
    fn linux_command(&self, command: &str) -> Result<String, String>  {
        let output = Command::new("sh")
                            .arg("-c")
                            .arg(command)
                            .output();

        match output {
            Ok(o) => {
                match String::from_utf8(o.stdout) {
                    Ok(s) => Ok(s),
                    Err(e) =>  Err(format!("Error during shell command output parsing: {}", e)),
                }
            },
            Err(e) => Err(format!("Error during shell command execution: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    fn macos_command(&self, command: &str) -> Result<String, String>  {
        let output = Command::new("zsh")
                            .arg("-c")
                            .arg(command)
                            .output();

        match output {
            Ok(o) => {
                match String::from_utf8(o.stdout) {
                    Ok(s) => Ok(s),
                    Err(e) =>  Err(format!("Error during shell command output parsing: {}", e)),
                }
            },
            Err(e) => Err(format!("Error during shell command execution: {}", e)),
        }
    }

    #[cfg(target_os = "windows")]
    fn windows_cmd_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("cmd")
                            .args(["/C", command])
                            .output();
        match output {
            Ok(o) => {
                match String::from_utf8(o.stdout) {
                    Ok(s) => Ok(s),
                    Err(e) =>  Err(format!("Error during cmd command output parsing: {}", e)),
                }
            },
            Err(e) => Err(format!("Error during cmd command execution: {}", e)),
        }
    }

    #[cfg(target_os = "windows")]
    fn windows_powershell_command(&self, command: &str) -> Result<String, String> {
        match powershell_script::run(command, false) {
            Ok(o) => Ok(o.to_string()),
            Err(e) => Err(format!("Error during powershell command execution: {}", e)),
        }
    }

    fn _get_splitter(&self) -> &str {
        match self.os() {
            OS::Windows => "\r\n",
            OS::Linux => "\n",
            OS::macOS => "\n",
        }
    }

    fn _split_list(&self, output: &str) -> Result<Value, String> {
        let mut list: Vec<Value> = Vec::new();
        for line in output.trim().split(self._get_splitter()) {
            list.push(serde_json::Value::String(line.trim().to_string()));
        }
        Ok(serde_json::Value::Array(list))
    }

    fn _convert_json_string(&self, output: &str) -> Result<Value, String> {
        match serde_json::from_str(output) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("Error during JSON parsing: {}", e)),
        }
    }

    fn _convert_csv_string_no_header(&self, output: &str, headers: &Vec<&str>, separator: &str) -> Result<Value, String> {
        let mut list: Vec<Value> = Vec::new();
        let mut map: Map::<String, Value>;
        let mut items;
        for line in output.trim().split(self._get_splitter()) {
            if !line.is_empty() {
                map = Map::new();
                items = line.split(separator);
                for header in headers {
                    map.insert(header.to_string(), serde_json::Value::String(items.next().unwrap().to_string()));
                }
                list.push(Value::Object(map));
            }
        }
        Ok(serde_json::Value::Array(list))   
    }
        
    fn _convert_csv_string_with_header(&self, output: &str, separator: &str) -> Result<Value, String> {
        let mut lines = output.trim().split(self._get_splitter());
        let headers: Vec<&str>  = lines.nth(0).unwrap().split(separator).collect(); 
        let _output: Vec<&str> = lines.collect();
        self._convert_csv_string_no_header(&_output.join(self._get_splitter()), &headers, separator)
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum OS {
    Windows,
    Linux,
    macOS
}

pub fn os() -> OS {
    match env::consts::OS {
        "linux" => OS::Linux,
        "windows" => OS::Windows,
        "macos" => OS::macOS,
        _ => {
            // ios
            // freebsd
            // dragonfly
            // netbsd
            // openbsd
            // solaris
            // android
            panic!("Unknown operating system")
        }
    }
}