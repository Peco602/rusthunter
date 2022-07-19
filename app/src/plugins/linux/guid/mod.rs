use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxGuid {}

impl Plugin for LinuxGuid {
    fn name(&self) -> &str {
        &"linux_guid"
    }

    fn description(&self) -> &str {
        &"Files with setgid permission"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "find / -uid 0 -perm -2000 -print 2>/dev/null";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxGuid {
    pub fn new() -> Self {
        LinuxGuid {}
    }
}