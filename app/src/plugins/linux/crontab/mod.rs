use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxCrontab {}

impl Plugin for LinuxCrontab {
    fn name(&self) -> &str {
        &"linux_crontab"
    }

    fn description(&self) -> &str {
        &"Crontab jobs"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = format!("{}\\{}", _binary_directory, "crontab.sh");
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxCrontab {
    pub fn new() -> Self {
        LinuxCrontab {}
    }
}