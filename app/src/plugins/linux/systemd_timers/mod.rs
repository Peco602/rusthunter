use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxSystemdTimers {}

impl Plugin for LinuxSystemdTimers {
    fn name(&self) -> &str {
        "linux_systemd_timers"
    }

    fn description(&self) -> &str {
        "Active systemd timers (scheduled tasks)"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        // List all active systemd timers
        let command = "systemctl list-timers --all --no-pager --plain | awk 'NR>1 && $1 != \"NEXT\" {print $NF}' | sort -u";
        match self.execute_command(command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl Default for LinuxSystemdTimers {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxSystemdTimers {
    pub fn new() -> Self {
        LinuxSystemdTimers {}
    }
}
