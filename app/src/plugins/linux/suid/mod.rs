use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxSuid {}

impl Plugin for LinuxSuid {
    fn name(&self) -> &str {
        &"linux_suid"
    }

    fn description(&self) -> &str {
        &"Files with setuid permissions"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "ls -la $(find / -uid 0 -perm -4000 -print 2>/dev/null)";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxSuid {
    pub fn new() -> Self {
        LinuxSuid {}
    }
}