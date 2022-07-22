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
        let command = "Get-LocalGroupMember -Group Administrators | Select-Object Name,ObjectClass | Sort-Object -Property Name | ForEach-Object { ConvertTo-Json @($_) }";
        match self.execute_command(&command) {
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