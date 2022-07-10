use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxUnusualNetworkUsage {}

impl Plugin for LinuxUnusualNetworkUsage {
    fn name(&self) -> &str {
        &"linux_unusual_network_usage"
    }

    fn description(&self) -> &str {
        &"Look for promiscuous mode, which might indicate a sniffer"
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

impl LinuxUnusualNetworkUsage {
    pub fn new() -> Self {
        LinuxUnusualNetworkUsage {}
    }
}