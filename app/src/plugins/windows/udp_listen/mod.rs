use serde_json::Value;

use crate::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsUDPListen {}

impl Plugin for WindowsUDPListen {
    fn name(&self) -> &str {
        &"windows_udp_listen"
    }

    fn description(&self) -> &str {
        &"List of users"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "Get-NetUDPEndpoint | Sort-Object -Property LocalAddress,LocalPort | Select-Object -Property LocalAddress,LocalPort,@{Name=\"ProcessName\";Expression={(Get-Process -Id $_.OwningProcess).Path}} | ConvertTo-Json";
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

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::include_str;
    use super::*;

    #[test]
    fn windows_udp_listen() {
        let data = json!([
            {
                "LocalAddress":  "::",
                "LocalPort":  123,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  500,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  3702,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  4500,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  5353,
                "ProcessName":  "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe"
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  5355,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  50417,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::",
                "LocalPort":  55553,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::1",
                "LocalPort":  1900,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "::1",
                "LocalPort":  58706,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  53,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  123,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  500,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  3702,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  4500,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  5050,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  5353,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  5355,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  50415,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  50416,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "0.0.0.0",
                "LocalPort":  55552,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "10.27.128.9",
                "LocalPort":  137,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "10.27.128.9",
                "LocalPort":  138,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "10.27.128.9",
                "LocalPort":  1900,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "10.27.128.9",
                "LocalPort":  58708,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "127.0.0.1",
                "LocalPort":  1900,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "127.0.0.1",
                "LocalPort":  53913,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "127.0.0.1",
                "LocalPort":  58709,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "172.21.128.1",
                "LocalPort":  137,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "172.21.128.1",
                "LocalPort":  138,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "172.21.128.1",
                "LocalPort":  1900,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "172.21.128.1",
                "LocalPort":  58710,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "fe80::89d6:45d4:d70e:b4ce%6",
                "LocalPort":  1900,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "fe80::89d6:45d4:d70e:b4ce%6",
                "LocalPort":  58705,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "fe80::a456:3d76:6dde:a4b6%19",
                "LocalPort":  1900,
                "ProcessName":  null
            },
            {
                "LocalAddress":  "fe80::a456:3d76:6dde:a4b6%19",
                "LocalPort":  58707,
                "ProcessName":  null
            }
        ]);
        let output = include_str!("output.txt");
        let windows_udp_listen = WindowsUDPListen::new();
        assert_eq!(data, windows_udp_listen.process(output).unwrap());
    }
}