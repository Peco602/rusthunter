use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxDnsInUse {}

impl Plugin for LinuxDnsInUse {
    fn name(&self) -> &str {
        &"dns_in_use"
    }

    fn description(&self) -> &str {
        &"Get ip of DNS in use via nmcli"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "( nmcli dev list || nmcli dev show ) 2>/dev/null | grep DNS | awk '{print $2}'";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxDnsInUse {
    pub fn new() -> Self {
        LinuxDnsInUse {}
    }
}