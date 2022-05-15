use serde_json::Value;

use crate::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsTCPListen {}

impl Plugin for WindowsTCPListen {
    fn name(&self) -> &str {
        &"windows_tcp_listen"
    }

    fn description(&self) -> &str {
        &"List of users"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "Get-NetTCPConnection -State Listen | Sort-Object -Property LocalAddress,LocalPort | Select-Object -Property LocalAddress,LocalPort,@{Name=\"ProcessName\";Expression={(Get-Process -Id $_.OwningProcess).Path}} | ConvertTo-Json";
        match self.windows_powershell_command(&command) {
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

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::include_str;
    use super::*;

    #[test]
    fn windows_tcp_listen() {
        let data = json!([
            {
                "LocalAddress":  "::",
                "LocalPort":  135,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  445,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  5357,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  5985,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  5986,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  7680,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  47001,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49664,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49665,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49666,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49667,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49668,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49669,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49670,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  49671,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  135,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  5040,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49664,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49665,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49666,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49667,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49668,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49669,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49670,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  49671,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "10.27.128.9",
                "LocalPort":  139,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "172.21.128.1",
                "LocalPort":  139,
                "ProcessName":  null
            }
        ]);
        let output = include_str!("output.txt");
        let windows_tcp_listen = WindowsTCPListen::new();
        assert_eq!(data, windows_tcp_listen.process(output).unwrap());
    }
}