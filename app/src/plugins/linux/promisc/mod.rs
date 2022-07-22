use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxPromisc {}

impl Plugin for LinuxPromisc {
    fn name(&self) -> &str {
        &"linux_promisc"
    }

    fn description(&self) -> &str {
        &"Network interfaces in promiscuous mode"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "ip link | grep PROMISC";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxPromisc {
    pub fn new() -> Self {
        LinuxPromisc {}
    }
}