use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct MacOSUsers {}

impl Plugin for MacOSUsers {
    fn name(&self) -> &str {
        &"macos_users"
    }

    fn description(&self) -> &str {
        &"Local users"
    }

    fn os(&self) -> OS {
        OS::MacOS
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "dscl . list /Users | grep -v \"^_\" | sort";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl MacOSUsers {
    pub fn new() -> Self {
        MacOSUsers {}
    }
}