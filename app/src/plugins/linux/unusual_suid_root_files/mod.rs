use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxUnusualSuidRootFiles {}

impl Plugin for LinuxUnusualSuidRootFiles {
    fn name(&self) -> &str {
        &"linux_unusual_suid_root_files"
    }

    fn description(&self) -> &str {
        &"This requires knowledge of normal SUID files."
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

impl LinuxUnusualSuidRootFiles {
    pub fn new() -> Self {
        LinuxUnusualSuidRootFiles {}
    }
}