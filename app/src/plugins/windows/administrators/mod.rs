use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsAdministrators {}

impl Plugin for WindowsAdministrators {
    fn name(&self) -> &str {
        &"windows_administrators"
    }

    fn description(&self) -> &str {
        &"Local administrators"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "Get-LocalGroupMember -Group Administrators | Select-Object Name,ObjectClass | Sort-Object -Property Name | ConvertTo-Json";
        match self.windows_powershell_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsAdministrators {
    pub fn new() -> Self {
        WindowsAdministrators {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::include_str;
    use super::*;

    #[test]
    fn test_windows_administrators() {
        let data = json!([
            {
                "Name":  "WINDEV2110EVAL\\Administrator",
                "ObjectClass":  "User"
            },
            {
                "Name":  "WINDEV2110EVAL\\giovanni",
                "ObjectClass":  "User"
            },
            {
                "Name":  "WINDEV2110EVAL\\User",
                "ObjectClass":  "User"
            }
        ]);
        let output = include_str!("output.txt");
        let windows_administrators = WindowsAdministrators::new();
        assert_eq!(data, windows_administrators.process(output).unwrap());
    }
}

