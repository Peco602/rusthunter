use serde_json::{json, Value};

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxNetworkConnections {}

impl Plugin for LinuxNetworkConnections {
    fn name(&self) -> &str {
        "linux_network_connections"
    }

    fn description(&self) -> &str {
        "Active network connections (TCP/UDP)"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        // Use ss command (modern replacement for netstat)
        let command = "ss -tuanp 2>/dev/null | tail -n +2";
        match self.execute_command(command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        let mut connections = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                // Extract process info if available
                let process = if parts.len() > 6 {
                    parts[6..].join(" ")
                } else {
                    "-".to_string()
                };

                let connection = json!({
                    "Protocol": parts[0],
                    "State": parts[1],
                    "LocalAddress": parts[4],
                    "RemoteAddress": if parts.len() > 5 { parts[5] } else { "-" },
                    "Process": process
                });
                connections.push(connection);
            }
        }

        Ok(json!(connections))
    }
}

impl Default for LinuxNetworkConnections {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxNetworkConnections {
    pub fn new() -> Self {
        LinuxNetworkConnections {}
    }
}

