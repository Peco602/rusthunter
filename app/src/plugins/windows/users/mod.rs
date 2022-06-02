use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsUsers {}

impl Plugin for WindowsUsers {
    fn name(&self) -> &str {
        &"windows_users"
    }

    fn description(&self) -> &str {
        &"Local users"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "Get-LocalUser | Select-Object Name,Enabled | Sort-Object -Property Name | ConvertTo-Json";
        match self.windows_powershell_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsUsers {
    pub fn new() -> Self {
        WindowsUsers {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::include_str;
    use super::*;

    #[test]
    fn test_windows_users() {
        let data = json!([
            {
                "Name":  "Administrator",
                "Enabled":  false
            },
            {
                "Name":  "DefaultAccount",
                "Enabled":  false
            },
            {
                "Name":  "giovanni",
                "Enabled":  true
            },
            {
                "Name":  "Guest",
                "Enabled":  false
            },
            {
                "Name":  "User",
                "Enabled":  true
            },
            {
                "Name":  "WDAGUtilityAccount",
                "Enabled":  false
            }
        ]);
        let output = include_str!("output.txt");
        let windows_users = WindowsUsers::new();
        assert_eq!(data, windows_users.process(output).unwrap());
    }
}