use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxUsers {}

impl Plugin for LinuxUsers {
    fn name(&self) -> &str {
        &"linux_users"
    }

    fn description(&self) -> &str {
        &"Local users"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "cat /etc/passwd | cut -d: -f1 | sort";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxUsers {
    pub fn new() -> Self {
        LinuxUsers {}
    }
}