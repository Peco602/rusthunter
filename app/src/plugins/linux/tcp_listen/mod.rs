use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxTCPListen {}

impl Plugin for LinuxTCPListen {
    fn name(&self) -> &str {
        &"linux_tcp_listen"
    }

    fn description(&self) -> &str {
        &"TCP listening ports"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "lsof -nP -iTCP -sTCP:LISTEN | grep -v COMMAND | tr -s ' ' |  cut -d ' ' -f 1,3,9 | sort";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_csv_string_no_header(output, &vec!["Process", "User", "Port"], &" ")
    }
}

impl LinuxTCPListen {
    pub fn new() -> Self {
        LinuxTCPListen {}
    }
}