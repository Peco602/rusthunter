use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsDomainComputers {}

impl Plugin for WindowsDomainComputers {
    fn name(&self) -> &str {
        &"windows_domain_computers"
    }

    fn description(&self) -> &str {
        &"Domain computers"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "Get-ADComputer -Filter * | Select-Object Name,Enabled | Sort-Object -Property Name | ForEach-Object { ConvertTo-Json @($_) }";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsDomainComputers {
    pub fn new() -> Self {
        WindowsDomainComputers {}
    }
}