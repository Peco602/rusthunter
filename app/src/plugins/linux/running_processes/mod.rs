use serde_json::{json, Value};

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxRunningProcesses {}

impl Plugin for LinuxRunningProcesses {
    fn name(&self) -> &str {
        "linux_running_processes"
    }

    fn description(&self) -> &str {
        "Running processes with full command line"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "ps aux --no-headers";
        match self.execute_command(command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        let mut processes = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 11 {
                // Combine command and arguments
                let command = parts[10..].join(" ");

                let process = json!({
                    "User": parts[0],
                    "PID": parts[1],
                    "CPU": parts[2],
                    "MEM": parts[3],
                    "VSZ": parts[4],
                    "RSS": parts[5],
                    "TTY": parts[6],
                    "STAT": parts[7],
                    "START": parts[8],
                    "TIME": parts[9],
                    "Command": command
                });
                processes.push(process);
            }
        }

        Ok(json!(processes))
    }
}

impl Default for LinuxRunningProcesses {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxRunningProcesses {
    pub fn new() -> Self {
        LinuxRunningProcesses {}
    }
}

