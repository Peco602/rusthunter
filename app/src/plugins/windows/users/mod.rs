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
        match self.execute_command(&command) {
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