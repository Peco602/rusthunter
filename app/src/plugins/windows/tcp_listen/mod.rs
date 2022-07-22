use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsTCPListen {}

impl Plugin for WindowsTCPListen {
    fn name(&self) -> &str {
        &"windows_tcp_listen"
    }

    fn description(&self) -> &str {
        &"TCP listening ports"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "Get-NetTCPConnection -State Listen | Select-Object -Property LocalAddress,LocalPort,@{Name='ProcessName';Expression={(Get-Process -Id $_.OwningProcess).Path}} | Sort-Object -Property LocalAddress,LocalPort | ForEach-Object { ConvertTo-Json @($_) }";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsTCPListen {
    pub fn new() -> Self {
        WindowsTCPListen {}
    }
}