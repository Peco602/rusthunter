use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxDns {}

impl Plugin for LinuxDns {
    fn name(&self) -> &str {
        &"linux_dns"
    }

    fn description(&self) -> &str {
        &"DNS in use"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "cat /etc/resolv.conf | grep nameserver | cut -d' ' -f2";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxDns {
    pub fn new() -> Self {
        LinuxDns {}
    }
}