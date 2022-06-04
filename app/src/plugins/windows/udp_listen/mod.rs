use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsUDPListen {}

impl Plugin for WindowsUDPListen {
    fn name(&self) -> &str {
        &"windows_udp_listen"
    }

    fn description(&self) -> &str {
        &"UDP listening ports"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "Get-NetUDPEndpoint | Select-Object -Property LocalAddress,LocalPort,@{Name=\"ProcessName\";Expression={(Get-Process -Id $_.OwningProcess).Path}} | Sort-Object -Property LocalAddress,LocalPort | ConvertTo-Json";
        match self.windows_powershell_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsUDPListen {
    pub fn new() -> Self {
        WindowsUDPListen {}
    }
}