use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxRoot {}

impl Plugin for LinuxRoot {
    fn name(&self) -> &str {
        &"linux_root" 
    }

    fn description(&self) -> &str {
        &"Local root users"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "cat /etc/passwd | grep :0: | cut -d : -f 1 | sort";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxRoot {
    pub fn new() -> Self {
        LinuxRoot {}
    }
}