use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxUptime {}

impl Plugin for LinuxUptime {
    fn name(&self) -> &str {
        &"linux_uptime"
    }

    fn description(&self) -> &str {
        &"Tell how long the system has been running"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "uptime -s; uptime -p";  // To be updated
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxUptime {
    pub fn new() -> Self {
        LinuxUptime {}
    }
}